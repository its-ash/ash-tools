<script setup lang="ts">
type Tool = {
  name: string
  href: string
  description: string
  keywords?: string
}

// Every tool page in the project, independent of what's linked in the header nav.
const tools: Tool[] = [
  { name: 'Video Studio', href: '/video/', description: 'Trim, crop, and compress video locally', keywords: 'mp4 webm trim crop compress' },
  { name: 'Image Editor', href: '/image/', description: 'Crop, resize, rotate, and adjust images', keywords: 'photo crop resize rotate filter heic' },
  { name: 'ZIP Archiver', href: '/zip/', description: 'Create and inspect ZIP archives', keywords: 'compress archive folder unzip' },
  { name: 'PDF Merger', href: '/pdf/', description: 'Merge, reorder, and compress PDFs', keywords: 'merge combine compress document' },
  { name: 'Code Sandbox', href: '/sandbox/', description: 'Run JavaScript, Python, and Rust in the browser', keywords: 'javascript python rust run execute code editor' },
  { name: 'Color Picker', href: '/color/', description: 'Convert between HEX, RGB, and HSL', keywords: 'hex rgb hsl palette convert' },
  { name: 'Diff Checker', href: '/diff/', description: 'Compare two blocks of text', keywords: 'compare text changes' },
  { name: 'Encode / Decode', href: '/encode/', description: 'Base64, URL, hex, and hash a string', keywords: 'base64 url hex sha md5 hash' },
  { name: 'JSON Formatter', href: '/json/', description: 'Validate and beautify JSON', keywords: 'validate beautify minify pretty' },
  { name: 'Markdown Previewer', href: '/markdown/', description: 'Preview Markdown as live HTML', keywords: 'md html preview render' },
  { name: 'QR Code Generator', href: '/qr/', description: 'Create a QR code from text or a link', keywords: 'qrcode generate scan' },
  { name: 'Regex Tester', href: '/regex/', description: 'Test regular expressions with live matches', keywords: 'regexp pattern match test' },
  { name: 'CSV to JSON', href: '/csv/', description: 'Convert tabular CSV data to JSON', keywords: 'csv tsv table convert spreadsheet' },
  { name: 'Text Case Converter', href: '/text/', description: 'Convert text case and generate Lorem Ipsum', keywords: 'uppercase lowercase camel snake lorem ipsum' },
  { name: 'Timestamp Converter', href: '/timestamp/', description: 'Convert Unix, ISO 8601, and human dates', keywords: 'unix epoch iso date time' },
  { name: 'Unit Converter', href: '/units/', description: 'Length, weight, temperature, and more', keywords: 'length weight temperature convert metric' },
  { name: 'UUID Generator', href: '/uuid/', description: 'Generate UUIDs and secure passwords', keywords: 'guid random password generator' },
]

const isOpen = ref(false)
const query = ref('')
const activeIndex = ref(0)
const inputEl = ref<HTMLInputElement | null>(null)
const router = useRouter()

const results = computed(() => {
  const q = query.value.trim().toLowerCase()
  if (!q) return tools
  return tools.filter((tool) =>
    tool.name.toLowerCase().includes(q) ||
    tool.description.toLowerCase().includes(q) ||
    tool.keywords?.toLowerCase().includes(q)
  )
})

watch(results, () => {
  activeIndex.value = 0
})

const open = () => {
  isOpen.value = true
  query.value = ''
  activeIndex.value = 0
  nextTick(() => inputEl.value?.focus())
}

const close = () => {
  isOpen.value = false
}

const go = (tool?: Tool) => {
  const target = tool ?? results.value[activeIndex.value]
  if (!target) return
  close()
  router.push(target.href)
}

const onKeydown = (event: KeyboardEvent) => {
  if (event.key === 'ArrowDown') {
    event.preventDefault()
    activeIndex.value = Math.min(activeIndex.value + 1, results.value.length - 1)
  } else if (event.key === 'ArrowUp') {
    event.preventDefault()
    activeIndex.value = Math.max(activeIndex.value - 1, 0)
  } else if (event.key === 'Enter') {
    event.preventDefault()
    go()
  } else if (event.key === 'Escape') {
    close()
  }
}

const onGlobalKeydown = (event: KeyboardEvent) => {
  const cmdOrCtrl = event.metaKey || event.ctrlKey
  if (cmdOrCtrl && event.key.toLowerCase() === 'k') {
    event.preventDefault()
    isOpen.value ? close() : open()
  }
}

