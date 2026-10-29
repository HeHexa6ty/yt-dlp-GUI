use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use serde_json::Value;
use tokio::process::Command;
use tokio::time::timeout;

use crate::error::AppError;
use crate::models::{DirectoryTree, MediaInfo, MediaType};

/// Analysis should never hang the app; kill yt-dlp after this long.
const ANALYZE_TIMEOUT: Duration = Duration::from_secs(45);

/// Resolves a Tauri sidecar next to the running executable. During tests the
/// test binary lives in `target/debug/deps`, so walk up one directory just like
/// Tauri's shell plugin does.
fn sidecar_candidate(executable: &Path, name: &str) -> Option<PathBuf> {
    let executable_dir = executable.parent()?;
    let base_dir = if executable_dir.ends_with("deps") {
        executable_dir.parent().unwrap_or(executable_dir)
    } else {
        executable_dir
    };

    let mut path = base_dir.join(name);
    #[cfg(windows)]
    {
        let already_exe = path.extension().is_some_and(|extension| extension == "exe");
        if !already_exe {
            // Keep dots in names such as `yt-dlp-sidecar` intact.
            path.as_mut_os_string().push(".exe");
        }
    }
    #[cfg(not(windows))]
    {
        if path.extension().is_some_and(|extension| extension == "exe") {
            path.set_extension("");
        }
    }

    path.is_file().then_some(path)
}

fn bundled_sidecar(name: &str) -> Option<PathBuf> {
    let executable = std::env::current_exe().ok()?;
    sidecar_candidate(&executable, name)
}

/// Picks the first available tool path: explicit override, bundled sidecar,
/// then the system PATH fallback.
fn choose_tool_path(
    override_path: Option<PathBuf>,
    sidecar: Option<PathBuf>,
    fallback: &str,
) -> PathBuf {
    override_path
        .or(sidecar)
        .unwrap_or_else(|| PathBuf::from(fallback))
}

fn resolve_tool(env_var: &str, sidecar_name: &str, fallback: &str) -> PathBuf {
    choose_tool_path(
        std::env::var_os(env_var).map(PathBuf::from),
        bundled_sidecar(sidecar_name),
        fallback,
    )
}

/// Path to the yt-dlp binary: `YT_DLP_PATH`, the bundled sidecar, then PATH.
pub fn yt_dlp_path() -> PathBuf {
    resolve_tool("YT_DLP_PATH", "yt-dlp-sidecar", "yt-dlp")
}

/// Path to the FFmpeg binary: `FFMPEG_PATH`, the bundled sidecar, then PATH.
pub fn ffmpeg_path() -> PathBuf {
    resolve_tool("FFMPEG_PATH", "ffmpeg-sidecar", "ffmpeg")
}

/// Path to ffprobe: `FFPROBE_PATH`, the bundled sidecar, then PATH. It is only
/// used to fill in missing durations so the GIF length limit can still be
/// applied.
pub fn ffprobe_path() -> PathBuf {
    resolve_tool("FFPROBE_PATH", "ffprobe-sidecar", "ffprobe")
}

/// Builds the yt-dlp command. The bundled sidecar wins over a system install;
/// `YT_DLP_PATH` remains the escape hatch for debugging or a custom build.
pub(crate) fn ytdlp_command() -> Command {
    let mut command = Command::new(yt_dlp_path());
    // Never leave a child behind when the future is dropped or timed out.
    command.kill_on_drop(true);
    command
}

/// Builds the FFmpeg command used for GIF/image conversions.
pub(crate) fn ffmpeg_command() -> Command {
    let mut command = Command::new(ffmpeg_path());
    command.kill_on_drop(true);
    command
}

/// Picks a direct media URL that ffprobe can open. Some extractors (Instagram)
/// leave the top-level `url` empty and only expose per-format URLs.
fn probe_url(info: &Value) -> Option<String> {
    if let Some(url) = string_field(info, "url")
        && url.starts_with("http")
    {
        return Some(url);
    }

    info.get("formats")
        .and_then(Value::as_array)?
        .iter()
        .find_map(|format| {
            let url = string_field(format, "url")?;
            let ext = string_field(format, "ext").unwrap_or_default();
            if url.starts_with("http") && ext != "mhtml" {
                Some(url)
            } else {
                None
            }
        })
}

