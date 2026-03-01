use std::collections::HashMap;
use crate::winapi::cursors::{CursorType, Scheme};

pub struct Cursors {
    pub schemes: HashMap<String, Scheme>,
    pub backup: Option<HashMap<CursorType, String>>,
}