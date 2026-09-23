<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
definePageMeta({ layout: 'tools' })

useHead({
  title: "Free Code Sandbox | Run JavaScript, Python & Rust in your browser",
  meta: [
    {
      name: "description",
      content:
        "Free, private, in-browser code sandbox. Run JavaScript, Python (Pyodide), and Rust (WASI) locally with no server, no signup, no uploads.",
    },
    {
      name: "keywords",
      content:
        "wasm runner, sandbox, compiler, wasm, rust, c, c++, python, javascript, browser runtime",
    },
    { name: "robots", content: "index,follow" },
    { property: "og:url", content: "https://ash-tools.store/sandbox/" },
    { property: "og:title", content: "Free Code Sandbox | Run JavaScript, Python & Rust" },
    { property: "og:description", content: "Run code privately in your browser. Nothing leaves your device." },
    { name: "twitter:card", content: "summary_large_image" },
  ],
  link: [{ rel: "canonical", href: "https://ash-tools.store/sandbox/" }],
});

useToolSchema({
  name: "Code Sandbox",
  description: "Run JavaScript, Python, and Rust locally in your browser.",
  url: "https://ash-tools.store/sandbox/",
  category: "DeveloperApplication",
});

let scriptEl: HTMLScriptElement | null = null;
let monacoInstance: any = null;
// Expose Monaco globally for main.js (use globalThis for SSR/browser safety)
if (typeof globalThis !== 'undefined') {
  globalThis.__getMonacoCode = () => monacoInstance ? monacoInstance.getValue() : '';
}
const selectedLanguage = ref('javascript');
const demoCodes: Record<string, string> = {
  javascript: `// JavaScript demo\nconsole.log('Hello, JavaScript!');`,
  python: `name = input("What is you name : ")\nprint('Hello, Python!', name)`,
  rust: `// Rust demo\nfn main() {\n    println!(\"Hello, Rust!\");\n}`
};

// Display-only derived labels for the tab bar / status bar (view layer, no effect on execution).
const scratchFileConfig: Record<string, { file: string; label: string }> = {
  javascript: { file: 'scratch.js', label: 'JavaScript' },
  python: { file: 'scratch.py', label: 'Python (Pyodide)' },
  rust: { file: 'scratch.rs', label: 'Rust (WASI)' },
};
const scratchFileName = computed(() => scratchFileConfig[selectedLanguage.value]?.file ?? 'scratch.txt');
const languageLabel = computed(() => scratchFileConfig[selectedLanguage.value]?.label ?? selectedLanguage.value);

function updateMonacoLanguageAndCode(lang: string) {
  if (monacoInstance && window.monaco) {
    const model = monacoInstance.getModel();
    if (model) {
      window.monaco.editor.setModelLanguage(model, lang === 'python' ? 'python' : lang);
      monacoInstance.setValue(demoCodes[lang] || '');
    }
  }
}