/// Reads the duration of a remote media URL with ffprobe. Instagram omits the
/// duration from its yt-dlp JSON, but the GIF limit still needs it. A missing
/// ffprobe or a failed probe is not fatal; analysis simply keeps duration unset.
async fn probe_duration(info: &Value) -> Option<f64> {
    let url = probe_url(info)?;
    let mut command = Command::new(ffprobe_path());
    command.kill_on_drop(true);
    command.args([
        "-v",
        "error",
        "-show_entries",
        "format=duration",
        "-of",
        "default=nw=1:nk=1",
    ]);

    if let Some(headers) = info.get("http_headers").and_then(Value::as_object) {
        let mut header_arg = String::new();
        for (name, value) in headers {
            if let Some(value) = value.as_str() {
                header_arg.push_str(name);
                header_arg.push_str(": ");
                header_arg.push_str(value);
                header_arg.push_str("\r\n");
            }
        }
        if !header_arg.is_empty() {
            command.arg("-headers").arg(header_arg);
        }
    }

    command
        .arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());

    let output = timeout(Duration::from_secs(10), command.output())
        .await
        .ok()?
        .ok()?;
    if !output.status.success() {
        return None;
    }

    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<f64>()
        .ok()
}

/// Runs `yt-dlp --dump-single-json` and converts the result into `MediaInfo`.
pub async fn analyze(url: &str) -> Result<MediaInfo, AppError> {
    let mut command = ytdlp_command();
    command
        .args([
            "--dump-single-json",
            "--no-playlist",
            "--no-warnings",
            "--skip-download",
            // Photo posts have no downloadable format but still expose image
            // thumbnails, so treat "no formats" as success and detect them as
            // images instead of failing the whole analysis.
            "--ignore-no-formats-error",
            url,
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let output = timeout(ANALYZE_TIMEOUT, command.output())
        .await
        .map_err(|_| {
            AppError::new(
                "timeout",
                "Analysis took too long. Check your connection and try again.",
            )
        })?
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                AppError::new(
                    "yt-dlp-missing",
                    "yt-dlp was not found. Reinstall the app or set YT_DLP_PATH.",
                )
            } else {
                AppError::new("yt-dlp-error", format!("Could not run yt-dlp: {error}"))
            }
        })?;

    if !output.status.success() {
        return Err(map_ytdlp_error(&String::from_utf8_lossy(&output.stderr)));
    }

    let json: Value = serde_json::from_slice(&output.stdout).map_err(|error| {
        AppError::new(
            "invalid-output",
            format!("yt-dlp returned unreadable JSON: {error}"),
        )
    })?;

    let mut info = parse_info(&json, url)?;
    if info.duration.is_none() && info.media_type == MediaType::Video {
        info.duration = probe_duration(&json).await;
    }

    Ok(info)
}

/// Shown when a site only serves a post to a logged-in session (for example an
/// age check on Instagram).
pub(crate) const ACCOUNT_REQUIRED_MESSAGE: &str =
    "This content may be age-restricted or private and requires a logged-in account.";

/// Detects the stderr messages sites use when an authenticated session is
/// required. Instagram reports an age-gated post as an "empty media response"
/// and points at `--cookies-from-browser`.
pub(crate) fn requires_account(message: &str) -> bool {
    let lower = message.to_lowercase();

    lower.contains("empty media response")
        || lower.contains("cookies-from-browser")
        || lower.contains("use --cookies")
        || lower.contains("login required")
        || lower.contains("login_required")
        || lower.contains("requires authentication")
        || lower.contains("authentication required")
}

/// Turns yt-dlp's stderr into a message the UI can display.
fn map_ytdlp_error(stderr: &str) -> AppError {
    let line = stderr
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("yt-dlp could not analyze this URL.");
    let message = line
        .trim()
        .strip_prefix("ERROR: ")
        .unwrap_or(line.trim())
        .to_string();
    let lower = message.to_lowercase();

    if requires_account(&message) {
        return AppError::new("login-required", ACCOUNT_REQUIRED_MESSAGE);
    }

    let code = if lower.contains("unsupported url") {
        "unsupported-url"
    } else if lower.contains("private") {
        "private-media"
    } else if lower.contains("sign in") || lower.contains("login") {
        "login-required"
    } else {
        "analyze-failed"
    };

    AppError::new(code, message)
}

/// Reads a string field, ignoring JSON nulls.
fn string_field(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|text| !text.is_empty())
        .map(str::to_string)
}

/// A codec is only meaningful when yt-dlp reports something other than "none".
fn codec(info: &Value, key: &str) -> Option<String> {
    string_field(info, key).filter(|value| value != "none")
}

