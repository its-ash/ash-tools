import { addVitePlugin, addPluginTemplate } from '@nuxt/kit';
import { vueDevtools } from 'vite-plugin-vue-devtools';
import { s as skipInSSR } from './module-main.mjs';
import 'node:os';
import '@nuxt/devtools-kit';
import 'consola/utils';
import 'devframe/utils/serve-static';
import 'pathe';
import 'verkit';
import 'vite';
import '../shared/devtools.C3s9ubDZ.mjs';
import '../dirs.mjs';
import 'node:path';
import 'node:url';
import 'node:child_process';
import 'node:fs';
import 'node:fs/promises';
import 'node:util';
import 'tinyglobby';
import 'destr';
import '../../dist/runtime/shared/hooks.js';
import 'devframe/utils/hash';
import 'magicast';
import 'magicast/helpers';
import 'node:process';
import 'node:module';
import 'local-pkg';
import '@vitejs/devtools-kit/constants';
import 'perfect-debounce';

function setup({ nuxt }) {
  if (!nuxt.options.dev || nuxt.options.test)
    return;
  const plugins = vueDevtools().map((plugin) => {
    const { devtools } = plugin;
    if (!devtools?.setup)
      return plugin;
    return {
      ...plugin,
      devtools: {
        ...devtools,
        setup: (ctx) => skipInSSR(ctx) ? void 0 : devtools.setup(ctx)
      }
    };
  });
  addVitePlugin(plugins);
  addPluginTemplate({
    filename: "devtools/vue-devtools-client.mjs",
    name: "vue-devtools-client",
    mode: "client",
    order: -1e3,
    getContents: () => [
      `import 'virtual:vue-devtools-client'`,
      ``,
      `export default () => {}`,
      ``
    ].join("\n")
  });
}

export { setup };
