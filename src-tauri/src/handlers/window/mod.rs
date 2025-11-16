use crate::winapi::{monitor::Monitor, window::Window};

#[tauri::command]
pub fn get_maximized_windows() -> Result<Vec<Window>, String> {
    crate::winapi::window::get_maximized_windows()
}

#[tauri::command]
pub fn get_monitor_maximized_window(monitor: Monitor) -> Result<Option<Window>, String> {
    crate::winapi::window::get_monitor_maximized_window(&monitor)
}

#[tauri::command]
pub fn get_monitor_windows(monitor: Monitor) -> Result<Vec<Window>, String> {
    crate::winapi::window::get_monitor_windows(&monitor)
}

#[tauri::command]
pub fn get_all_windows() -> Result<Vec<Window>, String> {
    crate::winapi::window::get_all_windows()
}
