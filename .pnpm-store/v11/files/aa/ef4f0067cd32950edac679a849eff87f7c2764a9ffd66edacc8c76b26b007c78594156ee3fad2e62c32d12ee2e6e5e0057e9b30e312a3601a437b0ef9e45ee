import { Server } from "@modelcontextprotocol/server";
import { MountMcpHttpOptions, MountMcpHttpOptions as MountMcpHttpOptions$1, MountedMcpHttp, MountedMcpHttp as MountedMcpHttp$1 } from "devframe/internal";
import { H3 } from "h3";
import { CreateMcpFetchHandlerOptions, CreateMcpFetchHandlerOptions as CreateMcpFetchHandlerOptions$1, CreateMcpServerOptions, CreateMcpServerOptions as CreateMcpServerOptions$1, DevframeDefinition, DevframeNodeContext, McpConnectionInfo, McpFetchHandler, McpFetchHandler as McpFetchHandler$1, McpServerHandle, McpServerHandle as McpServerHandle$1 } from "devframe/types";
//#region src/mcp/build-server.d.ts
/**
 * Build an MCP server over the agent surface of a devframe definition.
 * Currently supports `stdio` transport only.
 */
export declare function createMcpServer(definition: DevframeDefinition, options?: CreateMcpServerOptions$1): Promise<McpServerHandle$1>;
//#endregion
//#region src/mcp/fetch.d.ts
/**
 * Build a framework-agnostic MCP endpoint over a devframe context: a
 * web-standard `Request → Response` handler any host can mount: h3 (see
 * `mountMcpHttp`), a Next.js App Router route, or any other fetch-shaped
 * server.
 *
 * The endpoint is **stateless**: it serves the 2026-07-28 revision per request
 * through the SDK's {@link createMcpHandler}, which builds a fresh MCP server
 * (from the shared, live `ctx` via `buildMcpServerFromContext`) for each
 * request: no `Mcp-Session-Id` registry, no session-local routing, no
 * GET/DELETE teardown protocol. 2025-era clients are still served through the
 * SDK's default stateless legacy path. `list_changed` events reach modern
 * `subscriptions/listen` streams through the handler's `notify` bus.
 *
 * The origin gate guards every request: loopback-default DNS-rebinding
 * protection that (unlike the WS upgrade's `isAllowedOrigin`) also rejects
 * `Origin`-less requests, so a browser can't reach the route across origins (a
 * disallowed origin gets `403`). The `Origin` header is only browser hardening:
 * a non-browser client forges it. So on the zero-config default (no widened
 * `allowedOrigins`, no identity check) a second locality gate requires the
 * connected peer to be loopback, proven from the host-supplied
 * {@link McpConnectionInfo.remoteAddress} (which a client cannot forge), so the
 * "trusts same-machine callers" default holds against a remote raw client.
 * When same-machine isn't your trust boundary, add an identity gate
 * ({@link CreateMcpFetchHandlerOptions.authorization}), checked after the
 * origin gate: a bearer/callback check that proves *who* is calling (a
 * missing/invalid credential gets `401` with a `WWW-Authenticate: Bearer`
 * challenge), which also lifts the loopback-peer restriction for authenticated
 * callers.
 */
export declare function createMcpFetchHandler(ctx: DevframeNodeContext, options: CreateMcpFetchHandlerOptions$1): McpFetchHandler$1;
//#endregion
//#region src/mcp/http.d.ts
/**
 * Mount a stateless MCP endpoint on an h3 app at `path`: the h3 binding over
 * {@link createMcpFetchHandler}, which owns the per-request serving, the
 * origin gate, and the transport plumbing.
 *
 * The handler is web-standard: it takes the h3 event's web `Request` and
 * returns a web `Response` (an SSE `ReadableStream` body for a
 * `subscriptions/listen` stream). We copy that response onto `event.res` and
 * return its body rather than returning the `Response` object directly, so an
 * MCP error response (e.g. a 4xx) isn't swallowed by h3's "Response-with-404
 * falls through to the next handler" rule (which would otherwise hand the
 * request to the SPA static catch-all).
 */
export declare function mountMcpHttp(app: H3, ctx: DevframeNodeContext, path: string, options: MountMcpHttpOptions$1): MountedMcpHttp$1;
//#endregion
export type { CreateMcpFetchHandlerOptions, CreateMcpServerOptions, McpConnectionInfo, McpFetchHandler, McpServerHandle, MountMcpHttpOptions, MountedMcpHttp };