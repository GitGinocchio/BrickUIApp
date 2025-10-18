import { UnwatchFn, watch } from "@tauri-apps/plugin-fs";
import { appDataDir, dirname, filename } from "./utils";
import { invoke } from "@tauri-apps/api/core";
import { Brick } from "interfaces/brick";
import { reloadBrick } from "./index";

let unWatch: UnwatchFn = undefined;

export async function initBrickWatcher() {
  if (unWatch) {
    console.error("Watcher already initialized");
    stopBrickWatcher();
  }

  unWatch = await watch(
    `${appDataDir}/bricks/`,
    (event) => {
      event.paths.forEach(async (path) => {
        if (!path.endsWith(".vue")) return;

        const brick = await invoke<Brick>("get_brick_by_name", {
          name: filename(dirname(path)),
        });

        await reloadBrick(brick);
      });
    },
    { recursive: true, delayMs: 750 }
  );

  console.log("Brick watcher initialized");
}

export function stopBrickWatcher() {
  if (!unWatch) {
    console.error("Watcher not initialized");
    return;
  }

  unWatch();
  console.log("Brick watcher uninitialized");
}
