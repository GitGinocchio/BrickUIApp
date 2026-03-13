use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use tauri::{AppHandle, Manager as _, State};

use crate::{state::{cursors::CursorState, generic::GenericState}, winapi::cursors::{CursorType, Frame}};

// TODO: Rimuovere questo metodo in quanto pericoloso
// Rischia di rendere l'esperienza utente un problema
// Non e' il massimo nasconderlo del tutto
// Oppure una soluzione sarebbe capire quando una finestra va sopra l'overlay
// (l'unico momento in cui un cursore custom applicato dentro l'overlay)
// e far comparire il cursore nativo oppure cercare di portare la finestra topmost
#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn hide_all_cursors(
    app_handle: AppHandle
) -> Result<(), String> {
    let resolver = app_handle.path();
    let resource_path = resolver.resource_dir().map_err(|e| format!("Error resolving appdata dir: {e}"))?;

    let cursor_path = resource_path.join("assets").join("transparent.cur");

    //crate::winapi::cursors::hide_cursors(cursor_path.to_string_lossy().to_string().as_str())
    Ok(())
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn restore_all_cursors(
    state: State<'_, Arc<Mutex<GenericState>>>,
) -> Result<(), String> {
    let state_guard = state.lock().await;
    let backup = state_guard.get_backup();

    //crate::winapi::cursors::restore_cursors(&backup.cursors)
    Ok(())
}

#[tauri::command(async)]
pub async fn set_cursor(
    state: State<'_, Arc<RwLock<CursorState>>>,
    path: String,
    cursor_type: CursorType
) -> Result<(), String> {
    let mut state_guard = state.write().await;
    state_guard.set_cursor(&path, cursor_type)
}

#[tauri::command(async)]
pub async fn set_animated_cursor(
    state: State<'_, Arc<RwLock<CursorState>>>,
    cursor_type: CursorType,
    frames: Vec<Frame>,
    fps: u32
) -> Result<(), String> {
    let mut state_guard = state.write().await;
    state_guard.set_animated_cursor(cursor_type, frames, fps)
}