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
//#region src/inject/index.d.ts
/** Discovery file path, relative to the target process's cwd. */
export declare const DISCOVERY_FILE = "node_modules/.data-inspector/discovery.json";
export interface InjectDiscovery {
  /** WS endpoint of the inject endpoint, e.g. `ws://127.0.0.1:9878`. */
  websocket: string;
  /** Pre-shared token the inspector must present. Absent when `auth: false`. */
  token?: string;
  pid: number;
  startedAt: number;
}
export interface ExposeDataInspectorOptions {
  /**
   * Data sources to expose, registered before the endpoint opens. A
   * convenience over calling `registerDataSource` yourself; the two paths
   * share one process-global registry, so inline sources and separately
   * registered ones coexist (a later registration replaces an earlier one
   * with the same id).
   */
  sources?: DataSourceEntry[];
  /** Preferred port (falls back to a free one nearby). Default 9878. */
  port?: number;
  /**
   * Require the trust handshake with a pre-shared token (default `true`).
   * `false` opens the endpoint to any local connection.
   */
  auth?: boolean;
  /** Override the minted token (e.g. from an env secret). */
  token?: string;
  /** Skip writing the discovery file. */
  discoveryFile?: boolean;
  /** Suppress the stderr banner. */
  silent?: boolean;
  /**
   * Register the built-in example source (devframe context, OS, live
   * process stats of the target). Default `true`; set `false` (or
   * `DEVFRAME_DATA_INSPECTOR_EXAMPLE=0` on the `--import` path) to expose
   * only the target's own registrations.
   */
  exampleSource?: boolean;
}
export interface DataInspectorEndpoint {
  websocket: string;
  token?: string;
  close: () => Promise<void>;
}
/**
 * A data source exposing the inspected process's global object. On the
 * zero-code `--import` path there is nowhere to call `registerDataSource`, so
 * assigning to `globalThis` is how you surface things to inspect
 * (`globalThis.store = store`); this source makes them queryable live. The
 * factory reads `globalThis` at query time, so late assignments show up on the
 * next run.
 */
export declare function createGlobalThisDataSource(): DataSourceEntry;
/** Start the inject endpoint in the current process. */
export declare function exposeDataInspector(options?: ExposeDataInspectorOptions): Promise<DataInspectorEndpoint>;
//#endregion