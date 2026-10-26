import { n as DEVTOOLS_CLIENT_MODULE_RESOLUTION } from "./diagnostics-uBEOg880.js";
import { n as resolveDockRendererRegistrations, t as createViteDevToolsUi } from "./ui-BpHmaWlG.js";
import { randomBytes } from "node:crypto";
import process from "node:process";
import { createInteractiveAuth } from "devframe/recipes/interactive-auth";
import { DEVTOOLS_CONNECTION_META_FILENAME, DEVTOOLS_MOUNT_PATH } from "@vitejs/devtools-kit/constants";
import { initHub } from "@devframes/hub/initiate";
//#region \0rolldown/runtime.js
var __defProp = Object.defineProperty;
var __exportAll = (all, no_symbols) => {
	let target = {};
	for (var name in all) __defProp(target, name, {
		get: all[name],
		enumerable: true
	});
	if (!no_symbols) __defProp(target, Symbol.toStringTag, { value: "Module" });
	return target;
};
//#endregion
//#region src/node/resolved-config.ts
const resolvedDevToolsConfigs = /* @__PURE__ */ new WeakMap();
const defaultResolvedDevToolsConfig = {
	apply: "all",
	config: {
		clientAuth: true,
		clientAuthTokens: [],
		host: "localhost"
	},
	enabled: true
};
function setResolvedDevToolsConfig(context, config) {
	resolvedDevToolsConfigs.set(context, config);
}
function getResolvedDevToolsConfig(context) {
	return resolvedDevToolsConfigs.get(context) ?? defaultResolvedDevToolsConfig;
}
//#endregion
//#region src/node/auth-handler.ts
const handlers = /* @__PURE__ */ new WeakMap();
const capabilityTokens = /* @__PURE__ */ new WeakMap();
/**
* The per-process capability token minted for an implicit build-mode context
* (see {@link isBuildCapabilityAuth}). Created lazily and memoized per context,
* so `getAuthHandler` (which registers it as an always-trusted
* `clientAuthTokens` entry) and `createDevToolsHub` (which bakes it into the
* locally-served connection metadata's `authToken`) hand out the exact same
* value. Unguessable and never printed — only a same-origin client able to read
* the served `__connection.json` learns it, so a cross-origin loopback page or
* an `Origin`-less local process stays untrusted.
*/
function getBuildCapabilityToken(context) {
	let token = capabilityTokens.get(context);
	if (!token) {
		token = randomBytes(32).toString("base64url");
		capabilityTokens.set(context, token);
	}
	return token;
}
/**
* The interactive OTP auth handler for a context — created once and shared
* between context setup (which registers its handshake / self-revoke RPC
* functions) and the WS server (which uses its resolver gate and prints the
* one-time-code banner). Backed by devframe's `createInteractiveAuth` recipe,
* so the `anonymous:devframe:auth*` handlers, `devframe:auth:revoke`, and the
* banner all come from upstream rather than being hand-rolled here.
*
* In implicit build mode ({@link isBuildCapabilityAuth}) the handler additionally
* trusts the per-process {@link getBuildCapabilityToken} — the build viewer
* presents it automatically from the served connection meta — and its OTP
* banner is suppressed, since trust comes purely from that token.
*/
function getAuthHandler(context) {
	let handler = handlers.get(context);
	if (!handler) {
		const config = getResolvedDevToolsConfig(context).config;
		const buildCapability = isBuildCapabilityAuth(context);
		const clientAuthTokens = config.clientAuthTokens ? [...config.clientAuthTokens] : [];
		if (buildCapability) clientAuthTokens.push(getBuildCapabilityToken(context));
		handler = createInteractiveAuth(context, {
			clientAuthTokens,
			banner: buildCapability ? () => {} : config.banner,
			onTrusted: buildCapability ? void 0 : config.onTrusted
		});
		handlers.set(context, handler);
	}
	return handler;
}
/**
* Whether the interactive OTP gate stays fully off for this context — an
* explicit `devtools: { clientAuth: false }` or the
* `VITE_DEVTOOLS_DISABLE_CLIENT_AUTH` escape-hatch env var. Both are deliberate
* user opt-outs that trust every accepted client. Shared between
* `createDevToolsContext` (which then skips registering the interactive-auth
* RPC functions so devframe's `auth: false` auto-trust shim can register
* `anonymous:devframe:auth` itself) and `createDevToolsHub` (which feeds the
* same intent to `initHub`'s transport-level `auth` option) — both need to
* agree, or the client's session never gets marked trusted.
*
* Implicit build mode is deliberately absent: it keeps the auth gate installed
* but trusts via a capability token instead of a prompt — see
* {@link isBuildCapabilityAuth}.
*/
function isClientAuthDisabled(context) {
	return getResolvedDevToolsConfig(context).config.clientAuth === false || process.env.VITE_DEVTOOLS_DISABLE_CLIENT_AUTH === "true";
}
/**
* Whether this context uses the implicit build-mode capability-token posture:
* a build snapshot served by a live server (the standalone viewer) that keeps
* the zero-prompt UX but, instead of trusting all comers, requires the
* per-process {@link getBuildCapabilityToken}. Only the implicit `build` branch
* qualifies — the explicit `clientAuth: false` and
* `VITE_DEVTOOLS_DISABLE_CLIENT_AUTH` opt-outs ({@link isClientAuthDisabled})
* still disable the gate entirely.
*/
function isBuildCapabilityAuth(context) {
	return context.mode === "build" && !isClientAuthDisabled(context);
}
//#endregion
//#region src/node/server.ts
var server_exports = /* @__PURE__ */ __exportAll({ createDevToolsHub: () => createDevToolsHub });
/** Absolute path of the hub's top-level connection meta (`/__devtools/__connection.json`). */
const CONNECTION_META_PATH = `${DEVTOOLS_MOUNT_PATH}${DEVTOOLS_CONNECTION_META_FILENAME}`;
/**
* Stand up the DevTools client + RPC surface as a devframe hub instance
* (`initHub`). The hub serves the branded `@devframes/hub-ui` viewer and its
* embedded bootstrap (the `ui` slot), the RPC connection meta, and the shared
* WebSocket transport — all under the kit-pinned `/__devtools/` base. Vite
* DevTools supplies its own already-assembled kit context, so the hub runs in
* "bring your own context" mode and only owns the hub-level endpoints; each
* mounted devframe (Rolldown, Vite, …) keeps serving its own SPA at its own
* base and gets its `__connection.json` from {@link provideConnectionMeta}.
*/
async function createDevToolsHub(options) {
	const { context } = options;
	const authDisabled = isClientAuthDisabled(context);
	const capabilityToken = isBuildCapabilityAuth(context) ? getBuildCapabilityToken(context) : void 0;
	const resolvedConfig = getResolvedDevToolsConfig(context).config;
	const allowedOrigins = resolvedConfig.allowedOrigins;
	const mcp = resolvedConfig.mcp;
	const hub = initHub({
		base: DEVTOOLS_MOUNT_PATH,
		context,
		ui: createViteDevToolsUi(options.ui),
		renderers: resolveDockRendererRegistrations(options.renderers),
		...context.viteServer ? { register: {
			id: "vite-devtools",
			name: "Vite DevTools",
			rootDir: context.cwd
		} } : {},
		...context.viteServer ? { clientModuleResolution: DEVTOOLS_CLIENT_MODULE_RESOLUTION } : {},
		auth: authDisabled ? false : getAuthHandler(context),
		...allowedOrigins ? { allowedOrigins } : {},
		...mcp !== void 0 ? { mcp } : {},
		...options.server ? { server: options.server } : { ws: options.wsPort != null ? { port: options.wsPort } : { sidecar: true } },
		...options.host ? { host: options.host } : {}
	});
	await hub.ready;
	const getConnectionMeta = () => capabilityToken ? {
		...hub.connectionMeta(),
		authToken: capabilityToken
	} : hub.connectionMeta();
	context.host.provideConnectionMeta?.(getConnectionMeta);
	return {
		hub,
		middleware: capabilityToken ? (req, res, next) => {
			if (req.url?.split("?", 1)[0] === CONNECTION_META_PATH) {
				res.setHeader("Content-Type", "application/json");
				res.end(JSON.stringify(getConnectionMeta()));
				return;
			}
			hub.nodeMiddleware(req, res, next);
		} : hub.nodeMiddleware,
		getConnectionMeta,
		close: hub.close
	};
}
//#endregion
export { defaultResolvedDevToolsConfig as a, isClientAuthDisabled as i, server_exports as n, setResolvedDevToolsConfig as o, getAuthHandler as r, __exportAll as s, createDevToolsHub as t };
