import { DevframeNodeContext } from "devframe";
import { RpcDefinitionsToFunctions } from "devframe/rpc";
//#region src/node/engine/contract.d.ts
/**
 * Wire types shared by the server RPC functions and the SPA. Types only,
 * safe to import from browser code without dragging jora into the bundle.
 */
/** Client-controlled filtering, applied by the normalizer and the skeleton. */
interface FilterOptions {
  excludeFunctions?: boolean;
  excludeUnderscoreProps?: boolean;
  excludeDollarProps?: boolean;
}
/**
 * One structural step from a query result toward a nested node, recorded by
 * the normalizer on each depth-truncated marker so the client can lazily
 * re-fetch that subtree with a fresh depth budget (see `queryPath`). Each
 * segment mirrors exactly one descent the normalizer makes:
 *
 *   - `['k', key]`   own object property, or a string-keyed Map value
 *   - `['i', index]` array item (index into the filtered array)
 *   - `['s', index]` Set value
 *   - `['mk', index]` / `['mv', index]` non-string Map entry key / value
 */
type PathSegment = ['k', string] | ['i', number] | ['s', number] | ['mk', number] | ['mv', number];
/** A path from the query root to a nested node: a list of structural steps. */
type NodePath = PathSegment[];
/** A query recipe: the text plus the filter options it was authored with. */
interface Query extends FilterOptions {
  query: string;
  title?: string;
  description?: string;
}
/** What the client sees of a registered data source. */
interface DataSourceMeta {
  id: string;
  title: string;
  description?: string;
  /** Phosphor icon class shown in the source picker (e.g. `i-ph:database-duotone`). */
  icon?: string;
  /** Data never changes; the server memoizes the resolved value. */
  static: boolean;
  /** The source opted into live edits through the `write` RPC. */
  writable: boolean;
  /** Suggested queries provided by the source (shown read-only). */
  queries?: Query[];
}
/**
 * A value carried inside a write request. JSON can't express `undefined`,
 * so the payload is discriminated instead of raw.
 */
type WriteValue = {
  kind: 'json';
  value: unknown;
} | {
  kind: 'undefined';
};
/**
 * One mutation of a writable source's live object. Ops are container-generic:
 * the server resolves the path and dispatches on what it finds there
 * (object / array / Map / Set).
 *
 *   - `set`    replace the value at `path`.
 *   - `delete` remove the node at `path` from its container.
 *   - `add`    `path` addresses the CONTAINER; insert `key`/`value`
 *                (objects and Maps need `key`; arrays take an optional index
 *                `key` to splice at, else append; Sets take just `value`).
 *   - `rename` re-key the node at `path` under `key`, atomically
 *                (objects and Maps; the renamed key lands last).
 */
type WriteRequest = {
  op: 'set';
  path: NodePath;
  value: WriteValue;
} | {
  op: 'delete';
  path: NodePath;
} | {
  op: 'add';
  path: NodePath;
  key?: WriteValue;
  value: WriteValue;
} | {
  op: 'rename';
  path: NodePath;
  key: WriteValue;
};
type WriteOutcome = {
  ok: true;
} | {
  ok: false;
  error: {
    name: string;
    message: string;
  };
};
/** One completion candidate: replace [from, to) with `value`. */
interface SuggestItem {
  type: string;
  from: number;
  to: number;
  /** The fragment currently typed in that range. */
  current: string;
  /** The completion to insert. */
  value: string;
}
interface SuggestOutcome {
  ok: boolean;
  suggestions: SuggestItem[];
  statMs: number;
  error?: string;
}
interface NormalizeStatsWire {
  nodes: number;
  refs: number;
  truncatedDepth: number;
  truncatedEntries: number;
  truncatedProps: number;
  ms: number;
}
interface QueryStats {
  queryMs: number;
  normalize: NormalizeStatsWire;
  payloadBytes: number;
}
type QueryOutcome = {
  ok: true;
  result: unknown;
  stats: QueryStats;
} | {
  ok: false;
  error: {
    name: string;
    message: string;
  };
};
type SkeletonOutcome = {
  ok: true;
  skeleton: unknown;
  nodes: number;
  ms: number;
} | {
  ok: false;
  error: {
    name: string;
    message: string;
  };
};
/**
 * Where a saved query persists, mirroring the host storage scopes:
 * `workspace` is committable and shared with the team, `project` is
 * per-checkout private (node_modules).
 */
