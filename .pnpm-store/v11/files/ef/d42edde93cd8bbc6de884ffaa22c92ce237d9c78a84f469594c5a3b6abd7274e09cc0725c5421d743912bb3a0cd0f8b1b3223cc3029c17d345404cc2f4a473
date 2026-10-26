import { createOgDevframe } from '@devframes/plugin-og';
import { onDevtoolsReady, NUXT_DEVTOOLS_GROUP_ID } from '@nuxt/devtools-kit';

function setup({ nuxt }) {
  const defaultUrl = nuxt.options.devServer?.url;
  let liveOrigin;
  nuxt.hook("listen", (_server, listener) => {
    if (listener?.url)
      liveOrigin = new URL(listener.url).origin;
  });
  const definition = createOgDevframe({
    defaultUrl,
    fetch: (input, init) => {
      if (liveOrigin && defaultUrl) {
        const configured = new URL(defaultUrl).origin;
        if (input.startsWith(configured))
          input = liveOrigin + input.slice(configured.length);
      }
      return fetch(input, init);
    }
  });
  onDevtoolsReady((kit) => {
    return kit.install(definition, {
      dock: {
        groupId: NUXT_DEVTOOLS_GROUP_ID,
        category: "analyze"
      }
    });
  }, nuxt);
}

export { setup };
