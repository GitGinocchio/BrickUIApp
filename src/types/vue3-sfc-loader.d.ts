declare module "@vue3-sfc-loader.esm.js" {
  import type { ParserPlugin as babel_ParserPlugin } from "@babel/parser";

  export type ValueFactoryApi = {
    preventCache(): void;
  };

  export type ValueFactory = (api: ValueFactoryApi) => Promise<any>;

  export type Cache = {
    get(key: string): Promise<string | undefined>;
    set(key: string, value: string): Promise<void>;
  };

  export type ModuleCacheId = string;

  export type AbstractPath = {
    toString(): string;
  };

  export type PathContext = {
    refPath: AbstractPath | undefined;
    relPath: AbstractPath;
  };

  export type PathResolve = (
    pathCx: PathContext,
    options: Options
  ) => AbstractPath;

  export type ModuleHandler = (
    type: string,
    getContentData: File["getContentData"],
    path: AbstractPath,
    options: Options
  ) => Promise<ModuleExport | null>;

  export type ContentData = string | ArrayBuffer;

  export type File = {
    getContentData: (asBinary: Boolean) => Promise<ContentData>;
    type: string;
  };

  export type Resource = {
    id: ModuleCacheId;
    path: AbstractPath;
    getContent: () => Promise<File>;
  };

  export type CustomBlockCallback = (component: ModuleExport) => void;

  export type CustomBlock = {
    type: string;
    content: string;
    attrs: Record<string, string | true>;
  };

  export type ModuleExport = {} | null;

  export type Module = {
    exports: ModuleExport;
  };

  export type LoadingType<T> = {
    promise: Promise<T>;
  };

  export type Options = {
    moduleCache: Record<ModuleCacheId, LoadingType<ModuleExport> | ModuleExport>;

    getFile(path: AbstractPath): Promise<File | ContentData>;

    addStyle(style: string, scopeId: string | undefined): void;

    delimiters?: [string, string];

    whitespace?: "preserve" | "condense";

    isCustomElement: ((tag: string) => boolean) | undefined;

    additionalBabelParserPlugins?: babel_ParserPlugin[];

    additionalBabelPlugins?: Record<string, any>;

    handleModule?: ModuleHandler;

    compiledCache?: Cache;

    log?(type: string, ...data: any[]): void;

    loadModule?(
      path: AbstractPath,
      options: Options
    ): Promise<ModuleExport | undefined>;

    createCJSModule(
      refPath: AbstractPath,
      source: string,
      options: Options
    ): Module;

    pathResolve: PathResolve;

    getPathname: (path: string) => string;

    getResource(pathCx: PathContext, options: Options): Resource;

    customBlockHandler?(
      block: CustomBlock,
      filename: AbstractPath,
      options: Options
    ): Promise<CustomBlockCallback | undefined>;

    devMode?: boolean;

    processStyles(
      srcRaw: string,
      lang: string | undefined,
      filename: AbstractPath,
      options: Options
    ): Promise<string>;
  };

  export type LangProcessor = (
    source: string,
    preprocessOptions?: any
  ) => Promise<string> | string;

  export function loadModule(
    path: string,
    options: Options
  ): Promise<ModuleExport>;
}
