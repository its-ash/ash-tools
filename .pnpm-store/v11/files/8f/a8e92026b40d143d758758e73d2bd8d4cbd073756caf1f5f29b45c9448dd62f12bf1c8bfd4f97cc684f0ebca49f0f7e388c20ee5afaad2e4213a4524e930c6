import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { createServer } from "node:http";
import { homedir } from "node:os";
import { dirname, join } from "node:path";
import process from "node:process";
import { createContextRpcServer } from "devframe/internal";
import { createHostContext } from "devframe/node";
import { createInteractiveAuth } from "devframe/recipes/interactive-auth";
import { attachWsRpcTransport } from "devframe/rpc/transports/ws-server";
import { randomToken } from "devframe/utils/crypto-token";
import { getPort } from "devframe/utils/get-port";
//#region src/inject/index.ts
/** Discovery file path, relative to the target process's cwd. */
const DISCOVERY_FILE = "node_modules/.data-inspector/discovery.json";
/**
* A data source exposing the inspected process's global object. On the
* zero-code `--import` path there is nowhere to call `registerDataSource`, so
* assigning to `globalThis` is how you surface things to inspect
* (`globalThis.store = store`); this source makes them queryable live. The
* factory reads `globalThis` at query time, so late assignments show up on the
* next run.
*/
function createGlobalThisDataSource() {
	return {
		id: "globalThis",
		title: "globalThis",
		description: "The global object of the inspected process. Assign values to inspect them, e.g. globalThis.store = store",
		icon: "i-ph:globe-duotone",
		data: () => globalThis
	};
}
/** Start the inject endpoint in the current process. */
async function exposeDataInspector(options = {}) {
	const { setupDataInspector } = await import("../setup-D3BVizy8.mjs");
	if (options.sources?.length) {
		const { registerDataSource } = await import("../registry-Dl_TiAxW.mjs");
		for (const source of options.sources) registerDataSource(source);
	}
	const cwd = process.cwd();
	const port = await getPort({
		port: options.port ?? 9878,
		portRange: [9878, 9978]
	});
	const websocket = `ws://127.0.0.1:${port}`;
	const context = await createHostContext({
		cwd,
		mode: "dev",
		host: {
			mountStatic() {},
			mountConnectionMeta() {},
			resolveOrigin: () => `http://127.0.0.1:${port}`,
			getStorageDir(scope) {
				if (scope === "workspace") return join(cwd, ".devframe");
				if (scope === "project") return join(cwd, "node_modules/.data-inspector/devframe");
				return join(homedir(), ".data-inspector/devframe");
			}
		}
	});
	setupDataInspector(context, { exampleSource: options.exampleSource });
	const useAuth = options.auth ?? true;
	const token = useAuth ? options.token ?? randomToken() : void 0;
	const auth = useAuth ? createInteractiveAuth(context, {
		clientAuthTokens: [token],
		banner: () => {}
	}) : false;
	const httpServer = createServer();
	const { rpcGroup, onConnected, onDisconnected } = createContextRpcServer({
		context,
		auth
	});
	const { close: closeWs } = attachWsRpcTransport(rpcGroup, {
		server: httpServer,
		destroyUnmatched: true,
		onConnected,
		onDisconnected
	});
	await new Promise((resolve, reject) => {
		const onError = (error) => reject(error);
		httpServer.once("error", onError);
		httpServer.listen(port, "127.0.0.1", () => {
			httpServer.removeListener("error", onError);
			resolve();
		});
	});
	const discoveryPath = join(cwd, DISCOVERY_FILE);
	if (options.discoveryFile !== false) {
		const discovery = {
			websocket,
			token,
			pid: process.pid,
			startedAt: Date.now()
		};
		mkdirSync(dirname(discoveryPath), { recursive: true });
		writeFileSync(discoveryPath, `${JSON.stringify(discovery, null, 2)}\n`);
		process.once("exit", () => {
			try {
				rmSync(discoveryPath, { force: true });
			} catch {}
		});
	}
	if (!options.silent) {
		console.error(`[data-inspector] inject endpoint listening on ${websocket} (pid ${process.pid})`);
		console.error(`[data-inspector] attach with: devframe-data-inspector attach${options.discoveryFile === false && token ? ` ${websocket} --token ${token}` : ""}`);
	}
	return {
		websocket,
		token,
		close: async () => {
			if (options.discoveryFile !== false) try {
				rmSync(discoveryPath, { force: true });
			} catch {}
			await closeWs();
			await new Promise((resolve) => httpServer.close(() => resolve()));
		}
	};
}
if (process.env.DEVFRAME_DATA_INSPECTOR === "1" || process.env.DEVFRAME_DATA_INSPECTOR === "true") exposeDataInspector({
	port: process.env.DEVFRAME_DATA_INSPECTOR_PORT ? Number(process.env.DEVFRAME_DATA_INSPECTOR_PORT) : void 0,
	auth: process.env.DEVFRAME_DATA_INSPECTOR_AUTH !== "0",
	token: process.env.DEVFRAME_DATA_INSPECTOR_TOKEN,
	exampleSource: process.env.DEVFRAME_DATA_INSPECTOR_EXAMPLE !== "0",
	/**
	* Zero-code path: with no chance to call `registerDataSource`, expose
	* `globalThis` so assigning to it is enough to inspect anything.
	*/
	sources: process.env.DEVFRAME_DATA_INSPECTOR_GLOBAL !== "0" ? [createGlobalThisDataSource()] : void 0
}).catch((error) => {
	console.error("[data-inspector] inject endpoint failed to start:", error);
});
//#endregion
export { DISCOVERY_FILE, createGlobalThisDataSource, exposeDataInspector };
