import { DevframeHubUi } from "@devframes/hub/initiate";
//#region src/locales.d.ts
/**
 * UI languages the reference hub-ui ships translations for, keyed by BCP 47
 * tag with the language's own name as the picker label. Framework-free like
 * `./types.ts` so the node entry can type `createUi({ locale })` without
 * pulling in the client. Message files live in `client/i18n/locales/`, one
 * per key here; adding a language means adding both.
 */
export declare const HUB_UI_LOCALES: {
  readonly en: "English";
  readonly 'zh-CN': "简体中文";
  readonly 'zh-TW': "繁體中文";
  readonly ja: "日本語";
  readonly ko: "한국어";
  readonly es: "Español";
  readonly fr: "Français";
  readonly de: "Deutsch";
  readonly 'pt-BR': "Português (Brasil)";
  readonly ru: "Русский";
};
type HubUiLocale = keyof typeof HUB_UI_LOCALES;
//#endregion
//#region src/types.d.ts
/**
 * Published `createUi()` config types, kept framework-free so the node
 * entry's declaration rollup (`dist/index.d.mts`) never pulls in Vue's type
 * surface. Client modules that need these types import them *from* here
 * (never the reverse); see `client/state/branding.ts`,
 * `client/embedded/visibility.ts`.
 */
/**
 * A logo asset: a single URL/data-URI, or per-color-scheme variants. The dark
 * variant falls back to the light one when only `light` is given (or a bare
 * string is used for both).
 */
type BrandingLogo = string | {
  light: string;
  dark: string;
};
/** A value that can vary with the viewer color scheme. */
type ColorSchemeValue = string | {
  light: string;
  dark: string;
};
/**
 * The standalone viewer background. The flat form applies in every context;
 * the structured form may provide an iframe-specific value.
 */
type ViewerBackground = ColorSchemeValue | {
  standalone: ColorSchemeValue;
  iframe?: ColorSchemeValue;
};
/**
 * Consumer-facing branding for the reference hub-ui. Every field is optional
 * and falls back to devframe's own identity. Published as
 * `ConnectionMeta.configs.ui.branding` via `createUi({ branding })`, and
 * read from the one connection handshake the dock already performs;
 * `ConnectionMeta` has its own cross-realm propagation (see
 * `DEVFRAME_CONNECTION_KEY`), so branding needs no globals or query params
 * of its own.
 */
interface DevframeBranding {
  /** Product name: the wordmark, window titles, and all user-visible copy. */
  productName?: string;
  /** Logo mark (URL / data-URI), rendered via `<img>`. */
  logo?: BrandingLogo;
  /** Optional standalone wordmark image; when absent, mark + productName text is composed. */
  wordmark?: BrandingLogo;
  /** Brand color; feeds `--devframe-primary` and the whole primary ramp. */
  primaryColor?: string;
  /** Standalone viewer CSS `background`, optionally specialized for iframe use. */
  background?: ViewerBackground;
  /** Short line for the auth screen and the standalone meta description. */
  tagline?: string;
  /** Favicon URL, applied on the standalone viewer and the popped-out window only. */
  favicon?: string;
  /** Window/tab title; defaults to `productName`. */
  windowTitle?: string;
}
/**
 * The reference UI's dock-bar rendering preferences, set via
 * `createUi({ dockPreferences })` and published as
 * `ConnectionMeta.configs.ui.dockPreferences`. Read by the embedded dock and
 * the standalone viewer at boot.
 *
 * Like the float/edge dock mode, these seed user-overridable state; the
 * config sets the default, the visitor's own choice wins from then on.
 */
