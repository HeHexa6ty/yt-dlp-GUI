use tauri::{AppHandle, State};

use crate::download::{self, DownloadGuard, DownloadState};
use crate::error::AppError;
use crate::models::{DirectoryTree, DownloadRequest, DownloadResult, MediaInfo, MediaType};
use crate::paths::{ensure_platform_dirs, parse_supported_url};
use crate::x_api;
use crate::ytdlp;

/// Validates the URL, asks yt-dlp for metadata, then lazily creates the
/// platform folder structure under Downloads.
#[tauri::command]
pub async fn analyze_url(url: String) -> Result<MediaInfo, AppError> {
    let target = parse_supported_url(&url)?;

    let mut info = ytdlp::analyze(&target.url).await?;

    // yt-dlp filters photos out of X posts and cannot extract some animated GIF
    // posts, so both can arrive as an image with no formats. Fill the preview
    // from X's syndication data and promote animated GIFs so the GIF pipeline
    // runs. A failure here only costs the preview, not the whole analysis.
    if target.platform == "X" && info.media_type == MediaType::Image {
        let photos = if info.thumbnail.is_none() {
            x_api::fetch_photos(&target.url).await.unwrap_or_default()
        } else {
            Vec::new()
        };

        if let Some(photo) = photos.first() {
            info.thumbnail = Some(photo.url.clone());
        } else if let Ok(Some(gif)) = x_api::fetch_gif(&target.url).await {
            info.media_type = MediaType::Gif;
            if info.thumbnail.is_none() {
                info.thumbnail = gif.thumbnail;
            }
        }
    }

    let layout = ensure_platform_dirs(target.platform)?;
    info.platform = target.platform.to_string();
    info.default_directory = layout.platform_dir.to_string_lossy().to_string();
    info.directory_tree = DirectoryTree {
        root: layout.app_root.to_string_lossy().to_string(),
        platform: target.platform.to_string(),
        folders: layout.folders,
    };

    Ok(info)
}

/// Downloads one media item. A single download may run at a time; concurrent
/// requests are rejected with `download-in-progress`.
#[tauri::command]
pub async fn download_media(
    app: AppHandle,
    state: State<'_, DownloadState>,
    request: DownloadRequest,
) -> Result<DownloadResult, AppError> {
    let _guard = DownloadGuard::acquire(&state.0)?;
    download::download(&app, request).await
}
