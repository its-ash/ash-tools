<script setup lang="ts">
useHead({
  title: 'PDF Merger & Compressor | Ash Tools',
  meta: [
    { name: 'description', content: 'Free online PDF Merger and Compressor. Combine multiple PDF files, reorder pages, and compress the output. Runs fully offline in your browser using Rust + WebAssembly.' },
    { name: 'keywords', content: 'pdf merger, pdf compressor, combine pdf, merge pdf online, pdf join, compress pdf, webassembly pdf, offline pdf tool' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:title', content: 'PDF Merger & Compressor | Ash Tools' },
    { property: 'og:description', content: 'Merge and compress PDF files locally in your browser. No uploads, no tracking.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/pdf/' }],
})

type PdfFile = {
  id: string
  name: string
  size: number
  pageCount: number
  bytes: Uint8Array
  thumbnail: string | null
}

type WasmPdfModule = {
  default: () => Promise<void>
  get_page_count: (bytes: Uint8Array) => number
  merge_and_compress: (arrays: Uint8Array[], quality: number) => Uint8Array
}

const pdfFiles = ref<PdfFile[]>([])
const processing = ref(false)
const compressionLevel = ref(70)
const statusMessage = ref('Upload PDF files to get started.')
const progressPercent = ref(0)
const showExportPanel = ref(false)
const dragIndex = ref<number | null>(null)
const dragOverIndex = ref<number | null>(null)
const outputSize = ref<number | null>(null)
const estimating = ref(false)
let pdfJsLib: any = null
let estimateTimer: ReturnType<typeof setTimeout> | null = null

let wasmReadyPromise: Promise<void> | null = null
let wasmGetPageCount: ((bytes: Uint8Array) => number) | null = null
let wasmMergeAndCompress: ((arrays: Uint8Array[], quality: number) => Uint8Array) | null = null

const prettySize = (bytes: number) => {
  if (bytes < 1024) return `${bytes} B`
  const kb = bytes / 1024
  if (kb < 1024) return `${kb.toFixed(1)} KB`
  return `${(kb / 1024).toFixed(2)} MB`
}

const ensureWasm = async () => {
  if (!wasmReadyPromise) {
    wasmReadyPromise = ((0, eval)('import("/pdf/pkg/pdf_wasm.js")') as Promise<WasmPdfModule>)
      .then(async (module: WasmPdfModule) => {
        await module.default()
        wasmGetPageCount = module.get_page_count as (bytes: Uint8Array) => number
        wasmMergeAndCompress = module.merge_and_compress as (arrays: Uint8Array[], quality: number) => Uint8Array
      })
      .catch((error) => {
        wasmReadyPromise = null
        throw error
      })
  }
  await wasmReadyPromise
}

const readFileAsUint8 = async (file: File): Promise<Uint8Array> => {
  const buffer = await file.arrayBuffer()
  return new Uint8Array(buffer)
}

const generateId = () => Math.random().toString(36).slice(2, 10) + Date.now().toString(36)

// ── pdf.js for thumbnails ──
const ensurePdfJs = async () => {
  if (pdfJsLib) return pdfJsLib
  if ((window as any).pdfjsLib) {
    pdfJsLib = (window as any).pdfjsLib
    pdfJsLib.GlobalWorkerOptions.workerSrc = 'https://cdn.jsdelivr.net/npm/pdfjs-dist@3.11.174/build/pdf.worker.min.js'
    return pdfJsLib
  }
  return new Promise<any>((resolve, reject) => {
    const script = document.createElement('script')
    script.src = 'https://cdn.jsdelivr.net/npm/pdfjs-dist@3.11.174/build/pdf.min.js'
    script.onload = () => {
      pdfJsLib = (window as any).pdfjsLib
      if (!pdfJsLib) return reject(new Error('pdfjsLib not found on window'))
      pdfJsLib.GlobalWorkerOptions.workerSrc = 'https://cdn.jsdelivr.net/npm/pdfjs-dist@3.11.174/build/pdf.worker.min.js'
      resolve(pdfJsLib)
    }
    script.onerror = () => reject(new Error('Failed to load pdf.js'))
    document.head.appendChild(script)
  })
}

const generateThumbnail = async (bytes: Uint8Array): Promise<string | null> => {
  try {
    const lib = await ensurePdfJs()
    const doc = await lib.getDocument({ data: bytes.slice() }).promise
    const page = await doc.getPage(1)
    const viewport = page.getViewport({ scale: 1.0 })
    // Render at up to 300px wide for crisp thumbnails
    const scale = Math.min(300 / viewport.width, 400 / viewport.height, 1.0)
    const thumbViewport = page.getViewport({ scale })
    const canvas = document.createElement('canvas')
    canvas.width = thumbViewport.width
    canvas.height = thumbViewport.height
    const ctx = canvas.getContext('2d')
    if (!ctx) return null
    await page.render({ canvasContext: ctx, viewport: thumbViewport }).promise
    const dataUrl = canvas.toDataURL('image/png')
    doc.destroy()
    return dataUrl
  } catch (e) {
    console.warn('Thumbnail generation failed:', e)
    return null
  }
}

// ── Output size estimation ──
const estimateOutputSize = async () => {
  if (pdfFiles.value.length === 0) {
    outputSize.value = null
    return
  }
  // Ensure WASM is loaded before estimating
  try {
    await ensureWasm()
  } catch {
    outputSize.value = null
    return
  }
  if (!wasmMergeAndCompress) {
    outputSize.value = null
    return
  }
  estimating.value = true
  try {
    const bytesArray = pdfFiles.value.map(f => f.bytes)
    const result = wasmMergeAndCompress(bytesArray, compressionLevel.value)
    outputSize.value = result.length
  } catch {
    outputSize.value = null
  } finally {
    estimating.value = false
  }
}

const scheduleEstimate = () => {
  if (estimateTimer) clearTimeout(estimateTimer)
  estimateTimer = setTimeout(() => estimateOutputSize(), 200)
}

watch(compressionLevel, scheduleEstimate)
watch(pdfFiles, scheduleEstimate, { deep: true })

const addFiles = async (files: FileList | File[]) => {
  statusMessage.value = 'Loading PDFs...'
  try {
    await ensureWasm()
  } catch (e) {
    statusMessage.value = `Failed to load WASM module: ${e}`
    return
  }

  for (const file of Array.from(files)) {
    if (!file.name.toLowerCase().endsWith('.pdf')) {
      continue
    }
    try {
      const bytes = await readFileAsUint8(file)
      let pageCount = 0
      try {
        if (wasmGetPageCount) {
          pageCount = wasmGetPageCount(bytes)
        }
      } catch (e) {
        console.warn('Failed to get page count for', file.name, e)
      }
      pdfFiles.value.push({
        id: generateId(),
        name: file.name,
        size: bytes.length,
        pageCount,
        bytes,
        thumbnail: null,
      })
      // Generate thumbnail in the background
      const idx = pdfFiles.value.length - 1
      generateThumbnail(bytes).then(thumb => {
        if (pdfFiles.value[idx]) pdfFiles.value[idx].thumbnail = thumb
      })
    } catch (e) {
      console.error('Failed to read file', file.name, e)
    }
  }

  const count = pdfFiles.value.length
  statusMessage.value = count > 0
    ? `${count} PDF${count === 1 ? '' : 's'} loaded. Drag to reorder, then export.`
    : 'No valid PDF files found.'
}

const onFileInput = async (event: Event) => {
  const input = event.target as HTMLInputElement
  if (input.files && input.files.length > 0) {
    await addFiles(input.files)
    input.value = '' // reset so same file can be re-added
  }
}

const onDrop = async (event: DragEvent) => {
  event.preventDefault()
  dragOverIndex.value = null
  if (event.dataTransfer?.files && event.dataTransfer.files.length > 0) {
    await addFiles(event.dataTransfer.files)
  }
}

const onDragOver = (event: DragEvent) => {
  event.preventDefault()
}

const removeFile = (index: number) => {
  pdfFiles.value.splice(index, 1)
  if (pdfFiles.value.length === 0) {
    statusMessage.value = 'Upload PDF files to get started.'
    showExportPanel.value = false
    outputSize.value = null
  }
}

const clearAll = () => {
  pdfFiles.value = []
  statusMessage.value = 'Upload PDF files to get started.'
  showExportPanel.value = false
  outputSize.value = null
}

// ── Drag-to-reorder ──
const onItemDragStart = (event: DragEvent, index: number) => {
  dragIndex.value = index
  if (event.dataTransfer) {
    event.dataTransfer.effectAllowed = 'move'
    event.dataTransfer.setData('text/plain', String(index))
  }
}

const onItemDragOver = (event: DragEvent, index: number) => {
  event.preventDefault()
  if (event.dataTransfer) event.dataTransfer.dropEffect = 'move'
  dragOverIndex.value = index
}

const onItemDragLeave = () => {
  dragOverIndex.value = null
}

const onItemDrop = (event: DragEvent, targetIndex: number) => {
  event.preventDefault()
  dragOverIndex.value = null
  const sourceIndex = dragIndex.value
  if (sourceIndex === null || sourceIndex === targetIndex) return
  const items = [...pdfFiles.value]
  const [moved] = items.splice(sourceIndex, 1)
  items.splice(targetIndex, 0, moved)
  pdfFiles.value = items
  dragIndex.value = null
}

const onItemDragEnd = () => {
  dragIndex.value = null
  dragOverIndex.value = null
}

// ── Export ──
const openExport = () => {
  if (pdfFiles.value.length === 0) {
    statusMessage.value = 'Add at least one PDF file first.'
    return
  }
  showExportPanel.value = true
}

const handleExport = async () => {
  if (pdfFiles.value.length === 0) return
  processing.value = true
  progressPercent.value = 10
  statusMessage.value = 'Preparing merge...'

  try {
    await ensureWasm()
    if (!wasmMergeAndCompress) throw new Error('PDF WASM module not loaded')

    progressPercent.value = 30
    statusMessage.value = 'Merging and compressing...'

    const bytesArray = pdfFiles.value.map(f => f.bytes)
    const result = wasmMergeAndCompress(bytesArray, compressionLevel.value)

    progressPercent.value = 90
    statusMessage.value = 'Preparing download...'

    // Download
    const blob = new Blob([result.buffer as ArrayBuffer], { type: 'application/pdf' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `merged-${new Date().toISOString().replace(/[:.]/g, '-')}.pdf`
    document.body.appendChild(a)
    a.click()
    a.remove()
    URL.revokeObjectURL(url)

    progressPercent.value = 100
    const totalPages = pdfFiles.value.reduce((sum, f) => sum + f.pageCount, 0)
    const totalInput = pdfFiles.value.reduce((sum, f) => sum + f.size, 0)
    statusMessage.value = `Done! Merged ${pdfFiles.value.length} PDF${pdfFiles.value.length > 1 ? 's' : ''} (${totalPages} pages). ${prettySize(totalInput)} → ${prettySize(result.length)}`
  } catch (e) {
    statusMessage.value = `Error: ${e instanceof Error ? e.message : String(e)}`
    console.error('PDF merge failed:', e)
  } finally {
    processing.value = false
    setTimeout(() => { progressPercent.value = 0 }, 600)
  }
}

const totalPages = computed(() => pdfFiles.value.reduce((s, f) => s + f.pageCount, 0))
const totalSize = computed(() => pdfFiles.value.reduce((s, f) => s + f.size, 0))

const exportButtonLabel = computed(() => {
  if (processing.value) return 'Merging...'
  if (estimating.value) return 'Merge & Export (estimating…)'
  if (outputSize.value !== null && pdfFiles.value.length > 0) return `Merge & Export (${prettySize(outputSize.value)})`
  return 'Merge & Export'
})
</script>

<template>
  <div class="min-h-[calc(100vh-48px)] bg-slate-950 text-slate-200 flex justify-center p-3.5">
    <div class="w-full max-w-6xl flex flex-col gap-3.5">

      <!-- Hero Header -->
      <header class="bg-slate-900/90 border border-white/10 rounded-xl p-4 backdrop-blur-md shadow-lg">
        <p class="text-xs uppercase tracking-widest text-slate-500 font-semibold mb-1">Local only</p>
        <h1 class="text-2xl md:text-3xl font-bold tracking-tight -mt-0.5 mb-2">PDF Merger & Compressor</h1>
        <p class="text-slate-400 text-sm leading-relaxed">Upload multiple PDFs, drag to reorder, adjust compression, and export a single merged file. Everything happens locally in your browser.</p>
      </header>

      <!-- Main Content -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-3.5">

        <!-- Upload + File List (left 2 cols) -->
        <section class="lg:col-span-2 bg-slate-900/90 border border-white/10 rounded-xl p-5 backdrop-blur-md shadow-lg flex flex-col gap-4">

          <!-- Upload Area -->
          <label
            for="pdf-input"
            class="border-2 border-dashed border-white/10 rounded-xl p-8 bg-gradient-to-br from-violet-500/5 to-cyan-400/3 cursor-pointer hover:border-violet-500/40 transition-all duration-200 flex flex-col items-center justify-center text-center gap-2"
            @drop="onDrop"
            @dragover="onDragOver"
          >
            <div class="w-12 h-12 rounded-full bg-violet-500/10 flex items-center justify-center mb-1">
              <svg class="w-6 h-6 text-violet-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4" />
              </svg>
            </div>
            <span class="font-semibold text-sm">Drop PDF files here</span>
            <span class="text-slate-500 text-xs">or click to browse</span>
            <input id="pdf-input" type="file" accept=".pdf,application/pdf" multiple class="hidden" @change="onFileInput">
          </label>

          <!-- File List -->
          <div v-if="pdfFiles.length > 0" class="flex flex-col gap-1.5">
            <div class="flex items-center justify-between mb-1">
              <p class="text-xs uppercase tracking-widest text-slate-500 font-semibold">
                {{ pdfFiles.length }} file{{ pdfFiles.length > 1 ? 's' : '' }} • {{ totalPages }} page{{ totalPages !== 1 ? 's' : '' }} • {{ prettySize(totalSize) }}
              </p>
              <button @click="clearAll" class="text-xs text-slate-500 hover:text-rose-400 transition-colors">Clear all</button>
            </div>

            <div
              v-for="(file, index) in pdfFiles"
              :key="file.id"
              class="group flex items-start gap-3 rounded-lg px-3 py-3 border transition-all duration-150 cursor-grab active:cursor-grabbing"
              :class="dragOverIndex === index ? 'border-violet-500/50 bg-violet-500/5' : 'border-white/5 bg-white/[0.02] hover:bg-white/[0.04]'"
              draggable="true"
              @dragstart="onItemDragStart($event, index)"
              @dragover="onItemDragOver($event, index)"
              @dragleave="onItemDragLeave"
              @drop="onItemDrop($event, index)"
              @dragend="onItemDragEnd"
            >
              <!-- Drag Handle -->
              <span class="text-slate-600 group-hover:text-slate-400 transition-colors select-none text-lg leading-none mt-6" aria-hidden="true">⠿</span>

              <!-- Thumbnail / File Icon -->
              <div class="w-24 h-32 shrink-0 rounded-lg bg-white/5 border border-white/10 flex items-center justify-center overflow-hidden shadow-sm">
                <img v-if="file.thumbnail" :src="file.thumbnail" :alt="file.name" class="w-full h-full object-contain bg-white" />
                <svg v-else class="w-8 h-8 text-rose-400/60" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m2.25 0H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z" />
                </svg>
              </div>

              <!-- File Info -->
              <div class="min-w-0 flex-1 pt-1">
                <p class="text-sm font-medium truncate">{{ file.name }}</p>
                <p class="text-xs text-slate-500 mt-0.5">{{ file.pageCount }} page{{ file.pageCount !== 1 ? 's' : '' }} • {{ prettySize(file.size) }}</p>
              </div>

              <!-- Order Badge -->
              <span class="text-xs font-mono text-slate-600 bg-white/5 rounded px-1.5 py-0.5 mt-1">{{ index + 1 }}</span>

              <!-- Remove Button -->
              <button
                @click.stop="removeFile(index)"
                class="shrink-0 w-7 h-7 rounded-md flex items-center justify-center text-slate-600 hover:text-rose-400 hover:bg-rose-400/10 transition-all opacity-0 group-hover:opacity-100 mt-1"
                title="Remove"
              >✕</button>
            </div>
          </div>

          <!-- Empty State -->
          <div v-else class="py-8 text-center text-slate-500 text-sm">
            No PDF files added yet. Upload or drop files above.
          </div>
        </section>

        <!-- Export Panel (right col) -->
        <section class="bg-slate-900/90 border border-white/10 rounded-xl p-5 backdrop-blur-md shadow-lg flex flex-col gap-4">
          <div class="inline-flex items-center gap-2 uppercase tracking-wide text-xs text-slate-500 before:content-[''] before:w-4 before:h-0.5 before:bg-gradient-to-r before:from-violet-500 before:to-cyan-400 before:rounded">
            Export Settings
          </div>

          <!-- Compression Slider -->
          <div class="flex flex-col gap-2">
            <label class="text-xs uppercase tracking-widest text-slate-500 font-semibold" for="compression-slider">
              Compression Level
            </label>
            <input
              id="compression-slider"
              type="range"
              min="10"
              max="100"
              step="5"
              v-model.number="compressionLevel"
              class="w-full h-2 rounded-full appearance-none cursor-pointer accent-violet-500"
              style="background: linear-gradient(to right, #8b5cf6, #06b6d4)"
            >
            <div class="flex justify-between text-xs text-slate-500">
              <span>Smaller file</span>
              <span class="font-mono text-violet-400 font-semibold">{{ compressionLevel }}%</span>
              <span>Higher quality</span>
            </div>
          </div>

          <!-- Summary -->
          <div v-if="pdfFiles.length > 0" class="bg-white/[0.02] rounded-lg border border-white/5 p-3 text-sm space-y-1.5">
            <div class="flex justify-between">
              <span class="text-slate-500">Files</span>
              <span class="font-medium">{{ pdfFiles.length }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-slate-500">Total pages</span>
              <span class="font-medium">{{ totalPages }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-slate-500">Input size</span>
              <span class="font-medium">{{ prettySize(totalSize) }}</span>
            </div>
          </div>

          <!-- Export Button -->
          <button
            id="exportBtn"
            :disabled="pdfFiles.length === 0 || processing"
            @click="handleExport"
            class="w-full bg-gradient-to-r from-violet-500 to-cyan-400 text-slate-950 font-bold py-3 px-4 rounded-xl transition-all duration-150 hover:-translate-y-0.5 hover:shadow-lg disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ exportButtonLabel }}
          </button>

          <!-- Progress -->
          <div class="w-full h-2.5 rounded-full bg-white/5 border border-white/10 overflow-hidden transition-opacity duration-300" :class="progressPercent === 0 ? 'opacity-0' : 'opacity-100'">
            <div class="h-full bg-gradient-to-r from-violet-500 to-cyan-400 transition-all duration-200 rounded-full" :style="{ width: `${progressPercent}%` }" />
          </div>

          <!-- Status Message -->
          <div class="font-mono text-xs text-slate-500 min-h-5 leading-relaxed">{{ statusMessage }}</div>

          <!-- Info Box -->
          <div class="mt-auto bg-violet-500/5 border border-violet-500/10 rounded-lg p-3 text-xs text-slate-400 leading-relaxed">
            <p class="font-semibold text-violet-400 mb-1">🔒 100% Private</p>
            <p>Your PDFs never leave your device. All processing happens locally using Rust + WebAssembly.</p>
          </div>
        </section>

      </div>
    </div>
  </div>
</template>
