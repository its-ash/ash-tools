//#region src/node/engine/contract.d.ts
/**
 * Wire types shared by the server RPC functions and the SPA. Types only,
 * safe to import from browser code without dragging jora into the bundle.
 */
/** Client-controlled filtering, applied by the normalizer and the skeleton. */
export interface FilterOptions {
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
export type PathSegment = ['k', string] | ['i', number] | ['s', number] | ['mk', number] | ['mv', number];
/** A path from the query root to a nested node: a list of structural steps. */
export type NodePath = PathSegment[];
/** A query recipe: the text plus the filter options it was authored with. */
export interface Query extends FilterOptions {
  query: string;
  title?: string;
  description?: string;
}
/** What the client sees of a registered data source. */
export interface DataSourceMeta {
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
export type WriteValue = {
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
export type WriteRequest = {
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
export type WriteOutcome = {
  ok: true;
} | {
  ok: false;
  error: {
    name: string;
    message: string;
  };
};
/** One completion candidate: replace [from, to) with `value`. */
export interface SuggestItem {
  type: string;
  from: number;
  to: number;
  /** The fragment currently typed in that range. */
  current: string;
  /** The completion to insert. */
  value: string;
}
export interface SuggestOutcome {
  ok: boolean;
  suggestions: SuggestItem[];
  statMs: number;
  error?: string;
}
export interface NormalizeStatsWire {
  nodes: number;
  refs: number;
  truncatedDepth: number;
  truncatedEntries: number;
  truncatedProps: number;
  ms: number;
}
export interface QueryStats {
  queryMs: number;
  normalize: NormalizeStatsWire;
  payloadBytes: number;
}
export type QueryOutcome = {
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
export type SkeletonOutcome = {
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
export type SavedQueryScope = 'workspace' | 'project';
export interface SavedQuery extends Query {
  /** Storage key. Derived from the title (or query) when not supplied. */
  id: string;
  scope: SavedQueryScope;
  updatedAt: number;
}
export interface SaveQueryInput extends Query {
  id?: string;
  scope: SavedQueryScope;
}
//#endregion
//#region src/node/engine/normalize.d.ts
export interface NormalizeOptions {
  /** Max object/array nesting depth before truncation. */
  maxDepth?: number;
  /** Max array items / Map+Set entries emitted per collection. */
  maxEntries?: number;
  /** Max own properties emitted per object. */
  maxProps?: number;
  /** Max string length before truncation. */
  maxString?: number;
  /** Drop function values (object props and array items). */
  excludeFunctions?: boolean;
  /** Drop object properties whose key starts with `_`. */
  excludeUnderscoreProps?: boolean;
  /** Drop object properties whose key starts with `$`. */
  excludeDollarProps?: boolean;
}
/** True when a property key is excluded by the filter options. */
export declare function isExcludedKey(key: string, opts: Pick<NormalizeOptions, 'excludeUnderscoreProps' | 'excludeDollarProps'>): boolean;
export interface NormalizeStats {
  nodes: number;
  refs: number;
  truncatedDepth: number;
  truncatedEntries: number;
  truncatedProps: number;
  ms: number;
}
export declare function normalize(value: unknown, options?: NormalizeOptions): {
  data: unknown;
  stats: NormalizeStats;
};
/**
 * Re-descend a live graph along a `NodePath`, mirroring the walker's own
 * traversal (and re-applying `excludeFunctions` to array indices so a lazily
 * fetched index lines up with what the client saw). Returns the node the
 * path addresses, or `undefined` if any step falls off the graph.
 */
export declare function navigate(value: unknown, path: NodePath, options?: Pick<NormalizeOptions, 'excludeFunctions'>): unknown;
//#endregion
//#region src/node/engine/query-engine.d.ts
export declare function runQuery(target: unknown, query: string, options?: NormalizeOptions): Promise<QueryOutcome>;
/**
 * Lazy-expand a depth-truncated node: re-run the base query against the live
 * object, re-descend to the node the `NodePath` addresses, and normalize just
 * that subtree with a fresh depth budget. The path comes from a `$truncated:
 * 'depth'` marker the client is expanding, so the same filter options must be
 * threaded through (they shift array indices and drop keys).
 */
export declare function runQueryAtPath(target: unknown, query: string, path: NodePath, options?: NormalizeOptions): Promise<QueryOutcome>;
/**
 * jora stat mode: evaluates the (tolerant) query against the target and
 * reports completions for the given cursor position. Each stat entry carries
 * its candidates in a nested `suggestions` array, flattened here into plain,
 * RPC-safe completion items.
 */
export declare function suggest(target: unknown, query: string, pos: number, limit?: number): Promise<SuggestOutcome>;
//#endregion
//#region src/node/engine/skeleton.d.ts
export type SkeletonOptions = Pick<NormalizeOptions, 'maxDepth' | 'maxProps' | 'excludeFunctions' | 'excludeUnderscoreProps' | 'excludeDollarProps'>;
export declare function skeletonOf(value: unknown, options?: SkeletonOptions): {
  skeleton: unknown;
  nodes: number;
  ms: number;
};
//#endregion
//#region src/node/engine/write.d.ts
export interface WriteApplyOptions {
  /** Must match the client's view so `['i', n]` indices line up. */
  excludeFunctions?: boolean;
}
/**
 * Apply one write request to a live root object. Mutates in place;
 * returns a named error outcome instead of throwing.
 */
export declare function applyWrite(root: unknown, request: WriteRequest, options?: WriteApplyOptions): WriteOutcome;
//#endregion