import { c as onDataSourcesChanged, l as registerDataSource, n as createDataSourcesService, r as getDataSource, s as onDataSourceDataChanged, t as DATA_SOURCES_SERVICE_ID } from "./registry-BO6lIklX.mjs";
import { t as serverFunctions } from "./rpc-CdXqivOI.mjs";
import os from "node:os";
import process from "node:process";
//#region src/node/example-source.ts
/** Id of the built-in example source (namespaced like every source id). */
const EXAMPLE_SOURCE_ID = "devframes:plugin:data-inspector:example";
var RequestLog = class {
	startedAt = /* @__PURE__ */ new Date();
	secret = "own-fields-are-visible";
	entries = /* @__PURE__ */ new Map([
		["home", {
			path: "/",
			hits: 41
		}],
		["docs", {
			path: "/docs",
			hits: 7
		}],
		["api", {
			path: "/api/data",
			hits: 23
		}]
	]);
};
/** A small synthetic branch exercising every viewer capability. */
function createPlayground() {
	const node = { name: "root" };
	node.self = node;
	return {
		requests: new RequestLog(),
		tags: /* @__PURE__ */ new Set([
			"dev",
			"local",
			"example"
		]),
		middlewares: [{
			name: "compression",
			handler: () => {}
		}, {
			name: "static",
			handler: () => {}
		}],
		build: {
			hash: "c3f7c4e",
			size: 1024n * 512n,
			modules: Array.from({ length: 300 }, (_, i) => ({
				id: `src/module-${i}.ts`,
				imports: i % 7,
				sizeKb: Math.round(Math.sin(i) * 40 + 50)
			}))
		},
		tree: node
	};
}
/** The devframe context, projected to its inspectable surface. */
function describeContext(ctx) {
	return {
		cwd: ctx.cwd,
		workspaceRoot: ctx.workspaceRoot,
		mode: ctx.mode,
		rpcFunctions: Array.from(ctx.rpc.definitions.keys()).sort(),
		services: ctx.services.keys().sort(),
		storage: {
			workspace: ctx.host.getStorageDir("workspace"),
			project: ctx.host.getStorageDir("project"),
			global: ctx.host.getStorageDir("global")
		}
	};
}
function createExampleData(ctx) {
	const data = {
		...ctx ? { devframe: describeContext(ctx) } : {},
		os: {
			platform: os.platform(),
			arch: os.arch(),
			release: os.release(),
			hostname: os.hostname(),
			cpus: {
				count: os.cpus().length,
				model: os.cpus()[0]?.model
			},
			memory: {
				totalMb: Math.round(os.totalmem() / 1024 / 1024),
				freeMb: Math.round(os.freemem() / 1024 / 1024)
			},
			homedir: os.homedir(),
			uptimeHours: Math.round(os.uptime() / 36) / 100
		},
		process: {
			pid: process.pid,
			execPath: process.execPath,
			argv: process.argv,
			versions: process.versions
		},
		playground: createPlayground()
	};
	Object.defineProperties(data.process, {
		memory: {
			enumerable: true,
			get: () => process.memoryUsage()
		},
		uptimeSeconds: {
			enumerable: true,
			get: () => Math.round(process.uptime())
		}
	});
	Object.defineProperty(data, "queriedAt", {
		enumerable: true,
		get: () => (/* @__PURE__ */ new Date()).toISOString()
	});
	return data;
}
/**
* The built-in example source, always registered unless opted out
* (`exampleSource: false`). Lets the viewer query simple environment info
* (the devframe context: registered RPC functions, services, storage dirs;
* OS and live process stats) plus a small playground branch exercising
* everything the viewer can show (Maps, Sets, class instances, circulars,
* BigInt, functions, query-time getters).
*/
function createExampleDataSource(ctx) {
	let data;
	return {
		id: EXAMPLE_SOURCE_ID,
		title: "Example data",
		description: "Devframe context, OS and process info, plus a playground graph.",
		icon: "i-ph:flask-duotone",
		data: () => data ??= createExampleData(ctx),
		/**
		* The factory memoizes, so live edits stick between queries; the
		* playground doubles as a demo of writable sources.
		*/
		writable: true,
		queries: [
			...ctx ? [{
				title: "Registered RPC functions",
				description: "Everything callable on this devframe connection",
				query: "devframe.rpcFunctions"
			}] : [],
			{
				title: "OS at a glance",
				query: "os.({ platform, arch, cpus, memory })"
			},
			{
				title: "Live process memory",
				description: "Query-time getter: re-run and watch it change",
				query: "{ memory: process.memory, uptime: process.uptimeSeconds, at: queriedAt }"
			},
			{
				title: "Busiest endpoints",
				description: "Live Map entries, sorted",
				query: "playground.requests.entries.mapEntries().value.sort(hits desc)"
			},
			{
				title: "Large modules as a table",
				description: "Filter and project an array",
				query: "playground.build.modules.[sizeKb > 80].({ id, sizeKb })"
			},
			{
				title: "Middlewares (data only)",
				description: "The exclude-functions filter strips the handlers",
				query: "playground.middlewares",
				excludeFunctions: true
			},
			{
				title: "Everything",
				query: ""
			}
		]
	};
}
//#endregion
//#region src/node/setup.ts
/** Broadcast whenever the source registry changes (register/unregister). */
const SOURCES_CHANGED_EVENT = "devframes:plugin:data-inspector:sources:changed";
/**
* Broadcast whenever a source's DATA changes: a successful `write`, a
* `notifyChanged()` handle call, or a source's own `subscribe` bridge.
* Carries the source id so clients refresh only the affected view.
*/
const DATA_CHANGED_EVENT = "devframes:plugin:data-inspector:data:changed";
/**
* Register the data-inspector's RPC functions on a devframe node context,
* provide the source registry as a typed context service, and broadcast
* registry changes so connected UIs refresh their source list live.
*
* Called from the definition's `setup(ctx)` and reusable by host adapters
* (the CLI and the inject endpoint wire their own contexts through this).
*/
function setupDataInspector(ctx, options = {}) {
	for (const fn of serverFunctions) ctx.rpc.register(fn);
	if (!ctx.services.has("devframes:plugin:data-inspector:sources")) ctx.services.provide(DATA_SOURCES_SERVICE_ID, createDataSourcesService());
	if ((options.exampleSource ?? true) && !getDataSource("devframes:plugin:data-inspector:example")) registerDataSource(createExampleDataSource(ctx));
	onDataSourcesChanged(() => {
		ctx.rpc.broadcast({
			method: SOURCES_CHANGED_EVENT,
			args: []
		});
	});
	onDataSourceDataChanged((sourceId) => {
		ctx.rpc.broadcast({
			method: DATA_CHANGED_EVENT,
			args: [sourceId]
		});
	});
}
//#endregion
export { createExampleDataSource as a, EXAMPLE_SOURCE_ID as i, SOURCES_CHANGED_EVENT as n, setupDataInspector as r, DATA_CHANGED_EVENT as t };
