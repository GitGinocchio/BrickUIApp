use crate::bricks;
use crate::bricks::brick::Brick;

use crate::state::generic::BrickUIGenericState;

use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Manager as _, State};
use tokio::sync::Mutex;

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn get_bricks(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>
) -> Result<Vec<Brick>, String> {
    let state_guard = state.lock().await;

    Ok(state_guard.get_bricks().to_vec())
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn get_brick_by_name(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
    name: String,
) -> Result<Option<Brick>, String> {
    let state_guard = state.lock().await;

    Ok(state_guard.get_brick_by_name(&name))
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn load_bricks(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
    app_handle: AppHandle
) -> Result<Vec<Brick>, String> {
    let resolver = app_handle.path();
    let path = resolver.app_data_dir().map_err(|e| format!("Error resolving appdata dir: {e}"))?;

    let bricks = bricks::load_bricks(&path)?;

    let mut state_guard = state.lock().await;
    *state_guard.get_mut_bricks() = bricks.clone();

    Ok(bricks)
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn save_brick(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
    app_handle: AppHandle,
    brick: Brick,
) -> Result<(), String> {
    let resolver = app_handle.path();
    let path = resolver.app_data_dir().map_err(|e| format!("Error resolving appdata dir: {e}"))?;

    bricks::save_brick(&path, &brick)?;

    let mut state_guard = state.lock().await;

    // Aggiorna lo stato in memoria
    if let Some(existing) = state_guard
        .get_mut_bricks()
        .iter_mut()
        .find(|b| b.name == brick.name)
    {
        *existing = brick;
    } else {
        state_guard.get_mut_bricks().push(brick);
    }

    Ok(())
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn rename_brick(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
    app_handle: AppHandle,
    old_name: String,
    new_name: String,
) -> Result<(), String> {
    let resolver = app_handle.path();
    let path = resolver.app_data_dir().map_err(|e| format!("Error resolving appdata dir: {e}"))?;

    bricks::rename_brick(&path, &old_name, &new_name)?;

    let mut state_guard = state.lock().await;

    let bricks = state_guard.get_mut_bricks();

    if let Some(brick) = bricks.iter_mut().find(|b| b.name == old_name) {
        brick.name = new_name;
    }

    Ok(())
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn duplicate_brick(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
    app_handle: AppHandle,
    brick: Brick,
) -> Result<(), String> {
    let resolver = app_handle.path();
    let path = resolver.app_data_dir().map_err(|e| format!("Error resolving appdata dir: {e}"))?;

    let new_brick = bricks::duplicate_brick(&path, brick)?;
    
    let mut state_guard = state.lock().await;
    let bricks = state_guard.get_mut_bricks();
    bricks.push(new_brick);

    Ok(())
}

// Questo non serve a molto potrebbe essere sostituito con il plugin opener e basta
#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn open_brick(
    app_handle: AppHandle,
    brick_name: String,
) -> Result<(), String> {
    let resolver = app_handle.path();
    let path = resolver.app_data_dir().map_err(|e| format!("Error resolving appdata dir: {e}"))?;

    bricks::open_brick(&path, brick_name)
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn delete_brick(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
    app_handle: AppHandle,
    brick: Brick,
) -> Result<(), String> {
    let resolver = app_handle.path();
    let path = resolver.app_data_dir().map_err(|e| format!("Error resolving appdata dir: {e}"))?;

    bricks::delete_brick(&path, &brick)?;

    let mut state_guard = state.lock().await;
    let bricks = state_guard.get_mut_bricks();
    bricks.retain(|b| b.name != brick.name);

    Ok(())
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn new_brick(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
    app_handle: AppHandle,
    brick: Brick,
) -> Result<(), String> {
    let resolver = app_handle.path();
    let path = resolver.app_data_dir().map_err(|e| format!("Error resolving appdata dir: {e}"))?;
    let res_path = resolver.resource_dir().map_err(|e| format!("Error resolving resource dir: {e}"))?;

    bricks::create_brick(&path, &res_path, &brick)?;

    let mut state_guard = state.lock().await;
    let bricks = state_guard.get_mut_bricks();
    bricks.push(brick);

    Ok(())
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn pack_brick(
    app_handle: AppHandle,
    brick_name: String,
    output_path: String,
) -> Result<Option<String>, String> {
    let resolver = app_handle.path();
    let path = resolver.app_data_dir().map_err(|e| format!("Error resolving appdata dir: {e}"))?;

    crate::bricks::pack_brick(&path, brick_name, &PathBuf::from(output_path))?;

    Ok(None)
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn unpack_brick(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
    app_handle: AppHandle,
    brick_path: String,
    brick_name: String,
) -> Result<(), String> {
    let resolver = app_handle.path();
    let path = resolver.app_data_dir().map_err(|e| format!("Error resolving appdata dir: {e}"))?;

    let output_dir = path.join("bricks").join(brick_name);
    crate::bricks::unpack_brick(&PathBuf::from(brick_path), &output_dir)?;

    let brick = crate::bricks::load_brick(&output_dir.join("brick.yml"))?;

    let mut state_guard = state.lock().await;
    let bricks = state_guard.get_mut_bricks();
    bricks.push(brick);

    Ok(())
}
