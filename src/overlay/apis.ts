import * as fs from '@tauri-apps/plugin-fs';
import * as path from '@tauri-apps/api/path';
import * as core from '@tauri-apps/api/core';
//import type * as Tauri from '@tauri-apps/api/';

const allowedCommands = new Set<string>([
]);

async function invoke(cmd: string, args?: core.InvokeArgs, options?: core.InvokeOptions): Promise<any> {
  if (!allowedCommands.has(cmd)) {
    throw new Error(`Command not permitted: ${cmd}`);
  }
  return core.invoke(cmd, args, options);
}

const tauri_methods = {
  fs: fs,
  path: path,
  core: {
    convertFileSrc: (filePath: string, protocol?: string) => core.convertFileSrc(filePath, protocol),
    invoke: (cmd: string, args?: core.InvokeArgs, options?: core.InvokeOptions) => invoke(cmd, args, options)
  }
}

export function expose() {
  // @ts-ignore
  if (window.tauri !== undefined) {
    return;
  }

  Object.defineProperty(window, 'tauri', {
    value: tauri_methods,
    writable: false,
    configurable: false,
  });

  // keep this for compatibility
  Object.defineProperty(window, '__TAURI__', {
    value: tauri_methods,
    writable: false,
    configurable: false,
  });
}