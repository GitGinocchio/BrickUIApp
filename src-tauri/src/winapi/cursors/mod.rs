use std::{
    collections::HashMap,
    sync::Arc,
    thread,
    time::Duration,
};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator as _;
use strum_macros::{AsRefStr, Display, EnumIter};

use tauri::{AppHandle, Emitter as _};
use tokio::sync::RwLock;
use windows::{
    Win32::UI::WindowsAndMessaging::{
            CopyIcon, HCURSOR, HICON, IDC_ARROW, IMAGE_CURSOR, LR_SHARED, LoadCursorFromFileW, LoadCursorW, LoadImageW, OCR_CROSS, OCR_HAND, OCR_IBEAM, OCR_NORMAL, OCR_SIZEALL, OCR_SIZENESW, OCR_SIZENS, OCR_SIZENWSE, OCR_SIZEWE, OCR_WAIT, SYSTEM_CURSOR_ID, SetSystemCursor, ShowCursor
        }, 
    core::PCWSTR
};

use crate::state::cursors::CursorState;

/// Wrapper per rendere HCURSOR compatibile con i thread
#[derive(Clone, Copy, Debug)]
pub struct SafeHCursor(pub HCURSOR);

// Implementiamo i trait "pericolosi" manualmente. 
// È sicuro perché gli handle di Windows sono validi globalmente nel processo.
unsafe impl Send for SafeHCursor {}
unsafe impl Sync for SafeHCursor {}

#[repr(u32)]
#[derive(
    Clone, Copy, Debug, Serialize, Deserialize, PartialEq, 
    Eq, Hash, EnumIter, AsRefStr, JsonSchema
)]
pub enum CursorType {
    Normal = OCR_NORMAL.0,
    Hand = OCR_HAND.0,
    IBeam = OCR_IBEAM.0,
    Wait = OCR_WAIT.0,
    Cross = OCR_CROSS.0,
    SizeAll = OCR_SIZEALL.0,
    SizeNESW = OCR_SIZENESW.0,
    SizeNS = OCR_SIZENS.0,
    SizeNWSE = OCR_SIZENWSE.0,
    SizeWE = OCR_SIZEWE.0
}