type SavedQueryScope = 'workspace' | 'project';
interface SavedQuery extends Query {
  /** Storage key. Derived from the title (or query) when not supplied. */
  id: string;
  scope: SavedQueryScope;
  updatedAt: number;
}
interface SaveQueryInput extends Query {
  id?: string;
  scope: SavedQueryScope;
}
//#endregion
//#region src/node/registry/index.d.ts
interface DataSourceEntry {
  /** Unique id; namespace it with your plugin id (`my-plugin:thing`). */
  id: string;
  title: string;
  description?: string;
  /** Phosphor icon class shown in the source picker. */
  icon?: string;
  /**
   * The data to inspect: a plain value, or a factory returning it (sync or
   * async). Live objects passed directly stay live, so queries read their
   * current state. Wrap functions you want to inspect in a factory.
   */
  data: unknown | (() => unknown | Promise<unknown>);
  /**
   * The resolved data never changes: the factory runs once and the settled
   * value is memoized (default `false`).
   */
  static?: boolean;
  /**
   * Opt this source into live edits (default `false`): connected inspectors
   * may mutate the resolved object in place through the `write` RPC.
   * Contradicts `static: true`, since a memoized snapshot is read-only, so the
   * combination reports a diagnostic and stays read-only.
   */
  writable?: boolean;
  /**
   * Bridge the source's own change signal: called with a `notify` function
   * that broadcasts a `data:changed` event to connected clients (same signal
   * a successful `write` emits). Return a dispose function to unhook; it runs
   * when the source is unregistered or replaced.
   */
  subscribe?: (notify: () => void) => (() => void) | void;
  /** Suggested queries, surfaced read-only next to saved ones. */
  queries?: Query[];
}
/** Handle returned by `registerDataSource`, to notify clients or unregister. */
interface DataSourceHandle {
  /** Broadcast that this source's data changed, so connected UIs re-run. */
  notifyChanged: () => void;
  /** Remove this source from the registry. */
  unregister: () => void;
}
/** The service provided on `ctx.services` (same store as the module API). */
interface DataSourcesService {
  register: (entry: DataSourceEntry) => DataSourceHandle;
  unregister: (id: string) => void;
  list: () => DataSourceMeta[];
  get: (id: string) => DataSourceEntry | undefined;
  onChanged: (listener: () => void) => () => void;
  onDataChanged: (listener: (sourceId: string) => void) => () => void;
}
declare module 'devframe' {
  interface DevframeServicesRegistry {
    'devframes:plugin:data-inspector:sources': DataSourcesService;
  }
}
//#endregion
//#region src/node/example-source.d.ts
/** Id of the built-in example source (namespaced like every source id). */
export declare const EXAMPLE_SOURCE_ID = "devframes:plugin:data-inspector:example";
/**
 * The built-in example source, always registered unless opted out
 * (`exampleSource: false`). Lets the viewer query simple environment info
 * (the devframe context: registered RPC functions, services, storage dirs;
 * OS and live process stats) plus a small playground branch exercising
 * everything the viewer can show (Maps, Sets, class instances, circulars,
 * BigInt, functions, query-time getters).
 */
export declare function createExampleDataSource(ctx?: DevframeNodeContext): DataSourceEntry;
//#endregion
//#region src/node/engine/write.d.ts
interface WriteApplyOptions {
  /** Must match the client's view so `['i', n]` indices line up. */
  excludeFunctions?: boolean;
}
//#endregion
//#region src/node/rpc/index.d.ts
/**
 * The RPC functions registered by the data-inspector plugin.
 * Namespaced `devframes:plugin:data-inspector:*` (the plugin id).
 */
export declare const serverFunctions: readonly [import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:data-inspector:sources", "query", [], Promise<DataSourceMeta[]>, undefined, undefined, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:data-inspector:query", "query", [sourceId: string, joraQuery: string, options?: ({
  maxDepth?: number;
  maxEntries?: number;
} & FilterOptions) | undefined], Promise<QueryOutcome>, undefined, undefined, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:data-inspector:queryPath", "query", [sourceId: string, joraQuery: string, path: NodePath, options?: ({
  maxDepth?: number;
  maxEntries?: number;
} & FilterOptions) | undefined], Promise<QueryOutcome>, undefined, undefined, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:data-inspector:skeleton", "query", [sourceId: string, options?: FilterOptions | undefined], Promise<SkeletonOutcome>, undefined, undefined, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:data-inspector:suggest", "query", [sourceId: string, joraQuery: string, pos: number], Promise<SuggestOutcome>, undefined, undefined, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:data-inspector:saved:list", "query", [], Promise<SavedQuery[]>, undefined, undefined, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:data-inspector:saved:save", "action", [input: SaveQueryInput], Promise<SavedQuery>, undefined, undefined, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:data-inspector:saved:delete", "action", [id: string, scope: SavedQueryScope], Promise<boolean>, undefined, undefined, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:data-inspector:write", "action", [sourceId: string, request: WriteRequest, options?: WriteApplyOptions | undefined], Promise<WriteOutcome>, undefined, undefined, import("devframe").DevframeNodeContext>];
declare module 'devframe' {
  interface DevframeRpcServerFunctions extends RpcDefinitionsToFunctions<typeof serverFunctions> {}
}
//#endregion
//#region src/node/setup.d.ts
/** Broadcast whenever the source registry changes (register/unregister). */
export declare const SOURCES_CHANGED_EVENT = "devframes:plugin:data-inspector:sources:changed";
/**
 * Broadcast whenever a source's DATA changes: a successful `write`, a
 * `notifyChanged()` handle call, or a source's own `subscribe` bridge.
 * Carries the source id so clients refresh only the affected view.
 */
export declare const DATA_CHANGED_EVENT = "devframes:plugin:data-inspector:data:changed";
export interface SetupDataInspectorOptions {
  /** Register the built-in example source (default `true`). */
  exampleSource?: boolean;
}
/**
 * Register the data-inspector's RPC functions on a devframe node context,
 * provide the source registry as a typed context service, and broadcast
 * registry changes so connected UIs refresh their source list live.
 *
 * Called from the definition's `setup(ctx)` and reusable by host adapters
 * (the CLI and the inject endpoint wire their own contexts through this).
 */
export declare function setupDataInspector(ctx: DevframeNodeContext, options?: SetupDataInspectorOptions): void;
//#endregion