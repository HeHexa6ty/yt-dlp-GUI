//! Ignored integration harness for `Tests/TestSet.md`.
//!
//! Run it explicitly with:
//!
//! ```text
//! cargo test --lib verify_test_set -- --ignored --nocapture
//! ```
//!
//! The harness exercises the real `analyze_url` and `download` paths through a
//! mock Tauri runtime. It writes a Markdown report and a JSON report into
//! `Tests/Results/`, and keeps the produced files under
//! `Tests/Results/media/<run-id>/` for inspection.

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use tauri::test::mock_app;

use crate::commands::analyze_url;
use crate::download::download;
use crate::models::{
    ConflictMode, DirectoryMode, DirectoryRequest, DownloadRequest, GifQuality, MediaType,
    OutputFormat,
};
use crate::ytdlp::{ffmpeg_path, yt_dlp_path};

/// One row from `TestSet.md`: a method applied to one or more URLs.
#[derive(Debug, Clone)]
struct TestCase {
    platform: String,
    method: String,
    output: OutputFormat,
    expected_source: MediaType,
    expect_multiple: bool,
    urls: Vec<String>,
}

/// One URL's result, including the analysis and download phases.
#[derive(Debug, Serialize)]
struct CaseResult {
    index: usize,
    platform: String,
    method: String,
    url: String,
    expected_source: String,
    expected_format: String,
    expect_multiple: bool,
    ui_available: bool,
    analysis: AnalysisRecord,
    download: DownloadRecord,
    backend_passed: bool,
    passed: bool,
    /// Set when the requested output is impossible for the source itself.
    not_applicable: Option<String>,
}

#[derive(Debug, Serialize)]
struct AnalysisRecord {
    passed: bool,
    media_type: Option<String>,
    title: Option<String>,
    id: Option<String>,
    duration: Option<f64>,
    thumbnail: Option<String>,
    default_directory: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct DownloadRecord {
    passed: bool,
    expected_count: String,
    actual_count: usize,
    files: Vec<FileRecord>,
    error: Option<String>,
    error_code: Option<String>,
}

#[derive(Debug, Serialize)]
struct FileRecord {
    path: String,
    size: u64,
    magic: String,
    magic_ok: bool,
    extension_ok: bool,
}

#[derive(Debug, Serialize)]
struct RunReport {
    run_id: String,
    started_epoch_seconds: u64,
    finished_epoch_seconds: u64,
    test_set: String,
    results_dir: String,
    yt_dlp_version: String,
    ffmpeg_version: String,
    total: usize,
    backend_passed: usize,
    fully_passed: usize,
    ui_only: usize,
    not_applicable: usize,
    failed: usize,
    cases: Vec<CaseResult>,
}

#[test]
fn parses_the_test_set() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_set_path = manifest_dir
        .parent()
        .expect("src-tauri should have a parent directory")
        .join("Tests")
        .join("TestSet.md");
    let cases = parse_test_set(&test_set_path).expect("TestSet.md should parse");

    assert_eq!(cases.len(), 23);
    assert_eq!(cases.iter().map(|case| case.urls.len()).sum::<usize>(), 33);
    assert!(cases.iter().any(|case| {
        case.platform == "YouTube"
            && case.method == "Video as video"
            && case.output == OutputFormat::Mp4
    }));
    assert!(cases.iter().any(|case| {
        case.platform == "Instagram"
            && case.method == "Video as gif"
            && case.output == OutputFormat::Gif
            && case.urls.len() == 2
    }));
    assert!(cases.iter().any(|case| {
        case.platform == "X"
            && case.method == "GIF as a post"
            && case.expected_source == MediaType::Gif
    }));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "network-heavy TestSet verification; run explicitly"]
