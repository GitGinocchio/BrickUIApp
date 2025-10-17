use crate::winapi::{monitor::Monitor, rect::{OptionalRect, Rect}};

#[tauri::command]
pub fn set_workarea_margins(margins: OptionalRect, monitor: Option<Monitor>) -> Result<Rect, String> {
    crate::winapi::monitor::workarea::set_workarea_margins(&margins, monitor.as_ref())
}

#[tauri::command]
pub fn set_workareas_margins(margins: OptionalRect) -> Result<(), String> {
    crate::winapi::monitor::workarea::set_workareas_margins(&margins)
}
