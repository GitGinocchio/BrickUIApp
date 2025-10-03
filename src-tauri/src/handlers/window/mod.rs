use crate::winapi::{monitor::Monitor, window::Window};

#[tauri::command]
pub fn get_maximized_windows() -> Result<Vec<Window>, String> {
    crate::winapi::window::get_maximized_windows()
}

#[tauri::command]
pub fn get_maximized_window_for_monitor(monitor: Monitor) -> Result<Option<Window>, String> {
    crate::winapi::window::get_maximized_window_for_monitor(&monitor)
}

#[tauri::command]
pub fn get_windows_in_monitor(monitor: Monitor) -> Result<Vec<Window>, String> {
    crate::winapi::window::get_windows_in_monitor(&monitor)
}

#[tauri::command]
pub fn get_all_windows() -> Result<Vec<Window>, String> {
    crate::winapi::window::get_all_windows()
}