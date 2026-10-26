import { RpcDefinitionsToFunctions } from "devframe/rpc";
//#region src/node/rpc/functions/capabilities.d.ts
interface AssetsCapabilities {
  write: boolean;
  uploadExtensions: readonly string[] | '*';
}
/**
 * Lets the UI know upfront whether upload / rename / delete / mkdir are
 * available, so it can hide or disable those affordances proactively
 * instead of letting the user hit a "method not found" error, the same
 * `canWrite`-gating idea the git plugin's `GitStatus.canWrite` follows.
 */
export declare const capabilities: import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:capabilities", "query", readonly [], import("devframe/utils/simple-schema").SimpleSchema<{
  write: boolean;
  uploadExtensions: string[] | "*";
}, {
  write: boolean;
  uploadExtensions: string[] | "*";
}>, import("devframe").DevframeNodeContext>;
//#endregion
//#region src/node/rpc/functions/delete.d.ts
/** One request covers both single- and multi-select delete. */
export declare const deleteAssets: import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:delete", "action", readonly [import("devframe/utils/simple-schema").SimpleSchema<{
  paths: string[];
}, {
  paths: string[];
}>], import("devframe/utils/simple-schema").SimpleSchema<{
  deleted: string[];
}, {
  deleted: string[];
}>, import("devframe").DevframeNodeContext>;
//#endregion
//#region src/node/rpc/functions/list.d.ts
export declare const assetInfoSchema: import("devframe/utils/simple-schema").SimpleSchema<{
  path: string;
  type: "image" | "font" | "video" | "audio" | "text" | "other";
  publicPath: string;
  size: number;
  mtime: number;
  fsPath?: string | undefined;
}, {
  path: string;
  type: "image" | "font" | "video" | "audio" | "text" | "other";
  publicPath: string;
  size: number;
  mtime: number;
  fsPath?: string | undefined;
}>;
export declare const list: import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:list", "query", readonly [], import("devframe/utils/simple-schema").SimpleSchema<{
  path: string;
  type: "image" | "font" | "video" | "audio" | "text" | "other";
  publicPath: string;
  size: number;
  mtime: number;
  fsPath?: string | undefined;
}[], {
  path: string;
  type: "image" | "font" | "video" | "audio" | "text" | "other";
  publicPath: string;
  size: number;
  mtime: number;
  fsPath?: string | undefined;
}[]>, import("devframe").DevframeNodeContext>;
//#endregion
//#region src/node/rpc/functions/mkdir.d.ts
export declare const mkdir: import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:mkdir", "action", readonly [import("devframe/utils/simple-schema").SimpleSchema<{
  path: string;
}, {
  path: string;
}>], import("devframe/utils/simple-schema").SimpleSchema<void, void>, import("devframe").DevframeNodeContext>;
//#endregion
//#region src/node/rpc/functions/read-image-meta.d.ts
export declare const readImageMeta: import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:read-image-meta", "query", readonly [import("devframe/utils/simple-schema").SimpleSchema<string, string>], import("devframe/utils/simple-schema").SimpleSchema<{
  width?: number | undefined;
  height?: number | undefined;
  orientation?: number | undefined;
} | null, {
  width?: number | undefined;
  height?: number | undefined;
  orientation?: number | undefined;
} | null>, import("devframe").DevframeNodeContext>;
//#endregion
//#region src/node/rpc/functions/read-text.d.ts
export declare const readText: import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:read-text", "query", readonly [import("devframe/utils/simple-schema").SimpleSchema<string, string>, import("devframe/utils/simple-schema").SimpleSchema<number | undefined, number | undefined>], import("devframe/utils/simple-schema").SimpleSchema<string | null, string | null>, import("devframe").DevframeNodeContext>;
//#endregion
//#region src/node/rpc/functions/rename.d.ts
interface RenameArgs {
  /** Root-relative path of the asset to rename. */
  path: string;
  /**
   * New base name, without an extension; the original extension is
   * always preserved (mirrors Nuxt DevTools' rename dialog exactly). Any
   * extension included here is treated as part of the base name, not
   * stripped.
   */
  newName: string;
}
export declare const rename: import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:rename", "action", readonly [import("devframe/utils/simple-schema").SimpleSchema<{
  path: string;
  newName: string;
}, {
  path: string;
  newName: string;
}>], import("devframe/utils/simple-schema").SimpleSchema<{
  path: string;
  type: "image" | "font" | "video" | "audio" | "text" | "other";
  publicPath: string;
  size: number;
  mtime: number;
  fsPath?: string | undefined;
}, {
  path: string;
  type: "image" | "font" | "video" | "audio" | "text" | "other";
  publicPath: string;
  size: number;
  mtime: number;
  fsPath?: string | undefined;
}>, import("devframe").DevframeNodeContext>;
//#endregion
//#region src/node/rpc/functions/upload.d.ts
/** Streaming channel name, namespaced like every other RPC name in this plugin. */
export declare const UPLOAD_CHANNEL = "devframes:plugin:assets:upload";
/**
 * Allocates an upload and returns a streaming id. Actual bytes flow over
 * the `upload` channel opened in `setupAssets`; see the client-side
 * `useUpload` hook for the matching `rpc.streaming.upload()` call.
 */
