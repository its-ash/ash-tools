import { getDevframeClientContext } from "@devframes/hub/client";
import { NuxtDevtoolsInspectPanel } from "@nuxt/devtools/webcomponents";
import { createHooks } from "hookable";
import { debounce } from "perfect-debounce";
import { events as inspectorEvents, hasData as inspectorHasData, state as inspectorState } from "vite-plugin-vue-tracer/client/overlay";
import { computed, markRaw, nextTick, reactive, ref, shallowReactive, shallowRef, toRef, watch } from "vue";
import { useAppConfig } from "#imports";
import { initTimelineMetrics } from "../../function-metrics-helpers.js";
const NUXT_DOCK_ANCHOR_ID = "nuxt:devtools";
function getDevframeContext() {
  return getDevframeClientContext();
}
const clientRef = shallowRef();
export { clientRef as client };
export async function setupDevToolsClient({
  nuxt,
  clientHooks,
  timeMetric,
  router
}) {
  let inspector;
  const colorMode = useClientColorMode();
  const timeline = initTimelineMetrics();
  const client = shallowReactive({
    nuxt: markRaw(nuxt),
    hooks: createHooks(),
    inspector: getInspectorInstance(),
    devtools: {
      toggle() {
        const ctx = getDevframeContext();
        if (ctx)
          ctx.docks.toggleEntry(NUXT_DOCK_ANCHOR_ID);
      },
      close() {
        const ctx = getDevframeContext();
        if (ctx)
          ctx.panel.session.open = false;
      },
      open() {
        const ctx = getDevframeContext();
        if (ctx) {
          ctx.panel.session.open = true;
          ctx.docks.switchEntry(NUXT_DOCK_ANCHOR_ID);
        }
      },
      async navigate(path) {
        const ctx = getDevframeContext();
        if (ctx) {
          ctx.panel.session.open = true;
          ctx.docks.switchEntry(NUXT_DOCK_ANCHOR_ID);
        }
        await client.hooks.callHook("host:action:navigate", path);
      },
      async reload() {
        await client.hooks.callHook("host:action:reload");
      }
    },
    app: {
      appConfig: useAppConfig(),
      reload() {
        location.reload();
      },
      navigate(path, hard = false) {
        if (hard)
          location.href = path;
        else
          router.push(path);
      },
      colorMode,
      $fetch: globalThis.$fetch
    },
    metrics: {
      clientPlugins: () => window.__NUXT_DEVTOOLS_PLUGINS_METRIC__,
      clientHooks: () => Object.values(clientHooks),
      clientTimeline: () => timeline,
      loading: () => timeMetric
    },
    revision: ref(0)
  });
  window.__NUXT_DEVTOOLS_HOST__ = client;
  function getInspectorInstance() {
    if (inspector)
      return inspector;
    const props = reactive({
      mouse: { x: 0, y: 0 },
      hasParent: false,
      matched: void 0
    });
    const component = new NuxtDevtoolsInspectPanel(reactive({ props }));
    document.body.appendChild(component);
    Object.assign(component.style, {
      zIndex: 999999,
      position: "fixed",
      left: "0",
      top: "0"
    });
    component.addEventListener("close", () => {
      props.matched = void 0;
      inspectorState.isEnabled = false;
      inspectorState.isVisible = false;
    });
    component.addEventListener("selectParent", () => {
      const parent = inspectorState.main?.getParent();
      if (parent) {
        inspectorState.main = parent;
        props.matched = parent;
        nextTick(() => {
          props.hasParent = !!inspectorState.main?.getParent();
        });
      }
    });
    component.addEventListener("openInEditor", async (e) => {
      const url = e?.detail?.[0];
      if (url)
        await client.hooks.callHook("host:inspector:click", url);
    });
    inspectorEvents.on("hover", () => {
      inspectorState.isFocused = false;
      props.hasParent = !!inspectorState.main?.getParent();
    });
    inspectorEvents.on("disabled", () => {
      inspectorState.isVisible = false;
      client?.hooks.callHook("host:inspector:close");
    });
    inspectorEvents.on("enabled", () => {
      inspectorState.isVisible = true;
      inspectorState.isEnabled = true;
    });
    inspectorEvents.on("click", async (info, e) => {
      inspectorState.isEnabled = false;
      inspectorState.isFocused = true;
      inspectorState.isVisible = true;
      props.matched = info;
      props.mouse = { x: e.clientX, y: e.clientY };
    });
    const isAvailable = ref(inspectorHasData());
    if (!isAvailable.value) {
      inspectorEvents.on("hover", async () => {
        isAvailable.value = inspectorHasData();
      });
    }
    return inspector = markRaw({
      isAvailable,
      isEnabled: toRef(inspectorState, "isVisible"),
      enable: () => {
        inspectorState.isVisible = true;
        inspectorState.isEnabled = true;
      },
      disable: () => {
        inspectorState.isVisible = false;
        inspectorState.isEnabled = false;
      },
      toggle: () => {
        inspectorState.isEnabled = !inspectorState.isEnabled;
        inspectorState.isVisible = inspectorState.isEnabled;
      }
    });
  }
  setupRouteTracking(timeline, router);
  setupReactivity(client, router, timeline);
  clientRef.value = client;
  window.addEventListener("keydown", (e) => {
    if (e.code === "KeyD" && e.altKey && e.shiftKey)
      client.devtools.toggle();
  });
}
export function useClientColorMode() {
  const explicitColor = ref();
  const systemColor = ref();
  const elements = [
    document.documentElement,
    document.body
  ];
  const ob = new MutationObserver(getExplicitColor);
  elements.forEach((el) => {
    ob.observe(el, {
      attributes: true,
      attributeFilter: ["class"]
    });
  });
  const preferDarkQuery = window.matchMedia("(prefers-color-scheme: dark)");
  const preferLightQuery = window.matchMedia("(prefers-color-scheme: light)");
  preferDarkQuery.addEventListener("change", getSystemColor);
  preferLightQuery.addEventListener("change", getSystemColor);
  function getExplicitColor() {
    let color;
    for (const el of elements) {
      if (el.classList.contains("dark")) {
        color = "dark";
        break;
      }
      if (el.classList.contains("light")) {
        color = "light";
        break;
      }
    }
    explicitColor.value = color;
  }
  function getSystemColor() {
    if (preferDarkQuery.matches)
      systemColor.value = "dark";
    else if (preferLightQuery.matches)
      systemColor.value = "light";
    else
      systemColor.value = void 0;
  }
  getExplicitColor();
  getSystemColor();
  return computed(() => explicitColor.value || systemColor.value || "light");
}
function setupRouteTracking(timeline, router) {
  if (timeline.options.enabled && router?.currentRoute?.value?.path) {
    const start = timeline.events[0]?.start || Date.now();
    timeline.events.unshift({
      type: "route",
      from: router.currentRoute.value.path,
      to: router.currentRoute.value.path,
      start,
      end: start
    });
  }
  let lastRouteEvent;
  router?.afterEach(() => {
    if (lastRouteEvent && !lastRouteEvent?.end)
      lastRouteEvent.end = Date.now();
  });
  router?.beforeEach((to, from) => {
    if (!timeline.options.enabled)
      return;
    lastRouteEvent = {
      type: "route",
      from: from.path,
      to: to.path,
      start: Date.now()
    };
    timeline.events.push(lastRouteEvent);
  });
}
function setupReactivity(client, router, timeMetric) {
  const refreshReactivity = debounce(() => {
    client.hooks.callHook("host:update:reactivity");
  }, 100, { trailing: true });
  watch(() => [
    client.nuxt.payload,
    client.app.colorMode.value,
    client.metrics.loading(),
    timeMetric
  ], () => {
    refreshReactivity();
  }, { deep: true });
  router?.afterEach(() => {
    refreshReactivity();
  });
  client.nuxt.hook("app:mounted", () => {
    refreshReactivity();
  });
}
