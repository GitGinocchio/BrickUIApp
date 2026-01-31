import { UnwatchFn, watch } from "@tauri-apps/plugin-fs";
import { appDataDir } from "../utils/path";
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

        const parts = path.split(/[/\\]/);
        const bricksIndex = parts.indexOf("bricks");
        const brickName = parts[bricksIndex + 1];

        const brick = await invoke<Brick>("get_brick_by_name", {
          name: brickName,
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
