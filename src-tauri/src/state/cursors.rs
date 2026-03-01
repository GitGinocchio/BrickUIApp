use std::collections::HashMap;
use crate::winapi::cursors::{CursorType, Scheme};

pub struct CursorsState {
    pub schemes: HashMap<String, Scheme>,
    pub default: HashMap<CursorType, String>,
}

impl CursorsState {
    pub fn new(default: HashMap<CursorType, String>) -> Result<Self, String> {
        Ok(Self {
            schemes: HashMap::new(),
            default: default
        })
    }
}