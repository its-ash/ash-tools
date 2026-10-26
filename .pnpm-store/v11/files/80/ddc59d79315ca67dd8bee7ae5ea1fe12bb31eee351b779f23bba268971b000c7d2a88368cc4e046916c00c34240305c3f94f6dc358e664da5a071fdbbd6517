import { g as configureAssets, m as UPLOAD_CHANNEL, r as writeFunctions, t as readFunctions } from "./rpc-f6T4yE2q.mjs";
import process from "node:process";
import { existsSync } from "node:fs";
import fsp from "node:fs/promises";
import { watch } from "chokidar";
import { debounce } from "devframe/utils/debounce";
//#region src/node/constants.ts
/** Broadcast whenever a file is added, removed, or changed under the managed directory. */
const CHANGED_EVENT = "devframes:plugin:assets:changed";
/** Preferred standalone CLI port. */
const DEFAULT_PORT = 9015;
//#endregion
//#region src/node/watcher.ts
/**
* Watch the managed directory and broadcast {@link CHANGED_EVENT} (debounced)
* whenever a file is added, removed, or changed, so connected UIs refresh
* their listing live. Returns a disposer; call it when the devframe shuts
* down (tests in particular, since a leaked watcher keeps the process alive).
*/
function watchAssetsDir(ctx, dir) {
	const notify = debounce(async () => {
		await ctx.rpc.broadcast({
			method: CHANGED_EVENT,
			args: []
		});
	}, 300);
	const watcher = watch(dir, {
		ignoreInitial: true,
		/**
		* Directories can nest arbitrarily deep (icon sets, generated builds
		* dropped into the managed dir, …); chokidar's default depth is
		* unlimited, but pin a generous cap so a runaway symlink loop can't
		* spin the watcher forever.
		*/
		depth: 32
	});
	watcher.on("add", () => void notify()).on("unlink", () => void notify()).on("addDir", () => void notify()).on("unlinkDir", () => void notify()).on("change", () => void notify());
	return async () => {
		await watcher.close();
		if (process.platform === "win32") await new Promise((resolve) => setTimeout(resolve, 200));
	};
}
//#endregion
//#region src/node/setup.ts
const watchers = /* @__PURE__ */ new WeakMap();
/**
* Register the assets RPC surface on a devframe node context: ensures the
* managed directory exists, registers the read RPCs (always) and write
* RPCs (when enabled), and starts a live file watcher in dev mode.
*
* Raw asset bytes (for `<img>`/`<video>`/download previews) are served by
* the **host** the plugin is attached to: Vite, Nuxt, or any framework
* already serves its `public/` dir at {@link SetupAssetsOptions.baseURL}.
* The plugin only serves them itself when `serveStatic` is set (the
* standalone CLI, which is its own host).
*
* Called from the definition's `setup(ctx)` and reusable by host adapters
* that wire their own context.
*/
async function setupAssets(ctx, options) {
	if (!existsSync(options.dir)) await fsp.mkdir(options.dir, { recursive: true });
	const uploadChannel = options.write ? ctx.rpc.streaming.create(UPLOAD_CHANNEL) : void 0;
	configureAssets(ctx, {
		...options,
		uploadChannel
	});
	if (options.serveStatic) ctx.views.hostStatic(options.baseURL, options.dir);
	for (const fn of readFunctions) ctx.rpc.register(fn);
	if (options.write) for (const fn of writeFunctions) ctx.rpc.register(fn);
	if (ctx.mode === "dev" && options.watch) watchers.set(ctx, watchAssetsDir(ctx, options.dir));
}
/**
* Stop the live file watcher started by {@link setupAssets}. Test harnesses
* wrap their dev-server `close()` with this (see `test/_utils.ts`), so a
* leaked chokidar watcher doesn't keep the process alive.
*/
async function disposeAssetsWatcher(ctx) {
	const dispose = watchers.get(ctx);
	if (!dispose) return;
	watchers.delete(ctx);
	await dispose();
}
//#endregion
export { setupAssets as n, DEFAULT_PORT as r, disposeAssetsWatcher as t };
