import { DEVTOOLS_ASSETS_BASE } from "./dirs.js";
import { jsonRenderUiRenderer } from "@devframes/json-render-ui/hub";
import { createUi } from "@devframes/hub-ui";
//#region src/node/renderers.ts
/** Merge configured renderers over the built-ins while keeping one entry per type. */
function resolveDockRendererRegistrations(configuredRenderers = []) {
	const renderersByType = /* @__PURE__ */ new Map();
	const registrations = [jsonRenderUiRenderer(), ...configuredRenderers];
	for (const registration of registrations) renderersByType.set(registration.type, registration);
	return [...renderersByType.values()];
}
//#endregion
//#region src/node/ui.ts
function viteDevToolsBranding() {
	return {
		productName: "Vite DevTools",
		primaryColor: "#6b84fd",
		logo: `${DEVTOOLS_ASSETS_BASE}vite-plus.svg`,
		wordmark: {
			light: `${DEVTOOLS_ASSETS_BASE}vite-devtools-kit-light.svg`,
			dark: `${DEVTOOLS_ASSETS_BASE}vite-devtools-kit-dark.svg`
		},
		favicon: `${DEVTOOLS_ASSETS_BASE}vite-plus.svg`,
		tagline: "DevTools for Vite and the ecosystem",
		windowTitle: "Vite DevTools"
	};
}
/**
* Build the `@devframes/hub-ui` slot, branded as Vite DevTools, for the hub
* that serves the DevTools client. Hands the reference viewer + embedded
* bootstrap to `initHub({ ui })` (dev serve) or is copied out by the static
* build.
*
* A host embedding Vite DevTools (e.g. Nuxt DevTools) can re-skin the client
* via `options.branding`, which is shallow-merged field-by-field over the
* built-in {@link viteDevToolsBranding} defaults.
*/
function createViteDevToolsUi(options = {}) {
	return createUi({
		branding: resolveBranding(options.branding),
		embeddedVisibility: options.embeddedVisibility,
		dockPreferences: options.dockPreferences
	});
}
/**
* Merge a host's branding overrides over the Vite DevTools defaults. Only keys
* the host actually sets win; an explicit `undefined` is ignored so a partial
* override never clobbers a default with a hole.
*/
function resolveBranding(overrides) {
	const branding = viteDevToolsBranding();
	if (!overrides) return branding;
	for (const key of Object.keys(overrides)) {
		const value = overrides[key];
		if (value !== void 0) branding[key] = value;
	}
	return branding;
}
//#endregion
export { resolveDockRendererRegistrations as n, createViteDevToolsUi as t };
