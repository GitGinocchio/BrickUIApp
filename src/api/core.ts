import * as core from '@tauri-apps/api/core';
import { sanitizePath } from '../utils/path';

const allowedCommands = new Set<string>([
  // Bricks
  "get_bricks",

  // Settings
  "get_settings",
  
  // Taskbar / Apps
  "get_taskbar_apps",
  "get_active_taskbar_apps",
  "get_pinned_taskbar_apps",

  // Bluetooth
  "bluetooth_scan",
  "bluetooth_classic_scan",
  "bluetooth_connect",
  "bluetooth_disconnect",
  //"get_default_adapter",
  //"get_adapters",
  //"get_devices",

  // Taskbar / Tray icons
  //"get_tray_icons",

  // StarMenu
  "open_start_menu",
  "get_start_menu_favorites",

  // Explorer
  "get_explorer_recents",

  // Cursors
  "hide_all_cursors",
  "restore_all_cursors",
 
  // Window
  "get_maximized_window_for_monitor",
  "get_maximized_windows",
  "get_windows_in_monitor",
  "get_all_windows",

  // Workarea
  "set_workarea_margins",
  "set_workareas_margins",

  // Monitor
  "get_all_monitors",
  "get_monitor_from_point",
  "get_primary_monitor",
  "get_monitor"
]);

async function invoke(cmd: string, args?: core.InvokeArgs, options?: core.InvokeOptions): Promise<any> {
  if (!allowedCommands.has(cmd)) {
    const error = new Error(`Command "${cmd}" is forbidden`);
    return Promise.reject(error);
  }
  return core.invoke(cmd, args, options);
}

async function convertFileSrc(filePath: string, options?: { protocol?: string; root?: string }): Promise<string> {
  // Non e' del tutto sicuro, mettendo root -> null viene presa come root {appDataDir}/BrickUI/ 
  // Ma non la cartella del brick chiamante
  return await sanitizePath(filePath, { ...options, root: null })
}

export default { invoke, convertFileSrc };