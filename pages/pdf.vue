<script setup lang="ts">
definePageMeta({ layout: 'tools' })

useHead({
  title: 'PDF Merger & Compressor | Ash Tools',
  meta: [
    { name: 'description', content: 'Free online PDF Merger and Compressor. Combine multiple PDF files, reorder pages, and compress the output. Runs fully offline in your browser using Rust + WebAssembly.' },
    { name: 'keywords', content: 'pdf merger, pdf compressor, combine pdf, merge pdf online, pdf join, compress pdf, webassembly pdf, offline pdf tool' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/pdf/' },
    { property: 'og:title', content: 'PDF Merger & Compressor | Ash Tools' },
    { property: 'og:description', content: 'Merge and compress PDF files locally in your browser. No uploads, no tracking.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/pdf/' }],
})

useToolSchema({
  name: 'PDF Merger & Compressor',
  description: 'Merge, reorder, and compress PDF documents locally in your browser.',
  url: 'https://ash-tools.store/pdf/',
  category: 'BusinessApplication',
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
  img_to_pdf: (bytes: Uint8Array, width: number, height: number) => Uint8Array
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
let wasmImgToPdf: ((bytes: Uint8Array, width: number, height: number) => Uint8Array) | null = null

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
        wasmImgToPdf = module.img_to_pdf as (bytes: Uint8Array, width: number, height: number) => Uint8Array
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
  statusMessage.value = 'Loading files...'
  try {
    await ensureWasm()
  } catch (e) {
    statusMessage.value = `Failed to load WASM module: ${e}`
    return
  }

  for (const file of Array.from(files)) {
    const isPdf = file.name.toLowerCase().endsWith('.pdf')
    const isImage = /\.(jpe?g|png|webp)$/i.test(file.name)

    if (!isPdf && !isImage) {
      continue
    }

    try {
      let bytes = await readFileAsUint8(file)
      let pageCount = 0

      if (isImage) {
        statusMessage.value = `Converting ${file.name}...`
        // 1. Get dimensions and convert to JPEG
        const img = new Image()
        const url = URL.createObjectURL(file)
        await new Promise((resolve, reject) => {
          img.onload = resolve
          img.onerror = reject
          img.src = url
        })
        const width = img.naturalWidth
        const height = img.naturalHeight

        const canvas = document.createElement('canvas')
        canvas.width = width
        canvas.height = height
        const ctx = canvas.getContext('2d')
        if (!ctx) throw new Error('Could not create canvas context')
        ctx.drawImage(img, 0, 0)

        const jpegBlob = await new Promise<Blob>((resolve) => {
          canvas.toBlob((b) => resolve(b!), 'image/jpeg', 0.95)
        })
        const jpegBytes = new Uint8Array(await jpegBlob.arrayBuffer())

        // 2. Convert JPEG to PDF using WASM
        if (wasmImgToPdf) {
          bytes = wasmImgToPdf(jpegBytes, width, height)
          pageCount = 1
        }
        URL.revokeObjectURL(url)
      } else {
        // PDF logic
        try {
          if (wasmGetPageCount) {
            pageCount = wasmGetPageCount(bytes)
          }
        } catch (e) {
          console.warn('Failed to get page count for', file.name, e)
        }
      }

      pdfFiles.value.push({
        id: generateId(),
        name: isImage ? `${file.name}.pdf` : file.name,
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
      console.error('Failed to process file', file.name, e)
    }
  }

  const count = pdfFiles.value.length
  statusMessage.value = count > 0
    ? `${count} file${count === 1 ? '' : 's'} loaded. Drag to reorder, then export.`
    : 'No valid files found.'
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
  <div class="min-h-screen bg-[var(--surface-0)] text-[var(--ink-1)] flex flex-col">
    <div class="flex-1 flex flex-col md:flex-row">

      <!-- Left rail: navigation + primary actions -->
      <aside class="order-2 md:order-1 shrink-0 border-t md:border-t-0 md:border-r md:w-[76px] w-full" style="background: var(--surface-1); border-color: var(--border)">
        <div class="flex md:flex-col items-center gap-1 p-2 md:py-4 overflow-x-auto md:overflow-visible">
          <NuxtLink to="/" class="rail-btn shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Back to home" aria-label="Back to home">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6"><path stroke-linecap="round" stroke-linejoin="round" d="M11.5 4L6 9.5l5.5 5.5" /></svg>
            <span class="text-[10px] font-medium">Home</span>
          </NuxtLink>
          <label for="pdf-input" class="rail-btn cursor-pointer shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Open files" aria-label="Open files">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M3 6a1 1 0 011-1h3l1.5 2H16a1 1 0 011 1v7a1 1 0 01-1 1H4a1 1 0 01-1-1V6z" /><path stroke-linecap="round" d="M3 8h14" /></svg>
            <span class="text-[10px] font-medium">Open</span>
          </label>
          <button v-if="pdfFiles.length > 0" @click="clearAll" class="rail-btn shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Clear all" aria-label="Clear all">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18"></path><path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"></path></svg>
            <span class="text-[10px] font-medium">Clear</span>
          </button>
        </div>
      </aside>

      <!-- Main content -->
      <div class="order-1 md:order-2 flex-1 flex justify-center p-3.5 min-w-0">
      <div class="w-full max-w-6xl flex flex-col gap-3.5">

      <div class="flex-1 min-w-0 flex flex-col gap-3.5">

      <!-- Title bar -->
      <header class="ui-panel p-4 flex items-center justify-between gap-3">
        <div class="flex items-center gap-2">
          <svg class="h-4 w-4 text-[var(--ink-3)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m2.25 0H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z" />
          </svg>
          <h1 class="text-sm font-bold text-[var(--ink-1)]">PDF Merger & Compressor</h1>
        </div>
        <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
      </header>

      <!-- Main Content -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-3.5">

        <!-- Upload + File List (left 2 cols) -->
        <section
          class="lg:col-span-2 ui-panel p-5 flex flex-col gap-4">
          <div class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Documents</div>

          <!-- Upload Area -->
          <label for="pdf-input"
            class="ui-dropzone p-8 flex flex-col items-center justify-center text-center gap-2"
            @drop="onDrop" @dragover="onDragOver">
            <div class="w-12 h-12 rounded-none flex items-center justify-center mb-1" style="background: var(--accent-soft); border: 1px solid var(--border)">
              <svg class="w-6 h-6" style="color: var(--accent)" fill="none" viewBox="0 0 24 24" stroke="currentColor"
                stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4" />
              </svg>
            </div>
            <span class="font-semibold text-sm text-[var(--ink-1)]">Drop PDF or Image files here</span>
            <span class="text-[var(--ink-3)] text-xs">or click to browse</span>
            <input id="pdf-input" type="file" accept=".pdf,application/pdf,image/*" multiple class="hidden"
              @change="onFileInput">
          </label>

          <!-- File List -->
          <div v-if="pdfFiles.length > 0" class="flex flex-col gap-1.5">
            <div class="flex items-center justify-between mb-1">
              <p class="text-xs text-[var(--ink-3)] font-semibold">
                {{ pdfFiles.length }} file{{ pdfFiles.length > 1 ? 's' : '' }} · {{ totalPages }} page{{ totalPages !==
                  1 ? 's' : '' }} · {{ prettySize(totalSize) }}
              </p>
              <button @click="clearAll" class="text-xs text-[var(--ink-3)] hover:text-[var(--danger)] transition-colors">Clear
                all</button>
            </div>

            <div v-for="(file, index) in pdfFiles" :key="file.id"
              class="group flex items-start gap-3 rounded-[var(--radius-sm)] px-3 py-3 border transition-all duration-150 cursor-grab active:cursor-grabbing"
              :class="dragOverIndex === index ? 'border-[var(--accent)]' : 'border-[var(--border)]'"
              style="background: var(--surface-1)"
              draggable="true" @dragstart="onItemDragStart($event, index)" @dragover="onItemDragOver($event, index)"
              @dragleave="onItemDragLeave" @drop="onItemDrop($event, index)" @dragend="onItemDragEnd">
              <!-- Drag Handle -->
              <span
                class="text-[var(--ink-3)] transition-colors select-none text-lg leading-none mt-6"
                aria-hidden="true">⠿</span>

              <!-- Thumbnail / File Icon -->
              <div
                class="w-24 h-32 shrink-0 rounded-[var(--radius-sm)] flex items-center justify-center overflow-hidden"
                style="background: var(--surface-2); border: 1px solid var(--border)">
                <img v-if="file.thumbnail" :src="file.thumbnail" :alt="file.name"
                  class="w-full h-full object-contain bg-white" />
                <svg v-else class="w-8 h-8 text-[var(--ink-3)]" fill="none" viewBox="0 0 24 24" stroke="currentColor"
                  stroke-width="1.5">
                  <path stroke-linecap="round" stroke-linejoin="round"
                    d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m2.25 0H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z" />
                </svg>
              </div>

              <!-- File Info -->
              <div class="min-w-0 flex-1 pt-1">
                <p class="text-sm font-mono truncate text-[var(--ink-1)]">{{ file.name }}</p>
                <p class="text-xs font-mono text-[var(--ink-3)] mt-0.5">{{ file.pageCount }} page{{ file.pageCount !== 1 ? 's' : '' }}
                  · {{ prettySize(file.size) }}</p>
              </div>

              <!-- Order Badge -->
              <span class="ui-badge font-mono mt-1">{{ index + 1 }}</span>

              <!-- Remove Button -->
              <button @click.stop="removeFile(index)"
                class="shrink-0 w-7 h-7 rounded-[var(--radius-sm)] flex items-center justify-center text-[var(--ink-2)] hover:text-[var(--danger)] hover:bg-[var(--danger-soft)] transition-all opacity-0 group-hover:opacity-100 mt-1"
                title="Remove">✕</button>
            </div>
          </div>

          <!-- Empty State -->
          <div v-else class="py-8 text-center text-[var(--ink-3)] text-sm">
            No PDF files added yet. Upload or drop files above.
          </div>
        </section>

        <!-- Export Panel (right col) -->
        <section
          class="ui-panel p-5 flex flex-col gap-4">
          <div class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">
            Export Settings
          </div>

          <!-- Compression Slider -->
          <div class="flex flex-col gap-2">
            <label class="text-xs font-semibold text-[var(--ink-2)]" for="compression-slider">
              Compression Level
            </label>
            <input id="compression-slider" type="range" min="10" max="100" step="5" v-model.number="compressionLevel"
              class="ui-range w-full h-2 rounded-none appearance-none cursor-pointer"
              style="background: var(--surface-3); border: 1px solid var(--border)">
            <div class="flex justify-between text-xs text-[var(--ink-3)]">
              <span>Smaller file</span>
              <span class="font-mono text-[var(--ink-1)] font-semibold">{{ compressionLevel }}%</span>
              <span>Higher quality</span>
            </div>
          </div>

          <!-- Summary -->
          <div v-if="pdfFiles.length > 0"
            class="rounded-[var(--radius-sm)] p-3 text-sm space-y-1.5"
            style="background: var(--surface-2); border: 1px solid var(--border)">
            <div class="flex justify-between">
              <span class="text-[var(--ink-3)]">Files</span>
              <span class="font-medium text-[var(--ink-1)]">{{ pdfFiles.length }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-[var(--ink-3)]">Total pages</span>
              <span class="font-medium text-[var(--ink-1)]">{{ totalPages }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-[var(--ink-3)]">Input size</span>
              <span class="font-medium text-[var(--ink-1)]">{{ prettySize(totalSize) }}</span>
            </div>
          </div>

          <!-- Export Button -->
          <button id="exportBtn" :disabled="pdfFiles.length === 0 || processing" @click="handleExport"
            class="ui-button w-full py-3">
            {{ exportButtonLabel }}
          </button>

          <!-- Progress -->
          <div
            class="w-full h-2 rounded-none overflow-hidden transition-opacity duration-300"
            style="background: var(--surface-2); border: 1px solid var(--border)"
            :class="progressPercent === 0 ? 'opacity-0' : 'opacity-100'">
            <div class="h-full transition-all duration-200 rounded-none" style="background: var(--accent)"
              :style="{ width: `${progressPercent}%` }" />
          </div>

          <!-- Status Message -->
          <div class="font-mono text-xs text-[var(--ink-3)] min-h-5 leading-relaxed">{{ statusMessage }}</div>

          <!-- Info Box -->
          <div
            class="mt-auto rounded-[var(--radius-sm)] p-3 text-xs leading-relaxed"
            style="background: var(--accent-soft); border: 1px solid var(--border)">
            <p class="font-semibold mb-1" style="color: var(--accent-hover)">100% Private</p>
            <p class="text-[var(--ink-2)]">Your PDFs never leave your device. All processing happens locally using Rust + WebAssembly.</p>
          </div>
        </section>

      </div>
      </div>
      </div>
      </div>
    </div>

    <!-- Bottom status strip -->
    <div class="shrink-0 border-t px-5 py-2 flex items-center gap-4 text-xs font-mono" style="border-color: var(--border); background: var(--surface-1); color: var(--ink-3)">
      <span class="truncate">{{ statusMessage }}</span>
      <span v-if="pdfFiles.length > 0" class="ml-auto shrink-0">{{ pdfFiles.length }} file{{ pdfFiles.length > 1 ? 's' : '' }} · {{ totalPages }} page{{ totalPages !== 1 ? 's' : '' }} · {{ prettySize(totalSize) }}</span>
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
