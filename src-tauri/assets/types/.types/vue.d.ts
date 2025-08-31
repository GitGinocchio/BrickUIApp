// vue.d.ts
declare module 'vue' {
  // -----------------------------------
  // Ref & Computed
  // -----------------------------------
  export interface Ref<T = any> {
    value: T
  }
  export function ref<T>(value: T): Ref<T>
  export function computed<T>(getter: () => T): Readonly<Ref<T>>

  // -----------------------------------
  // Lifecycle hooks
  // -----------------------------------
  export function onMounted(cb: () => void): void
  export function onUnmounted(cb: () => void): void
  export function onBeforeMount(cb: () => void): void
  export function onUpdated(cb: () => void): void

  // -----------------------------------
  // Props & Emits
  // -----------------------------------
  export type PropType<T> = { (): T } | { new (...args: any[]): T & object }

  export function defineProps<T = Record<string, any>>(props: Record<string, any>): Readonly<T>
  export function defineEmits<T extends Record<string, (...args: any[]) => any> | string[] = string[]>(): T

  // -----------------------------------
  // Watch
  // -----------------------------------
  export function watch<T>(
    source: () => T,
    cb: (newVal: T, oldVal: T) => void
  ): void

  export function watchEffect(effect: () => void): void

  // -----------------------------------
  // Component helpers
  // -----------------------------------
  export function defineComponent<
    Props = {},
    RawBindings = {},
    Data = {},
    Computed = {},
    Methods = {}
  >(options: any): any
}