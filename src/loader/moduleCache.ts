// src/loader/moduleCache.ts
import * as Vue from "vue";
import core from "../api/core";
import event from "../api/event";
import type { InvokeArgs, InvokeOptions } from "@tauri-apps/api/core";

export interface BrickContext {
  name: string;
  path: string;
}

export function createModuleCache(ctx: BrickContext) {
  const defaultOptions = {
    caller: {
      name: ctx.name,
      path: ctx.path,
    }
  };

  return {
    vue: {
      ...Vue,
      createApp: () => {
        throw new Error(
          `Access denied: brick "${ctx.name}" is not allowed to create a new Vue app.`
        );
      }
    },

    core: {
      ...core,
      invoke: (cmd: string, args?: InvokeArgs, options?: InvokeOptions) =>
        core.invoke(cmd, args, { ...options, ...defaultOptions }),
      convertFileSrc: (path: string, protocol?: string) =>
        core.convertFileSrc(path, { protocol, root: ctx.path })
    },

    event: event
  };
}
