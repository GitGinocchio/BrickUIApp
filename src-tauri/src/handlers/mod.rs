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


pub fn generate_handlers() -> impl Fn(tauri::ipc::Invoke<tauri::Wry>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        get_settings,
        save_settings,
        
        show_taskbar,
        hide_taskbar,
        get_active_taskbar_apps,
        get_pinned_taskbar_apps,
        get_taskbar_apps,

        get_explorer_recents,

        open_start_menu,
        get_start_menu_favorites,

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

