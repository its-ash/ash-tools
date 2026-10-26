import { defineRpcFunction } from "devframe";
import { defineDiagnostics } from "devframe/utils/nostics";
import process from "node:process";
import { listLiveDevframeInstances } from "devframe/internal";
//#region src/node/rpc/functions/describe-agent.ts
/**
* Surface the agent-exposed surface of the connection: the unified
* manifest of tools (RPC functions flagged with `agent`, plus
* host-registered tools) and readable resources. `snapshot: true` bakes
* the manifest into the static dump for `build`/`spa` mode.
*/
const describeAgent = defineRpcFunction({
	name: "devframes:plugin:inspect:describe-agent",
	type: "query",
	jsonSerializable: true,
	snapshot: true,
	agent: {
		description: "Describe the agent-exposed surface of this devframe: the list of tools (agent-flagged RPC functions and host-registered tools) and readable resources, with their titles, descriptions, safety hints, and schemas. Read-only.",
		title: "Describe agent surface"
	},
	setup: (ctx) => ({ handler: async () => {
		return ctx.agent.list();
	} })
});
//#endregion
//#region src/node/diagnostics.ts
/**
* Structured diagnostics for `@devframes/plugin-inspect`. Node-side only.
* Codes use the plugin-private `DP_INSPECT_` band (see the built-in
* plugins planning index) so they never collide with devframe core
* (`DF00xx`) or `@devframes/hub` (`DF80xx`).
*/
const diagnostics = defineDiagnostics({
	docsBase: "https://devfra.me/errors",
	codes: {
		DP_INSPECT_0001: {
			why: (p) => `Cannot invoke "${p.name}": no RPC function with that name is registered on this connection.`,
			fix: "Call `devframes:plugin:inspect:list-functions` to see the registered names, or check for a typo."
		},
		DP_INSPECT_0002: {
			why: (p) => `Refusing to invoke "${p.name}": only read-only "query" and "static" functions are invokable from the inspector, but this one is "${p.type}".`,
			fix: "The inspector deliberately blocks `action`/`event` functions to avoid triggering side effects. Invoke those through their own UI instead."
		},
		DP_INSPECT_0003: {
			why: (p) => `Cannot execute command "${p.id}": this connection has no hub commands host.`,
			fix: "Commands are a `@devframes/hub` feature. Mount this devframe inside a hub that registers commands via `ctx.commands.register()`, or check `devframes:plugin:inspect:list-commands` (returns an empty list outside a hub)."
		}
	}
});
//#endregion
//#region src/node/rpc/functions/_hub-commands.ts
function looksLikeHubCommandsHost(value) {
	return !!value && typeof value === "object" && value.commands instanceof Map && typeof value.execute === "function";
}
/**
* Resolve `ctx.commands` when this connection is mounted inside a hub, or
* `undefined` on a plain devframe connection (CLI / Vite / build without a
* hub). Structural rather than an `instanceof`/import check, matching the
* pattern established in `plugins/terminals/src/node/manager.ts`.
*/
function resolveHubCommands(ctx) {
	const commands = ctx.commands;
	return looksLikeHubCommandsHost(commands) ? commands : void 0;
}
/**
* Project a raw hub command (still carrying its `handler`) into the
* serializable shape the inspector sends over RPC, recursing into
* children. `hasHandler` survives the projection even though the hub's
* own `commands.list()` strips `handler` entirely before it's callable
* from here; reading the raw command straight off `host.commands`
* preserves that flag.
*/
function projectCommand(cmd) {
	return {
		id: cmd.id,
		title: cmd.title,
		description: cmd.description,
		icon: cmd.icon,
		category: cmd.category,
		hasHandler: typeof cmd.handler === "function",
		children: cmd.children?.map(projectCommand)
	};
}
//#endregion
//#region src/node/rpc/functions/execute-command.ts
/**
* Execute a hub command by id and return a result envelope, mirroring
* `devframes:plugin:inspect:invoke`. Unlike that function, this one is not
* gated to read-only types, since a command is, by construction, something a
* user explicitly runs (unlike an arbitrary RPC `action`, which may be a
* side effect the inspector shouldn't fire unprompted).
*
* Throws `DP_INSPECT_0003` when this connection has no hub commands host
* (not mounted inside `@devframes/hub`). A found-but-unregistered id, or a
* group-only command with no handler, surfaces as `{ ok: false, error }`
* from the hub's own `commands.execute()` instead of throwing.
*/
const executeCommand = defineRpcFunction({
	name: "devframes:plugin:inspect:execute-command",
	type: "action",
	setup: (ctx) => ({ handler: async (id, args = []) => {
		const host = resolveHubCommands(ctx);
		if (!host) throw diagnostics.DP_INSPECT_0003({ id });
		const start = Date.now();
		try {
			return {
				ok: true,
				result: await host.execute(id, ...args),
				durationMs: Date.now() - start
			};
		} catch (error) {
			const e = error;
			return {
				ok: false,
				error: {
					name: e?.name ?? "Error",
					message: e?.message ?? String(error),
					stack: e?.stack
				},
				durationMs: Date.now() - start
			};
		}
	} })
});
//#endregion
//#region src/node/rpc/functions/invoke.ts
const INVOKABLE_TYPES$1 = /* @__PURE__ */ new Set(["query", "static"]);
/**
* Invoke a read-only RPC function by name and return a result envelope.
*
* Deliberately gated to `query` / `static` functions, because `action` and
* `event` functions may carry side effects, so the inspector refuses to
* fire them (`DP_INSPECT_0002`). Uses structured-clone serialization
* (default) so arbitrary return values round-trip without the strict-JSON
* constraints that `jsonSerializable: true` would impose.
*/
const invoke = defineRpcFunction({
	name: "devframes:plugin:inspect:invoke",
	type: "action",
	setup: (ctx) => ({ handler: async (name, args = []) => {
		const def = ctx.rpc.definitions.get(name);
		if (!def) throw diagnostics.DP_INSPECT_0001({ name });
		const type = def.type ?? "query";
		if (!INVOKABLE_TYPES$1.has(type)) throw diagnostics.DP_INSPECT_0002({
			name,
			type
		});
		const start = Date.now();
		try {
			return {
				ok: true,
				result: await ctx.rpc.invokeLocal(name, ...args),
				durationMs: Date.now() - start
			};
		} catch (error) {
			const e = error;
			return {
				ok: false,
				error: {
					name: e?.name ?? "Error",
					message: e?.message ?? String(error),
					stack: e?.stack
				},
				durationMs: Date.now() - start
			};
		}
	} })
});
//#endregion
//#region src/node/rpc/functions/invoke-agent-tool.ts
const invokeAgentTool = defineRpcFunction({
	name: "devframes:plugin:inspect:invoke-agent-tool",
	type: "action",
	setup: (ctx) => ({ handler: async (id, args) => {
		const start = Date.now();
		try {
			return {
				ok: true,
				result: await ctx.agent.invoke(id, args),
				durationMs: Date.now() - start
			};
		} catch (error) {
			const e = error;
			return {
				ok: false,
				error: {
					name: e?.name ?? "Error",
					message: e?.message ?? String(error),
					stack: e?.stack
				},
				durationMs: Date.now() - start
			};
		}
	} })
});
//#endregion
//#region src/node/rpc/functions/list-commands.ts
/**
* Enumerate every command registered on the hub's commands host (id,
* title, description, icon, category, whether it carries its own handler,
* and nested children) when this connection is mounted inside a hub
* (`@devframes/hub`). Returns an empty list on a plain devframe connection,
* so the inspector's Commands tab degrades gracefully rather than erroring.
* `snapshot: true` bakes the (possibly empty) list into the static dump so
* the inspector still lists commands in `build`/`spa` mode.
*/
const listCommands = defineRpcFunction({
	name: "devframes:plugin:inspect:list-commands",
	type: "query",
	jsonSerializable: true,
	snapshot: true,
	agent: {
		description: "List every command registered on the hub commands host (id, title, description, category, whether it has a handler, children), when this connection is mounted inside a hub. Read-only. Empty outside a hub.",
		title: "List hub commands"
	},
	setup: (ctx) => ({ handler: async () => {
		const host = resolveHubCommands(ctx);
		if (!host) return [];
		const out = [...host.commands.values()].map(projectCommand);
		out.sort((a, b) => a.title.localeCompare(b.title));
		return out;
	} })
});
//#endregion
//#region src/function-info.ts
const FALLBACK_SCHEMA = Object.freeze({
	type: "object",
	additionalProperties: true
});
/** Function types the inspector invokes (read-only; no side effects). */
const INVOKABLE_TYPES = /* @__PURE__ */ new Set(["query", "static"]);
/**
* Project one registered RPC definition into the JSON-safe
* {@link RpcFunctionInfo} the inspector renders. Shared by the node-side
* `list-functions` RPC and the browser-side Client tab (which projects
* `rpc.client` definitions locally), so the two listings cannot drift.
*/
function projectRpcFunctionInfo(name, fn) {
	const type = fn.type ?? "query";
	let agent;
	if (fn.agent) agent = {
		description: fn.agent.description,
		title: fn.agent.title,
		safety: fn.agent.safety,
		tags: fn.agent.tags
	};
	return {
		name,
		type,
		jsonSerializable: fn.jsonSerializable === true,
		snapshot: fn.snapshot === true,
		cacheable: fn.cacheable === true,
		hasArgs: !!fn.args,
		hasReturns: !!fn.returns,
		hasDump: !!fn.dump,
		hasSetup: !!fn.setup,
		hasHandler: !!fn.handler,
		invokable: INVOKABLE_TYPES.has(type),
		agent,
		argsSchema: argsSchemaToJson(fn.args),
		returnsSchema: returnSchemaToJson(fn.returns)
	};
}
/**
* Convert a schema to JSON Schema for the inspector, vendor-neutrally.
*
* Uses the schema's own [Standard JSON Schema](https://standardschema.dev/)
* converter (`~standard.jsonSchema`, implemented by e.g. zod 4) when the
* validator provides one, and degrades to a permissive object schema
* otherwise (e.g. valibot, which has no native converter), so introspection
* never throws and never pulls in a converter library for a validator devframe
* doesn't otherwise depend on.
*/
function convert(schema) {
	const standard = schema["~standard"];
	if (!standard.jsonSchema) return FALLBACK_SCHEMA;
	try {
		return standard.jsonSchema.input({ target: "draft-2020-12" });
	} catch {
		return FALLBACK_SCHEMA;
	}
}
/**
* Convert an RPC return schema to JSON Schema, swallowing conversion
* failures into a permissive fallback so introspection never throws.
*/
function returnSchemaToJson(schema) {
	if (!schema) return void 0;
	return convert(schema);
}
/**
* Convert positional args schemas to a single JSON Schema tuple
* (`type: 'array'` + `prefixItems`). Returns `undefined` when the function
* declares no args.
*/
function argsSchemaToJson(args) {
	if (!args || args.length === 0) return void 0;
	return {
		type: "array",
		prefixItems: args.map((arg) => convert(arg))
	};
}
//#endregion
//#region src/node/rpc/functions/list-functions.ts
/**
* Enumerate every RPC function registered on the connection, projecting
* each definition's metadata into a JSON-safe {@link RpcFunctionInfo}.
* `snapshot: true` bakes the registry into the static dump so the
* inspector still lists functions in `build`/`spa` mode.
*/
const listFunctions = defineRpcFunction({
	name: "devframes:plugin:inspect:list-functions",
	type: "query",
	jsonSerializable: true,
	snapshot: true,
	agent: {
		description: "List every RPC function registered on this devframe connection, with metadata (name, type, JSON-serializable/snapshot flags, args/return schema, agent exposure). Read-only, the canonical way to discover what the running devframe can do.",
		title: "List RPC functions"
	},
	setup: (ctx) => ({ handler: async () => {
		const out = [];
		for (const [name, fn] of ctx.rpc.definitions) out.push(projectRpcFunctionInfo(name, fn));
		out.sort((a, b) => a.name.localeCompare(b.name));
		return out;
	} })
});
//#endregion
//#region src/node/rpc/functions/list-instances.ts
/**
* Enumerate every devframe dev server currently running on this machine,
* discovered through the shared instance registry (`~/.devframe/instances/`)
* with a `__connection.json` liveness probe, the same discovery that backs
* the `devframe connect` bin. Powers the inspector's read-only Instances tab.
*
* Deliberately **not** `snapshot` (the set of live processes is meaningless
* baked into a static dump, so the Instances tab is hidden in `build`/`spa`
* mode) and **not** agent-exposed (the `devframe connect` bin already offers
* instance discovery to agents over MCP, so exposing it here would duplicate
* that surface).
*/
const listInstances = defineRpcFunction({
	name: "devframes:plugin:inspect:list-instances",
	type: "query",
	jsonSerializable: true,
	setup: () => ({ handler: async () => {
		const { live } = await listLiveDevframeInstances();
		const currentPid = process.pid;
		return live.map((record) => ({
			id: record.id,
			name: record.name,
			port: record.port,
			origin: record.origin,
			basePath: record.basePath,
			url: `${record.origin}${record.basePath}`,
			pid: record.pid,
			rootDir: record.rootDir,
			startedAt: record.startedAt,
			hasMcp: record.mcp != null,
			isCurrent: record.pid === currentPid
		}));
	} })
});
//#endregion
//#region src/node/rpc/index.ts
/**
* The introspection RPC functions registered by the inspector plugin.
* Namespaced `devframes:plugin:inspect:*` per the plugin convention.
*/
const serverFunctions = [
	listFunctions,
	invoke,
	defineRpcFunction({
		name: "devframes:plugin:inspect:list-state-keys",
		type: "query",
		jsonSerializable: true,
		snapshot: true,
		agent: {
			description: "List the keys of all shared-state entries published by this devframe connection. Read-only. Pair with the built-in shared-state reads to inspect individual values.",
			title: "List shared-state keys"
		},
		setup: (ctx) => ({ handler: async () => {
			return [...ctx.rpc.sharedState.keys()].sort();
		} })
	}),
	describeAgent,
	invokeAgentTool,
	defineRpcFunction({
		name: "devframes:plugin:inspect:read-agent-resource",
		type: "action",
		setup: (ctx) => ({ handler: async (id) => {
			const start = Date.now();
			try {
				return {
					ok: true,
					result: await ctx.agent.read(id),
					durationMs: Date.now() - start
				};
			} catch (error) {
				const e = error;
				return {
					ok: false,
					error: {
						name: e?.name ?? "Error",
						message: e?.message ?? String(error),
						stack: e?.stack
					},
					durationMs: Date.now() - start
				};
			}
		} })
	}),
	listCommands,
	executeCommand,
	listInstances
];
//#endregion
export { serverFunctions as t };
