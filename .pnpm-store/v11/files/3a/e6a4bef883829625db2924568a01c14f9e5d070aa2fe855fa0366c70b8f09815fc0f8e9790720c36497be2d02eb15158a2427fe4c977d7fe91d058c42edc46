import { DevframeInstanceRecord } from "devframe/internal";
//#region src/connect/index.d.ts
export interface ConnectServerOptions {
  /**
   * Explicit ports to probe besides the registry, for instances started
   * before the registry existed, or reachable only by convention. Each port
   * is probed at {@link ConnectServerOptions.base} (default `/`, i.e.
   * `http://localhost:<port>/__connection.json`).
   */
  ports?: number[];
  /**
   * Base path the explicit {@link ConnectServerOptions.ports} probes look for
   * `__connection.json` under, for a devframe or hub mounted below the root
   * of its host (e.g. `/__devtools/` for Vite DevTools). The advertised MCP
   * path is resolved against it. Default `/`. Registry records carry their
   * own base and ignore this.
   */
  base?: string;
  /** Override the registry directory (`DEVFRAME_INSTANCES_DIR` also applies). */
  instancesDir?: string;
  /** Probe timeout per instance, ms. Default 1000. */
  timeoutMs?: number;
  /**
   * The bearer credential the connector presents to each instance's
   * authenticated MCP route, sent as `Authorization: Bearer <token>`.
   *
   * - a **string**: one shared token for every instance;
   * - a **resolver** `(record) => string | undefined`: a per-instance token,
   *   for connecting to a fleet with distinct credentials (return `undefined`
   *   to send none for that instance).
   *
   * The token is only ever placed in a request header: it never enters the
   * instance registry records, the indexed results, connection URLs, or
   * formatted errors. Left unset, no `Authorization` header is sent, so only an
   * instance whose route opted out of identity (`authorization: false`) will
   * answer.
   */
  authToken?: string | ((record: DevframeInstanceRecord) => string | undefined);
}
/**
 * Resolve the per-record bearer from the {@link ConnectServerOptions.authToken}
 * option. Exported for focused tests of the credential resolution.
 */
export declare function resolveAuthToken(authToken: ConnectServerOptions['authToken'], record: DevframeInstanceRecord): string | undefined;
/**
 * Build the request headers the connector sends to one instance's MCP route:
 * the instance's own (loopback) `origin` so the route's origin gate accepts
 * this native client, plus `Authorization: Bearer <token>` when a bearer is
 * configured. The bearer appears **only** here, never in the connection URL,
 * the registry records, or the indexed results. Exported for focused tests.
 */
export declare function buildInstanceRequestHeaders(url: string, token: string | undefined): Record<string, string>;
export interface ConnectServerHandle {
  stop: () => Promise<void>;
}
/**
 * Start the devframe MCP connector on stdio: a thin discovery + proxy server
 * in the shape Vercel's next-devtools-mcp (https://github.com/vercel/next-devtools-mcp)
 * validated; credit is due there for the architecture this connector follows.
 * It exposes two gateway tools:
 * `devframe_connect_list-instances` (discover running devframe instances via
 * the instance registry and list each one's MCP tools) and
 * `devframe_connect_call-tool` (invoke one tool on one instance over its
 * Streamable-HTTP endpoint), and holds no domain knowledge of its own.
 */
export declare function startConnectServer(options?: ConnectServerOptions): Promise<ConnectServerHandle>;
/**
 * Probe an explicit port for a devframe serving `__connection.json` under
 * `base` (default `/`), reusing the registry's origin-candidate probe (a
 * `localhost`-bound server may listen on either address family). The
 * advertised MCP path is relative to that base, as in the registry records
 * the instance shell writes. Exported for focused tests.
 */
export declare function probePort(port: number, base?: string, timeoutMs?: number): Promise<DevframeInstanceRecord | null>;
//#endregion