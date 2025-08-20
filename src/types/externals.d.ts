
declare module "@public/js/externals.js" {
  export interface ScriptConfig {
    src: string;
    type?: string;
    async?: boolean;
    defer?: boolean;
    integrity?: string;
    crossorigin?: string;
  }

  export interface StyleConfig {
    href: string;
    rel?: string;
    media?: string;
    integrity?: string;
    crossorigin?: string;
  }

  export const scripts: ScriptConfig[] = [];
  export const styles: StyleConfig[] = [];
}