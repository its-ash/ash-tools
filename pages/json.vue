<script setup lang="ts">
definePageMeta({ layout: 'tools' })

useHead({
  title: 'Free JSON Formatter | Validate & Beautify JSON in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser JSON formatter and validator. Beautify, minify, and check JSON locally on your device. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/json/' },
    { property: 'og:title', content: 'Free JSON Formatter | Validate & Beautify JSON in your browser' },
    { property: 'og:description', content: 'Format, minify, and validate JSON privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/json/' }],
})

useToolSchema({
  name: 'JSON Formatter',
  description: 'Validate, beautify, and minify JSON locally in your browser.',
  url: 'https://ash-tools.store/json/',
  category: 'DeveloperApplication',
})

const INDENT_SIZE = 2

const input = ref('')
const output = ref('')
const status = ref('Paste or type JSON to begin.')
const isError = ref(false)
const indentSize = ref(INDENT_SIZE)

const stats = computed(() => {
  if (!output.value || isError.value) return null
  try {
    const parsed = JSON.parse(input.value)
    const keys = (val: unknown): number => {
      if (Array.isArray(val)) return val.reduce((sum: number, item) => sum + keys(item), 0)
      if (val && typeof val === 'object') {
        return Object.keys(val).length + Object.values(val).reduce((sum: number, item) => sum + keys(item), 0)
      }
      return 0
    }
    return { keys: keys(parsed), bytes: new Blob([output.value]).size }
  } catch {
    return null
  }
})

const prettySize = (bytes: number) => {
  if (bytes < 1024) return `${bytes} B`
  return `${(bytes / 1024).toFixed(1)} KB`
}

const parseInput = (): unknown => JSON.parse(input.value)

const formatJson = () => {
  if (!input.value.trim()) {
    status.value = 'Paste or type JSON to begin.'
    isError.value = false
    output.value = ''
    return
  }
  try {
    const parsed = parseInput()
    output.value = JSON.stringify(parsed, null, indentSize.value)
    isError.value = false
    status.value = 'Valid JSON.'
  } catch (error) {
    isError.value = true
    output.value = ''
    status.value = error instanceof Error ? error.message : 'Invalid JSON.'
  }
}

const minifyJson = () => {
  if (!input.value.trim()) return
  try {
    const parsed = parseInput()
    output.value = JSON.stringify(parsed)
    isError.value = false
    status.value = 'Valid JSON (minified).'
  } catch (error) {
    isError.value = true
    output.value = ''
    status.value = error instanceof Error ? error.message : 'Invalid JSON.'
  }
}

const copyOutput = async () => {
  if (!output.value) return
  try {
    await navigator.clipboard.writeText(output.value)
    status.value = 'Copied to clipboard.'
  } catch {
    status.value = 'Could not copy — select and copy manually.'
  }
}

const downloadOutput = () => {
  if (!output.value) return
  const blob = new Blob([output.value], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const anchor = document.createElement('a')
  anchor.href = url
  anchor.download = 'formatted.json'
  document.body.appendChild(anchor)
  anchor.click()
  anchor.remove()
  URL.revokeObjectURL(url)
}

const clearAll = () => {
  input.value = ''
  output.value = ''
  isError.value = false
  status.value = 'Paste or type JSON to begin.'
}

const loadSample = () => {
  input.value = JSON.stringify(
    { name: 'Ash Tools', version: 1, active: true, tags: ['local', 'private', 'offline'], meta: { author: null } },
    null,
    0,
  )
  formatJson()
}

const onFileChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (!file) return
  const reader = new FileReader()
  reader.onload = () => {
    input.value = String(reader.result || '')
    formatJson()
  }
  reader.readAsText(file)
}

watch(input, () => {
  if (!input.value.trim()) {
    output.value = ''
    isError.value = false
    status.value = 'Paste or type JSON to begin.'
  }
})
</script>