onMounted(() => window.addEventListener('keydown', onGlobalKeydown))
onUnmounted(() => window.removeEventListener('keydown', onGlobalKeydown))

const isMac = ref(false)
onMounted(() => {
  isMac.value = /Mac|iPhone|iPad/.test(navigator.platform || navigator.userAgent)
})
</script>

<template>
  <button
    type="button"
    v-bind="$attrs"
    class="search-trigger flex items-center gap-2 rounded-none border border-[var(--border)] bg-[var(--surface-1)] px-3 py-2 text-sm text-[var(--ink-3)] transition-colors duration-150 hover:bg-[var(--surface-2)] hover:text-[var(--ink-1)]"
    aria-label="Search tools"
    @click="open"
  >
    <svg class="h-4 w-4 shrink-0" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6">
      <circle cx="9" cy="9" r="6" />
      <path stroke-linecap="round" d="m17 17-4.3-4.3" />
    </svg>
    <span class="hidden sm:inline">Search tools</span>
    <kbd class="ml-1 hidden shrink-0 items-center gap-0.5 rounded-none border border-[var(--border)] bg-[var(--surface-2)] px-1.5 py-0.5 font-mono text-[11px] text-[var(--ink-1)] sm:flex">
      {{ isMac ? '⌘' : 'Ctrl' }}K
    </kbd>
  </button>

  <Teleport to="body">
    <Transition
      enter-active-class="transition-opacity duration-150 ease-out"
      leave-active-class="transition-opacity duration-100 ease-out"
      enter-from-class="opacity-0"
      leave-to-class="opacity-0"
    >
      <div
        v-if="isOpen"
        class="fixed inset-0 z-[100] flex items-start justify-center bg-[var(--ink-1)]/50 px-4 pt-[12vh]"
        @click.self="close"
      >
        <Transition
          enter-active-class="transition-all duration-150 ease-out"
          leave-active-class="transition-all duration-100 ease-out"
          enter-from-class="opacity-0 scale-[0.97] -translate-y-1"
          leave-to-class="opacity-0 scale-[0.97] -translate-y-1"
        >
          <div
            v-if="isOpen"
            class="w-full max-w-lg overflow-hidden rounded-none border-2 border-[var(--border)] bg-[var(--surface-1)]"
            role="dialog"
            aria-modal="true"
            aria-label="Search tools"
          >
            <div class="flex items-center gap-3 border-b border-[var(--border)] px-4 py-3">
              <svg class="h-4 w-4 shrink-0 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6">
                <circle cx="9" cy="9" r="6" />
                <path stroke-linecap="round" d="m17 17-4.3-4.3" />
              </svg>
              <input
                ref="inputEl"
                v-model="query"
                type="text"
                placeholder="Search tools..."
                class="flex-1 bg-transparent text-[15px] text-[var(--ink-1)] outline-none placeholder:text-[var(--ink-3)]"
                @keydown="onKeydown"
              >
              <kbd class="hidden shrink-0 rounded-none border border-[var(--border)] bg-[var(--surface-2)] px-1.5 py-0.5 font-mono text-[11px] text-[var(--ink-1)] sm:block">Esc</kbd>
            </div>

            <ul class="max-h-[60vh] overflow-y-auto p-2" role="listbox">
              <li v-if="!results.length" class="px-3 py-8 text-center text-sm text-[var(--ink-3)]">
                No tools match "{{ query }}"
              </li>
              <li v-for="(tool, index) in results" :key="tool.href" role="option" :aria-selected="index === activeIndex">
                <button
                  type="button"
                  class="flex w-full items-center justify-between gap-3 rounded-none px-3 py-2.5 text-left transition-colors duration-100"
                  :class="index === activeIndex ? 'bg-[var(--ink-1)] text-white' : 'hover:bg-[var(--surface-2)]'"
                  @click="go(tool)"
                  @mouseenter="activeIndex = index"
                >
                  <span class="min-w-0">
                    <span class="block truncate text-sm font-medium" :class="index === activeIndex ? 'text-white' : 'text-[var(--ink-1)]'">{{ tool.name }}</span>
                    <span class="block truncate text-xs" :class="index === activeIndex ? 'text-white/70' : 'text-[var(--ink-3)]'">{{ tool.description }}</span>
                  </span>
                  <svg v-if="index === activeIndex" class="h-3.5 w-3.5 shrink-0 text-white" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M8.5 4L14 9.5 8.5 15" />
                  </svg>
                </button>
              </li>
            </ul>
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>
