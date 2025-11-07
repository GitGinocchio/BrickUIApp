mod bricks;
use bricks::*;

mod taskbar;
use taskbar::*;

mod settings;
use settings::*;

mod explorer;
use explorer::*;

mod startmenu;
use startmenu::*;

mod cursor;
use cursor::*;

mod window;
use window::*;

mod monitor;
use monitor::*;

mod workarea;
use workarea::*;

pub fn generate_handlers() -> impl Fn(tauri::ipc::Invoke<tauri::Wry>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        // Taskbar
        show_taskbar,
        hide_taskbar,
        is_taskbar_autohide,

        // Taskbar / Apps
        get_active_taskbar_apps,
        get_pinned_taskbar_apps,
        get_taskbar_apps,

        // Taskbar / Tray icons
        get_tray_icons,

        // Explorer
        get_explorer_recents,

        // Start Menu
        open_start_menu,
        get_start_menu_favorites,

        // Cursor
        hide_all_cursors,
        restore_all_cursors,

        // Window
        get_maximized_window_for_monitor,
        get_maximized_windows,
        get_windows_in_monitor,
        get_all_windows,

        // Workarea
        set_workarea_margins,
        set_workareas_margins,

        // Monitor
        get_all_monitors,
        get_monitor_from_point,
        get_primary_monitor,
        get_monitor,

        // Settings
        get_settings,
        save_settings,

        // Bricks
        get_bricks,
        get_brick_by_name,
        load_bricks,
        duplicate_brick,
        delete_brick,
        rename_brick,
        save_brick,
        open_brick,
        new_brick,
        pack_brick,
        unpack_brick
    ]
}

