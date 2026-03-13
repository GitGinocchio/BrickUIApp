
/**
 * Command arguments.
 *
 * @since 1.0.0
 */
type InvokeArgs = Record<string, unknown> | number[] | ArrayBuffer | Uint8Array;
/**
 * @since 2.0.0
 */
interface InvokeOptions {
    headers: HeadersInit;
}

type InvokeCommand =
  | "get_bricks"
  | "get_settings"

  | "get_taskbar_apps"
  | "get_active_taskbar_apps"
  | "get_pinned_taskbar_apps"

  | "bluetooth_get_devices"
  | "bluetooth_pair"
  | "bluetooth_pair_confirm"
  | "bluetooth_pair_provide_pin"
  | "bluetooth_pair_provide_address"
  | "bluetooth_unpair"
  | "bluetooth_connect"
  | "bluetooth_disconnect"

  | "open_start_menu"
  | "get_start_menu_favorites"

  | "get_explorer_recents"

  | "get_maximized_window_for_monitor"
  | "get_maximized_windows"
  | "get_windows_in_monitor"
  | "get_all_windows"

  | "set_workarea_margins"
  | "set_workareas_margins"

  | "get_all_monitors"
  | "get_monitor_from_point"
  | "get_primary_monitor"
  | "get_monitor";

/**
 * Sends a message to the backend.
 * @example
 * ```typescript
 * import { invoke } from '@tauri-apps/api/core';
 * await invoke('login', { user: 'tauri', password: 'poiwe3h4r5ip3yrhtew9ty' });
 * ```
 *
 * @param cmd The command name.
 * @param args The optional arguments to pass to the command.
 * @param options The request options.
 * @return A promise resolving or rejecting to the backend response.
 *
 * @since 1.0.0
 */
declare function invoke<T>(cmd: InvokeCommand, args?: InvokeArgs, options?: InvokeOptions): Promise<T>;
/**
 * Convert a device file path to an URL that can be loaded by the webview.
 * Note that `asset:` and `http://asset.localhost` must be added to [`app.security.csp`](https://v2.tauri.app/reference/config/#csp-1) in `tauri.conf.json`.
 * Example CSP value: `"csp": "default-src 'self' ipc: http://ipc.localhost; img-src 'self' asset: http://asset.localhost"` to use the asset protocol on image sources.
 *
 * Additionally, `"enable" : "true"` must be added to [`app.security.assetProtocol`](https://v2.tauri.app/reference/config/#assetprotocolconfig)
 * in `tauri.conf.json` and its access scope must be defined on the `scope` array on the same `assetProtocol` object.
 *
 * @param  filePath The file path.
 * @param  protocol The protocol to use. Defaults to `asset`. You only need to set this when using a custom protocol.
 * @example
 * ```typescript
 * import { appDataDir, join } from '@tauri-apps/api/path';
 * import { convertFileSrc } from '@tauri-apps/api/core';
 * const appDataDirPath = await appDataDir();
 * const filePath = await join(appDataDirPath, 'assets/video.mp4');
 * const assetUrl = await convertFileSrc(filePath);
 *
 * const video = document.getElementById('my-video');
 * const source = document.createElement('source');
 * source.type = 'video/mp4';
 * source.src = assetUrl;
 * video.appendChild(source);
 * video.load();
 * ```
 *
 * @return the URL that can be used as source on the webview.
 *
 * @since 1.0.0
 */
declare function convertFileSrc(filePath: string, protocol?: string): Promise<string>;

export type { InvokeArgs, InvokeOptions };
export { invoke, convertFileSrc };