<template>
  <div class="min-h-screen bg-[var(--surface-0)] text-[var(--ink-1)] flex flex-col">
    <div class="flex-1 flex flex-col md:flex-row">
      <aside class="order-2 md:order-1 shrink-0 border-t md:border-t-0 md:border-r md:w-[76px] w-full" style="background: var(--surface-1); border-color: var(--border)">
        <div class="flex md:flex-col items-center gap-1 p-2 md:py-4 overflow-x-auto md:overflow-visible">
          <NuxtLink to="/" class="rail-btn shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Back to home" aria-label="Back to home">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6"><path stroke-linecap="round" stroke-linejoin="round" d="M11.5 4L6 9.5l5.5 5.5" /></svg>
            <span class="text-[10px] font-medium">Home</span>
          </NuxtLink>
          <label for="json-file-input" class="rail-btn cursor-pointer shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Open a file" aria-label="Open a file">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M6 3h5l3 3v11a1 1 0 01-1 1H6a1 1 0 01-1-1V4a1 1 0 011-1z" /><path stroke-linecap="round" stroke-linejoin="round" d="M11 3v3h3" /></svg>
            <span class="text-[10px] font-medium">Open</span>
          </label>
          <input id="json-file-input" type="file" accept=".json,application/json,text/plain" class="hidden" @change="onFileChange">
          <button class="rail-btn shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Clear" aria-label="Clear" @click="clearAll">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M5 6h10M8 6V4h4v2m-6 0l1 10h6l1-10" /></svg>
            <span class="text-[10px] font-medium">Clear</span>
          </button>
        </div>
      </aside>

      <main class="order-1 md:order-2 flex-1 min-w-0 flex flex-col p-4 md:p-6">
      <div class="mx-auto w-full max-w-6xl flex flex-col gap-4">
        <div class="ui-panel p-4 flex items-center justify-between gap-3">
          <div class="flex items-center gap-2">
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M6 3.5c-1 0-1.5.5-1.5 1.5v2c0 1-.5 1.5-1.5 1.5.9 0 1.5.5 1.5 1.5v2c0 1 .5 1.5 1.5 1.5M14 3.5c1 0 1.5.5 1.5 1.5v2c0 1 .5 1.5 1.5 1.5-.9 0-1.5.5-1.5 1.5v2c0 1-.5 1.5-1.5 1.5" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">JSON Formatter</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <div class="ui-panel p-3 flex flex-wrap items-center gap-2">
          <button class="ui-button !py-1.5 !px-3 text-xs" @click="formatJson">Format</button>
          <button class="ui-button-secondary !py-1.5 !px-3 text-xs" @click="minifyJson">Minify</button>
          <label class="flex items-center gap-1.5 text-xs text-[var(--ink-3)]">
            Indent
            <select v-model.number="indentSize" class="ui-input !py-1 !px-2 !w-auto text-xs" @change="formatJson">
              <option :value="2">2</option>
              <option :value="4">4</option>
              <option :value="0">Tab</option>
            </select>
          </label>
          <div class="ml-auto flex items-center gap-2">
            <button class="ui-button-secondary !py-1.5 !px-3 text-xs" @click="loadSample">Load sample</button>
            <button class="ui-button-secondary !py-1.5 !px-3 text-xs" :disabled="!output" @click="copyOutput">Copy</button>
            <button class="ui-button !py-1.5 !px-3 text-xs" :disabled="!output" @click="downloadOutput">Download</button>
          </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 flex-1 min-h-0">
          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Input</span>
              <span class="text-xs font-mono text-[var(--ink-3)]">{{ input.length }} chars</span>
            </div>
            <div class="px-4 pb-4 flex-1 min-h-0">
              <textarea
                v-model="input"
                spellcheck="false"
                placeholder='{"key": "value"}'
                class="ui-input font-mono text-xs w-full h-full min-h-[280px] resize-none"
                @input="formatJson"
              />
            </div>
          </section>

          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Output</span>
              <span v-if="stats" class="text-xs font-mono text-[var(--ink-3)]">{{ stats.keys }} keys · {{ prettySize(stats.bytes) }}</span>
            </div>
            <div class="px-4 pb-4 flex-1 min-h-0">
              <pre
                class="ui-input font-mono text-xs w-full h-full min-h-[280px] overflow-auto whitespace-pre-wrap break-words"
                :class="isError ? 'text-red-600' : ''"
              >{{ isError ? status : (output || 'Formatted JSON will appear here.') }}</pre>
            </div>
          </section>
        </div>
      </div>
      </main>
    </div>

    <div class="shrink-0 border-t px-5 py-2 flex items-center gap-4 text-xs font-mono" style="border-color: var(--border); background: var(--surface-1); color: var(--ink-3)">
      <span class="truncate" :class="isError ? 'text-red-600' : ''">{{ status }}</span>
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
