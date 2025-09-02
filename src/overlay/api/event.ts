import * as event from '@tauri-apps/api/event';
import { catchBrickError } from '../utils/errors';

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

async function emit(e: string, payload: any): Promise<void> {
  if (!allowedEmitEvents.has(e)) {
    const error = new Error(`Event "${e}" is forbidden`);

    catchBrickError(error);

    return Promise.reject(error);
  }
  await event.emit(e, payload);
}

async function emitTo(target: string | event.EventTarget, e: string, payload: any): Promise<void> {
  if (!allowedEmitToEvents.some(entry => entry.target === target && entry.event === e)) {
    const error = new Error(`Event and/or target forbidden: "${e}" with "${target}"`);

    catchBrickError(error);

    return Promise.reject(error);
  }
  await event.emitTo(target, e, payload);
}

async function listen(e: event.EventName, handler: event.EventCallback<unknown>, options?: event.Options): Promise<event.UnlistenFn> {
  if (!allowedListenEvents.has(e)) {
    const error = new Error(`Event "${e}" is forbidden`);

    catchBrickError(error);

    return Promise.reject(error);
  }

  return await event.listen(e, handler, options);
}

async function once(e: event.EventName, handler: event.EventCallback<unknown>, options?: event.Options): Promise<event.UnlistenFn> {
  if (!allowedOnceEvents.has(e)) {
    const error = new Error(`Event "${e}" is forbidden`);

    catchBrickError(error);

    return Promise.reject(error);
  }

  return await event.once(e, handler, options);
}

export default { listen, once, emit, emitTo };