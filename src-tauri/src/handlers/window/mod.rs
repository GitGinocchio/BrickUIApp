use std::{collections::HashSet, sync::Arc};
use tokio::sync::Mutex;

use tauri::State;

use crate::{state::generic::BrickUIGenericState, winapi::{events::hitboxes::{ClickableRect, HITBOXES}, monitor::Monitor, window::Window}};

// TODO: magari si potrebbe creare un metodo che permette a tutti di ottenere i rect cliccabili
// (non e' poi cosi' utile nel senso che si puo' gia' fare in javascript, pero' evita codice in piu' nei brick)
#[tauri::command(async)]
pub async fn update_clickable_rects(
    rects: HashSet<ClickableRect>
) -> Result<(), String> {
    println!("Updating clickable rects");
    let mut state_guard = HITBOXES.write().await;
    *state_guard = rects;


    Ok(())
}

#[tauri::command]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn get_maximized_windows() -> Result<Vec<Window>, String> {
    crate::winapi::window::get_maximized_windows()
}

#[tauri::command]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn get_monitor_maximized_window(monitor: Monitor) -> Result<Option<Window>, String> {
    crate::winapi::window::get_monitor_maximized_window(&monitor)
}

#[tauri::command]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn get_monitor_windows(monitor: Monitor) -> Result<Vec<Window>, String> {
    crate::winapi::window::get_monitor_windows(&monitor)
}

#[tauri::command]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub fn get_all_windows() -> Result<Vec<Window>, String> {
    crate::winapi::window::get_all_windows()
}
