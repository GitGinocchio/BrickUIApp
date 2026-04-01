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

mod bluetooth;
use bluetooth::*;

mod api;
use api::*;

#[cfg_attr(feature = "profiling", tracing::instrument)]
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
        //get_tray_icons,
        // Bluetooth
        bluetooth_get_devices,
        bluetooth_pair,
        bluetooth_pair_confirm,
        bluetooth_pair_provide_pin,
        bluetooth_pair_provide_address,
        
        bluetooth_unpair,

        /*
        Metodi scan deprecati a favore del bluetooth_watcher
        bluetooth_scan,
        bluetooth_win32_scan,
        bluetooth_winrt_scan,
        */

        /*
        Al momento non supportiamo connessione diretta via socket
        bluetooth_connect,
        bluetooth_disconnect,
        */
        // Explorer
        open_file_folder,
        get_explorer_recents,
        // Start Menu
        open_start_menu,
        get_start_menu_favorites,
        // Cursor
        hide_all_cursors,
        restore_all_cursors,
        // Window
        get_monitor_maximized_window,
        get_maximized_windows,
        get_monitor_windows,
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
        unpack_brick,
        
        // Api
        auth_register,
        auth_login,
        auth_resend_email,

        auth_is_logged_in,
        auth_is_session_expired,
        auth_get_identity,

        users_get_me,
        users_update_me
    ]
}
