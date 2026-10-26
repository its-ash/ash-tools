import { DockRendererRegistration } from "@devframes/hub/initiate";
//#region src/hub.d.ts
/**
 * Register the reference json-render frontend on a hub: the one-liner that
 * composes `@devframes/json-render-ui` into any prebuilt viewer:
 *
 * ```ts
 * import { initHub } from '@devframes/hub/initiate'
 * import { createUi } from '@devframes/hub-ui'
 * import { jsonRenderUiRenderer } from '@devframes/json-render-ui/hub'
 *
 * initHub({ ui: createUi(), renderers: [jsonRenderUiRenderer()] })
 * ```
 *
 * The hub serves the package's prebuilt, self-contained renderer module at
 * `<base>__renderers/json-render.mjs` and publishes it in the renderer
 * manifest; viewers import it lazily the first time a `'json-render'` dock
 * mounts. Without any registration for that type, a viewer shows its
 * missing-renderer fallback instead; swap this helper for any community
 * implementation of the `JsonRenderDockRenderer` contract.
 *
 * This entry is node-safe: it imports no Vue and no `@antfu/design`, so a
 * host can read the registration without pulling the browser renderer graph.
 */
export declare function jsonRenderUiRenderer(): DockRendererRegistration;
//#endregion