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