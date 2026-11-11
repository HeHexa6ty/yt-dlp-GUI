//! Locating and installing FFmpeg.
//!
//! FFmpeg is not bundled with the application. It is looked up in this order:
//! the `FFMPEG_PATH` environment variable, a path saved from the FFmpeg tab,
//! `PATH`, and a short list of common installation directories. When it is
//! missing, the FFmpeg tab can install it through the platform package manager
//! or point the app at an existing binary.

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tokio::time::timeout;

use crate::error::AppError;
use crate::models::{FfmpegInstaller, FfmpegStatus};

/// Shown whenever FFmpeg cannot be spawned.
pub const MISSING_MESSAGE: &str =
    "FFmpeg was not found. Open the FFmpeg tab to install it or choose an existing binary.";

const CONFIG_DIR: &str = "yt-dlp-gui";
const CONFIG_FILE: &str = "ffmpeg.json";
const PROBE_TIMEOUT: Duration = Duration::from_secs(10);
const INSTALL_TIMEOUT: Duration = Duration::from_secs(15 * 60);
const DOWNLOAD_URL: &str = "https://ffmpeg.org/download.html";

/// Saved user preference. Kept small so it can be written synchronously.
#[derive(Debug, Default, Serialize, Deserialize)]
struct SavedConfig {
    ffmpeg_path: Option<String>,
}

fn config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join(CONFIG_DIR).join(CONFIG_FILE))
}

fn read_config() -> SavedConfig {
    config_path()
        .and_then(|path| std::fs::read_to_string(path).ok())
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

fn write_config(config: &SavedConfig) -> Result<(), AppError> {
    let path = config_path().ok_or_else(|| {
        AppError::new(
            "config-error",
            "Could not locate the user configuration directory.",
        )
    })?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| {
            AppError::new(
                "config-error",
                format!("Could not create {}: {error}", parent.display()),
            )
        })?;
    }

    let json = serde_json::to_string_pretty(config).map_err(|error| {
        AppError::new(
            "config-error",
            format!("Could not encode settings: {error}"),
        )
    })?;

    std::fs::write(&path, json).map_err(|error| {
        AppError::new(
            "config-error",
            format!(
                "Could not save FFmpeg settings to {}: {error}",
                path.display()
            ),
        )
    })
}

fn save_ffmpeg_path(path: &Path) -> Result<(), AppError> {
    write_config(&SavedConfig {
        ffmpeg_path: Some(path.to_string_lossy().to_string()),
    })
}

fn clear_saved_path() -> Result<(), AppError> {
    let Some(path) = config_path() else {
        return Ok(());
    };

    match std::fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(AppError::new(
            "config-error",
            format!("Could not clear FFmpeg settings: {error}"),
        )),
    }
}

/// Path saved from the FFmpeg tab, when the file still exists.
pub fn saved_ffmpeg_path() -> Option<PathBuf> {
    let path = PathBuf::from(read_config().ffmpeg_path?);
    path.is_file().then_some(path)
}

/// Platform-specific executable names for a bare command such as `ffmpeg`.
fn executable_names(base: &str) -> Vec<String> {
    #[cfg(windows)]
    {
        // `PATHEXT` is the authoritative list, but these three cover winget,
        // chocolatey, and normal installs without touching the environment.
        vec![
            format!("{base}.exe"),
            format!("{base}.cmd"),
            format!("{base}.bat"),
        ]
    }
    #[cfg(not(windows))]
    {
        vec![base.to_string()]
    }
}

