import { homedir } from "node:os";
import process from "node:process";
import { Server, createMcpHandler } from "@modelcontextprotocol/server";
import { DEVFRAME_EVENTS } from "devframe/constants";
import { argsToJsonSchema, diagnostics, formatMcpError, getMcpContent, returnToJsonSchema, stringifyForMcp } from "devframe/internal";
import { createHostContext } from "devframe/node";
import { toAgentToolName } from "devframe/utils/agent-tool-name";
import { join } from "pathe";
import { timingSafeEqual } from "devframe/utils/crypto-token";
import { isAllowedOrigin, isLoopbackAddress } from "devframe/utils/origin";
import { defineHandler, getRequestIP } from "h3";
//#region src/mcp/build-server.ts
/**
* Build a fresh MCP {@link Server} over a devframe context, registering its
* tool and resource handlers. This is a pure factory: it sets up no
* long-lived subscriptions and holds no per-connection state, so it is safe
* to call once per request under `createMcpHandler` or once per connection
* under `serveStdio`. Change notifications are published separately: over
* HTTP through the handler's `notify` bus (see `createMcpFetchHandler`), and
* on stdio through the connection's own `send*ListChanged` calls (see
* {@link bridgeListChanged}, wired by `serveStdio`).
*
* @internal
*/
function buildMcpServerFromContext(ctx, options) {
	const server = new Server({
		name: options.serverName,
		version: options.serverVersion
	}, {
		capabilities: {
			tools: { listChanged: true },
			resources: { listChanged: true }
		},
		...options.instructions ? { instructions: options.instructions } : {}
	});
	registerToolHandlers(server, ctx, options.exposeSharedState);
	registerResourceHandlers(server, ctx, options.exposeSharedState);
	return server;
}
/**
* Publish devframe's `list_changed` events through a set of typed sinks:
* `tools()` for tool-list changes and `resources()` for resource-list
* changes (shared-state keys are surfaced as resources). Returns an
* unsubscribe function.
*
* The HTTP path passes the handler's `notify` bus sugar; the stdio path
* passes the pinned server's `send*ListChanged` methods, which `serveStdio`
* routes onto the connection's active `subscriptions/listen` streams.
*
* @internal
*/
function bridgeListChanged(ctx, sinks) {
	const offManifest = ctx.agent.events.on(DEVFRAME_EVENTS.bus.agentManifestChanged, () => {
		sinks.tools();
		sinks.resources();
	});
	const offKeyAdded = ctx.rpc.sharedState.onKeyAdded(() => {
		sinks.resources();
	});
	return () => {
		offManifest();
		offKeyAdded();
	};
}
/**
* Build an MCP server over the agent surface of a devframe definition.
* Currently supports `stdio` transport only.
*/
async function createMcpServer(definition, options = {}) {
	const transport = options.transport ?? "stdio";
	if (transport !== "stdio") throw diagnostics.DF0017({
		transport,
		reason: "Only stdio transport is supported in this release."
	});
	const ctx = await createHostContext({
		cwd: process.cwd(),
		mode: "dev",
		host: {
			mountStatic: () => {},
			resolveOrigin: () => "mcp://devframe",
			getStorageDir: (scope) => {
				if (scope === "workspace") return join(process.cwd(), ".devframe");
				if (scope === "project") return join(process.cwd(), `node_modules/.${definition.id}/devframe`);
				return join(homedir(), `.${definition.id}/devframe`);
			}
		},
		importMetaUrl: definition.importMetaUrl
	});
	for (const input of definition.services ?? []) ctx.services.install(input, { resolveFrom: definition.importMetaUrl });
	await ctx.services.ready();
	await definition.setup(ctx);
	const buildOptions = {
		serverName: options.serverName ?? `${definition.id} (devframe)`,
		serverVersion: options.serverVersion ?? definition.version ?? "0.0.0",
		exposeSharedState: options.exposeSharedState ?? true,
		instructions: options.instructions
	};
	let handle;
	try {
		const { serveStdio } = await import("@modelcontextprotocol/server/stdio");
		handle = serveStdio(() => {
			const server = buildMcpServerFromContext(ctx, buildOptions);
			const unbridge = bridgeListChanged(ctx, {
				tools: () => {
					server.sendToolListChanged().catch(() => {});
				},
				resources: () => {
					server.sendResourceListChanged().catch(() => {});
				}
			});
			const priorOnClose = server.onclose;
			server.onclose = () => {
				unbridge();
				priorOnClose?.();
			};
			return server;
		});
	} catch (error) {
		const reason = error instanceof Error ? error.message : String(error);
		throw diagnostics.DF0017({
			transport,
			reason,
			cause: error
		});
	}
	options.onReady?.({ transport: "stdio" });
	return { async stop() {
		await handle.close();
	} };
}
/**
* Id of the built-in shared-state read tool, namespaced like every other
* built-in (`devframe:<area>:<fn>`). Tool-shaped access matters because many
* MCP clients only consume tools; the parallel `devframe://state/<key>`
* resource projection stays for the clients that do read resources.
*/
const READ_STATE_TOOL = "devframe:state:read";
/** Wire name of the built-in shared-state read tool: `devframe_state_read`. */
const READ_STATE_NAME = toAgentToolName(READ_STATE_TOOL);
function sharedStateFilter(exposeSharedState) {
	if (exposeSharedState === false) return void 0;
	return typeof exposeSharedState === "function" ? exposeSharedState : () => true;
}
function readStateToolProjection() {
	return {
		name: READ_STATE_NAME,
		title: "Read shared state",
		description: "Read this devtool's live shared state. Call without arguments to list the available keys, then with a key to get that value as JSON. Safe to call freely.",
		inputSchema: {
			type: "object",
			properties: { key: {
				type: "string",
				description: "A shared-state key from the key list. Omit to list all keys."
			} }
		},
		annotations: {
			title: "Read shared state",
			readOnlyHint: true,
			destructiveHint: false
		}
	};
}
async function readStateResult(ctx, filter, key) {
	const keys = ctx.rpc.sharedState.keys().filter(filter);
	if (key === void 0) return { keys };
	if (!keys.includes(key)) throw diagnostics.DF0048({ key });
	return {
		key,
		value: (await ctx.rpc.sharedState.get(key)).value()
	};
}
function registerToolHandlers(server, ctx, exposeSharedState) {
	const stateFilter = sharedStateFilter(exposeSharedState);
	const warnedCollisions = /* @__PURE__ */ new Set();
	/**
	* Resolve a wire tool name back to the registered {@link AgentTool}.
	* Wire-name matching runs first, in manifest order (the same tool the
	* list projection advertises under that name), with a raw-id fallback so
	* a colon-namespaced id keeps working as a call name.
	*/
	const resolveTool = (name) => {
		return ctx.agent.list().tools.find((tool) => toAgentToolName(tool.id) === name) ?? ctx.agent.getTool(name);
	};
	server.setRequestHandler("tools/list", async () => {
		const byName = /* @__PURE__ */ new Map();
		for (const tool of ctx.agent.list().tools) {
			const name = toAgentToolName(tool.id);
			const existing = byName.get(name);
			if (existing) {
				if (!warnedCollisions.has(`${name}|${tool.id}`)) {
					warnedCollisions.add(`${name}|${tool.id}`);
					diagnostics.DF0047({
						name,
						id: tool.id,
						existing: existing.id
					});
				}
				continue;
			}
			byName.set(name, tool);
		}
		const tools = [...byName.entries()].map(([name, tool]) => projectTool(name, tool, ctx));
		if (stateFilter && !byName.has(READ_STATE_NAME)) tools.push(readStateToolProjection());
		return { tools };
	});
	server.setRequestHandler("tools/call", async (request) => {
		const { name, arguments: args } = request.params;
		try {
			const tool = resolveTool(name);
			if (stateFilter && !tool && (name === READ_STATE_NAME || name === READ_STATE_TOOL)) {
				const key = args?.key;
				const result = await readStateResult(ctx, stateFilter, key);
				return {
					content: [{
						type: "text",
						text: stringifyForMcp(result)
					}],
					structuredContent: result
				};
			}
			const outputSchema = tool ? usableOutputSchema(tool.outputSchema ?? computeOutputSchema(tool, ctx)) : void 0;
			const result = await ctx.agent.invoke(tool?.id ?? name, args ?? {});
			return {
				content: mcpContentOf(name, result) ?? [{
					type: "text",
					text: stringifyForMcp(result)
				}],
				...outputSchema ? { structuredContent: result } : {}
			};
		} catch (error) {
			return {
				isError: true,
				content: [{
					type: "text",
					text: `Error invoking "${name}": ${formatMcpError(error)}`
				}]
			};
		}
	});
}
function registerResourceHandlers(server, ctx, exposeSharedState) {
	const stateFilter = sharedStateFilter(exposeSharedState);
	server.setRequestHandler("resources/list", async () => {
		const resources = ctx.agent.list().resources.map((resource) => ({
			uri: resource.uri,
			name: resource.name,
			description: resource.description,
			mimeType: resource.mimeType
		}));
		if (stateFilter) for (const key of ctx.rpc.sharedState.keys()) {
			if (!stateFilter(key)) continue;
			resources.push({
				uri: `devframe://state/${encodeURIComponent(key)}`,
				name: key,
				description: `Shared state: ${key}`,
				mimeType: "application/json"
			});
		}
		return { resources };
	});
	server.setRequestHandler("resources/read", async (request) => {
		const { uri } = request.params;
		const parsed = parseResourceUri(uri);
		if (parsed.kind === "resource") {
			const content = await ctx.agent.read(parsed.id);
			return { contents: [{
				uri,
				mimeType: content.mimeType ?? "application/json",
				text: content.text ?? stringifyForMcp(content.json)
			}] };
		}
		if (parsed.kind === "state") {
			if (!stateFilter || !stateFilter(parsed.key)) throw diagnostics.DF0048({ key: parsed.key });
			const state = await ctx.rpc.sharedState.get(parsed.key);
			return { contents: [{
				uri,
				mimeType: "application/json",
				text: stringifyForMcp(state.value())
			}] };
		}
		throw new Error(`[devframe/mcp] unknown resource URI "${uri}"`);
	});
}
/**
* The content blocks a tool result carries through `withMcpContent`, checked
* and copied field by field, or `undefined` when it carries none. Invalid
* blocks throw DF0081, which the `tools/call` handler turns into an
* `isError` result.
*/
function mcpContentOf(name, result) {
	const blocks = getMcpContent(result);
	if (blocks === void 0) return void 0;
	if (!Array.isArray(blocks) || blocks.length === 0) throw diagnostics.DF0081({
		name,
		reason: "expected a non-empty array"
	});
	return Array.from(blocks, (block, index) => {
		const b = block ?? {};
		if (b.type === "text" && typeof b.text === "string") return {
			type: "text",
			text: b.text
		};
		if ((b.type === "image" || b.type === "audio") && typeof b.data === "string" && typeof b.mimeType === "string") return {
			type: b.type,
			data: b.data,
			mimeType: b.mimeType
		};
		throw diagnostics.DF0081({
			name,
			reason: `block ${index} is not a valid text, image or audio block`
		});
	});
}
/**
* MCP constrains a tool's `outputSchema` to a JSON Schema of `type:
* "object"`; clients (the SDK included) reject anything else. Non-object
* return schemas (e.g. a schema for `void` / a bare string) simply project
* no output schema; the text content still carries the result.
*/
function usableOutputSchema(schema) {
	return schema && typeof schema === "object" && schema.type === "object" ? schema : void 0;
}
function projectTool(name, tool, ctx) {
	const inputSchema = tool.inputSchema ?? computeInputSchema(tool, ctx);
	const outputSchema = usableOutputSchema(tool.outputSchema ?? computeOutputSchema(tool, ctx));
	return {
		name,
		title: tool.title,
		description: tool.description,
		inputSchema,
		...outputSchema ? { outputSchema } : {},
		annotations: {
			title: tool.title,
			readOnlyHint: tool.safety === "read",
			destructiveHint: tool.safety === "destructive"
		}
	};
}
function computeInputSchema(tool, ctx) {
	if (tool.kind === "tool") return argsToJsonSchema(tool.args);
	if (tool.kind !== "rpc" || !tool.rpcName) return {
		type: "object",
		properties: {}
	};
	const def = ctx.rpc.definitions.get(tool.rpcName);
	if (!def) return {
		type: "object",
		properties: {}
	};
	const args = def.args;
	return argsToJsonSchema(args);
}
function computeOutputSchema(tool, ctx) {
	if (tool.kind !== "rpc" || !tool.rpcName) return void 0;
	const def = ctx.rpc.definitions.get(tool.rpcName);
	if (!def) return void 0;
	return returnToJsonSchema(def.returns);
}
function parseResourceUri(uri) {
	const match = uri.match(/^devframe:\/\/(resource|state)\/(.+)$/);
	if (!match) return { kind: "unknown" };
	const [, kind, rest] = match;
	const decoded = decodeURIComponent(rest);
	if (kind === "resource") return {
		kind: "resource",
		id: decoded
	};
	return {
		kind: "state",
		key: decoded
	};
}
//#endregion
//#region src/mcp/fetch.ts
/**
* Parse exactly one `Authorization: Bearer <token>` credential, returning the
* token or `undefined` for a missing, malformed, empty, or multi-credential
* header. The token itself is never logged. `\S+` rejects the whitespace that
* a second credential (fetch merges duplicate headers as `a, b`) or an empty
* value would introduce.
*/
function parseBearerToken(header) {
	if (!header) return void 0;
	const match = /^Bearer (\S+)$/i.exec(header.trim());
	return match ? match[1] : void 0;
}
/**
* Resolve the identity gate for one request. `false` is the origin-only
* opt-out; a callback delegates identity; a string requires a constant-time
* bearer match. Never reveals whether a supplied token was close to correct.
*/
async function isAuthorized(req, authorization) {
	if (authorization === false) return true;
	if (typeof authorization === "function") return await authorization(req) === true;
	const token = parseBearerToken(req.headers.get("authorization"));
	if (token === void 0) return false;
	return timingSafeEqual(token, authorization);
}
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
function createMcpFetchHandler(ctx, options) {
	const allowedOrigins = options.allowedOrigins;
	const authorization = options.authorization ?? false;
	const handler = createMcpHandler(() => buildMcpServerFromContext(ctx, {
		serverName: options.serverName,
		serverVersion: options.serverVersion,
		exposeSharedState: options.exposeSharedState,
		instructions: options.instructions
	}));
	const unbridge = bridgeListChanged(ctx, {
		tools: () => {
			handler.notify.toolsChanged();
		},
		resources: () => {
			handler.notify.resourcesChanged();
		}
	});
	const originOnlyDefault = allowedOrigins === void 0 && authorization === false;
	async function handle(req, connection) {
		const origin = req.headers.get("origin") ?? void 0;
		if (allowedOrigins !== false && (origin === void 0 || !isAllowedOrigin(origin, allowedOrigins ?? []))) return new Response("Forbidden", { status: 403 });
		if (originOnlyDefault && connection?.remoteAddress !== void 0 && !isLoopbackAddress(connection.remoteAddress)) return new Response("Forbidden", { status: 403 });
		if (!await isAuthorized(req, authorization)) return new Response("Unauthorized", {
			status: 401,
			headers: { "WWW-Authenticate": "Bearer" }
		});
		return handler.fetch(req);
	}
	return {
		fetch: handle,
		dispose: async () => {
			unbridge();
			await handler.close();
		}
	};
}
//#endregion
//#region src/mcp/http.ts
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
function mountMcpHttp(app, ctx, path, options) {
	const handler = createMcpFetchHandler(ctx, options);
	app.use(path, defineHandler(async (event) => respond(event, await handler.fetch(event.req, { remoteAddress: getRequestIP(event) }))));
	return { dispose: handler.dispose };
}
/**
* Copy a web `Response` from the MCP transport onto the h3 event's response
* and return its body. Returning the body (a `ReadableStream` or `null`)
* rather than the `Response` object avoids h3's 404-fall-through behavior.
*/
function respond(event, response) {
	event.res.status = response.status;
	event.res.statusText = response.statusText;
	response.headers.forEach((value, key) => {
		event.res.headers.set(key, value);
	});
	return response.body ?? "";
}
//#endregion
export { createMcpFetchHandler, createMcpServer, mountMcpHttp };