/// Prefers the full source image for image posts, otherwise the best thumbnail.
fn pick_thumbnail(info: &Value, media_type: MediaType) -> Option<String> {
    let candidate = if media_type == MediaType::Image {
        string_field(info, "url")
    } else {
        None
    }
    .or_else(|| string_field(info, "thumbnail"))
    .or_else(|| {
        info.get("thumbnails")
            .and_then(Value::as_array)
            .and_then(|list| list.last())
            .and_then(|thumbnail| string_field(thumbnail, "url"))
    })?;

    // Some extractors return protocol-relative URLs.
    Some(if candidate.starts_with("//") {
        format!("https:{candidate}")
    } else {
        candidate
    })
}

/// X serves animated GIFs as silent MP4s from `video.twimg.com/tweet_video/`.
/// yt-dlp reports them without codecs or duration, so without this check they
/// would fall through to the image fallback.
fn is_x_animated_gif(info: &Value) -> bool {
    string_field(info, "extractor")
        .is_some_and(|extractor| extractor.eq_ignore_ascii_case("twitter"))
        && string_field(info, "url").is_some_and(|url| url.contains("/tweet_video/"))
}

/// Decides the source media type from codecs/extensions in the yt-dlp payload.
fn detect_media_type(info: &Value) -> MediaType {
    let ext = string_field(info, "ext").unwrap_or_default().to_lowercase();

    // Animated images are reported as GIF sources.
    if ext == "gif" {
        return MediaType::Gif;
    }

    // X animated GIFs are MP4 containers but are still GIF sources.
    if is_x_animated_gif(info) {
        return MediaType::Gif;
    }

    if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "webp" | "avif") {
        return MediaType::Image;
    }

    let formats = info.get("formats").and_then(Value::as_array);
    let has_video = codec(info, "vcodec").is_some()
        || formats.is_some_and(|list| list.iter().any(|f| codec(f, "vcodec").is_some()));
    if has_video {
        return MediaType::Video;
    }

    let has_audio = codec(info, "acodec").is_some()
        || formats.is_some_and(|list| list.iter().any(|f| codec(f, "acodec").is_some()));
    if has_audio {
        return MediaType::Audio;
    }

    // No codecs and no recognized extension: image posts end up here.
    MediaType::Image
}

