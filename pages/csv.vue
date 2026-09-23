<script setup lang="ts">
definePageMeta({ layout: 'tools' })

useHead({
  title: 'Free CSV to JSON Converter | Convert Tabular Data in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser CSV to JSON converter, and back again. Configure delimiters and headers. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/csv/' },
    { property: 'og:title', content: 'Free CSV to JSON Converter | Convert Tabular Data' },
    { property: 'og:description', content: 'Convert CSV and JSON privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/csv/' }],
})

useToolSchema({
  name: 'CSV to JSON Converter',
  description: 'Convert CSV or TSV tabular data to JSON and back, locally in your browser.',
  url: 'https://ash-tools.store/csv/',
  category: 'DeveloperApplication',
})

type Direction = 'csv-to-json' | 'json-to-csv'

const SAMPLE_CSV = 'name,role,active\nAsh,Engineer,true\nJordan,Designer,false'
const placeholderCsv = 'name,role\nAsh,Engineer'
const placeholderJson = '[{"name": "Ash"}]'

const direction = ref<Direction>('csv-to-json')
const delimiter = ref(',')
const hasHeader = ref(true)
const input = ref(SAMPLE_CSV)
const output = ref('')
const status = ref('Paste CSV or JSON to convert.')
const isError = ref(false)

const delimiterChar = computed(() => (delimiter.value === '\\t' ? '\t' : delimiter.value || ','))

// RFC 4180-aware CSV line parser (handles quoted fields, embedded delimiters/newlines/escaped quotes).
const parseCsv = (text: string, delim: string): string[][] => {
  const rows: string[][] = []
  let row: string[] = []
  let field = ''
  let inQuotes = false
  let i = 0
  while (i < text.length) {
    const char = text[i]
    if (inQuotes) {
      if (char === '"') {
        if (text[i + 1] === '"') { field += '"'; i += 2; continue }
        inQuotes = false
        i += 1
        continue
      }
      field += char
      i += 1
      continue
    }
    if (char === '"') { inQuotes = true; i += 1; continue }
    if (char === delim) { row.push(field); field = ''; i += 1; continue }
    if (char === '\n' || char === '\r') {
      if (char === '\r' && text[i + 1] === '\n') i += 1
      row.push(field)
      rows.push(row)
      row = []
      field = ''
      i += 1
      continue
    }
    field += char
    i += 1
  }
  if (field.length || row.length) { row.push(field); rows.push(row) }
  return rows.filter((r) => !(r.length === 1 && r[0] === ''))
}

const csvEscape = (value: string, delim: string) => {
  if (value.includes(delim) || value.includes('"') || value.includes('\n') || value.includes('\r')) {
    return `"${value.replace(/"/g, '""')}"`
  }
  return value
}

const toCsv = (rows: Record<string, unknown>[], delim: string) => {
  if (!rows.length) return ''
  const headers = Array.from(rows.reduce((set, row) => { Object.keys(row).forEach((k) => set.add(k)); return set }, new Set<string>()))
  const lines = [headers.map((h) => csvEscape(h, delim)).join(delim)]
  rows.forEach((row) => {
    lines.push(headers.map((h) => csvEscape(row[h] === undefined || row[h] === null ? '' : String(row[h]), delim)).join(delim))
  })
  return lines.join('\n')
}

const inferValue = (raw: string): unknown => {
  if (raw === '') return ''
  if (raw === 'true') return true
  if (raw === 'false') return false
  if (/^-?\d+(\.\d+)?$/.test(raw)) return Number(raw)
  return raw
}

const convert = () => {
  if (!input.value.trim()) {
    output.value = ''
    isError.value = false
    status.value = 'Paste CSV or JSON to convert.'
    return
  }

  try {
    if (direction.value === 'csv-to-json') {
      const rows = parseCsv(input.value, delimiterChar.value)
      if (!rows.length) throw new Error('No rows found.')
      let headers: string[]
      let dataRows: string[][]
      if (hasHeader.value) {
        headers = rows[0]
        dataRows = rows.slice(1)
      } else {
        headers = rows[0].map((_, i) => `field${i + 1}`)
        dataRows = rows
      }
      const objects = dataRows.map((row) => {
        const obj: Record<string, unknown> = {}
        headers.forEach((h, i) => { obj[h] = inferValue(row[i] ?? '') })
        return obj
      })
      output.value = JSON.stringify(objects, null, 2)
      status.value = `Converted ${objects.length} row${objects.length === 1 ? '' : 's'} to JSON.`
    } else {
      const parsed = JSON.parse(input.value)
      const rows = Array.isArray(parsed) ? parsed : [parsed]
      if (!rows.every((r) => r && typeof r === 'object' && !Array.isArray(r))) {
        throw new Error('JSON must be an array of objects (or a single object).')
      }
      output.value = toCsv(rows, delimiterChar.value)
      status.value = `Converted ${rows.length} row${rows.length === 1 ? '' : 's'} to CSV.`
    }
    isError.value = false
  } catch (error) {
    isError.value = true
    output.value = ''
    status.value = error instanceof Error ? error.message : 'Conversion failed.'
  }
}

