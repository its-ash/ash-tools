import { createAssetsDevframe } from '@devframes/plugin-assets';
import { onDevtoolsReady, NUXT_DEVTOOLS_GROUP_ID } from '@nuxt/devtools-kit';
import { resolve } from 'pathe';
import { d as defaultAllowedExtensions } from '../shared/devtools.C3s9ubDZ.mjs';
import '@nuxt/kit';

function setup({ nuxt, options }) {
  const definition = createAssetsDevframe({
    dir: resolve(nuxt.options.srcDir, nuxt.options.dir.public),
    baseURL: nuxt.options.app.baseURL,
    uploadExtensions: options.assets?.uploadExtensions || defaultAllowedExtensions
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