/// Maps the yt-dlp JSON into our own shape. Playlists fall back to the first
/// entry so the preview always describes a single media item.
fn parse_info(root: &Value, original_url: &str) -> Result<MediaInfo, AppError> {
    let info = root
        .get("entries")
        .and_then(Value::as_array)
        .and_then(|entries| entries.first())
        .unwrap_or(root);

    let media_type = detect_media_type(info);

    Ok(MediaInfo {
        id: string_field(info, "id").unwrap_or_else(|| "unknown".to_string()),
        title: string_field(info, "title").unwrap_or_else(|| "Untitled media".to_string()),
        uploader: string_field(info, "uploader")
            .or_else(|| string_field(info, "channel"))
            .or_else(|| string_field(info, "uploader_id")),
        thumbnail: pick_thumbnail(info, media_type),
        duration: info.get("duration").and_then(Value::as_f64),
        webpage_url: string_field(info, "webpage_url").unwrap_or_else(|| original_url.to_string()),
        media_type,
        // Filled in by the command once the platform folder is known.
        platform: String::new(),
        default_directory: String::new(),
        directory_tree: DirectoryTree {
            root: String::new(),
            platform: String::new(),
            folders: Vec::new(),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INSTAGRAM_AGE_GATE: &str = "ERROR: [Instagram] DecrKlojBOb: Instagram sent an empty media response. Check if this post is accessible in your browser without being logged-in. If it is not, then use --cookies-from-browser or --cookies for the authentication.";

    #[test]
    fn maps_an_instagram_age_gate_to_a_clear_message() {
        let error = map_ytdlp_error(INSTAGRAM_AGE_GATE);

        assert_eq!(error.code, "login-required");
        assert_eq!(error.message, ACCOUNT_REQUIRED_MESSAGE);
    }

    #[test]
    fn maps_a_login_required_response_to_a_clear_message() {
        let error = map_ytdlp_error(r#"ERROR: {"message":"login_required"}"#);

        assert_eq!(error.code, "login-required");
        assert_eq!(error.message, ACCOUNT_REQUIRED_MESSAGE);
    }

    #[test]
    fn keeps_unrelated_errors_unchanged() {
        let error = map_ytdlp_error("ERROR: Unsupported URL: https://example.com");

        assert_eq!(error.code, "unsupported-url");
        assert_eq!(error.message, "Unsupported URL: https://example.com");
    }

    #[test]
    fn picks_a_direct_probe_url() {
        let info = serde_json::json!({
            "formats": [
                {"url": "https://example.com/audio.m4a", "ext": "m4a"},
                {"url": "https://example.com/video.mp4", "ext": "mp4"}
            ]
        });
        assert_eq!(
            probe_url(&info).as_deref(),
            Some("https://example.com/audio.m4a")
        );

        let with_top_level = serde_json::json!({
            "url": "https://example.com/top.mp4",
            "formats": [{"url": "https://example.com/format.mp4", "ext": "mp4"}]
        });
        assert_eq!(
            probe_url(&with_top_level).as_deref(),
            Some("https://example.com/top.mp4")
        );

        let storyboard_only = serde_json::json!({
            "formats": [{"url": "https://example.com/storyboard.mhtml", "ext": "mhtml"}]
        });
        assert_eq!(probe_url(&storyboard_only), None);
    }

    #[test]
    fn detects_x_animated_gif_as_gif() {
        let info = serde_json::json!({
            "extractor": "twitter",
            "ext": "mp4",
            "url": "https://video.twimg.com/tweet_video/HVaAzjfaYAAe8x0.mp4",
            "formats": [{
                "format_id": "http",
                "ext": "mp4",
                "vcodec": null,
                "acodec": null,
                "url": "https://video.twimg.com/tweet_video/HVaAzjfaYAAe8x0.mp4"
            }]
        });

        assert_eq!(detect_media_type(&info), MediaType::Gif);
    }

    #[test]
    fn keeps_x_photo_posts_detected_as_images() {
        let info = serde_json::json!({
            "extractor": "twitter",
            "ext": null,
            "formats": [],
            "thumbnails": [{
                "url": "https://pbs.twimg.com/media/AAA.jpg"
            }]
        });

        assert_eq!(detect_media_type(&info), MediaType::Image);
    }

    fn sidecar_file_name(name: &str) -> String {
        #[cfg(windows)]
        {
            format!("{name}.exe")
        }
        #[cfg(not(windows))]
        {
            name.to_string()
        }
    }

    fn sidecar_test_dir() -> PathBuf {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);

        let unique = NEXT.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "yt-dlp-gui-sidecar-test-{}-{unique}",
            std::process::id()
        ))
    }

    #[test]
    fn resolves_a_sidecar_next_to_the_executable_and_out_of_deps() {
        let dir = sidecar_test_dir();
        let deps = dir.join("deps");
        std::fs::create_dir_all(&deps).expect("temp deps directory should be created");
        let sidecar = dir.join(sidecar_file_name("ffmpeg-sidecar"));
        std::fs::write(&sidecar, b"x").expect("sidecar file should be written");

        assert_eq!(
            sidecar_candidate(&dir.join("app"), "ffmpeg-sidecar"),
            Some(sidecar.clone())
        );
        assert_eq!(
            sidecar_candidate(&deps.join("app-test"), "ffmpeg-sidecar"),
            Some(sidecar)
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn prefers_an_override_then_the_sidecar_then_path() {
        let override_path = PathBuf::from("/custom/yt-dlp");
        let sidecar = PathBuf::from("/bundle/yt-dlp-sidecar");

        assert_eq!(
            choose_tool_path(Some(override_path.clone()), Some(sidecar.clone()), "yt-dlp"),
            override_path
        );
        assert_eq!(
            choose_tool_path(None, Some(sidecar.clone()), "yt-dlp"),
            sidecar
        );
        assert_eq!(
            choose_tool_path(None, None, "yt-dlp"),
            PathBuf::from("yt-dlp")
        );
    }

    #[test]
    fn bundled_sidecars_are_copied_next_to_the_test_binary() {
        // tauri-build copies configured sidecars to target/debug before tests.
        // If they are absent, the resolver falls back to PATH, so only assert
        // when the packaged sidecars are present.
        if bundled_sidecar("yt-dlp-sidecar").is_none() {
            return;
        }

        assert!(bundled_sidecar("ffmpeg-sidecar").is_some());
        assert!(bundled_sidecar("ffprobe-sidecar").is_some());
    }
}
