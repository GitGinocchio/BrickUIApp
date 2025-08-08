

#[tauri::command]
pub fn get_mouse_position() -> Option<(i32, i32)> {
    match rdev::get_mouse_location() {
        Ok(pos) => Some((pos.x as i32, pos.y as i32)),
        Err(_) => None,
    }
}