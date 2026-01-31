use crate::winapi::{
    monitor::Monitor,
    rect::{OptionalRect, Rect},
};

#[tauri::command]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn set_workarea_margins(
    margins: OptionalRect,
    monitor: Option<Monitor>,
) -> Result<Rect, String> {
    crate::winapi::monitor::workarea::set_workarea_margins(&margins, monitor.as_ref())
}

#[tauri::command]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn set_workareas_margins(margins: OptionalRect) -> Result<(), String> {
    crate::winapi::monitor::workarea::set_workareas_margins(&margins)
}
