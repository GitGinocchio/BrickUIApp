use crate::winapi::{monitor::Monitor, Rect};



#[tauri::command]
pub fn set_monitor_workarea(margins: Rect, monitor: Option<Monitor>) -> Result<(), String> {
    crate::winapi::workarea::set_monitor_workarea(margins, monitor)
}

#[tauri::command]
pub fn set_workarea_for_all_monitors(margins: Rect) -> Result<(), String> {
    crate::winapi::workarea::set_workarea_for_all_monitors(margins)
}