onMounted(() => {
  try {
    const params = new URLSearchParams(window.location.search);
    const q = params.get('lang') || "javascript";
    if (q && ['javascript', 'python', 'rust'].includes(q)) {
      selectedLanguage.value = q;
    }
  } catch (e) { }
  if (scriptEl) scriptEl.remove();
  const monacoScript = document.createElement("script");
  monacoScript.src =
    "https://cdn.jsdelivr.net/npm/monaco-editor@0.45.0/min/vs/loader.js";
  monacoScript.onload = () => {
    // @ts-ignore
    window.require.config({
      paths: { vs: "https://cdn.jsdelivr.net/npm/monaco-editor@0.45.0/min/vs" },
    });
    // @ts-ignore
    window.require(["vs/editor/editor.main"], () => {
      monacoInstance = window.monaco.editor.create(
        document.getElementById("codeEditor"),
        {
          value: demoCodes[selectedLanguage.value],
          language: selectedLanguage.value === 'python' ? 'python' : selectedLanguage.value,
          theme: "dracula",
          fontSize: 14,
          minimap: { enabled: false },
          automaticLayout: true,
        }
      );
      // Dracula theme definition
      window.monaco.editor.defineTheme("dracula", {
        base: "vs-dark",
        inherit: true,
        rules: [
          { background: "282A36" },
          { token: "", foreground: "F8F8F2", background: "282A36" },
          { token: "comment", foreground: "6272A4" },
          { token: "string", foreground: "F1FA8C" },
          { token: "keyword", foreground: "FF79C6" },
          { token: "number", foreground: "BD93F9" },
          { token: "regexp", foreground: "FFB86C" },
          { token: "operator", foreground: "FF79C6" },
          { token: "namespace", foreground: "8BE9FD" },
          { token: "type", foreground: "8BE9FD" },
          { token: "function", foreground: "50FA7B" },
          { token: "variable", foreground: "F8F8F2" },
        ],
        colors: {
          "editor.background": "#282A36",
          "editor.foreground": "#F8F8F2",
          "editor.lineHighlightBackground": "#44475A",
          "editorCursor.foreground": "#F8F8F0",
          "editorWhitespace.foreground": "#3B3A32",
          "editorIndentGuide.background": "#44475A",
          "editor.selectionBackground": "#44475A",
          "editor.inactiveSelectionBackground": "#44475A99",
        },
      });
      window.monaco.editor.setTheme("dracula");
    });
  };
  document.body.appendChild(monacoScript);

  scriptEl = document.createElement("script");
  scriptEl.type = "module";
  scriptEl.src = `/sandbox/main.js?t=${Date.now()}`;
  document.head.appendChild(scriptEl);
});

watch(selectedLanguage, (lang) => {
  updateMonacoLanguageAndCode(lang);
  // update URL without reloading
  try {
    const params = new URLSearchParams(window.location.search);
    params.set('lang', lang);
    const url = `${window.location.pathname}?${params.toString()}`;
    window.history.replaceState({}, '', url);
  } catch (e) { }
});

onUnmounted(() => {
  scriptEl?.remove();
  scriptEl = null;
  if (monacoInstance) {
    monacoInstance.dispose();
    monacoInstance = null;
  }
});
</script>