async fn verify_test_set() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let project_dir = manifest_dir
        .parent()
        .expect("src-tauri should have a parent directory");
    let test_set_path = project_dir.join("Tests").join("TestSet.md");
    let results_dir = project_dir.join("Tests").join("Results");

    let started = now_epoch_seconds();
    let run_id = format!("run-{started}");
    let media_root = results_dir.join("media").join(&run_id);
    std::fs::create_dir_all(&media_root).expect("results media folder should be created");

    let cases = parse_test_set(&test_set_path).expect("TestSet.md should parse");
    assert!(
        !cases.is_empty(),
        "TestSet.md should contain at least one case"
    );

    let app = mock_app();
    let mut results = Vec::new();
    let mut index = 0;

    for case in &cases {
        for (url_index, url) in case.urls.iter().enumerate() {
            index += 1;
            let case_dir = media_root
                .join(slug(&case.platform))
                .join(slug(&case.method))
                .join(format!("url-{}", url_index + 1));
            std::fs::create_dir_all(&case_dir).expect("case folder should be created");

            let analysis = analyze_url(url.clone()).await;
            let mut analysis_record = AnalysisRecord {
                passed: false,
                media_type: None,
                title: None,
                id: None,
                duration: None,
                thumbnail: None,
                default_directory: None,
                error: None,
            };
            let mut download_record = DownloadRecord {
                passed: false,
                expected_count: if case.expect_multiple {
                    ">= 2".to_string()
                } else {
                    "1".to_string()
                },
                actual_count: 0,
                files: Vec::new(),
                error: None,
                error_code: None,
            };

            let mut backend_passed = false;
            let mut ui_available = false;
            let mut not_applicable = None;

            match analysis {
                Ok(info) => {
                    let source_ok = info.media_type == case.expected_source;
                    ui_available = format_is_available(
                        info.media_type,
                        info.duration,
                        &info.platform,
                        case.output,
                    );
                    analysis_record = AnalysisRecord {
                        passed: source_ok,
                        media_type: Some(media_type_name(info.media_type).to_string()),
                        title: Some(info.title.clone()),
                        id: Some(info.id.clone()),
                        duration: info.duration,
                        thumbnail: info.thumbnail.clone(),
                        default_directory: Some(info.default_directory.clone()),
                        error: None,
                    };

                    if source_ok {
                        let request = DownloadRequest {
                            url: url.clone(),
                            title: info.title.clone(),
                            id: info.id.clone(),
                            media_type: info.media_type,
                            format: case.output,
                            gif_quality: Some(GifQuality::Better),
                            conflict: Some(ConflictMode::Rename),
                            directory: DirectoryRequest {
                                mode: DirectoryMode::Custom,
                                path: Some(case_dir.to_string_lossy().to_string()),
                            },
                        };

                        match download(app.handle(), request).await {
                            Ok(result) => {
                                download_record = verify_download(
                                    &result.file_paths,
                                    case.output,
                                    case.expect_multiple,
                                );
                                backend_passed = analysis_record.passed && download_record.passed;
                            }
                            Err(error) => {
                                download_record.error_code = Some(error.code.to_string());
                                if error.code == "no-audio" {
                                    not_applicable = Some("Source has no audio track".to_string());
                                }
                                download_record.error = Some(error.to_string());
                            }
                        }
                    }
                }
                Err(error) => {
                    analysis_record.error = Some(error.to_string());
                }
            }

            let passed = not_applicable.is_none() && backend_passed && ui_available;
            results.push(CaseResult {
                index,
                platform: case.platform.clone(),
                method: case.method.clone(),
                url: url.clone(),
                expected_source: media_type_name(case.expected_source).to_string(),
                expected_format: format_name(case.output).to_string(),
                expect_multiple: case.expect_multiple,
                ui_available,
                analysis: analysis_record,
                download: download_record,
                backend_passed,
                passed,
                not_applicable: not_applicable.clone(),
            });

            println!(
                "[{index}] {} / {} / {}: {}",
                case.platform,
                case.method,
                url,
                if let Some(reason) = not_applicable {
                    format!("N/A ({reason})")
                } else if passed {
                    "PASS".to_string()
                } else if backend_passed {
                    "PASS (backend only; format not offered by UI)".to_string()
                } else {
                    "FAIL".to_string()
                }
            );
        }
    }

    let finished = now_epoch_seconds();
    let backend_passed = results
        .iter()
        .filter(|result| result.backend_passed)
        .count();
    let fully_passed = results.iter().filter(|result| result.passed).count();
    let ui_only = results
        .iter()
        .filter(|result| result.backend_passed && !result.ui_available)
        .count();
    let not_applicable = results
        .iter()
        .filter(|result| result.not_applicable.is_some())
        .count();
    let failed = results
        .iter()
        .filter(|result| !result.backend_passed && result.not_applicable.is_none())
        .count();

    let report = RunReport {
        run_id: run_id.clone(),
        started_epoch_seconds: started,
        finished_epoch_seconds: finished,
        test_set: relative_path(project_dir, &test_set_path),
        results_dir: relative_path(project_dir, &results_dir),
        yt_dlp_version: command_version(&yt_dlp_path(), &["--version"]),
        ffmpeg_version: command_version(&ffmpeg_path(), &["-version"]),
        total: results.len(),
        backend_passed,
        fully_passed,
        ui_only,
        not_applicable,
        failed,
        cases: results,
    };

    write_reports(&results_dir, &report);
    println!(
        "TestSet: {} total, {} backend pass, {} fully pass, {} UI-only, {} N/A, {} fail. Reports in {}",
        report.total,
        report.backend_passed,
        report.fully_passed,
        report.ui_only,
        report.not_applicable,
        report.failed,
        results_dir.display()
    );

    assert_eq!(
        report.failed,
        0,
        "{} TestSet cases failed; see {}",
        report.failed,
        results_dir.join("report.md").display()
    );
}