impl From<&str> for CursorType {
    fn from(value: &str) -> Self {
        match value.trim().to_lowercase().as_str() {
            "hand" => CursorType::Hand,
            "ibeam" => CursorType::IBeam,
            "wait" => CursorType::Wait,
            "cross" => CursorType::Cross,
            "sizeall" | "size-all" => CursorType::SizeAll,
            "sizenesw" | "size-nesw" => CursorType::SizeNESW,
            "sizens" | "size-ns" => CursorType::SizeNS,
            "sizenwse" | "size-nwse" => CursorType::SizeNWSE,
            "sizewe" | "size-we" => CursorType::SizeWE,
            "normal" | _ => CursorType::Normal
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, AsRefStr, Display)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum CursorEvent {
    AnimationStarted { target: Cursor },
    AnimationFinished { target: Cursor },
    AnimationStopped { target: Cursor },
    AnimationRestarted { target: Cursor },

    CursorChanged { cursor_type: CursorType, previous: Option<Cursor>, current: Cursor },
    CursorReset,

    Hidden,
    Shown,
    Frame {
        #[serde(flatten)]
        frame: Frame, 
        current_index: Option<usize>,
        total_frames: Option<usize>
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Frame {
    pub path: String,                   // Percorso del file (.cur, .png, etc.)
    pub delay_ms: Option<u32>,          // Durata specifica del frame (opzionale)
    pub hotspot: Option<(u32, u32)>,    // Rappresenta il punto del cursore da cui parte il click
    
    #[serde(skip)]
    pub hcursor: Option<SafeHCursor>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum Cursor {
    Static {
        #[serde(flatten)]
        frame: Frame
    },
    Animated {
        #[serde(flatten)]
        frames: Vec<Frame>,
        current_frame: usize,
        fps: u32
    },
}

impl Cursor {
    pub fn is_animated(&self) -> bool {
        matches!(self, Cursor::Animated { .. })
    }
}

impl CursorState {
    pub fn new(app: AppHandle) -> Result<Self, String> {
        let mut original = HashMap::new();

        unsafe {
            for id in CursorType::iter() {
                // Prendiamo l'handle ufficiale di sistema per questo specifico ID
                let h_handle = LoadImageW(
                    None,
                    PCWSTR(id as usize as *const u16),
                    IMAGE_CURSOR,
                    0, 0,
                    LR_SHARED
                ).map_err(|e| e.to_string())?;

                if !h_handle.is_invalid() {
                    let h_icon = HICON(h_handle.0);

                    // Creiamo la nostra copia privata
                    let copy = CopyIcon(h_icon).map_err(|e| e.to_string())?;
                    
                    if !copy.is_invalid() {
                        original.insert(id.clone(), SafeHCursor(HCURSOR(copy.0)));
                    }
                }
            }
        }

        Ok(Self {
            app,
            current: HashMap::new(),
            original,
            visible: true,
            paused: false,
            active: None,
        })
    }

    fn emit(&self, event: CursorEvent) -> Result<(), String> {
        self.app.emit(event.as_ref(), &event)
            .map_err(|e| format!("Error sending event '{event}': {e}"))
    }

    pub fn set_animated_cursor(
        &mut self,
        cursor_type: CursorType,
        frames: Vec<Frame>,
        fps: u32
    ) -> Result<(), String> {
        let fps = fps.min(30).max(1);
        let previous = self.current
            .get(&cursor_type)
            .cloned();

        let current = Cursor::Animated {
            frames: frames,
            current_frame: 0,
            fps
        };

        self.emit(CursorEvent::CursorChanged { 
            cursor_type: cursor_type.clone(), 
            previous: previous, 
            current: current.clone() 
        }).expect("Error sending cursor changed event");

        self.current.insert(cursor_type, current);

        Ok(())
    }

    pub fn set_cursor(
        &mut self,
        path: &str,
        cursor_type: CursorType,
    ) -> Result<(), String> {
        let previous = self.current
            .get(&cursor_type)
            .cloned();

        /*
        let hcursor = unsafe {
            let wide: Vec<u16> = path
                .encode_utf16()
                .chain(Some(0))
                .collect();

            let cursor = LoadCursorFromFileW(PCWSTR(wide.as_ptr()))
                .map_err(|e| e.to_string())?;

            let h_copy = CopyIcon(HICON(cursor.0))
                .map_err(|e| format!("Fallita la copia del cursore: {}", e))?;

            SetSystemCursor(HCURSOR(h_copy.0), SYSTEM_CURSOR_ID(cursor_type as u32));

            cursor
        };
        */

        let frame = Frame { 
            path: path.to_string(), 
            delay_ms: None, 
            hotspot: None,
            hcursor: None
        };

        let current = Cursor::Static { frame };

        self.emit(CursorEvent::CursorChanged { 
            cursor_type: cursor_type.clone(), 
            previous: previous, 
            current: current.clone() 
        }).expect("Error sending cursor changed event");

        self.current.insert(cursor_type, current);

        Ok(())
    }

    pub fn play_cursor(&mut self, cursor_type: CursorType) {
        self.active = Some(cursor_type)
    }

    pub fn stop_cursor(&mut self) {
        self.active = None;
    }

    pub fn pause_cursor(&mut self) {
        self.paused = true;
    }

    pub fn hide(&mut self) -> Result<(), String> {
        if self.visible {
            unsafe { ShowCursor(false); }
            self.visible = false;
            self.emit(CursorEvent::Hidden)?;
        }

        Ok(())
    }

    pub fn show(&mut self) -> Result<(), String> {
        if !self.visible {
            unsafe { ShowCursor(true); }
            self.visible = true;
            self.emit(CursorEvent::Shown)?;
        }

        Ok(())
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn is_animated(&self, cursor_type: &CursorType) -> bool {
        self.current
            .get(cursor_type)
            .map(|c| c.is_animated())
            .unwrap_or(false)
    }
}

impl Drop for CursorState {
    fn drop(&mut self) {
        self.active = None;

        unsafe {
            // 2. Usiamo drain() per prendere la proprietà degli handle
            // mentre svuotiamo la mappa.
            for (id, cursor) in self.original.drain() {
                let handle_to_restore = if !cursor.0.is_invalid() {
                    cursor.0
                } else {
                    // Se il backup è rotto, carichiamo il default assoluto di Windows
                    // IDC_ARROW è il fallback sicuro per quasi tutto
                    LoadCursorW(None, IDC_ARROW).unwrap_or_default()
                };

                if !handle_to_restore.is_invalid() {
                    let _ = SetSystemCursor(handle_to_restore, SYSTEM_CURSOR_ID(id as u32));
                }
            }
        }
    }
}

pub fn start_animation_thread(cm: Arc<RwLock<CursorState>>) -> Result<(), String> {
    let app = {
        cm.blocking_read().app.clone()
    };

    thread::spawn(move || {
        // Valore di default se non c'è nulla in riproduzione
        let mut current_wait = Duration::from_millis(33); 

        loop {
            let start_time = std::time::Instant::now();

            // --- FASE 1: Estrazione dati (LOCK CORTO) ---
            let animation_step = {
                let mut cm_guard = cm.blocking_write();
                
                if let Some(c_type) = cm_guard.active && !cm_guard.paused {
                    if let Some(cursor) = cm_guard.current.get_mut(&c_type) {
                        match cursor {
                            Cursor::Animated { frames, current_frame, fps } => {
                                if frames.is_empty() {
                                    None
                                } else {
                                    let idx = *current_frame % frames.len();
                                    let frame = &frames[idx];
                                    
                                    // Calcoliamo la durata basandoci sull'FPS specifico dell'animazione
                                    // Assicuriamoci che fps sia almeno 1 per evitare divisioni per zero
                                    let target_fps = (*fps).max(1);
                                    current_wait = Duration::from_millis(1000 / target_fps as u64);

                                    *current_frame = (idx + 1) % frames.len();
                                    
                                    Some((c_type, frame.clone(), current_frame.clone(), frames.len()))
                                }
                            },
                            Cursor::Static { frame } => {
                                // Per i cursori statici, impostiamo un wait di default (es. 100ms) 
                                // per non martellare la CPU inutilmente
                                current_wait = Duration::from_millis(100);
                                Some((c_type, frame.clone(), 0, 0))
                            }
                        }
                    } else { 
                        current_wait = Duration::from_millis(100);
                        None 
                    }
                } else { 
                    current_wait = Duration::from_millis(100);
                    None 
                }
            };

            // --- FASE 2: Interazione con Windows ---
            if let Some((c_type, frame, current_frame, frames)) = animation_step {
                if let Some(hcursor) = frame.hcursor {
                    unsafe {
                        if !hcursor.0.is_invalid() {
                            if let Ok(h_copy) = CopyIcon(HICON(hcursor.0.0)) {
                                let _ = SetSystemCursor(
                                    HCURSOR(h_copy.0), 
                                    SYSTEM_CURSOR_ID(c_type as u32)
                                );

                                let event = CursorEvent::Frame { 
                                    frame: frame, 
                                    current_index: if frames > 0 { Some(current_frame) } else { None }, 
                                    total_frames: if frames > 0 { Some(frames) } else { None }
                                };

                                app
                                    .emit(event.as_ref(), &event)
                                    .expect("Error sending frame event");
                            }
                        }
                    }
                }
            }

            // --- FASE 3: Timing Dinamico ---
            let elapsed = start_time.elapsed();
            if elapsed < current_wait {
                thread::sleep(current_wait - elapsed);
            }
        }
    });

    Ok(())
}