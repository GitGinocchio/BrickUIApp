use crate::bricks;
use crate::bricks::brick::Brick;

use crate::state::BrickUIState;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::State;

#[tauri::command]
pub fn get_bricks(state: State<'_, Arc<Mutex<BrickUIState>>>) -> Result<Vec<Brick>, String> {
    let state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;

    Ok(state_guard.get_bricks().to_vec())
}

#[tauri::command]
pub fn get_brick_by_name(
    state: State<'_, Arc<Mutex<BrickUIState>>>,
    name: String,
) -> Result<Option<Brick>, String> {
    let state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;

    Ok(state_guard.get_brick_by_name(&name))
}

#[tauri::command]
pub fn load_bricks(state: State<'_, Arc<Mutex<BrickUIState>>>) -> Result<Vec<Brick>, String> {
    let mut state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;

    let path = state_guard.get_path();
    let bricks = bricks::load_bricks(&path)?;

    let bricks_return = bricks.clone();
    *state_guard.get_mut_bricks() = bricks;

    Ok(bricks_return)
}

#[tauri::command]
pub fn save_brick(state: State<'_, Arc<Mutex<BrickUIState>>>, brick: Brick) -> Result<(), String> {
    let mut state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;

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

#[tauri::command]
pub fn rename_brick(
    state: State<'_, Arc<Mutex<BrickUIState>>>,
    old_name: String,
    new_name: String,
) -> Result<(), String> {
    let mut state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    let path = state_guard.get_path();

    bricks::rename_brick(&path, &old_name, &new_name)?;

    if let Some(brick) = state_guard.bricks.iter_mut().find(|b| b.name == old_name) {
        brick.name = new_name;
    }

    Ok(())
}

#[tauri::command]
pub fn duplicate_brick(state: State<'_, Arc<Mutex<BrickUIState>>>, brick: Brick) -> Result<(), String> {
    let mut state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    let path = state_guard.get_path();

    let new_brick = bricks::duplicate_brick(&path, brick)?;

    state_guard.bricks.push(new_brick);

    Ok(())
}

// Questo non serve a molto potrebbe essere sostituito con il plugin opener e basta
#[tauri::command]
pub fn open_brick(
    state: State<'_, Arc<Mutex<BrickUIState>>>,
    brick_name: String,
) -> Result<(), String> {
    let state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    let path = state_guard.get_path();

    bricks::open_brick(&path, brick_name)
}

#[tauri::command]
pub fn delete_brick(state: State<'_, Arc<Mutex<BrickUIState>>>, brick: Brick) -> Result<(), String> {
    let mut state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    let path = state_guard.get_path();

    state_guard.bricks.retain(|b| b.name != brick.name);

    bricks::delete_brick(&path, &brick)
}

#[tauri::command]
pub fn new_brick(state: State<'_, Arc<Mutex<BrickUIState>>>, brick: Brick) -> Result<(), String> {
    let mut state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    let path = state_guard.get_path();

    bricks::create_brick(&path, &brick)?;
    state_guard.bricks.push(brick);

    Ok(())
}

#[tauri::command]
pub fn pack_brick(
    state: State<'_, Arc<Mutex<BrickUIState>>>,
    brick_name: String,
    output_path: String,
) -> Result<Option<String>, String> {
    let state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    let path = state_guard.get_path();

    crate::bricks::pack_brick(&path, brick_name, &PathBuf::from(output_path))?;

    Ok(None)
}

#[tauri::command]
pub fn unpack_brick(
    state: State<'_, Arc<Mutex<BrickUIState>>>,
    brick_path: String,
    brick_name: String
) -> Result<(), String> {
    let state_guard = state.lock().map_err(|e| format!("Mutex poisoned: {e}"))?;
    let path = state_guard.get_path();

    let output_dir = path.join("bricks").join(brick_name);

    println!("{brick_path:?}, {output_dir:?}");

    crate::bricks::unpack_brick(&PathBuf::from(brick_path), &output_dir)?;

    Ok(())
}
