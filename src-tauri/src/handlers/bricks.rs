use crate::bricks;
use crate::bricks::brick::Brick;

use crate::state::generic::BrickUIGenericState;

use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn get_bricks(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
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
) -> Result<Vec<Brick>, String> {
    let mut state_guard = state.lock().await;

    let path = state_guard.get_path();
    let bricks = bricks::load_bricks(&path)?;

    let bricks_return = bricks.clone();
    *state_guard.get_mut_bricks() = bricks;

    Ok(bricks_return)
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn save_brick(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
    brick: Brick,
) -> Result<(), String> {
    let mut state_guard = state.lock().await;

    let path = state_guard.get_path();

    bricks::save_brick(&path, &brick)?;

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
    old_name: String,
    new_name: String,
) -> Result<(), String> {
    let mut state_guard = state.lock().await;
    let path = state_guard.get_path();

    bricks::rename_brick(&path, &old_name, &new_name)?;

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
    brick: Brick,
) -> Result<(), String> {
    let mut state_guard = state.lock().await;
    let path = state_guard.get_path();

    let new_brick = bricks::duplicate_brick(&path, brick)?;

    let bricks = state_guard.get_mut_bricks();
    bricks.push(new_brick);

    Ok(())
}

// Questo non serve a molto potrebbe essere sostituito con il plugin opener e basta
#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn open_brick(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
    brick_name: String,
) -> Result<(), String> {
    let state_guard = state.lock().await;
    let path = state_guard.get_path();

    bricks::open_brick(&path, brick_name)
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn delete_brick(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
    brick: Brick,
) -> Result<(), String> {
    let mut state_guard = state.lock().await;

    let path = state_guard.get_path();

    bricks::delete_brick(&path, &brick)?;

    let bricks = state_guard.get_mut_bricks();
    bricks.retain(|b| b.name != brick.name);

    Ok(())
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn new_brick(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
    brick: Brick,
) -> Result<(), String> {
    let mut state_guard = state.lock().await;
    let path = state_guard.get_path();
    let res_path = state_guard.get_resource_path();

    bricks::create_brick(&path, &res_path, &brick)?;

    let bricks = state_guard.get_mut_bricks();
    bricks.push(brick);

    Ok(())
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn pack_brick(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
    brick_name: String,
    output_path: String,
) -> Result<Option<String>, String> {
    let state_guard = state.lock().await;
    let path = state_guard.get_path();

    crate::bricks::pack_brick(&path, brick_name, &PathBuf::from(output_path))?;

    Ok(None)
}

#[tauri::command(async)]
#[cfg_attr(feature = "profiling", tracing::instrument)]
pub async fn unpack_brick(
    state: State<'_, Arc<Mutex<BrickUIGenericState>>>,
    brick_path: String,
    brick_name: String,
) -> Result<(), String> {
    let state_guard = state.lock().await;
    let path = state_guard.get_path();

    let output_dir = path.join("bricks").join(brick_name);

    println!("{brick_path:?}, {output_dir:?}");

    crate::bricks::unpack_brick(&PathBuf::from(brick_path), &output_dir)?;

    Ok(())
}