fn parse_test_set(path: &Path) -> Result<Vec<TestCase>, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|error| format!("Could not read {}: {error}", path.display()))?;
    let mut platform = String::new();
    let mut current_case: Option<usize> = None;
    let mut cases: Vec<TestCase> = Vec::new();

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(header) = line.strip_prefix('#') {
            platform = normalize_platform(header).unwrap_or_default();
            current_case = None;
            continue;
        }

        if line.starts_with('-') || line.starts_with('*') {
            if let (Some(index), Some(url)) = (current_case, extract_url(line)) {
                cases[index].urls.push(url);
            }
            continue;
        }

        let lower = line.to_lowercase();
        let is_method_line = lower.contains(" as ") || lower.starts_with("gif ");
        if !is_method_line {
            continue;
        }

        let method = method_label(line);
        if method.is_empty() {
            continue;
        }
        let explicit_format = parse_extension(line);
        let Some(output) = infer_output(&method, explicit_format) else {
            continue;
        };
        let Some(expected_source) = infer_source(&platform, &method) else {
            continue;
        };

        cases.push(TestCase {
            platform: platform.clone(),
            method: method.clone(),
            output,
            expected_source,
            expect_multiple: method.to_lowercase().contains("carousel"),
            urls: Vec::new(),
        });
        current_case = Some(cases.len() - 1);

        if let Some(url) = extract_url(line)
            && let Some(index) = current_case
        {
            cases[index].urls.push(url);
        }
    }

    cases.retain(|case| !case.platform.is_empty() && !case.urls.is_empty());
    Ok(cases)
}

fn normalize_platform(header: &str) -> Option<String> {
    let name = header.trim().trim_end_matches(':').trim().to_lowercase();
    if name.starts_with("youtube") {
        Some("YouTube".to_string())
    } else if name.starts_with("instagram") {
        Some("Instagram".to_string())
    } else if name == "x" || name.starts_with("x ") {
        Some("X".to_string())
    } else if name.starts_with("tiktok") {
        Some("TikTok".to_string())
    } else {
        None
    }
}

fn method_label(line: &str) -> String {
    let without_url = line
        .split("https://")
        .next()
        .unwrap_or(line)
        .split("http://")
        .next()
        .unwrap_or(line);
    let without_extension = without_url.split("(.").next().unwrap_or(without_url);

    without_extension
        .trim()
        .trim_end_matches(':')
        .trim()
        .to_string()
}

fn parse_extension(line: &str) -> Option<OutputFormat> {
    let start = line.find("(.")? + 2;
    let rest = &line[start..];
    let end = rest.find(')')?;
    match rest[..end].to_ascii_lowercase().as_str() {
        "mp4" => Some(OutputFormat::Mp4),
        "mp3" => Some(OutputFormat::Mp3),
        "gif" => Some(OutputFormat::Gif),
        "png" => Some(OutputFormat::Png),
        "jpg" | "jpeg" => Some(OutputFormat::Jpg),
        _ => None,
    }
}

fn infer_output(method: &str, explicit: Option<OutputFormat>) -> Option<OutputFormat> {
    if explicit.is_some() {
        return explicit;
    }

    let lower = method.to_lowercase();
    if lower.contains("audio") {
        Some(OutputFormat::Mp3)
    } else if lower.contains("video as video") {
        Some(OutputFormat::Mp4)
    } else if lower.contains("gif") {
        Some(OutputFormat::Gif)
    } else {
        None
    }
}

fn infer_source(platform: &str, method: &str) -> Option<MediaType> {
    let lower = method.to_lowercase();
    if lower.contains("gif in post") || lower.contains("gif as a post") {
        Some(MediaType::Gif)
    } else if lower.contains("carousel") || lower.contains("image") {
        Some(MediaType::Image)
    } else if lower.contains("video") || platform == "YouTube" || platform == "TikTok" {
        Some(MediaType::Video)
    } else {
        None
    }
}

fn extract_url(line: &str) -> Option<String> {
    let start = line.find("https://").or_else(|| line.find("http://"))?;
    let rest = &line[start..];
    let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
    let url = rest[..end].trim_end_matches(['.', ',', ';']).to_string();

    if url.starts_with("http://") || url.starts_with("https://") {
        Some(url)
    } else {
        None
    }
}

