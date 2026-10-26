import { a as listDataSources, d as resolveSourceData, i as isWritableEntry, o as notifyDataSourceChanged, r as getDataSource } from "./registry-BO6lIklX.mjs";
import { a as suggest$1, i as runQueryAtPath, n as skeletonOf, r as runQuery, t as applyWrite } from "./write-BcfDKuf7.mjs";
import { defineRpcFunction } from "devframe";
import { join } from "node:path";
import { createStorage } from "devframe/node";
//#region src/node/rpc/functions/_define.ts
/** RPC namespace, the plugin id. */
const NS = "devframes:plugin:data-inspector";
//#endregion
//#region src/node/rpc/functions/query.ts
/**
* Execute a jora query against a registered source. Runs in-process against
* the live object; the result is normalized to strict JSON (circulars ->
* `$ref`, exotic types tagged, depth/entry caps) before it rides the wire.
*/
const query = defineRpcFunction({
	name: `${NS}:query`,
	type: "query",
	jsonSerializable: true,
	agent: {
		title: "Run a jora query",
		description: "Execute a jora query against a registered data source and return the normalized result with stats."
	},
	setup: () => ({ handler: async (sourceId, joraQuery, options) => {
		const source = getDataSource(sourceId);
		if (!source) return {
			ok: false,
			error: {
				name: "UnknownSource",
				message: `No data source "${sourceId}"`
			}
		};
		return runQuery(await resolveSourceData(source), joraQuery, options);
	} })
});
//#endregion
//#region src/node/rpc/functions/query-path.ts
/**
* Lazily expand a depth-truncated node. Re-runs the base jora query against
* the live source, re-descends to the node addressed by `path` (a `NodePath`
* lifted from the `$truncated: 'depth'` marker the client is expanding), and
* returns just that subtree normalized with a fresh depth budget, so huge
* graphs load a level at a time instead of all at once.
*/
const queryPath = defineRpcFunction({
	name: `${NS}:queryPath`,
	type: "query",
	jsonSerializable: true,
	agent: {
		title: "Expand a nested node",
		description: "Re-run a jora query and return a fresh, depth-limited slice of the subtree at a given node path."
	},
	setup: () => ({ handler: async (sourceId, joraQuery, path, options) => {
		const source = getDataSource(sourceId);
		if (!source) return {
			ok: false,
			error: {
				name: "UnknownSource",
				message: `No data source "${sourceId}"`
			}
		};
		return runQueryAtPath(await resolveSourceData(source), joraQuery, path, options);
	} })
});
//#endregion
//#region src/node/saved-queries.ts
const stores = /* @__PURE__ */ new WeakMap();
function storesFor(ctx) {
	let byScope = stores.get(ctx);
	if (!byScope) {
		const open = (scope) => createStorage({
			filepath: join(ctx.host.getStorageDir(scope), "data-inspector/queries.json"),
			initialValue: { queries: {} }
		});
		byScope = {
			workspace: open("workspace"),
			project: open("project")
		};
		stores.set(ctx, byScope);
	}
	return byScope;
}
function slugify(text) {
	return text.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "").slice(0, 60);
}
/** djb2, a stable short id for untitled queries (same query, same id). */
function hashOf(text) {
	let hash = 5381;
	for (let i = 0; i < text.length; i++) hash = (hash << 5) + hash + text.charCodeAt(i) >>> 0;
	return hash.toString(36);
}
function deriveId(input) {
	if (input.id) return input.id;
	if (input.title?.trim()) return slugify(input.title);
	return `q-${hashOf(input.query)}`;
}
function listSavedQueries(ctx) {
	const byScope = storesFor(ctx);
	const out = [];
	for (const scope of ["workspace", "project"]) for (const entry of Object.values(byScope[scope].value().queries)) out.push({
		...entry,
		scope
	});
	return out.sort((a, b) => b.updatedAt - a.updatedAt);
}
function saveQuery(ctx, input) {
	const byScope = storesFor(ctx);
	const id = deriveId(input);
	const record = {
		id,
		query: input.query,
		title: input.title?.trim() || void 0,
		description: input.description?.trim() || void 0,
		excludeFunctions: input.excludeFunctions || void 0,
		excludeUnderscoreProps: input.excludeUnderscoreProps || void 0,
		excludeDollarProps: input.excludeDollarProps || void 0,
		updatedAt: Date.now()
	};
	byScope[input.scope].mutate((draft) => {
		draft.queries[id] = record;
	});
	const other = input.scope === "project" ? "workspace" : "project";
	if (byScope[other].value().queries[id]) byScope[other].mutate((draft) => {
		delete draft.queries[id];
	});
	return {
		...record,
		scope: input.scope
	};
}
function deleteSavedQuery(ctx, id, scope) {
	const byScope = storesFor(ctx);
	const exists = !!byScope[scope].value().queries[id];
	if (exists) byScope[scope].mutate((draft) => {
		delete draft.queries[id];
	});
	return exists;
}
//#endregion
//#region src/node/rpc/functions/saved.ts
const savedList = defineRpcFunction({
	name: `${NS}:saved:list`,
	type: "query",
	jsonSerializable: true,
	setup: (ctx) => ({ handler: async () => listSavedQueries(ctx) })
});
const savedSave = defineRpcFunction({
	name: `${NS}:saved:save`,
	type: "action",
	jsonSerializable: true,
	setup: (ctx) => ({ handler: async (input) => saveQuery(ctx, input) })
});
const savedDelete = defineRpcFunction({
	name: `${NS}:saved:delete`,
	type: "action",
	jsonSerializable: true,
	setup: (ctx) => ({ handler: async (id, scope) => deleteSavedQuery(ctx, id, scope) })
});
//#endregion
//#region src/node/rpc/functions/skeleton.ts
/** The type skeleton of a source ("what data are available"), query-independent. */
const skeleton = defineRpcFunction({
	name: `${NS}:skeleton`,
	type: "query",
	jsonSerializable: true,
	setup: () => ({ handler: async (sourceId, options) => {
		const source = getDataSource(sourceId);
		if (!source) return {
			ok: false,
			error: {
				name: "UnknownSource",
				message: `No data source "${sourceId}"`
			}
		};
		try {
			return {
				ok: true,
				...skeletonOf(await resolveSourceData(source), options)
			};
		} catch (error) {
			const e = error instanceof Error ? error : new Error(String(error));
			return {
				ok: false,
				error: {
					name: e.name,
					message: e.message
				}
			};
		}
	} })
});
//#endregion
//#region src/node/rpc/index.ts
/**
* The RPC functions registered by the data-inspector plugin.
* Namespaced `devframes:plugin:data-inspector:*` (the plugin id).
*/
const serverFunctions = [
	defineRpcFunction({
		name: `${NS}:sources`,
		type: "query",
		jsonSerializable: true,
		agent: {
			title: "List data sources",
			description: "List the data sources registered with the data inspector (id, title, description, suggested queries)."
		},
		setup: () => ({ handler: async () => listDataSources() })
	}),
	query,
	queryPath,
	skeleton,
	defineRpcFunction({
		name: `${NS}:suggest`,
		type: "query",
		jsonSerializable: true,
		setup: () => ({ handler: async (sourceId, joraQuery, pos) => {
			const source = getDataSource(sourceId);
			if (!source) return {
				ok: false,
				suggestions: [],
				statMs: 0,
				error: `No data source "${sourceId}"`
			};
			return suggest$1(await resolveSourceData(source), joraQuery, pos);
		} })
	}),
	savedList,
	savedSave,
	savedDelete,
	defineRpcFunction({
		name: `${NS}:write`,
		type: "action",
		jsonSerializable: true,
		setup: () => ({ handler: async (sourceId, request, options) => {
			const source = getDataSource(sourceId);
			if (!source) return {
				ok: false,
				error: {
					name: "UnknownSource",
					message: `No data source "${sourceId}"`
				}
			};
			if (!isWritableEntry(source)) return {
				ok: false,
				error: {
					name: "ReadonlySource",
					message: `Data source "${sourceId}" is not writable`
				}
			};
			const outcome = applyWrite(await resolveSourceData(source), request, options);
			if (outcome.ok) notifyDataSourceChanged(sourceId);
			return outcome;
		} })
	})
];
//#endregion
export { serverFunctions as t };
