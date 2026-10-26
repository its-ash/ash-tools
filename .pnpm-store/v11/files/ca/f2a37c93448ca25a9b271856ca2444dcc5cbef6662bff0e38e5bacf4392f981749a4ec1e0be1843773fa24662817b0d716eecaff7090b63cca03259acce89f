import { registerDataSource, createDataInspectorDevframe } from '@devframes/plugin-data-inspector';
import { onDevtoolsReady, NUXT_DEVTOOLS_GROUP_ID, deprecate } from '@nuxt/devtools-kit';

const captured = {};
function normalizeViteConfig(config) {
  return {
    ...config,
    environments: Object.fromEntries(
      Object.entries(config.env ?? {}).map(([key, _]) => {
        return [key, null];
      })
    ),
    plugins: (config.plugins ?? []).map((i) => {
      const clone = { ...i };
      delete clone.api;
      return clone;
    }),
    inlineConfig: null
  };
}
function setup(ctx) {
  const { nuxt } = ctx;
  nuxt.hook("nitro:build:before", (nitro) => {
    captured.nitro = nitro;
  });
  nuxt.hook("vite:serverCreated", (server) => {
    captured.vite = server;
  });
  const dataSource = registerDataSource({
    id: "nuxt:application",
    title: "Nuxt Application",
    description: "Live Nuxt, Nitro, and Vite configuration",
    icon: "i-ph:database-duotone",
    static: false,
    data: () => ({
      nuxt: nuxt.options,
      nitro: captured.nitro?.options,
      vite: captured.vite
    }),
    queries: [
      { title: "Overview", query: "", excludeFunctions: true },
      { title: "Nuxt modules", query: "nuxt.modules", excludeFunctions: true },
      { title: "Vite plugins", query: "vite.config.plugins", excludeFunctions: true },
      { title: "Vite config", query: "vite.config", excludeFunctions: true },
      { title: "Vite environments", query: "vite.environments", excludeFunctions: true },
      { title: "Nitro routes", query: "nitro.handlers", excludeFunctions: true }
    ]
  });
  nuxt.hook("close", () => {
    dataSource.unregister();
    captured.nitro = void 0;
    captured.vite = void 0;
  });
  const definition = createDataInspectorDevframe({ exampleSource: false });
  onDevtoolsReady((kit) => {
    return kit.install(definition, {
      dock: {
        groupId: NUXT_DEVTOOLS_GROUP_ID,
        category: "advanced"
      }
    });
  }, nuxt);
}
function getServerData(nuxt) {
  deprecate(nuxt, "NDT_DEP_0009", {
    api: "getServerData()",
    replacement: "the Nuxt Application source in the Data Inspector panel"
  });
  const config = captured.vite?.config;
  return {
    nuxt: nuxt.options,
    nitro: captured.nitro?.options,
    vite: {
      server: config ? normalizeViteConfig(config) : void 0,
      client: config ? normalizeViteConfig(config) : void 0
    }
    // `nuxt.options` is `@nuxt/schema`'s `NuxtOptions` while `NuxtServerData`
    // pins the structurally-identical `nuxt/schema` one; bridge the two.
  };
}

export { getServerData, setup };
