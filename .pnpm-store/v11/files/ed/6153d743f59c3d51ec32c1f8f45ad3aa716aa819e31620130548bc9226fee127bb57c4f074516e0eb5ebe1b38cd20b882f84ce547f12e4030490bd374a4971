import { defineDiagnostics } from "devframe/utils/nostics";
//#region src/node/diagnostics.ts
/**
* Structured diagnostics for `@devframes/plugin-data-inspector`. Node-side
* only. Codes use the plugin-private `DP_DATA_INSPECTOR_` band so they never
* collide with devframe core (`DF00xx`) or `@devframes/hub` (`DF80xx`).
*/
const diagnostics = defineDiagnostics({
	docsBase: "https://devfra.me/errors",
	codes: {
		DP_DATA_INSPECTOR_0001: {
			why: (p) => `No data source is registered under "${p.id}".`,
			fix: "List the available sources via the `sources` RPC (or `listDataSources()`), and make sure the contributor registered the source before querying it."
		},
		DP_DATA_INSPECTOR_0002: {
			why: (p) => `Unsupported data file "${p.filepath}".`,
			fix: "The standalone CLI loads .json and .jsonl/.ndjson files. Convert the data, or register a custom source via the registry."
		},
		DP_DATA_INSPECTOR_0003: {
			why: (p) => `Failed to load data file "${p.filepath}": ${p.message}`,
			fix: "Check that the file exists and contains valid JSON (or one JSON value per line for .jsonl/.ndjson)."
		},
		DP_DATA_INSPECTOR_0004: {
			why: (p) => `Data source "${p.id}" declares both \`static: true\` and \`writable: true\`; a memoized snapshot is read-only.`,
			fix: "Drop `static: true` to make the live object writable, or drop `writable: true` to keep the memoized snapshot. The source stays read-only."
		}
	}
});
//#endregion
//#region src/node/registry/index.ts
/** Id under which the registry is provided on `ctx.services`. */
const DATA_SOURCES_SERVICE_ID = "devframes:plugin:data-inspector:sources";
const GLOBAL_KEY = Symbol.for("devframes:plugin:data-inspector:registry@1");
function store() {
	const holder = globalThis;
	let value = holder[GLOBAL_KEY];
	if (!value) {
		value = {
			entries: /* @__PURE__ */ new Map(),
			staticCache: /* @__PURE__ */ new Map(),
			listeners: /* @__PURE__ */ new Set(),
			dataListeners: /* @__PURE__ */ new Set(),
			subscriptions: /* @__PURE__ */ new Map()
		};
		holder[GLOBAL_KEY] = value;
	}
	value.dataListeners ??= /* @__PURE__ */ new Set();
	value.subscriptions ??= /* @__PURE__ */ new Map();
	return value;
}
function notify(registry) {
	for (const listener of registry.listeners) listener();
}
/** Broadcast that a source's data changed to every data-changed listener. */
function notifyDataSourceChanged(id) {
	const registry = store();
	for (const listener of registry.dataListeners) listener(id);
}
/** Effective writability of an entry: opt-in, and never on a static source. */
function isWritableEntry(entry) {
	return (entry.writable ?? false) && !entry.static;
}
function unwireSubscription(registry, id) {
	const dispose = registry.subscriptions.get(id);
	if (dispose) {
		registry.subscriptions.delete(id);
		try {
			dispose();
		} catch {}
	}
}
/** Register (or replace) a data source. Returns a handle to the registration. */
function registerDataSource(entry) {
	const registry = store();
	if (entry.static && entry.writable) diagnostics.DP_DATA_INSPECTOR_0004({ id: entry.id });
	unwireSubscription(registry, entry.id);
	registry.entries.set(entry.id, entry);
	registry.staticCache.delete(entry.id);
	const notifyChanged = () => notifyDataSourceChanged(entry.id);
	if (entry.subscribe) {
		const dispose = entry.subscribe(notifyChanged);
		if (dispose) registry.subscriptions.set(entry.id, dispose);
	}
	notify(registry);
	return {
		notifyChanged,
		unregister: () => unregisterDataSource(entry.id)
	};
}
function unregisterDataSource(id) {
	const registry = store();
	unwireSubscription(registry, id);
	if (registry.entries.delete(id)) {
		registry.staticCache.delete(id);
		notify(registry);
	}
}
function listDataSources() {
	return Array.from(store().entries.values()).map((entry) => ({
		id: entry.id,
		title: entry.title,
		description: entry.description,
		icon: entry.icon,
		static: entry.static ?? false,
		writable: isWritableEntry(entry),
		queries: entry.queries
	}));
}
function getDataSource(id) {
	return store().entries.get(id);
}
/** Resolve a source's data, honoring value-vs-factory and `static` memoization. */
async function resolveSourceData(entry) {
	if (typeof entry.data !== "function") return entry.data;
	const factory = entry.data;
	if (!entry.static) return factory();
	const registry = store();
	let cached = registry.staticCache.get(entry.id);
	if (!cached) {
		cached = Promise.resolve(factory());
		registry.staticCache.set(entry.id, cached);
		cached.catch(() => registry.staticCache.delete(entry.id));
	}
	return cached;
}
/** Subscribe to registry changes (register/unregister). Returns unsubscribe. */
function onDataSourcesChanged(listener) {
	const registry = store();
	registry.listeners.add(listener);
	return () => {
		registry.listeners.delete(listener);
	};
}
/**
* Subscribe to data-changed notifications (writes, `notifyChanged` handles,
* `subscribe` bridges). The listener receives the source id.
*/
function onDataSourceDataChanged(listener) {
	const registry = store();
	registry.dataListeners.add(listener);
	return () => {
		registry.dataListeners.delete(listener);
	};
}
/** The service implementation provided on `ctx.services` by `setup()`. */
function createDataSourcesService() {
	return {
		register: registerDataSource,
		unregister: unregisterDataSource,
		list: listDataSources,
		get: getDataSource,
		onChanged: onDataSourcesChanged,
		onDataChanged: onDataSourceDataChanged
	};
}
//#endregion
export { DATA_SOURCES_SERVICE_ID, createDataSourcesService, getDataSource, isWritableEntry, listDataSources, notifyDataSourceChanged, onDataSourceDataChanged, onDataSourcesChanged, registerDataSource, resolveSourceData };
