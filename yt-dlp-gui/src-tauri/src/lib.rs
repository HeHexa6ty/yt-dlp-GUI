// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod download;
mod error;
mod ffmpeg;
mod models;
mod paths;
#[cfg(test)]
mod test_set;
mod x_api;
mod ytdlp;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(download::DownloadState::default())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::analyze_url,
            commands::download_media,
            ffmpeg::ffmpeg_status,
            ffmpeg::ffmpeg_set_path,
            ffmpeg::ffmpeg_clear_path,
            ffmpeg::ffmpeg_install
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
