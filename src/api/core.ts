import * as core from '@tauri-apps/api/core';
import { normalizePath } from '../utils/normUtils';

const allowedCommands = new Set<string>([
  "get_bricks",
  "get_taskbar_apps",
  "get_active_taskbar_apps",
  "get_pinned_taskbar_apps",
  "open_start_menu",
  "get_start_menu_favorites",
  "get_explorer_recents",
  "hide_all_cursors",
  "restore_all_cursors"
]);

async function invoke(cmd: string, args?: core.InvokeArgs, options?: core.InvokeOptions): Promise<any> {
  if (!allowedCommands.has(cmd)) {
    const error = new Error(`Command "${cmd}" is forbidden`);
    return Promise.reject(error);
  }
  return core.invoke(cmd, args, options);
}

function convertFileSrc(filePath: string, options?: { protocol?: string; root?: string }): string {
  return normalizePath(filePath, options)
}

export default { invoke, convertFileSrc };