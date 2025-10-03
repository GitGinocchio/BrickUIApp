use crate::winapi::{monitor::Monitor, rect::OptionalRect};



#[tauri::command]
pub fn set_monitor_workarea(margins: OptionalRect, monitor: Option<Monitor>) -> Result<(), String> {
    crate::winapi::monitor::workarea::set_monitor_workarea(&margins, monitor.as_ref())
}

#[tauri::command]
pub fn set_workarea_for_all_monitors(margins: OptionalRect) -> Result<(), String> {
    crate::winapi::monitor::workarea::set_workarea_for_all_monitors(&margins)
}