import { defineRpcFunction } from "devframe";
import { dirname, extname, join, normalize, resolve } from "pathe";
import { createWriteStream } from "node:fs";
import fsp from "node:fs/promises";
import { defineDiagnostics } from "devframe/utils/nostics";
import { s } from "devframe/utils/simple-schema";
import { joinURL } from "devframe/utils/url";
import { glob } from "tinyglobby";
import { imageMeta } from "image-meta";
//#region src/node/diagnostics.ts
/**
* Uses the plugin's own `DP_ASSETS_` prefix per the built-in plugin
* convention, keeping it collision-free with devframe core (`DF`) and the
* hub (`DF8xxx`).
*/
const diagnostics = defineDiagnostics({
	docsBase: "https://devfra.me/errors",
	codes: {
		DP_ASSETS_0001: {
			why: (p) => `"${p.path}" resolves outside the managed directory.`,
			fix: "Use a path relative to the configured `dir`, without leading `/` or `..` segments."
		},
		DP_ASSETS_0002: {
			why: (p) => `Cannot upload "${p.path}" because the "${p.extension}" extension is not allowed. Allowed extensions: ${p.allowed.join(", ")}.`,
			fix: "Configure `uploadExtensions` on the assets devframe to allow this extension, or pass `'*'` to allow any."
		},
		DP_ASSETS_0003: {
			why: (p) => `Cannot rename to "${p.path}" because a file already exists there.`,
			fix: "Choose a different name."
		},
		DP_ASSETS_0004: {
			why: (p) => `No asset found at "${p.path}".`,
			fix: "Refresh the asset list; it may have been moved or deleted already."
		},
		DP_ASSETS_0005: {
			why: (p) => `Cannot create folder "${p.path}" because a file already exists there.`,
			fix: "Choose a different folder name."
		},
		DP_ASSETS_0006: {
			why: (p) => `"${p.name}" is not a valid file name.`,
			fix: "Names must be non-empty and cannot contain `/` or `\\`."
		},
		DP_ASSETS_0007: {
			why: "The upload streaming channel is unavailable because this devframe was set up with `write: false`.",
			fix: "This indicates an internal registration bug, since `upload` should never be reachable without `write: true`. Please report it."
		}
	}
});
//#endregion
//#region src/node/paths.ts
/** realpath, pathe-normalized, or `null` when the path doesn't exist. */
async function realpath(path) {
	try {
		return normalize(await fsp.realpath(path));
	} catch {
		return null;
	}
}
/**
* Resolve a client-supplied, root-relative path against the managed
* directory, rejecting anything that would escape it lexically (`..`
* traversal, a rogue absolute path). The first guard every RPC handler runs;
* symlink-aware containment is layered on by {@link resolveAssetReadPath}
* (reads) and {@link assertAssetMutationPath} (mutations).
*/
function resolveAssetPath(root, relativePath) {
	const normalizedRoot = resolve(root);
	const absolute = resolve(normalizedRoot, relativePath.replace(/^[/\\]+/, ""));
	if (absolute !== normalizedRoot && !absolute.startsWith(`${normalizedRoot}/`)) throw diagnostics.DP_ASSETS_0001({ path: relativePath });
	return absolute;
}
/**
* Resolve a path for a **read**, allowing a symlink only when its canonical
* target stays inside the canonical managed root. A target resolving outside
* throws `DP_ASSETS_0001`; a missing target is left for the caller's own read
* to fail.
*/
async function resolveAssetReadPath(root, relativePath) {
	const absolute = resolveAssetPath(root, relativePath);
	const real = await realpath(absolute);
	const canonRoot = await realpath(root) ?? resolve(root);
	if (real && real !== canonRoot && !real.startsWith(`${canonRoot}/`)) throw diagnostics.DP_ASSETS_0001({ path: relativePath });
	return absolute;
}
/**
* Resolve a path for a **mutation**, rejecting every pre-existing symlink
* among the path components from the managed root down to the target
* (including in-root symlinks) so a mutation can never follow a symlink out
* of, or around, the root. Only existing components are inspected, so it is
* safe for not-yet-created upload/mkdir targets; call it again after
* creating directories and right before the I/O. This closes deterministic,
* pre-existing symlink escapes, not concurrent component-swap races.
*/
async function assertAssetMutationPath(root, relativePath) {
	const lexRoot = resolve(root);
	const absolute = resolveAssetPath(root, relativePath);
	let current = await realpath(root) ?? lexRoot;
	for (const segment of absolute.slice(lexRoot.length).split("/").filter(Boolean)) {
		current += `/${segment}`;
		const stat = await fsp.lstat(current).catch(() => null);
		if (!stat) break;
		if (stat.isSymbolicLink()) throw diagnostics.DP_ASSETS_0001({ path: relativePath });
	}
	return absolute;
}
//#endregion
//#region src/node/context.ts
const configs = /* @__PURE__ */ new WeakMap();
const contexts = /* @__PURE__ */ new WeakMap();
/**
* Record the managed directory and write policy for a context. Called from
* the devframe's `setup` before any RPC handler runs.
*/
function configureAssets(ctx, config) {
	configs.set(ctx, config);
	contexts.delete(ctx);
}
/**
* Per-`DevframeNodeContext` assets state. Each RPC function pulls its
* managed directory and path resolver from here instead of re-reading
* `configureAssets`'s config on every call.
*/
function getAssetsContext(ctx) {
	const existing = contexts.get(ctx);
	if (existing) return existing;
	const config = configs.get(ctx);
	const dir = config?.dir ?? ctx.cwd;
	const built = {
		dir,
		write: config?.write ?? true,
		uploadExtensions: config?.uploadExtensions ?? "*",
		baseURL: config?.baseURL ?? "/",
		uploadChannel: config?.uploadChannel,
		resolvePath: (relativePath) => resolveAssetPath(dir, relativePath)
	};
	contexts.set(ctx, built);
	return built;
}
//#endregion
//#region src/node/rpc/functions/upload.ts
/** Streaming channel name, namespaced like every other RPC name in this plugin. */
const UPLOAD_CHANNEL = "devframes:plugin:assets:upload";
function isExtensionAllowed(path, allowed) {
	if (allowed === "*") return true;
	const ext = extname(path).slice(1).toLowerCase();
	return allowed.includes(ext);
}
/**
* Allocates an upload and returns a streaming id. Actual bytes flow over
* the `upload` channel opened in `setupAssets`; see the client-side
* `useUpload` hook for the matching `rpc.streaming.upload()` call.
*/
const upload = defineRpcFunction({
	name: "devframes:plugin:assets:upload",
	type: "action",
	jsonSerializable: true,
	args: [s.object({ path: s.string() })],
	returns: s.object({ uploadId: s.string() }),
	agent: {
		title: "Upload an asset",
		description: "Allocate an upload slot for a new file at the given path. The caller streams the bytes over the paired upload channel.",
		safety: "action",
		tags: ["assets"]
	},
	setup: (ctx) => {
		const assets = getAssetsContext(ctx);
		return { 
		/** See `list.ts` for why the async handler is cast. */
handler: (async ({ path }) => {
			if (!isExtensionAllowed(path, assets.uploadExtensions)) throw diagnostics.DP_ASSETS_0002({
				path,
				extension: extname(path).slice(1).toLowerCase() || "(none)",
				allowed: assets.uploadExtensions === "*" ? [] : assets.uploadExtensions
			});
			const absolute = await assertAssetMutationPath(assets.dir, path);
			await fsp.mkdir(dirname(absolute), { recursive: true });
			await assertAssetMutationPath(assets.dir, path);
			const channel = assets.uploadChannel;
			if (!channel) throw diagnostics.DP_ASSETS_0007();
			const reader = channel.openInbound();
			(async () => {
				const file = createWriteStream(absolute);
				try {
					for await (const chunk of reader) file.write(chunk);
				} catch {
					file.close();
					await fsp.unlink(absolute).catch(() => {});
					return;
				}
				file.close();
			})();
			return { uploadId: reader.id };
		}) };
	}
});
//#endregion
//#region src/node/rpc/functions/capabilities.ts
/**
* Lets the UI know upfront whether upload / rename / delete / mkdir are
* available, so it can hide or disable those affordances proactively
* instead of letting the user hit a "method not found" error, the same
* `canWrite`-gating idea the git plugin's `GitStatus.canWrite` follows.
*/
const capabilities = defineRpcFunction({
	name: "devframes:plugin:assets:capabilities",
	type: "query",
	snapshot: true,
	jsonSerializable: true,
	args: [],
	returns: s.object({
		write: s.boolean(),
		uploadExtensions: s.union([s.array(s.string()), s.literal("*")])
	}),
	agent: {
		title: "Read assets capabilities",
		description: "Read whether upload, rename, delete, and folder creation are enabled, and which upload extensions are allowed.",
		safety: "read",
		tags: ["assets"]
	},
	setup: (ctx) => {
		const assets = getAssetsContext(ctx);
		return { handler: (async () => ({
			write: assets.write,
			uploadExtensions: assets.uploadExtensions
		})) };
	}
});
//#endregion
//#region src/node/rpc/functions/delete.ts
/** One request covers both single- and multi-select delete. */
const deleteAssets = defineRpcFunction({
	name: "devframes:plugin:assets:delete",
	type: "action",
	jsonSerializable: true,
	args: [s.object({ paths: s.array(s.string()) })],
	returns: s.object({ deleted: s.array(s.string()) }),
	agent: {
		title: "Delete assets",
		description: "Delete one or more assets from the managed directory.",
		safety: "destructive",
		tags: ["assets"]
	},
	setup: (ctx) => {
		const assets = getAssetsContext(ctx);
		return { 
		/** See `list.ts` for why the async handler is cast. */
handler: (async ({ paths }) => {
			const deleted = [];
			for (const path of paths) {
				const absolute = await assertAssetMutationPath(assets.dir, path);
				try {
					await fsp.unlink(absolute);
					deleted.push(path);
				} catch (error) {
					if (error.code !== "ENOENT") throw error;
				}
			}
			return { deleted };
		}) };
	}
});
//#endregion
//#region src/node/scanner.ts
const reImage = /\.(?:png|jpe?g|jxl|gif|svg|webp|avif|ico|bmp|tiff?)$/i;
const reVideo = /\.(?:mp4|webm|ogv|mov|avi|flv|wmv|mpg|mpeg|mkv|3gp|3g2|ts|mts|m2ts|vob|ogm|ogx|rm|rmvb|asf|amv|divx|m4v|svi|viv|f4v|f4p|f4a|f4b)$/i;
const reAudio = /\.(?:mp3|wav|ogg|flac|aac|wma|alac|ape|ac3|dts|tta|opus|amr|aiff|au|mid|midi|ra|rm|wv|weba|dss|spx|vox|tak|dsf|dff|dsd|cda)$/i;
const reFont = /\.(?:woff2?|eot|ttf|otf|ttc|pfa|pfb|pfm|afm)$/i;
const reText = /\.(?:json[5c]?|te?xt|[mc]?[jt]sx?|md[cx]?|markdown|ya?ml|toml|csv|css|scss|less|styl|html?|xml|vue|svelte|astro|env|ini|conf|sh)$/i;
/** Classifies a path by extension, driving preview and icon choice. */
function guessAssetType(path) {
	if (reImage.test(path)) return "image";
	if (reVideo.test(path)) return "video";
	if (reAudio.test(path)) return "audio";
	if (reFont.test(path)) return "font";
	if (reText.test(path)) return "text";
	return "other";
}
function toPublicPath(baseURL, posixPath) {
	const encoded = posixPath.split("/").map(encodeURIComponent).join("/");
	return joinURL(baseURL, encoded);
}
/**
* Builds an {@link AssetInfo} from an already-resolved `fs.Stats`. Pass
* `includeFsPath` (dev mode only) to attach the absolute `fsPath` the client
* hands to the open wire service.
*/
function statToAssetInfo(dir, baseURL, relPath, stat, includeFsPath = false) {
	const posixPath = relPath.replace(/\\/g, "/");
	return {
		path: posixPath,
		type: guessAssetType(posixPath),
		publicPath: toPublicPath(baseURL, posixPath),
		size: stat.size,
		mtime: stat.mtimeMs,
		...includeFsPath ? { fsPath: join(dir, relPath) } : {}
	};
}
/** Recursively lists every file under `dir`, sorted alphabetically by path. */
async function scanAssets(dir, baseURL, includeFsPath = false) {
	const files = await glob(["**/*"], {
		cwd: dir,
		onlyFiles: true,
		dot: false,
		followSymbolicLinks: false
	});
	return (await Promise.all(files.map(async (relPath) => {
		try {
			const stat = await fsp.lstat(join(dir, relPath));
			if (stat.isSymbolicLink()) return void 0;
			return statToAssetInfo(dir, baseURL, relPath, stat, includeFsPath);
		} catch {
			return;
		}
	}))).filter((info) => info !== void 0).sort((a, b) => a.path.localeCompare(b.path));
}
//#endregion
//#region src/node/rpc/functions/list.ts
const assetInfoSchema = s.object({
	path: s.string(),
	type: s.picklist([
		"image",
		"font",
		"video",
		"audio",
		"text",
		"other"
	]),
	publicPath: s.string(),
	size: s.number(),
	mtime: s.number(),
	fsPath: s.optional(s.string())
});
const list = defineRpcFunction({
	name: "devframes:plugin:assets:list",
	type: "query",
	snapshot: true,
	jsonSerializable: true,
	args: [],
	returns: s.array(assetInfoSchema),
	agent: {
		title: "List managed assets",
		description: "List every file under the managed directory with its type, size, and last-modified time.",
		safety: "read",
		tags: ["assets", "files"]
	},
	setup: (ctx) => {
		const assets = getAssetsContext(ctx);
		return { 
		/**
		* The RPC runtime awaits handlers before validating `returns`; its
		* public setup type currently models schema-backed returns as
		* synchronous.
		* `fsPath` is dev-only, never baked into a static build's dump.
		*/
handler: (async () => scanAssets(assets.dir, assets.baseURL, ctx.mode === "dev")) };
	}
});
//#endregion
//#region src/node/rpc/functions/mkdir.ts
const mkdir = defineRpcFunction({
	name: "devframes:plugin:assets:mkdir",
	type: "action",
	jsonSerializable: true,
	args: [s.object({ path: s.string() })],
	returns: s.void(),
	agent: {
		title: "Create a folder",
		description: "Create a folder under the managed directory, including any missing parent folders.",
		safety: "action",
		tags: ["assets"]
	},
	setup: (ctx) => {
		const assets = getAssetsContext(ctx);
		return { 
		/** See `list.ts` for why the async handler is cast. */
handler: (async ({ path }) => {
			const absolute = await assertAssetMutationPath(assets.dir, path);
			const stat = await fsp.stat(absolute).catch(() => void 0);
			if (stat && !stat.isDirectory()) throw diagnostics.DP_ASSETS_0005({ path });
			await fsp.mkdir(absolute, { recursive: true });
			await assertAssetMutationPath(assets.dir, path);
		}) };
	}
});
//#endregion
//#region src/node/rpc/functions/read-image-meta.ts
const readImageMeta = defineRpcFunction({
	name: "devframes:plugin:assets:read-image-meta",
	type: "query",
	jsonSerializable: true,
	args: [s.string()],
	returns: s.nullable(s.object({
		width: s.optional(s.number()),
		height: s.optional(s.number()),
		orientation: s.optional(s.number())
	})),
	agent: {
		title: "Read image dimensions",
		description: "Read width, height, and orientation for an image asset.",
		safety: "read",
		tags: ["assets", "images"]
	},
	setup: (ctx) => {
		const assets = getAssetsContext(ctx);
		return { 
		/** See `list.ts` for why the async handler is cast. */
handler: (async (path) => {
			try {
				const buffer = await fsp.readFile(await resolveAssetReadPath(assets.dir, path));
				const meta = imageMeta(buffer);
				return {
					width: meta.width,
					height: meta.height,
					orientation: meta.orientation
				};
			} catch {
				return null;
			}
		}) };
	}
});
//#endregion
//#region src/node/rpc/functions/read-text.ts
const DEFAULT_LIMIT = 5e3;
const readText = defineRpcFunction({
	name: "devframes:plugin:assets:read-text",
	type: "query",
	jsonSerializable: true,
	args: [s.string(), s.optional(s.number())],
	returns: s.nullable(s.string()),
	agent: {
		title: "Read a text asset",
		description: "Read the (possibly truncated) text content of a text-type asset, for preview or editing.",
		safety: "read",
		tags: ["assets", "text"]
	},
	setup: (ctx) => {
		const assets = getAssetsContext(ctx);
		return { 
		/** See `list.ts` for why the async handler is cast. */
handler: (async (path, limit = DEFAULT_LIMIT) => {
			try {
				return (await fsp.readFile(await resolveAssetReadPath(assets.dir, path), "utf-8")).slice(0, limit);
			} catch {
				return null;
			}
		}) };
	}
});
//#endregion
//#region src/node/rpc/functions/rename.ts
const rename = defineRpcFunction({
	name: "devframes:plugin:assets:rename",
	type: "action",
	jsonSerializable: true,
	args: [s.object({
		path: s.string(),
		newName: s.string()
	})],
	returns: assetInfoSchema,
	agent: {
		title: "Rename an asset",
		description: "Rename an asset, keeping it in the same folder.",
		safety: "destructive",
		tags: ["assets"]
	},
	setup: (ctx) => {
		const assets = getAssetsContext(ctx);
		return { 
		/** See `list.ts` for why the async handler is cast. */
handler: (async ({ path, newName }) => {
			const trimmed = newName.trim();
			if (!trimmed || /[/\\]/.test(trimmed)) throw diagnostics.DP_ASSETS_0006({ name: newName });
			const folder = dirname(path);
			const nextName = `${trimmed}${extname(path)}`;
			const nextRelPath = folder === "." ? nextName : `${folder}/${nextName}`;
			const from = await assertAssetMutationPath(assets.dir, path);
			const to = await assertAssetMutationPath(assets.dir, nextRelPath);
			if (from === to) {
				const stat = await fsp.lstat(from);
				return statToAssetInfo(assets.dir, assets.baseURL, path, stat, true);
			}
			if (await fsp.access(to).then(() => true).catch(() => false)) throw diagnostics.DP_ASSETS_0003({ path: nextRelPath });
			await fsp.mkdir(dirname(to), { recursive: true });
			await assertAssetMutationPath(assets.dir, path);
			await assertAssetMutationPath(assets.dir, nextRelPath);
			try {
				await fsp.rename(from, to);
			} catch (error) {
				if (error.code === "ENOENT") throw diagnostics.DP_ASSETS_0004({ path });
				throw error;
			}
			const stat = await fsp.lstat(to);
			return statToAssetInfo(assets.dir, assets.baseURL, nextRelPath, stat, true);
		}) };
	}
});
//#endregion
//#region src/node/rpc/index.ts
/** Read-only RPC, always registered. */
const readFunctions = [
	list,
	readImageMeta,
	readText,
	capabilities
];
/** Mutating RPC, registered only when write actions are enabled. */
const writeFunctions = [
	upload,
	rename,
	deleteAssets,
	mkdir
];
const serverFunctions = [...readFunctions, ...writeFunctions];
//#endregion
export { getAssetsContext as _, readText as a, assetInfoSchema as c, scanAssets as d, deleteAssets as f, configureAssets as g, upload as h, rename as i, list as l, UPLOAD_CHANNEL as m, serverFunctions as n, readImageMeta as o, capabilities as p, writeFunctions as r, mkdir as s, readFunctions as t, guessAssetType as u, resolveAssetPath as v };
