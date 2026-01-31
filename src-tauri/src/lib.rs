mod handlers;
use crate::handlers::generate_handlers;

mod setup;
use crate::setup::setup_single_instance;
use crate::setup::on_window_event;
use crate::setup::setup;

mod profiler;
mod winapi;
mod config;
mod state;
mod keyring;
mod bricks;
mod utils;
mod api;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent,None))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| setup_single_instance(app, args, cwd)))
        .setup(|app| setup(app))
        .invoke_handler(generate_handlers())
        .on_window_event(|window, event| on_window_event(window, event))
        .run(context)
        .expect("error while running brickui application");
}
