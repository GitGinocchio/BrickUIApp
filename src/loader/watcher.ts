import { type UnwatchFn, watch } from "@tauri-apps/plugin-fs";
import { appDataDir } from "#utils/path";
import { invoke } from "@tauri-apps/api/core";
import type { Brick } from "#interfaces/brick";
import { reloadBrick } from "./index";

let unWatch: UnwatchFn = undefined;

// Mappa per tenere traccia dei timer attivi per ogni brick
const debounceTimers = new Map<string, ReturnType<typeof setTimeout>>();

export async function initBrickWatcher() {
  if (unWatch) {
    console.warn("Watcher already initialized");
    stopBrickWatcher();
  }

  unWatch = await watch(
    `${appDataDir}/bricks/`,
    (event) => {
      event.paths.forEach((path) => {
        if (!path.endsWith(".vue")) return;

        const parts = path.split(/[/\\]/);
        const bricksIndex = parts.indexOf("bricks");
        const brickName = parts[bricksIndex + 1];

        if (!brickName) return;

        // --- Logica di Debounce ---
        if (debounceTimers.has(brickName)) {
          clearTimeout(debounceTimers.get(brickName));
        }

        const timer = setTimeout(async () => {
          try {
            const brick = await invoke<Brick>("get_brick_by_name", {
              name: brickName,
            });
            await reloadBrick(brick);
            console.log(`Brick reloaded: ${brickName}`);
          } catch (err) {
            console.error(`Failed to reload brick ${brickName}:`, err);
          } finally {
            debounceTimers.delete(brickName);
          }
        }, 300); // 300ms è solitamente sufficiente per il FS

        debounceTimers.set(brickName, timer);
      });
    },
    { recursive: true }
  );

  console.log("Brick watcher initialized");
}

export function stopBrickWatcher() {
  if (!unWatch) return;

  unWatch();
  // Pulizia dei timer pendenti
  debounceTimers.forEach(clearTimeout);
  debounceTimers.clear();
  
  unWatch = undefined;
  console.log("Brick watcher uninitialized");
}