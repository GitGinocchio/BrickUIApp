use serde::{Deserialize, Serialize};
use windows::Win32::Foundation::RECT;



#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Rect {
    pub left: i32, 
    pub right: i32,
    pub top: i32,
    pub bottom: i32
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct OptionalRect {
    pub left: Option<i32>,
    pub right: Option<i32>,
    pub top: Option<i32>,
    pub bottom: Option<i32>,
}

impl OptionalRect {
    /// Restituisce la larghezza, usando 0 se left o right sono None
    pub fn width(&self) -> i32 {
        let left = self.left.unwrap_or(0);
        let right = self.right.unwrap_or(0);
        right - left
    }

    /// Restituisce l'altezza, usando 0 se top o bottom sono None
    pub fn height(&self) -> i32 {
        let top = self.top.unwrap_or(0);
        let bottom = self.bottom.unwrap_or(0);
        bottom - top
    }

    /// Applica i valori presenti a un Rect esistente
    pub fn apply_to(&self, base: &mut Rect) {
        if let Some(l) = self.left { base.left = l; }
        if let Some(r) = self.right { base.right = r; }
        if let Some(t) = self.top { base.top = t; }
        if let Some(b) = self.bottom { base.bottom = b; }
    }

    /// Crea un Rect completo usando valori di default per i campi None
    pub fn into_rect(self, default: Rect) -> Rect {
        Rect {
            left: self.left.unwrap_or(default.left),
            right: self.right.unwrap_or(default.right),
            top: self.top.unwrap_or(default.top),
            bottom: self.bottom.unwrap_or(default.bottom),
        }
    }
}

impl Rect {
    pub fn width(&self) -> i32 { self.right - self.left }
    pub fn height(&self) -> i32 { self.bottom - self.top }
    
    /// Applica margini rispetto al Rect originale.
    /// - Some(v) → usa v
    /// - None → lascia invariato il bordo corrente
    pub fn apply_margins(&self, margins: &OptionalRect, original: &Rect) -> Rect {
        Rect {
            left: margins.left.map_or(self.left, |v| original.left + v),
            top: margins.top.map_or(self.top, |v| original.top + v),
            right: margins.right.map_or(self.right, |v| original.right - v),
            bottom: margins.bottom.map_or(self.bottom, |v| original.bottom - v),
        }
    }
}

impl From<RECT> for Rect {
    fn from(r: RECT) -> Self {
        Self {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}
impl From<Rect> for RECT {
    fn from(r: Rect) -> Self {
        RECT {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}
impl From<&RECT> for Rect {
    fn from(r: &RECT) -> Self {
        Self {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}
impl From<&Rect> for RECT {
    fn from(r: &Rect) -> Self {
        RECT {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}



impl From<RECT> for OptionalRect {
    fn from(r: RECT) -> Self {
        Self {
            left: Some(r.left),
            right: Some(r.right),
            top: Some(r.top),
            bottom: Some(r.bottom),
        }
    }
}
impl From<&RECT> for OptionalRect {
    fn from(r: &RECT) -> Self {
        Self {
            left: Some(r.left),
            right: Some(r.right),
            top: Some(r.top),
            bottom: Some(r.bottom),
        }
    }
}
impl From<OptionalRect> for RECT {
    fn from(r: OptionalRect) -> Self {
        RECT {
            left: r.left.unwrap_or(0),
            right: r.right.unwrap_or(0),
            top: r.top.unwrap_or(0),
            bottom: r.bottom.unwrap_or(0),
        }
    }
}
impl From<&OptionalRect> for RECT {
    fn from(r: &OptionalRect) -> Self {
        RECT {
            left: r.left.unwrap_or(0),
            right: r.right.unwrap_or(0),
            top: r.top.unwrap_or(0),
            bottom: r.bottom.unwrap_or(0),
        }
    }
}
impl From<Rect> for OptionalRect {
    fn from(r: Rect) -> Self {
        Self {
            left: Some(r.left),
            right: Some(r.right),
            top: Some(r.top),
            bottom: Some(r.bottom),
        }
    }
}
impl From<&Rect> for OptionalRect {
    fn from(r: &Rect) -> Self {
        Self {
            left: Some(r.left),
            right: Some(r.right),
            top: Some(r.top),
            bottom: Some(r.bottom),
        }
    }
}