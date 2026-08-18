import { getCurrentInstance } from "vue";
import type { ComponentPublicInstance } from "vue";
import { app } from "../loader";
import { invoke } from "@tauri-apps/api/core";

// Interfaccia per il contesto del Brick personalizzato
export interface BrickContext {
  name: string;
  author?: string;
}

export interface ExtendedError extends Error {
  timestamp?: string;
  line?: number;
  column?: number;
}

// Tipo esteso per l'istanza Vue con le opzioni del Brick
type BrickComponentInstance = ComponentPublicInstance & {
  $options?: {
    __brickContext?: BrickContext;
  };
  $parent?: BrickComponentInstance | null;
};

// Tipo flessibile per l'istanza o contesto del brick
export type BrickInstance = BrickComponentInstance | BrickContext | null | undefined;

/**
 * Cattura un errore interno del Brick e lo inoltra all'errorHandler di Vue/Nuxt
 */
export function catchBrickError(
  error: Error,
  brickContext?: BrickContext,
  info: string = ''
) {
  const instance = getCurrentInstance();
  const extError = error as ExtendedError;

  extError.timestamp = new Date().toISOString();

  // Estrai linea e colonna dallo stack dell'errore
  const stack = error.stack;
  if (stack) {
    const match = stack.match(/:(\d+):(\d+)\)?$/);
    if (match) {
      extError.line = parseInt(match[1], 10);
      extError.column = parseInt(match[2], 10);
    }
  }

  if (!instance) {
    app.config.errorHandler?.(error, brickContext as unknown as ComponentPublicInstance, info);
    return;
  }

  instance.appContext.config.errorHandler?.(error, instance as unknown as ComponentPublicInstance, info);
}

export function catchBrickWarning(warn: Error) {
  const instance = getCurrentInstance()?.appContext ?? app;
  instance.config.warnHandler?.(warn.message, instance as unknown as ComponentPublicInstance, warn.stack ?? '');
}

function extractBrickContext(instance: BrickInstance): { name?: string; author?: string } {
  if (!instance) return {};

  let current: BrickInstance = instance;
  let brickName: string | undefined;
  let brickAuthor: string | undefined;

  if ("$options" in current && current.$options) {
    while (current && "$options" in current && !current.$options?.__brickContext) {
      current = current.$parent ?? null;
    }
    if (current && "$options" in current) {
      brickName = current.$options?.__brickContext?.name;
      brickAuthor = current.$options?.__brickContext?.author;
    }
  } else if ("name" in current) {
    brickName = current.name;
    brickAuthor = current.author;
  }

  return { name: brickName, author: brickAuthor };
}

export async function onBrickError(
  toast: ReturnType<typeof useToast>,
  error: Error,
  instance: BrickInstance,
  info: string = ''
) {
  console.error(error, instance, info);
  if (!instance) return;
  if ("type" in instance) return;

  const { name: brickName, author: brickAuthor } = extractBrickContext(instance);
  const authorSuffix = brickAuthor && brickAuthor !== "undefined" ? ` by ${brickAuthor}` : "";
  const title = `Error ${info ? `in ${info}` : ""} — brick "${brickName ?? 'Unknown'}"${authorSuffix}`;

  toast.add({
    title,
    description: `${error.message ?? error}\n${error.cause ?? ""}`,
    color: 'error',
    icon: 'i-lucide-alert-circle',
    duration: 10000,
    actions: brickName ? [
      {
        label: 'Open Brick',
        color: 'neutral',
        variant: 'outline',
        onClick: async () => {
          // @ts-ignore - Invoke di Tauri / IPC
          if (typeof invoke !== 'undefined') {
            await invoke("open_brick", { brickName });
          }
        }
      }
    ] : []
  });
}

/**
 * Mostra una notifica di Warning tramite Nuxt UI useToast()
 */
export async function onBrickWarn(
  toast: ReturnType<typeof useToast>,
  message: string,
  instance: BrickInstance,
  _trace: string
) {
  console.warn(message, instance, _trace);
  if (!instance) return;

  const { name: brickName, author: brickAuthor } = extractBrickContext(instance);
  const authorSuffix = brickAuthor && brickAuthor !== "undefined" ? ` by ${brickAuthor}` : "";
  const title = `Warn in brick "${brickName ?? 'Unknown'}"${authorSuffix}`;

  toast.add({
    title,
    description: message,
    color: 'warning',
    icon: 'i-lucide-alert-triangle',
    duration: 10000,
    actions: brickName ? [
      {
        label: 'Open Brick',
        color: 'neutral',
        variant: 'outline',
        onClick: async () => {
          // @ts-ignore - Invoke di Tauri / IPC
          if (typeof invoke !== 'undefined') {
            await invoke("open_brick", { brickName });
          }
        }
      }
    ] : []
  });
}