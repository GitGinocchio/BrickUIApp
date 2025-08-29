import * as fs from '@tauri-apps/plugin-fs';
import * as path from '@tauri-apps/api/path';
import * as core from '@tauri-apps/api/core';
import * as event from '@tauri-apps/api/event';
//import type * as Tauri from '@tauri-apps/api/';

const allowedCommands = new Set<string>([
  "get_bricks",
  "get_taskbar_icons"
]);

const allowedEmitEvents = new Set<string>([
]);

const allowedEmitToEvents: { target: string | event.EventTarget, event: string}[] = [

];

const allowedListenEvents = new Set<event.EventName>([
  "global_mouse_moved",
  "global_mouse_pressed",
  "global_mouse_released",
  "global_key_pressed",
  "global_key_released",
  "global_wheel",
  "toggle_brick",
  "update_prop"
]);

const allowedOnceEvents = new Set<event.EventName>([
  ...allowedListenEvents
]);

async function invoke(cmd: string, args?: core.InvokeArgs, options?: core.InvokeOptions): Promise<any> {
  if (!allowedCommands.has(cmd)) {
    throw new Error(`Command forbidden: "${cmd}"`);
  }
  return core.invoke(cmd, args, options);
}

async function emit(e: string, payload: any): Promise<void> {
  if (!allowedEmitEvents.has(e)) {
    throw new Error(`Event forbidden: "${e}"`);
  }
  await event.emit(e, payload);
}

async function emitTo(target: string | event.EventTarget, e: string, payload: any): Promise<void> {
  if (!allowedEmitToEvents.some(entry => entry.target === target && entry.event === e)) {
    throw new Error(`Event and/or target forbidden: "${e}" with "${target}"`);
  }
  await event.emitTo(target, e, payload);
}

async function listen(e: event.EventName, handler: event.EventCallback<unknown>, options?: event.Options): Promise<event.UnlistenFn> {
  if (!allowedListenEvents.has(e)) {
    throw new Error(`Event forbidden: "${e}"`);
  }

  return await event.listen(e, handler, options);
}

async function once(e: event.EventName, handler: event.EventCallback<unknown>, options?: event.Options): Promise<event.UnlistenFn> {
  if (!allowedOnceEvents.has(e)) {
    throw new Error(`Event forbidden: "${e}"`);
  }

  return await event.once(e, handler, options);
}

const tauri_methods = {
  fs: fs,
  path: path,
  core: {
    convertFileSrc: (filePath: string, protocol?: string) => core.convertFileSrc(filePath, protocol),
    invoke: (cmd: string, args?: core.InvokeArgs, options?: core.InvokeOptions) => invoke(cmd, args, options)
  },
  event: {
    TauriEvent: event.TauriEvent,
    emit: (event: string, payload: any) => emit(event, payload),
    emitTo: (target: string | event.EventTarget, event: string, payload: any) => emitTo(target, event, payload),
    listen: (e: event.EventName, handler: event.EventCallback<unknown>, options?: event.Options) => listen(e, handler, options),
    once: (e: event.EventName, handler: event.EventCallback<unknown>, options?: event.Options) => once(e, handler, options)
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