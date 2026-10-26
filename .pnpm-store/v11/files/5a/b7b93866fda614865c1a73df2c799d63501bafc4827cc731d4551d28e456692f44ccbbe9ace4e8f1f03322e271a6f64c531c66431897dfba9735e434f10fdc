import { onDevtoolsReady } from '@nuxt/devtools-kit';
import { addVitePlugin, logger } from '@nuxt/kit';
import { createInstallLauncher } from '@vitejs/devtools-kit/node';
import { isPackageExists } from 'local-pkg';
import { R as RPC_NAMESPACE } from './module-main.mjs';
import 'node:os';
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
import '@vitejs/devtools-kit/constants';
import 'perfect-debounce';

const VERSION_QUERY_RE = /\?v=\w+$/;
const VUE_EXT_RE = /\.vue($|\?v=)/;
const INSPECT_DOCK_ID = "vite-plugin-inspect";
const INSPECT_DOCK_TITLE = "Inspect";
const INSPECT_DOCK_ICON = "ph:magnifying-glass-duotone";
function isVitePluginInspectAvailable(rootDir) {
  return isPackageExists("vite-plugin-inspect", { paths: [rootDir] });
}
async function createVitePluginInspect(options) {
  return await import('vite-plugin-inspect').then((r) => r.default(options));
}
async function setup(ctx) {
  if (!isVitePluginInspectAvailable(ctx.nuxt.options.rootDir)) {
    addVitePlugin(createInstallLauncher({
      id: INSPECT_DOCK_ID,
      title: INSPECT_DOCK_TITLE,
      icon: INSPECT_DOCK_ICON,
      label: "Vite Inspect",
      install: ["vite-plugin-inspect"]
    }));
    return;
  }
  const plugin = await createVitePluginInspect();
  addVitePlugin(plugin);
  onDevtoolsReady((kit) => {
    async function getComponentsRelationships() {
      if (!kit.rpc.has("vite-plugin-inspect:get-metadata")) {
        logger.warn("[nuxt-devtools] vite-plugin-inspect RPC functions not registered, component relationships unavailable");
        return [];
      }
      const meta = await kit.rpc.invokeLocal("vite-plugin-inspect:get-metadata");
      const modules = (meta && meta.instances[0] ? await kit.rpc.invokeLocal("vite-plugin-inspect:get-modules-list", {
        vite: meta.instances[0].vite,
        env: meta.instances[0].environments[0]
      }) : null) || [];
      const components = await kit.rpc.invokeLocal(`${RPC_NAMESPACE}:getComponents`) || [];
      const vueModules = modules.filter((m) => {
        const plainId = m.id.replace(VERSION_QUERY_RE, "");
        if (components.some((c) => c.filePath === plainId))
          return true;
        return m.id.match(VUE_EXT_RE);
      });
      const graph = vueModules.map((i) => {
        function searchForVueDeps(id, seen = /* @__PURE__ */ new Set()) {
          if (seen.has(id))
            return [];
          seen.add(id);
          const module = modules.find((m) => m.id === id);
          if (!module)
            return [];
          return module.deps.flatMap((i2) => {
            if (vueModules.some((m) => m.id === i2))
              return [i2];
            return searchForVueDeps(i2, seen);
          });
        }
        return {
          id: i.id,
          deps: searchForVueDeps(i.id)
        };
      });
      return graph;
    }
    kit.rpc.register({ name: `${RPC_NAMESPACE}:getComponentsRelationships`, handler: getComponentsRelationships }, true);
  }, ctx.nuxt);
}

export { createVitePluginInspect, isVitePluginInspectAvailable, setup };
