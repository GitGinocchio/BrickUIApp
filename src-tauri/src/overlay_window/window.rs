use tauri::{Manager, Window, WindowBuilder, Runtime};

pub struct Overlay<R: Runtime> {
    window: Window<R>,
}

impl<R: Runtime> Overlay<R> {
    pub fn new<M: Manager<R>>(manager: &M, label: &str) -> tauri::Result<Self> {
        let window = WindowBuilder::new(manager, label)
            .always_on_top(true)
            .decorations(false)
            .drag_and_drop(true)
            .fullscreen(false)
            .maximized(true)
            .shadow(false)
            .skip_taskbar(true)
            .inner_size(100.0, 100.0)
            .transparent(true)
            .window_classname("TransparentOverlay")
            .focused(true)
            .build()?;

        Ok(Self { window })
    }



    pub fn show(&self) -> Result<(), tauri::Error> {
        self.window.show()
    }
}
