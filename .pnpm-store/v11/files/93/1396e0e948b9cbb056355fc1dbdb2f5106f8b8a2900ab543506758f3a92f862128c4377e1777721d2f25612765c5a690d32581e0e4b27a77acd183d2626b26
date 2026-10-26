import { existsSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { createAuthBanner } from "devframe/recipes/interactive-auth";
//#region src/locales.ts
/**
* UI languages the reference hub-ui ships translations for, keyed by BCP 47
* tag with the language's own name as the picker label. Framework-free like
* `./types.ts` so the node entry can type `createUi({ locale })` without
* pulling in the client. Message files live in `client/i18n/locales/`, one
* per key here; adding a language means adding both.
*/
const HUB_UI_LOCALES = {
	"en": "English",
	"zh-CN": "简体中文",
	"zh-TW": "繁體中文",
	"ja": "日本語",
	"ko": "한국어",
	"es": "Español",
	"fr": "Français",
	"de": "Deutsch",
	"pt-BR": "Português (Brasil)",
	"ru": "Русский"
};
//#endregion
//#region src/index.ts
/**
* The built client assets live next to the built entry (`dist/index.mjs` →
* `dist/client/`). When this module runs from source instead (tests and
* playgrounds resolving the workspace alias), fall back to the package's
* `dist/client/`; the assets are build artifacts either way.
*/
function clientDir() {
	const here = fileURLToPath(new URL(".", import.meta.url));
	const sibling = join(here, "client");
	if (existsSync(join(sibling, "embedded.js"))) return sibling;
	return join(here, "../dist/client");
}
/**
* The reference implementation of the hub's {@link DevframeHubUi} slot,
* prebuilt from this package's web components (the floating `DockEmbedded`
* bootstrap and the standalone `DockStandalone` SPA), styled with the
* shared devframe design system.
*
* ```ts
* import { initHub } from '@devframes/hub/initiate'
* import { createUi } from '@devframes/hub-ui'
*
* const hub = initHub({ devframes: [git, terminals], ui: createUi() })
* ```
*
* The hub stays headless either way; this object is one implementation of
* the slot; a hub UI provider (a product's or your own) supplies a different one to the
* same option and reuses all the infrastructure.
*/
function createUi(options = {}) {
	const client = clientDir();
	return {
		...options.viewer !== false ? { viewer: { distDir: join(client, "standalone") } } : {},
		...options.embedded !== false ? { embedded: { entry: join(client, "embedded.js") } } : {},
		/**
		* Publish the reference UI's config through the generic `ctx.staticConfig`,
		* which rides the connection handshake to every mounted frame and the
		* standalone viewer as `ConnectionMeta.configs.ui`.
		*/
		setup(ctx) {
			ctx.staticConfig.ui = {
				branding: options.branding || {},
				...options.embeddedVisibility ? { embeddedVisibility: options.embeddedVisibility } : {},
				...options.dockPreferences ? { dockPreferences: options.dockPreferences } : {},
				...options.locale ? { locale: options.locale } : {}
			};
		},
		/**
		* Node-side counterpart to the browser rebrand above: the same
		* `productName` titles the interactive-auth box printed to the terminal,
		* so a rebranded hub doesn't print a stray "Devframe" code prompt.
		*/
		authBanner: createAuthBanner({ title: options.branding?.productName })
	};
}
//#endregion
export { HUB_UI_LOCALES, createUi };