/// Looks for an executable on `PATH` without spawning it.
fn find_on_path(base: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    for directory in std::env::split_paths(&path) {
        for name in executable_names(base) {
            let candidate = directory.join(name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

fn command_exists(base: &str) -> bool {
    find_command(base).is_some()
}

/// Looks for a command on `PATH`, then in the Windows app-alias directories that
/// GUI processes do not always inherit.
fn find_command(base: &str) -> Option<PathBuf> {
    if let Some(path) = find_on_path(base) {
        return Some(path);
    }

    #[cfg(windows)]
    {
        if let Some(local) = dirs::data_local_dir() {
            for directory in [
                local.join("Microsoft").join("WindowsApps"),
                local.join("Microsoft").join("WinGet").join("Links"),
            ] {
                for name in executable_names(base) {
                    let candidate = directory.join(name);
                    if candidate.is_file() {
                        return Some(candidate);
                    }
                }
            }
        }
    }

    None
}

/// Common install locations that are not always present on `PATH` (GUI apps
/// launched from a desktop session inherit a minimal environment).
fn common_locations() -> Vec<PathBuf> {
    #[cfg(target_os = "linux")]
    {
        let mut locations: Vec<PathBuf> = [
            "/usr/bin/ffmpeg",
            "/usr/local/bin/ffmpeg",
            "/snap/bin/ffmpeg",
            "/var/lib/flatpak/exports/bin/ffmpeg",
        ]
        .into_iter()
        .map(PathBuf::from)
        .collect();
        if let Some(home) = dirs::home_dir() {
            locations.push(home.join(".local").join("bin").join("ffmpeg"));
        }
        locations
    }

    #[cfg(target_os = "macos")]
    {
        [
            "/opt/homebrew/bin/ffmpeg",
            "/usr/local/bin/ffmpeg",
            "/usr/bin/ffmpeg",
        ]
        .into_iter()
        .map(PathBuf::from)
        .collect()
    }

    #[cfg(windows)]
    {
        let mut locations = Vec::new();
        if let Some(local) = dirs::data_local_dir() {
            locations.push(
                local
                    .join("Microsoft")
                    .join("WinGet")
                    .join("Links")
                    .join("ffmpeg.exe"),
            );
        }
        if let Some(program_files) = std::env::var_os("ProgramFiles") {
            locations.push(
                PathBuf::from(program_files)
                    .join("ffmpeg")
                    .join("bin")
                    .join("ffmpeg.exe"),
            );
        }
        locations.push(PathBuf::from(r"C:\ffmpeg\bin\ffmpeg.exe"));
        locations
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", windows)))]
    {
        Vec::new()
    }
}

#[derive(Debug, Clone)]
struct Candidate {
    path: PathBuf,
    source: &'static str,
}

/// Adds a candidate once, comparing canonical paths so the same binary reported
/// by several sources is probed only once.
fn push_candidate(list: &mut Vec<Candidate>, path: PathBuf, source: &'static str) {
    if path.as_os_str().is_empty() {
        return;
    }

    let path = path.canonicalize().unwrap_or(path);
    if list.iter().any(|candidate| candidate.path == path) {
        return;
    }

    list.push(Candidate { path, source });
}

fn candidates() -> Vec<Candidate> {
    let mut list = Vec::new();

    if let Some(path) = std::env::var_os("FFMPEG_PATH").map(PathBuf::from) {
        push_candidate(&mut list, path, "environment");
    }
    if let Some(path) = saved_ffmpeg_path() {
        push_candidate(&mut list, path, "setting");
    }
    if let Some(path) = find_on_path("ffmpeg") {
        push_candidate(&mut list, path, "PATH");
    }
    for path in common_locations() {
        push_candidate(&mut list, path, "system");
    }

    list
}

struct Captured {
    success: bool,
    stdout: String,
    stderr: String,
}

fn combined(output: &Captured) -> String {
    format!("{}\n{}", output.stdout, output.stderr)
}

async fn run_quiet(program: &Path, args: &[&str], limit: Duration) -> Option<Captured> {
    let mut command = Command::new(program);
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);

    let output = timeout(limit, command.output()).await.ok()?.ok()?;

    Some(Captured {
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
}

/// Extracts `7.0.2` from `ffmpeg version 7.0.2 Copyright …`.
fn parse_version(line: &str) -> Option<String> {
    let (_, rest) = line.split_once("version ")?;
    let version = rest.split_whitespace().next()?.to_string();

    // Nightly builds are labelled `n7.1`; keep the readable part.
    let version = match version.strip_prefix('n') {
        Some(stripped) if stripped.starts_with(|character: char| character.is_ascii_digit()) => {
            stripped.to_string()
        }
        _ => version,
    };

    (!version.is_empty()).then_some(version)
}

/// True when `ffmpeg -encoders` lists the native-lame MP3 encoder.
fn lists_libmp3lame(text: &str) -> bool {
    text.lines().any(|line| {
        let mut words = line.split_whitespace();
        words.next();
        words.next() == Some("libmp3lame") || line.contains("--enable-libmp3lame")
    })
}

struct ProbedFfmpeg {
    version: Option<String>,
    version_line: String,
    has_libmp3lame: bool,
}

async fn probe_ffmpeg(path: &Path) -> Option<ProbedFfmpeg> {
    let output = run_quiet(path, &["-hide_banner", "-version"], PROBE_TIMEOUT).await?;
    if !output.success {
        return None;
    }

    let text = combined(&output);
    let version_line = text
        .lines()
        .find(|line| line.contains("version"))
        .unwrap_or_default()
        .trim()
        .to_string();
    let version = parse_version(&version_line);

    let has_libmp3lame = match run_quiet(path, &["-hide_banner", "-encoders"], PROBE_TIMEOUT).await
    {
        Some(encoders) if encoders.success => lists_libmp3lame(&combined(&encoders)),
        _ => text.to_lowercase().contains("libmp3lame"),
    };

    Some(ProbedFfmpeg {
        version,
        version_line,
        has_libmp3lame,
    })
}

fn sibling_ffprobe(ffmpeg: &Path) -> Option<PathBuf> {
    let name = if cfg!(windows) {
        "ffprobe.exe"
    } else {
        "ffprobe"
    };
    let path = ffmpeg.parent()?.join(name);
    path.is_file().then_some(path)
}

async fn probe_ffprobe(ffmpeg: &Path) -> Option<(PathBuf, String)> {
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Some(path) = sibling_ffprobe(ffmpeg) {
        candidates.push(path);
    }
    if let Some(path) = find_on_path("ffprobe")
        && !candidates.contains(&path)
    {
        candidates.push(path);
    }

    for candidate in candidates {
        let Some(output) =
            run_quiet(&candidate, &["-hide_banner", "-version"], PROBE_TIMEOUT).await
        else {
            continue;
        };
        if !output.success {
            continue;
        }

        let text = combined(&output);
        let version_line = text
            .lines()
            .find(|line| line.contains("version"))
            .unwrap_or_default()
            .trim()
            .to_string();
        let version = parse_version(&version_line).unwrap_or_default();
        return Some((candidate, version));
    }

    None
}

struct InstallerPlan {
    id: &'static str,
    label: &'static str,
    program: &'static str,
    args: &'static [&'static str],
    display: &'static str,
    /// Whether the command must run through `pkexec` on Linux.
    elevated: bool,
}

#[cfg(target_os = "linux")]
fn installer_plans() -> Vec<InstallerPlan> {
    vec![
        InstallerPlan {
            id: "apt",
            label: "APT (Debian, Ubuntu)",
            program: "apt-get",
            args: &["install", "-y", "ffmpeg"],
            display: "sudo apt-get install -y ffmpeg",
            elevated: true,
        },
        InstallerPlan {
            id: "dnf",
            label: "DNF (Fedora)",
            program: "dnf",
            args: &["install", "-y", "ffmpeg"],
            display: "sudo dnf install -y ffmpeg",
            elevated: true,
        },
        InstallerPlan {
            id: "pacman",
            label: "Pacman (Arch)",
            program: "pacman",
            args: &["-S", "--noconfirm", "ffmpeg"],
            display: "sudo pacman -S --noconfirm ffmpeg",
            elevated: true,
        },
        InstallerPlan {
            id: "zypper",
            label: "Zypper (openSUSE)",
            program: "zypper",
            args: &["install", "-y", "ffmpeg"],
            display: "sudo zypper install -y ffmpeg",
            elevated: true,
        },
    ]
}

#[cfg(target_os = "windows")]
fn installer_plans() -> Vec<InstallerPlan> {
    vec![InstallerPlan {
        id: "winget",
        label: "winget (Gyan.FFmpeg)",
        program: "winget",
        args: &[
            "install",
            "--id",
            "Gyan.FFmpeg",
            "-e",
            "--accept-source-agreements",
            "--accept-package-agreements",
            "--silent",
        ],
        display: "winget install --id Gyan.FFmpeg -e",
        elevated: false,
    }]
}

#[cfg(target_os = "macos")]
fn installer_plans() -> Vec<InstallerPlan> {
    vec![InstallerPlan {
        id: "brew",
        label: "Homebrew",
        program: "brew",
        args: &["install", "ffmpeg"],
        display: "brew install ffmpeg",
        elevated: false,
    }]
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
fn installer_plans() -> Vec<InstallerPlan> {
    Vec::new()
}

fn installers() -> Vec<FfmpegInstaller> {
    installer_plans()
        .into_iter()
        .map(|plan| {
            let available =
                command_exists(plan.program) && (!plan.elevated || command_exists("pkexec"));

            FfmpegInstaller {
                id: plan.id.to_string(),
                label: plan.label.to_string(),
                available,
                command: plan.display.to_string(),
            }
        })
        .collect()
}

async fn run_installer(plan: &InstallerPlan) -> Result<(), AppError> {
    let program = find_command(plan.program).ok_or_else(|| {
        AppError::new(
            "ffmpeg-install-unavailable",
            format!("{} is not installed.", plan.program),
        )
    })?;

    let mut command = if plan.elevated {
        let pkexec = find_command("pkexec").ok_or_else(|| {
            AppError::new(
                "ffmpeg-install-unavailable",
                "pkexec is not available. Run the command shown in the FFmpeg tab in a terminal.",
            )
        })?;
        let mut command = Command::new(pkexec);
        command.arg(program);
        command
    } else {
        Command::new(program)
    };

    command
        .args(plan.args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);

    let output = timeout(INSTALL_TIMEOUT, command.output())
        .await
        .map_err(|_| {
            AppError::new(
                "ffmpeg-install-failed",
                format!("{} did not finish in time.", plan.label),
            )
        })?
        .map_err(|error| {
            AppError::new(
                "ffmpeg-install-failed",
                format!("Could not run {}: {error}", plan.label),
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let line = stderr
            .lines()
            .chain(stdout.lines())
            .rev()
            .find(|line| !line.trim().is_empty())
            .unwrap_or("The installer stopped with an error.");

        return Err(AppError::new(
            "ffmpeg-install-failed",
            format!("{} failed: {}", plan.label, line.trim()),
        ));
    }

    Ok(())
}

/// Probes the candidates in priority order and reports the first working build.
pub async fn status() -> FfmpegStatus {
    let mut detected = None;

    for candidate in candidates() {
        if let Some(probed) = probe_ffmpeg(&candidate.path).await {
            let ffprobe = probe_ffprobe(&candidate.path).await;
            detected = Some((candidate, probed, ffprobe));
            break;
        }
    }

    FfmpegStatus {
        platform: std::env::consts::OS.to_string(),
        path: detected
            .as_ref()
            .map(|(candidate, ..)| candidate.path.to_string_lossy().to_string()),
        source: detected
            .as_ref()
            .map(|(candidate, ..)| candidate.source.to_string()),
        version: detected
            .as_ref()
            .and_then(|(_, probed, _)| probed.version.clone()),
        version_line: detected
            .as_ref()
            .map(|(_, probed, _)| probed.version_line.clone()),
        has_libmp3lame: detected
            .as_ref()
            .is_some_and(|(_, probed, _)| probed.has_libmp3lame),
        ffprobe_path: detected.as_ref().and_then(|(_, _, ffprobe)| {
            ffprobe
                .as_ref()
                .map(|(path, _)| path.to_string_lossy().to_string())
        }),
        ffprobe_version: detected
            .as_ref()
            .and_then(|(_, _, ffprobe)| ffprobe.as_ref().map(|(_, version)| version.clone())),
        installers: installers(),
        download_url: DOWNLOAD_URL.to_string(),
    }
}

/// Looks for FFmpeg and returns everything the FFmpeg tab needs to render.
#[tauri::command]
pub async fn ffmpeg_status() -> FfmpegStatus {
    status().await
}

/// Validates and saves a user-selected `ffmpeg` binary. An empty path clears
/// the saved preference and returns to automatic detection.
#[tauri::command]
pub async fn ffmpeg_set_path(path: String) -> Result<FfmpegStatus, AppError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        clear_saved_path()?;
        return Ok(status().await);
    }

    let path = PathBuf::from(trimmed);
    if !path.is_file() {
        return Err(AppError::new(
            "ffmpeg-invalid",
            format!("{} is not a file.", path.display()),
        ));
    }

    probe_ffmpeg(&path).await.ok_or_else(|| {
        AppError::new(
            "ffmpeg-invalid",
            "This file does not respond like an FFmpeg binary.",
        )
    })?;

    let path = path.canonicalize().unwrap_or(path);
    save_ffmpeg_path(&path)?;

    Ok(status().await)
}

/// Forgets the saved path and falls back to `FFMPEG_PATH`, `PATH`, and the
/// common install locations.
#[tauri::command]
pub async fn ffmpeg_clear_path() -> Result<FfmpegStatus, AppError> {
    clear_saved_path()?;
    Ok(status().await)
}

/// Runs the chosen platform package manager, then re-runs detection.
#[tauri::command]
pub async fn ffmpeg_install(installer: String) -> Result<FfmpegStatus, AppError> {
    let plan = installer_plans()
        .into_iter()
        .find(|plan| plan.id == installer)
        .ok_or_else(|| {
            AppError::new(
                "ffmpeg-install-unsupported",
                "This installation method is not supported on this platform.",
            )
        })?;

    if !command_exists(plan.program) {
        return Err(AppError::new(
            "ffmpeg-install-unavailable",
            format!("{} is not installed.", plan.program),
        ));
    }
    if plan.elevated && !command_exists("pkexec") {
        return Err(AppError::new(
            "ffmpeg-install-unavailable",
            "pkexec is not available. Run the command shown in the FFmpeg tab in a terminal.",
        ));
    }

    run_installer(&plan).await?;

    Ok(status().await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_release_version() {
        assert_eq!(
            parse_version("ffmpeg version 7.0.2 Copyright (c) 2000-2024 the FFmpeg developers")
                .as_deref(),
            Some("7.0.2")
        );
    }

    #[test]
    fn parses_a_nightly_version() {
        assert_eq!(
            parse_version("ffmpeg version n7.1 Copyright (c) 2000-2024 the FFmpeg developers")
                .as_deref(),
            Some("7.1")
        );
    }

    #[test]
    fn detects_libmp3lame_in_the_encoder_list() {
        assert!(lists_libmp3lame(
            " A....D libmp3lame           libmp3lame MP3 (MPEG audio layer 3)"
        ));
        assert!(!lists_libmp3lame(
            " A....D aac                  AAC (Advanced Audio Coding)"
        ));
        assert!(lists_libmp3lame(
            "configuration: --enable-gpl --enable-libmp3lame"
        ));
    }

    #[test]
    fn finds_ffprobe_next_to_ffmpeg() {
        let dir =
            std::env::temp_dir().join(format!("yt-dlp-gui-ffmpeg-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp directory should be created");
        let ffmpeg = dir.join(if cfg!(windows) {
            "ffmpeg.exe"
        } else {
            "ffmpeg"
        });
        let ffprobe = dir.join(if cfg!(windows) {
            "ffprobe.exe"
        } else {
            "ffprobe"
        });
        std::fs::write(&ffmpeg, b"x").expect("ffmpeg placeholder should be written");
        std::fs::write(&ffprobe, b"x").expect("ffprobe placeholder should be written");

        assert_eq!(sibling_ffprobe(&ffmpeg), Some(ffprobe));

        let _ = std::fs::remove_dir_all(&dir);
    }
}
