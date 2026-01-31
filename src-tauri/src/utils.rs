use tauri::{AppHandle, Manager as _};



pub fn focus_window(app: &AppHandle, window_name: &str) -> Result<(), String> {
    let webview_window = app
        .get_webview_window(window_name)
        .ok_or(format!("Could not find window: '{window_name}'"))?;

    webview_window
        .unminimize()
        .map_err(|e| format!("Error unminizing webview window: {e}"))?;

    if !webview_window.is_visible().unwrap_or(false) {
        webview_window
            .show()
            .map_err(|e| format!("Error showing webview window: {e}"))?;
    }

    webview_window
        .set_focus()
        .map_err(|e| format!("Error focusing webview window: {e}"))
}