const swapDirection = () => {
  direction.value = direction.value === 'csv-to-json' ? 'json-to-csv' : 'csv-to-json'
  if (output.value) { input.value = output.value; output.value = '' }
  convert()
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
  const isJson = direction.value === 'csv-to-json'
  const blob = new Blob([output.value], { type: isJson ? 'application/json' : 'text/csv' })
  const url = URL.createObjectURL(blob)
  const anchor = document.createElement('a')
  anchor.href = url
  anchor.download = isJson ? 'data.json' : 'data.csv'
  document.body.appendChild(anchor)
  anchor.click()
  anchor.remove()
  URL.revokeObjectURL(url)
}

const onFileChange = (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) return
  const reader = new FileReader()
  reader.onload = () => { input.value = String(reader.result || ''); convert() }
  reader.readAsText(file)
}

const clearAll = () => {
  input.value = ''
  output.value = ''
  isError.value = false
  status.value = 'Paste CSV or JSON to convert.'
}

watch([direction, delimiter, hasHeader], convert)
onMounted(convert)
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
          <label for="csv-file-input" class="rail-btn cursor-pointer shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Open a file" aria-label="Open a file">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M6 3h5l3 3v11a1 1 0 01-1 1H6a1 1 0 01-1-1V4a1 1 0 011-1z" /><path stroke-linecap="round" stroke-linejoin="round" d="M11 3v3h3" /></svg>
            <span class="text-[10px] font-medium">Open</span>
          </label>
          <input id="csv-file-input" type="file" accept=".csv,.json,text/csv,application/json,text/plain" class="hidden" @change="onFileChange">
          <button class="rail-btn shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Swap direction" aria-label="Swap direction" @click="swapDirection">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M7 4L4 7l3 3M4 7h12M13 16l3-3-3-3M16 13H4" /></svg>
            <span class="text-[10px] font-medium">Swap</span>
          </button>
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
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="3" y="4" width="14" height="12" rx="1.5" /><path stroke-linecap="round" d="M3 8h14M8 8v8" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">CSV ⇄ JSON Converter</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <div class="ui-panel p-3 flex flex-wrap items-center gap-3">
          <div class="flex items-center gap-1 rounded-[var(--radius-sm)] p-1" style="background: var(--surface-2)">
            <button
              class="px-3 py-1.5 text-xs font-semibold rounded-[var(--radius-sm)] transition-colors duration-150"
              :class="direction === 'csv-to-json' ? 'ui-button !py-1.5 !px-3' : 'text-[var(--ink-3)] hover:text-[var(--ink-1)]'"
              @click="direction = 'csv-to-json'; convert()"
            >
              CSV → JSON
            </button>
            <button
              class="px-3 py-1.5 text-xs font-semibold rounded-[var(--radius-sm)] transition-colors duration-150"
              :class="direction === 'json-to-csv' ? 'ui-button !py-1.5 !px-3' : 'text-[var(--ink-3)] hover:text-[var(--ink-1)]'"
              @click="direction = 'json-to-csv'; convert()"
            >
              JSON → CSV
            </button>
          </div>

          <label class="flex items-center gap-1.5 text-xs text-[var(--ink-3)]">
            Delimiter
            <select v-model="delimiter" class="ui-input !py-1 !px-2 !w-auto text-xs">
              <option value=",">Comma</option>
              <option value=";">Semicolon</option>
              <option value="\t">Tab</option>
              <option value="|">Pipe</option>
            </select>
          </label>
          <label v-if="direction === 'csv-to-json'" class="flex items-center gap-1.5 text-xs text-[var(--ink-3)]">
            <input v-model="hasHeader" type="checkbox" class="ui-range"> First row is header
          </label>

          <div class="ml-auto flex items-center gap-2">
            <button class="ui-button-secondary !py-1.5 !px-3 text-xs" :disabled="!output" @click="copyOutput">Copy</button>
            <button class="ui-button !py-1.5 !px-3 text-xs" :disabled="!output" @click="downloadOutput">Download</button>
          </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 flex-1 min-h-0">
          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">{{ direction === 'csv-to-json' ? 'CSV input' : 'JSON input' }}</span>
            </div>
            <div class="px-4 pb-4 flex-1 min-h-0">
              <textarea
                v-model="input"
                spellcheck="false"
                :placeholder="direction === 'csv-to-json' ? placeholderCsv : placeholderJson"
                class="ui-input font-mono text-xs w-full h-full min-h-[280px] resize-none"
                @input="convert"
              />
            </div>
          </section>

          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">{{ direction === 'csv-to-json' ? 'JSON output' : 'CSV output' }}</span>
            </div>
            <div class="px-4 pb-4 flex-1 min-h-0">
              <pre
                class="ui-input font-mono text-xs w-full h-full min-h-[280px] overflow-auto whitespace-pre-wrap break-words"
                :class="isError ? 'text-red-600' : ''"
              >{{ isError ? status : (output || 'Result will appear here.') }}</pre>
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