export declare const upload: import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:upload", "action", readonly [import("devframe/utils/simple-schema").SimpleSchema<{
  path: string;
}, {
  path: string;
}>], import("devframe/utils/simple-schema").SimpleSchema<{
  uploadId: string;
}, {
  uploadId: string;
}>, import("devframe").DevframeNodeContext>;
//#endregion
//#region src/node/rpc/index.d.ts
/** Read-only RPC, always registered. */
export declare const readFunctions: readonly [import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:list", "query", readonly [], import("devframe/utils/simple-schema").SimpleSchema<{
  path: string;
  type: "image" | "font" | "video" | "audio" | "text" | "other";
  publicPath: string;
  size: number;
  mtime: number;
  fsPath?: string | undefined;
}[], {
  path: string;
  type: "image" | "font" | "video" | "audio" | "text" | "other";
  publicPath: string;
  size: number;
  mtime: number;
  fsPath?: string | undefined;
}[]>, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:read-image-meta", "query", readonly [import("devframe/utils/simple-schema").SimpleSchema<string, string>], import("devframe/utils/simple-schema").SimpleSchema<{
  width?: number | undefined;
  height?: number | undefined;
  orientation?: number | undefined;
} | null, {
  width?: number | undefined;
  height?: number | undefined;
  orientation?: number | undefined;
} | null>, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:read-text", "query", readonly [import("devframe/utils/simple-schema").SimpleSchema<string, string>, import("devframe/utils/simple-schema").SimpleSchema<number | undefined, number | undefined>], import("devframe/utils/simple-schema").SimpleSchema<string | null, string | null>, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:capabilities", "query", readonly [], import("devframe/utils/simple-schema").SimpleSchema<{
  write: boolean;
  uploadExtensions: string[] | "*";
}, {
  write: boolean;
  uploadExtensions: string[] | "*";
}>, import("devframe").DevframeNodeContext>];
/** Mutating RPC, registered only when write actions are enabled. */
export declare const writeFunctions: readonly [import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:upload", "action", readonly [import("devframe/utils/simple-schema").SimpleSchema<{
  path: string;
}, {
  path: string;
}>], import("devframe/utils/simple-schema").SimpleSchema<{
  uploadId: string;
}, {
  uploadId: string;
}>, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:rename", "action", readonly [import("devframe/utils/simple-schema").SimpleSchema<{
  path: string;
  newName: string;
}, {
  path: string;
  newName: string;
}>], import("devframe/utils/simple-schema").SimpleSchema<{
  path: string;
  type: "image" | "font" | "video" | "audio" | "text" | "other";
  publicPath: string;
  size: number;
  mtime: number;
  fsPath?: string | undefined;
}, {
  path: string;
  type: "image" | "font" | "video" | "audio" | "text" | "other";
  publicPath: string;
  size: number;
  mtime: number;
  fsPath?: string | undefined;
}>, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:delete", "action", readonly [import("devframe/utils/simple-schema").SimpleSchema<{
  paths: string[];
}, {
  paths: string[];
}>], import("devframe/utils/simple-schema").SimpleSchema<{
  deleted: string[];
}, {
  deleted: string[];
}>, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:mkdir", "action", readonly [import("devframe/utils/simple-schema").SimpleSchema<{
  path: string;
}, {
  path: string;
}>], import("devframe/utils/simple-schema").SimpleSchema<void, void>, import("devframe").DevframeNodeContext>];
export declare const serverFunctions: readonly [import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:list", "query", readonly [], import("devframe/utils/simple-schema").SimpleSchema<{
  path: string;
  type: "image" | "font" | "video" | "audio" | "text" | "other";
  publicPath: string;
  size: number;
  mtime: number;
  fsPath?: string | undefined;
}[], {
  path: string;
  type: "image" | "font" | "video" | "audio" | "text" | "other";
  publicPath: string;
  size: number;
  mtime: number;
  fsPath?: string | undefined;
}[]>, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:read-image-meta", "query", readonly [import("devframe/utils/simple-schema").SimpleSchema<string, string>], import("devframe/utils/simple-schema").SimpleSchema<{
  width?: number | undefined;
  height?: number | undefined;
  orientation?: number | undefined;
} | null, {
  width?: number | undefined;
  height?: number | undefined;
  orientation?: number | undefined;
} | null>, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:read-text", "query", readonly [import("devframe/utils/simple-schema").SimpleSchema<string, string>, import("devframe/utils/simple-schema").SimpleSchema<number | undefined, number | undefined>], import("devframe/utils/simple-schema").SimpleSchema<string | null, string | null>, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:capabilities", "query", readonly [], import("devframe/utils/simple-schema").SimpleSchema<{
  write: boolean;
  uploadExtensions: string[] | "*";
}, {
  write: boolean;
  uploadExtensions: string[] | "*";
}>, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:upload", "action", readonly [import("devframe/utils/simple-schema").SimpleSchema<{
  path: string;
}, {
  path: string;
}>], import("devframe/utils/simple-schema").SimpleSchema<{
  uploadId: string;
}, {
  uploadId: string;
}>, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:rename", "action", readonly [import("devframe/utils/simple-schema").SimpleSchema<{
  path: string;
  newName: string;
}, {
  path: string;
  newName: string;
}>], import("devframe/utils/simple-schema").SimpleSchema<{
  path: string;
  type: "image" | "font" | "video" | "audio" | "text" | "other";
  publicPath: string;
  size: number;
  mtime: number;
  fsPath?: string | undefined;
}, {
  path: string;
  type: "image" | "font" | "video" | "audio" | "text" | "other";
  publicPath: string;
  size: number;
  mtime: number;
  fsPath?: string | undefined;
}>, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:delete", "action", readonly [import("devframe/utils/simple-schema").SimpleSchema<{
  paths: string[];
}, {
  paths: string[];
}>], import("devframe/utils/simple-schema").SimpleSchema<{
  deleted: string[];
}, {
  deleted: string[];
}>, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:assets:mkdir", "action", readonly [import("devframe/utils/simple-schema").SimpleSchema<{
  path: string;
}, {
  path: string;
}>], import("devframe/utils/simple-schema").SimpleSchema<void, void>, import("devframe").DevframeNodeContext>];
declare module 'devframe' {
  interface DevframeRpcServerFunctions extends RpcDefinitionsToFunctions<typeof serverFunctions> {}
}
//#endregion
export type { AssetsCapabilities, RenameArgs };