<template>
  <div class="min-h-screen bg-[var(--surface-0)] text-[var(--ink-1)] flex flex-col">
    <main class="relative flex flex-col md:flex-row flex-1 min-h-0">

      <!-- Left rail: navigation + run controls -->
      <aside class="order-2 md:order-1 shrink-0 border-t md:border-t-0 md:border-r md:w-[76px] w-full" style="background: var(--surface-1); border-color: var(--border)">
        <div class="flex md:flex-col items-center gap-1 p-2 md:py-4 overflow-x-auto md:overflow-visible">
          <NuxtLink to="/" class="rail-btn shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Back to home" aria-label="Back to home">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6"><path stroke-linecap="round" stroke-linejoin="round" d="M11.5 4L6 9.5l5.5 5.5" /></svg>
            <span class="text-[10px] font-medium">Home</span>
          </NuxtLink>
          <button id="runBtn" class="rail-btn shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1" style="color: var(--accent)" title="Run" aria-label="Run">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 12 12" fill="currentColor"><path d="M2 1.5v9l8-4.5-8-4.5z" /></svg>
            <span class="text-[10px] font-medium">Run</span>
          </button>
          <button id="stopBtn" disabled class="rail-btn shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)] disabled:opacity-40 disabled:pointer-events-none" title="Stop" aria-label="Stop">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 12 12" fill="currentColor"><rect x="2" y="2" width="8" height="8" rx="1" /></svg>
            <span class="text-[10px] font-medium">Stop</span>
          </button>
        </div>
      </aside>

      <section class="order-1 md:order-2 flex flex-col gap-2 p-2 md:p-3 flex-1 min-h-0">
        <!-- Toolbar -->
        <div class="flex flex-wrap items-center justify-between gap-3 md:gap-4">
          <div class="flex flex-wrap items-center gap-2">
            <label
              class="ui-card flex items-center gap-2 px-3 py-2">
              <svg class="h-3.5 w-3.5 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M7 8l-4 4 4 4M13 8l4 4-4 4" /></svg>
              <span class="text-xs font-semibold text-[var(--ink-3)]">Language</span>
              <select id="languageSelect" aria-label="Language"
                class="bg-transparent text-[var(--ink-1)] border-0 font-mono text-xs outline-none"
                v-model="selectedLanguage">
                <option value="javascript">JavaScript</option>
                <option value="python">Python (Pyodide)</option>
                <option value="rust">Rust (WASI)</option>
              </select>
            </label>
            <label
              class="ui-card flex items-center gap-2 px-3 py-2">
              <svg class="h-3.5 w-3.5 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M10 6v4l2.5 2.5" /><circle cx="10" cy="10" r="7" stroke-linecap="round" /></svg>
              <span class="text-xs font-semibold text-[var(--ink-3)]">Timeout</span>
              <select id="timeoutSelect" aria-label="Timeout"
                class="bg-transparent text-[var(--ink-1)] border-0 font-mono text-xs outline-none">
                <option value="3000">3s</option>
                <option value="5000" selected>5s</option>
                <option value="10000">10s</option>
              </select>
            </label>
          </div>

          <div class="flex items-center gap-2">
            <div
              class="ui-badge ui-badge-accent inline-flex items-center gap-2 px-3 py-2"
              aria-live="polite">
              <span class="status-dot ok h-1.5 w-1.5 rounded-full" aria-hidden="true"></span>
              <span class="text-xs font-semibold">Runtime</span>
              <span id="runtimeStatus" class="text-xs font-mono">Idle</span>
            </div>
          </div>
        </div>

        <!-- Workspace grid: Editor (2fr) + Side panel (1fr) -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-2 md:gap-3 flex-1 min-h-0">
          <!-- Code editor card -->
          <div
            class="md:col-span-2 flex flex-col ui-panel overflow-hidden min-h-0" style="background: #21232d">
            <!-- File tab bar -->
            <div class="flex items-center gap-2 px-3 pt-2.5" style="background: #21232d">
              <div
                class="flex items-center gap-2 rounded-t-[var(--radius-sm)] px-3 py-1.5 text-xs font-mono"
                style="background: #282A36; color: #F8F8F2"
              >
                <svg class="h-3 w-3 shrink-0" style="color: #8BE9FD" viewBox="0 0 16 16" fill="currentColor"><path d="M9 1H4a1 1 0 00-1 1v12a1 1 0 001 1h8a1 1 0 001-1V5l-4-4z" /></svg>
                {{ scratchFileName }}
                <span class="h-1.5 w-1.5 rounded-full" style="background: #6272A4" aria-hidden="true"></span>
              </div>
            </div>
            <div id="codeEditor" class="flex-1 min-h-0"
              style="background: #282A36"
              aria-label="Code editor"></div>
            <!-- Editor status bar -->
            <div
              class="flex items-center justify-between px-3 py-1 text-[11px] font-mono"
              style="background: #21232d; color: #a4a9c9"
            >
              <span>{{ languageLabel }}</span>
              <span>UTF-8 · LF · Dracula</span>
            </div>
          </div>

          <!-- Side panel: 5 cards stacked -->
          <div class="flex flex-col gap-2 min-h-0 overflow-y-auto pr-1 custom-scrollbar">
            <!-- Execution card -->
            <div class="ui-panel p-3 shrink-0">
              <div class="flex items-baseline justify-between mb-2">
                <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Execution</span>
                <span class="text-xs font-mono text-[var(--ink-3)]" id="timing">time: --</span>
              </div>
              <div class="grid grid-cols-2 gap-2 text-[11px] mb-2">
                <div>
                  <span class="block text-xs text-[var(--ink-3)] mb-1">Memory</span>
                  <span class="font-mono text-[var(--ink-1)]" id="memory">--</span>
                </div>
                <div>
                  <span class="block text-xs text-[var(--ink-3)] mb-1">Exit</span>
                  <span class="font-mono text-[var(--ink-1)]" id="exitCode">--</span>
                </div>
              </div>
              <div id="status"
                class="ui-badge text-xs font-mono">
                Idle
              </div>
            </div>

            <!-- Input card -->
            <div class="ui-panel p-3 shrink-0">
              <div class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)] mb-2">
                Input <span class="normal-case font-mono tracking-normal text-[var(--ink-3)]">(stdin)</span>
              </div>
              <textarea id="stdinInput" placeholder="Enter input here..."
                class="w-full h-14 rounded-[var(--radius-sm)] p-2 font-mono text-[11px] resize-none focus:outline-none focus:ring-2"
                style="background: #1e1b16; color: #e8e2d6; border: 1px solid var(--border-strong)">Ash</textarea>
            </div>

            <!-- Console Output card -->
            <div class="ui-panel p-3 shrink-0">
              <div class="flex items-center justify-between mb-2">
                <div>
                  <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)] block">Console Output</span>
                  <span class="text-xs font-mono text-[var(--ink-3)]">stdout / stderr</span>
                </div>
                <button id="copyConsoleBtn"
                  class="ui-button-secondary px-2 py-1 text-xs">
                  Copy
                </button>
              </div>
              <pre id="consoleOutput"
                class="h-20 w-full rounded-[var(--radius-sm)] p-2 font-mono text-[11px] overflow-auto whitespace-pre-wrap wrap-break-word"
                style="background: #1e1b16; color: #e8e2d6; border: 1px solid var(--border-strong)"></pre>
            </div>

            <!-- Errors card -->
            <div
              class="ui-panel p-3 flex-1 flex flex-col min-h-40" style="border-color: var(--danger-soft)">
              <div class="flex items-center justify-between mb-2">
                <div>
                  <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)] block">Errors</span>
                  <span class="text-xs font-mono text-[var(--ink-3)]">runtime</span>
                </div>
                <button id="copyErrorBtn"
                  class="ui-button-secondary px-2 py-1 text-xs">
                  Copy
                </button>
              </div>
              <pre id="errorOutput"
                class="flex-1 w-full rounded-[var(--radius-sm)] p-2 font-mono text-[11px] leading-relaxed overflow-auto whitespace-pre-wrap wrap-break-word"
                style="background: #1e1b16; color: #e08a72; border: 1px solid var(--danger-soft)"></pre>
            </div>

            <!-- Compile Logs card -->
            <div class="ui-panel p-3 shrink-0">
              <div class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)] mb-1">
                Compile Logs
              </div>
              <span class="text-xs font-mono text-[var(--ink-3)] block mb-2">wasm toolchain</span>
              <pre id="compileOutput"
                class="h-16 w-full rounded-[var(--radius-sm)] p-2 font-mono text-[11px] overflow-auto whitespace-pre-wrap wrap-break-word"
                style="background: #1e1b16; color: #a8a196; border: 1px solid var(--border-strong)"></pre>
            </div>
          </div>
        </div>
      </section>
    </main>

    <!-- Bottom status strip -->
    <div class="shrink-0 border-t px-3 md:px-4 py-2 flex items-center gap-4 text-xs font-mono" style="border-color: var(--border); background: var(--surface-1); color: var(--ink-3)">
      <span>{{ languageLabel }}</span>
      <span class="ml-auto">local · offline · in-browser sandbox</span>
    </div>
  </div>
</template>

<style scoped>
.rail-btn {
  transition: transform 140ms cubic-bezier(0.23, 1, 0.32, 1), background-color 140ms ease, color 140ms ease;
}

.rail-btn:hover {
  background: var(--surface-2);
  color: var(--ink-1);
}

.rail-btn:active {
  transform: scale(0.94);
}

@media (prefers-reduced-motion: reduce) {
  .rail-btn {
    transition: none !important;
  }
}
</style>
