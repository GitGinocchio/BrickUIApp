import { reactive, markRaw, type Component } from "vue";

export const bricksCache = reactive(new Map<string, Component>());

export function addBrickToCache(name: string, component: Component) {
  bricksCache.set(name, markRaw(component));
}

export function removeBrickFromCache(name: string) {
  bricksCache.delete(name);
}

export function isBrickCached(name: string): boolean {
  return bricksCache.has(name);
}

export function getBrickFromCache(name: string): Component | undefined {
  return bricksCache.get(name);
}