
declare module "@public/js/utils.js" {
  export function appendScript(
    src: string,
    type?: string,
    async?: boolean,
    defer?: boolean,
    integrity?: string,
    crossorigin?: string
  ): Promise<void>;

  export function appendStyle(
    href: string,
    rel?: string,
    media?: string | null,
    integrity?: string | null,
    crossorigin?: string | null
  ): Promise<void>;
}