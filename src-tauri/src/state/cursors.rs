use std::collections::HashMap;
use tauri::AppHandle;

use crate::winapi::cursors::{Cursor, CursorType, SafeHCursor};

pub struct CursorState {
    pub original: HashMap<CursorType, SafeHCursor>,
    pub current: HashMap<CursorType, Cursor>,
    pub active: Option<CursorType>,
    pub paused: bool,
    pub visible: bool,
    pub app: AppHandle
}