fn verify_download(
    paths: &[String],
    format: OutputFormat,
    expect_multiple: bool,
) -> DownloadRecord {
    let expected_count = if expect_multiple { ">= 2" } else { "1" };
    let count_ok = if expect_multiple {
        paths.len() >= 2
    } else {
        paths.len() == 1
    };

    let mut files = Vec::new();
    let mut files_ok = count_ok;
    for (index, path) in paths.iter().enumerate() {
        let path = PathBuf::from(path);
        let metadata = std::fs::metadata(&path);
        let size = metadata
            .as_ref()
            .map(|metadata| metadata.len())
            .unwrap_or(0);
        let magic = detect_magic(&path);
        let magic_ok = magic == expected_magic(format);
        let extension_ok = path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case(format.extension()));
        let numbering_ok = !expect_multiple
            || path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .is_some_and(|stem| stem.ends_with(&format!("_{}", index + 1)));

        files_ok &= size > 0 && magic_ok && extension_ok && numbering_ok;
        files.push(FileRecord {
            path: path.to_string_lossy().to_string(),
            size,
            magic,
            magic_ok,
            extension_ok,
        });
    }

    DownloadRecord {
        passed: count_ok && files_ok,
        expected_count: expected_count.to_string(),
        actual_count: paths.len(),
        files,
        error: None,
        error_code: None,
    }
}

fn detect_magic(path: &Path) -> String {
    let Ok(bytes) = std::fs::read(path) else {
        return "unreadable".to_string();
    };

    if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        "GIF".to_string()
    } else if bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]) {
        "PNG".to_string()
    } else if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        "JPG".to_string()
    } else if bytes.len() > 12 && &bytes[4..8] == b"ftyp" {
        "MP4".to_string()
    } else if bytes.starts_with(b"ID3")
        || (bytes.len() > 2 && bytes[0] == 0xFF && (bytes[1] & 0xE0) == 0xE0)
    {
        "MP3".to_string()
    } else {
        "unknown".to_string()
    }
}

fn expected_magic(format: OutputFormat) -> &'static str {
    match format {
        OutputFormat::Mp4 => "MP4",
        OutputFormat::Mp3 => "MP3",
        OutputFormat::Gif => "GIF",
        OutputFormat::Png => "PNG",
        OutputFormat::Jpg => "JPG",
    }
}

/// Mirrors the frontend's `getAvailableFormats` / `isGifAllowed` rules.
fn format_is_available(
    media_type: MediaType,
    duration: Option<f64>,
    platform: &str,
    format: OutputFormat,
) -> bool {
    let gif_allowed = platform != "YouTube"
        && (media_type != MediaType::Video || duration.is_some_and(|duration| duration <= 60.0));

    match media_type {
        MediaType::Video => match format {
            OutputFormat::Mp4 | OutputFormat::Mp3 => true,
            OutputFormat::Gif => gif_allowed,
            OutputFormat::Png | OutputFormat::Jpg => false,
        },
        MediaType::Audio => format == OutputFormat::Mp3,
        MediaType::Gif => format == OutputFormat::Gif,
        MediaType::Image => matches!(
            format,
            OutputFormat::Png | OutputFormat::Jpg | OutputFormat::Gif
        ),
    }
}

fn media_type_name(media_type: MediaType) -> &'static str {
    match media_type {
        MediaType::Video => "video",
        MediaType::Audio => "audio",
        MediaType::Gif => "gif",
        MediaType::Image => "image",
    }
}

fn format_name(format: OutputFormat) -> &'static str {
    match format {
        OutputFormat::Mp4 => "mp4",
        OutputFormat::Mp3 => "mp3",
        OutputFormat::Gif => "gif",
        OutputFormat::Png => "png",
        OutputFormat::Jpg => "jpg",
    }
}

fn slug(value: &str) -> String {
    let slug: String = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect();

    slug.split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn relative_path(project_dir: &Path, path: &Path) -> String {
    path.strip_prefix(project_dir)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
}

fn now_epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn command_version(binary: &Path, args: &[&str]) -> String {
    std::process::Command::new(binary)
        .args(args)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .next()
                .unwrap_or_default()
                .trim()
                .to_string()
        })
        .unwrap_or_else(|| "unavailable".to_string())
}

