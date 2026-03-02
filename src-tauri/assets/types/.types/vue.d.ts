// vue.d.ts
/// <reference path="./vue/runtime-core.d.ts" />
/// <reference path="./vue/runtime-dom.d.ts" />
/// <reference path="./vue/reactivity.d.ts" />
/// <reference path="./vue/shared.d.ts" />
/// <reference path="./vue/csstype.d.ts" />

export * from "./vue/runtime-dom";
export { WatchOptions, computed, watch } from "./vue/runtime-dom";
export * from "./vue/reactivity";
export * from "./vue/csstype";
export * from "./vue/shared";

// Shim per i file .vue
declare module '*.vue'