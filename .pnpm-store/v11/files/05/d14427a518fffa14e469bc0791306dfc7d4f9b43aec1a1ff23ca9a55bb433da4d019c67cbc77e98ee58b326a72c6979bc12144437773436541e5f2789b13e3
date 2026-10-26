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
//#endregion
//#region src/node/registry/index.d.ts
export interface DataSourceEntry {
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
export interface DataSourceHandle {
  /** Broadcast that this source's data changed, so connected UIs re-run. */
  notifyChanged: () => void;
  /** Remove this source from the registry. */
  unregister: () => void;
}
/** The service provided on `ctx.services` (same store as the module API). */
export interface DataSourcesService {
  register: (entry: DataSourceEntry) => DataSourceHandle;
  unregister: (id: string) => void;
  list: () => DataSourceMeta[];
  get: (id: string) => DataSourceEntry | undefined;
  onChanged: (listener: () => void) => () => void;
  onDataChanged: (listener: (sourceId: string) => void) => () => void;
}
/** Id under which the registry is provided on `ctx.services`. */
export declare const DATA_SOURCES_SERVICE_ID = "devframes:plugin:data-inspector:sources";
declare module 'devframe' {
  interface DevframeServicesRegistry {
    'devframes:plugin:data-inspector:sources': DataSourcesService;
  }
}
/** Broadcast that a source's data changed to every data-changed listener. */
export declare function notifyDataSourceChanged(id: string): void;
/** Effective writability of an entry: opt-in, and never on a static source. */
export declare function isWritableEntry(entry: DataSourceEntry): boolean;
/** Register (or replace) a data source. Returns a handle to the registration. */
export declare function registerDataSource(entry: DataSourceEntry): DataSourceHandle;
export declare function unregisterDataSource(id: string): void;
export declare function listDataSources(): DataSourceMeta[];
export declare function getDataSource(id: string): DataSourceEntry | undefined;
/** Resolve a source's data, honoring value-vs-factory and `static` memoization. */
export declare function resolveSourceData(entry: DataSourceEntry): Promise<unknown>;
/** Subscribe to registry changes (register/unregister). Returns unsubscribe. */
export declare function onDataSourcesChanged(listener: () => void): () => void;
/**
 * Subscribe to data-changed notifications (writes, `notifyChanged` handles,
 * `subscribe` bridges). The listener receives the source id.
 */
export declare function onDataSourceDataChanged(listener: (sourceId: string) => void): () => void;
/** Drop every registration and cache; a test isolation helper. */
export declare function resetDataSources(): void;
/** The service implementation provided on `ctx.services` by `setup()`. */
export declare function createDataSourcesService(): DataSourcesService;
//#endregion