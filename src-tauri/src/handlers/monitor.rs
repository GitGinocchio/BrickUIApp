use crate::winapi::monitor::Monitor;

#[tauri::command]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn get_all_monitors() -> Result<Vec<Monitor>, String> {
    crate::winapi::monitor::get_all_monitors()
}

#[tauri::command]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn get_monitor(index: usize) -> Result<Monitor, String> {
    crate::winapi::monitor::get_monitor(index)
}

#[tauri::command]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn get_monitor_from_point(x: i32, y: i32) -> Result<Monitor, String> {
    crate::winapi::monitor::get_monitor_from_point(x, y)
}

#[tauri::command]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn get_primary_monitor() -> Result<Monitor, String> {
    crate::winapi::monitor::get_primary_monitor()
}