fn write_reports(results_dir: &Path, report: &RunReport) {
    std::fs::create_dir_all(results_dir).expect("results folder should be created");

    let json = serde_json::to_string_pretty(report).expect("report should serialize");
    std::fs::write(results_dir.join("results.json"), json).expect("JSON report should be written");

    let mut markdown = String::new();
    markdown.push_str("# TestSet verification\n\n");
    markdown.push_str(&format!("- Run: `{}`\n", report.run_id));
    markdown.push_str(&format!(
        "- Started (epoch seconds): `{}`\n",
        report.started_epoch_seconds
    ));
    markdown.push_str(&format!(
        "- Finished (epoch seconds): `{}`\n",
        report.finished_epoch_seconds
    ));
    markdown.push_str(&format!("- Test set: `{}`\n", report.test_set));
    markdown.push_str(&format!("- Results: `{}`\n", report.results_dir));
    markdown.push_str(&format!("- yt-dlp: `{}`\n", report.yt_dlp_version));
    markdown.push_str(&format!("- FFmpeg: `{}`\n", report.ffmpeg_version));
    markdown.push_str(&format!(
        "- Cases: **{}** total, **{}** backend pass, **{}** fully pass, **{}** UI-only, **{}** N/A, **{}** fail\n\n",
        report.total,
        report.backend_passed,
        report.fully_passed,
        report.ui_only,
        report.not_applicable,
        report.failed
    ));

    markdown.push_str("## Summary\n\n");
    markdown.push_str("| # | Platform | Method | URL | Analysis | Download | UI | Result |\n");
    markdown.push_str("|---|----------|--------|-----|----------|----------|----|--------|\n");
    for case in &report.cases {
        let analysis = if case.analysis.passed { "PASS" } else { "FAIL" };
        let download = if case.download.passed { "PASS" } else { "FAIL" };
        let ui = if case.ui_available { "yes" } else { "no" };
        let result = if let Some(reason) = &case.not_applicable {
            format!("N/A ({reason})")
        } else if case.passed {
            "PASS".to_string()
        } else if case.backend_passed {
            "PASS (backend only)".to_string()
        } else {
            "FAIL".to_string()
        };
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            case.index,
            md_escape(&case.platform),
            md_escape(&case.method),
            md_escape(&case.url),
            analysis,
            download,
            ui,
            result
        ));
    }

    markdown.push_str("\n## Details\n\n");
    for case in &report.cases {
        markdown.push_str(&format!(
            "### {}. {} — {}\n\n",
            case.index, case.platform, case.method
        ));
        markdown.push_str(&format!("- URL: `{}`\n", case.url));
        markdown.push_str(&format!(
            "- Expected source / format: `{}` / `{}`\n",
            case.expected_source, case.expected_format
        ));
        markdown.push_str(&format!(
            "- Actual source: `{}`\n",
            case.analysis
                .media_type
                .as_deref()
                .unwrap_or("analysis failed")
        ));
        if let Some(title) = &case.analysis.title {
            markdown.push_str(&format!("- Title: {}\n", md_escape(title)));
        }
        if let Some(id) = &case.analysis.id {
            markdown.push_str(&format!("- Id: `{}`\n", id));
        }
        if let Some(duration) = case.analysis.duration {
            markdown.push_str(&format!("- Duration: `{duration}` seconds\n"));
        }
        if let Some(error) = &case.analysis.error {
            markdown.push_str(&format!("- Analysis error: `{}`\n", md_escape(error)));
        }
        if let Some(code) = &case.download.error_code {
            markdown.push_str(&format!("- Download error code: `{code}`\n"));
        }
        if let Some(error) = &case.download.error {
            markdown.push_str(&format!("- Download error: `{}`\n", md_escape(error)));
        }
        if let Some(reason) = &case.not_applicable {
            markdown.push_str(&format!("- Not applicable: {reason}\n"));
        }
        markdown.push_str(&format!(
            "- Expected file count: `{}`; actual: `{}`\n",
            case.download.expected_count, case.download.actual_count
        ));
        if case.download.files.is_empty() {
            markdown.push_str("- Files: none\n");
        } else {
            markdown.push_str("- Files:\n");
            for file in &case.download.files {
                markdown.push_str(&format!(
                    "  - `{}` — {} bytes — magic `{}` ({}) — extension {}\n",
                    file.path,
                    file.size,
                    file.magic,
                    if file.magic_ok { "OK" } else { "MISMATCH" },
                    if file.extension_ok { "OK" } else { "MISMATCH" }
                ));
            }
        }
        markdown.push_str(&format!(
            "- Backend: {}; UI format offered: {}\n\n",
            if case.backend_passed { "PASS" } else { "FAIL" },
            if case.ui_available { "yes" } else { "no" }
        ));
    }

    std::fs::write(results_dir.join("report.md"), markdown)
        .expect("Markdown report should be written");
}

fn md_escape(value: &str) -> String {
    value.replace('|', "\\|")
}