interface DevframeDockPreferences {
  /**
   * The top-level dock-bar **category** ordering: a map of category id →
   * ordering weight (lower sorts earlier), merged beneath
   * `DEFAULT_CATEGORIES_ORDER`.
   */
  categoryOrder?: Record<string, number>;
  /**
   * Preferred inline-item capacity for the floating dock bar before entries
   * overflow. Edge mode ignores it; it shows every entry with no cutoff.
   */
  maxVisibleItems?: number;
  /** Seeds a first-run visitor's dock mode (float vs edge). */
  defaultMode?: 'float' | 'edge';
  /** Seeds a first-run visitor's dock position. */
  defaultPosition?: 'left' | 'right' | 'top' | 'bottom';
}
/**
 * How the embedded floating dock reveals itself on a fresh page: the
 * reference UI's port of Nuxt DevTools' opt-in overlay, published as
 * `ConnectionMeta.configs.ui.embeddedVisibility` and set via
 * `createUi({ embeddedVisibility })`.
 *
 * - `normal` (default): the dock is shown immediately.
 * - `passive`: the dock starts hidden and a console hint offers the reveal
 *   shortcut; revealing persists per-origin, so later sessions on this
 *   browser start shown. The "Hide" command returns to passive.
 * - `hidden`: the dock starts hidden and the shortcut reveals it for the
 *   current session only; nothing is persisted.
 *
 * Whatever the policy, the reveal state is a user-overridable preference,
 * the same shape as the float/edge dock mode: the config seeds it, the
 * visitor's own reveal/hide wins from then on.
 */
type EmbeddedVisibility = 'normal' | 'passive' | 'hidden';
//#endregion
//#region src/index.d.ts
declare module 'devframe/types' {
  interface DevframeConnectionConfigsRegistry {
    ui: {
      branding?: DevframeBranding;
      embeddedVisibility?: EmbeddedVisibility;
      dockPreferences?: DevframeDockPreferences;
      /**
       * The hub UI language. `createUi({ locale })` publishes the host's
       * default; the running hub UI then overwrites it with the language it
       * resolved (the visitor's own pick, else this default, else the
       * browser's), so a mounted frame reads the effective one.
       */
      locale?: HubUiLocale;
    };
  }
}
export interface CreateUiOptions {
  /** Serve the standalone viewer SPA at the hub base. Default: `true`. */
  viewer?: boolean;
  /** Serve the floating-dock bootstrap at `<base>embedded.js`. Default: `true`. */
  embedded?: boolean;
  /**
   * Rebrand the reference UI: logo, product name, primary color, and more.
   * Published as `ConnectionMeta.configs.ui.branding`, read by the dock at
   * boot from the one connection handshake it already performs. Reaches
   * both the embedded dock and the standalone viewer.
   */
  branding?: DevframeBranding;
  /**
   * How the embedded floating dock reveals itself on a fresh page:
   *
   * - `'normal'` (default): shown immediately.
   * - `'passive'`: starts hidden with a console hint; `Shift+Alt+D` reveals
   *   it, and the reveal persists per-origin so later sessions start shown.
   * - `'hidden'`: starts hidden; `Shift+Alt+D` reveals it for the current
   *   session only.
   *
   * Published as `ConnectionMeta.configs.ui.embeddedVisibility`. Like the
   * float/edge dock mode, it seeds a user-overridable preference; the
   * visitor's own reveal/hide wins from then on. Applies to the embedded
   * dock only; the standalone viewer is an explicit visit and always shows.
   */
  embeddedVisibility?: EmbeddedVisibility;
  /**
   * Dock-bar rendering preferences: category ordering, floating-dock
   * inline-item capacity, and the first-run float/edge mode and position.
   * Published as `ConnectionMeta.configs.ui.dockPreferences`; each seeds a
   * user-overridable preference the visitor's own choice then wins.
   */
  dockPreferences?: DevframeDockPreferences;
  /**
   * Default UI language, one of {@link HUB_UI_LOCALES}. Published as
   * `ConnectionMeta.configs.ui.locale` and, like the dock mode, only a
   * seed: a visitor's own pick in Settings → Appearance wins. Without it
   * the UI follows the browser language, falling back to English.
   */
  locale?: HubUiLocale;
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
export declare function createUi(options?: CreateUiOptions): DevframeHubUi;
//#endregion
export type { ColorSchemeValue, DevframeBranding, DevframeDockPreferences, EmbeddedVisibility, HubUiLocale, ViewerBackground };