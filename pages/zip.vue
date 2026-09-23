<script setup lang="ts">
definePageMeta({ layout: 'tools' })

useHead({
  title: 'Free File Compress | Local ZIP in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser ZIP compressor. Select folders or files and create archives locally on your device. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/zip/' },
    { property: 'og:title', content: 'Free File Compress | Local ZIP in your browser' },
    { property: 'og:description', content: 'Zip folders and files privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/zip/' }],
})

useToolSchema({
  name: 'ZIP Archiver',
  description: 'Create and inspect ZIP archives locally in your browser.',
  url: 'https://ash-tools.store/zip/',
  category: 'UtilitiesApplication',
})

const folderFiles = ref<File[]>([])
const fileFiles = ref<File[]>([])
const compressing = ref(false)
const createStatus = ref('Awaiting files.')
const createProgress = ref(0)

const zipInputFile = ref<File | null>(null)
const openStatus = ref('Drop a ZIP file to inspect its contents.')

type ZipEntry = {
  name: string
  size: number
  isImage: boolean
  mime: string
  bytes: Uint8Array
  previewUrl: string | null
}

type WasmZipEntry = {
  name: string
  bytes: Uint8Array
}

type WasmCompressorModule = {
  default: () => Promise<void>
  zip_files: (names: string[], contents: Uint8Array[]) => Uint8Array
  unzip_files: (zipBytes: Uint8Array) => WasmZipEntry[]
}

const zipEntries = ref<ZipEntry[]>([])

let wasmReadyPromise: Promise<void> | null = null
let wasmZipFiles: ((names: string[], contents: Uint8Array[]) => Uint8Array) | null = null
let wasmUnzipFiles: ((zipBytes: Uint8Array) => WasmZipEntry[]) | null = null

const acceptedImageExtensions = new Set([
  'png',
  'jpg',
  'jpeg',
  'gif',
  'webp',
  'svg',
  'bmp',
  'avif',
])

const acceptedVideoExtensions = new Set([
  'mp4',
  'webm',
  'ogg',
  'mov',
  'mkv',
])

const prettySize = (bytes: number) => {
  if (bytes < 1024) return `${bytes} B`
  const kb = bytes / 1024
  if (kb < 1024) return `${kb.toFixed(1)} KB`
  return `${(kb / 1024).toFixed(2)} MB`
}

const sanitizePath = (value: string) => {
  const trimmed = value.replace(/^[/\\]+/, '')
  return trimmed || 'file'
}

const selectedFiles = computed(() => {
  const merged = [...folderFiles.value, ...fileFiles.value]
  const deduped = new Map<string, File>()
  merged.forEach((file) => {
    const key = file.webkitRelativePath || file.name
    if (!deduped.has(key)) deduped.set(key, file)
  })
  return Array.from(deduped.values())
})

const selectedFilesSummary = computed(() => {
  const count = selectedFiles.value.length
  if (!count) return 'No files selected.'
  const size = selectedFiles.value.reduce((sum: number, file: File) => sum + file.size, 0)
  return `${count} file${count === 1 ? '' : 's'} selected • ${prettySize(size)} total`
})

const revokePreviews = () => {
  zipEntries.value.forEach((entry: ZipEntry) => {
    if (entry.previewUrl) URL.revokeObjectURL(entry.previewUrl)
  })
}

const toArrayBuffer = (bytes: Uint8Array): ArrayBuffer => Uint8Array.from(bytes).buffer

const readFileAsUint8 = async (file: File) => {
  const buffer = await file.arrayBuffer()
  return new Uint8Array(buffer)
}

const downloadBytes = (filename: string, bytes: Uint8Array, mime = 'application/octet-stream') => {
  const blob = new Blob([toArrayBuffer(bytes)], { type: mime })
  const url = URL.createObjectURL(blob)
  const anchor = document.createElement('a')
  anchor.href = url
  anchor.download = filename
  document.body.appendChild(anchor)
  anchor.click()
  anchor.remove()
  URL.revokeObjectURL(url)
}

const modalEntry = ref<ZipEntry | null>(null)
const modalUrl = ref<string | null>(null)
let modalCreatedUrl = false

const openEntry = (entry: ZipEntry) => {
  try {
    const mime = entry.mime || 'application/octet-stream'

    // Images: use existing preview URL if available, otherwise create one
    if (entry.isImage || mime.startsWith('image/')) {
      modalEntry.value = entry
      if (entry.previewUrl) {
        modalUrl.value = entry.previewUrl
        modalCreatedUrl = false
      } else {
        modalUrl.value = URL.createObjectURL(new Blob([toArrayBuffer(entry.bytes)], { type: mime }))
        modalCreatedUrl = true
      }
      return
    }

    // Videos: create an object URL and open in modal
    if (mime.startsWith('video/')) {
      modalEntry.value = entry
      modalUrl.value = URL.createObjectURL(new Blob([toArrayBuffer(entry.bytes)], { type: mime }))
      modalCreatedUrl = true
      return
    }

    // Fallback: open other files in a new tab
    const blob = new Blob([toArrayBuffer(entry.bytes)], { type: mime })
    const url = URL.createObjectURL(blob)
    const opened = window.open(url, '_blank')
    if (!opened) {
      const a = document.createElement('a')
      a.href = url
      a.target = '_blank'
      document.body.appendChild(a)
      a.click()
      a.remove()
    }
    setTimeout(() => {
      try {
        URL.revokeObjectURL(url)
      } catch (e) {
        // noop
      }
    }, 60000)
  } catch (e) {
    openStatus.value = `Error opening file: ${e instanceof Error ? e.message : String(e)}`
  }
}

const closeModal = () => {
  if (modalUrl.value && modalCreatedUrl) {
    try {
      URL.revokeObjectURL(modalUrl.value)
    } catch (e) {
      // noop
    }
  }
  modalUrl.value = null
  modalEntry.value = null
  modalCreatedUrl = false
}

const ensureWasm = async () => {
  if (!wasmReadyPromise) {
    wasmReadyPromise = ((0, eval)('import("/zip/pkg/compressor.js")') as Promise<WasmCompressorModule>)
      .then(async (module: WasmCompressorModule) => {
        await module.default()
        wasmZipFiles = module.zip_files as (names: string[], contents: Uint8Array[]) => Uint8Array
        wasmUnzipFiles = module.unzip_files as (zipBytes: Uint8Array) => WasmZipEntry[]
      })
      .catch((error) => {
        wasmReadyPromise = null
        throw error
      })
  }

  await wasmReadyPromise
}

const onFolderChange = (event: Event) => {
  const input = event.target as HTMLInputElement
  folderFiles.value = input.files ? Array.from(input.files) : []
  createStatus.value = ''
}

const onFileChange = (event: Event) => {
  const input = event.target as HTMLInputElement
  fileFiles.value = input.files ? Array.from(input.files) : []
  createStatus.value = ''
}

const buildZipFromSelectedFiles = async () => {
  if (!selectedFiles.value.length) {
    createStatus.value = 'Add one or more files first.'
    return
  }

  compressing.value = true
  createProgress.value = 0

  try {
    await ensureWasm()

    if (!wasmZipFiles) {
      throw new Error('ZIP WebAssembly module is not ready')
    }

    const names: string[] = []
    const contents: Uint8Array[] = []

    for (let index = 0; index < selectedFiles.value.length; index += 1) {
      const file = selectedFiles.value[index]
      createStatus.value = `Reading files (${index + 1}/${selectedFiles.value.length})`
      createProgress.value = Math.round(((index + 0.5) / selectedFiles.value.length) * 60)
      const bytes = await readFileAsUint8(file)
      const path = sanitizePath(file.webkitRelativePath || file.name)
      names.push(path)
      contents.push(bytes)
    }

    createStatus.value = 'Compressing...'
    createProgress.value = 85

    const zipped = wasmZipFiles(names, contents)

    const filename = `bundle-${new Date().toISOString().replace(/[:.]/g, '-')}.zip`
    downloadBytes(filename, zipped, 'application/zip')
    createProgress.value = 100
    createStatus.value = `Done. Downloaded ${filename}.`
  } catch (error) {
    createStatus.value = `Error: ${error instanceof Error ? error.message : String(error)}`
  } finally {
    compressing.value = false
    setTimeout(() => {
      createProgress.value = 0
    }, 500)
  }
}

const getEntryExtension = (name: string) => {
  const cleanName = name.split('/').pop() || name
  const parts = cleanName.split('.')
  return parts.length > 1 ? parts.pop()!.toLowerCase() : ''
}

const openZipFile = async (file: File) => {
  if (!file.name.toLowerCase().endsWith('.zip')) {
    openStatus.value = 'Please select a valid .zip file.'
    return
  }

  revokePreviews()
  zipEntries.value = []
  zipInputFile.value = file

  try {
    await ensureWasm()

    if (!wasmUnzipFiles) {
      throw new Error('UNZIP WebAssembly module is not ready')
    }

    openStatus.value = 'Reading ZIP file...'
    const bytes = await readFileAsUint8(file)
    const unzipped = wasmUnzipFiles(bytes)

    const entries: ZipEntry[] = unzipped
      .filter((entry: WasmZipEntry) => !entry.name.endsWith('/'))
      .map((entry: WasmZipEntry) => {
        const data = entry.bytes
        const name = entry.name
        const extension = getEntryExtension(name)
        const isImage = acceptedImageExtensions.has(extension)
        const isVideo = acceptedVideoExtensions.has(extension)

        let mime = 'application/octet-stream'
        if (isImage) {
          mime = extension === 'svg' ? 'image/svg+xml' : `image/${extension === 'jpg' ? 'jpeg' : extension}`
        } else if (isVideo) {
          if (extension === 'mp4') mime = 'video/mp4'
          else if (extension === 'webm') mime = 'video/webm'
          else if (extension === 'ogg') mime = 'video/ogg'
          else if (extension === 'mov') mime = 'video/quicktime'
          else if (extension === 'mkv') mime = 'video/x-matroska'
        }

        return {
          name,
          size: data.length,
          isImage,
          mime,
          bytes: data,
          previewUrl: isImage ? URL.createObjectURL(new Blob([toArrayBuffer(data)], { type: mime })) : null,
        }
      })
      .sort((a, b) => a.name.localeCompare(b.name))

    zipEntries.value = entries
    openStatus.value = entries.length
      ? `Found ${entries.length} file${entries.length === 1 ? '' : 's'} in ${file.name}.`
      : 'No files found in this ZIP.'
  } catch (error) {
    openStatus.value = `Error: ${error instanceof Error ? error.message : String(error)}`
  }
}

const onZipFileChange = async (event: Event) => {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  await openZipFile(file)
}

const onZipDrop = async (event: DragEvent) => {
  event.preventDefault()
  const file = event.dataTransfer?.files?.[0]
  if (!file) return
  await openZipFile(file)
}

const onZipDragOver = (event: DragEvent) => {
  event.preventDefault()
}

onMounted(() => {
  createStatus.value = 'Awaiting files.'
  openStatus.value = 'Drop a ZIP file to inspect its contents.'
})

onUnmounted(() => {
  revokePreviews()
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
          <label for="folder-input" class="rail-btn cursor-pointer shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Choose folder" aria-label="Choose folder">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M3 6a1 1 0 011-1h3l1.5 2H16a1 1 0 011 1v7a1 1 0 01-1 1H4a1 1 0 01-1-1V6z" /></svg>
            <span class="text-[10px] font-medium">Folder</span>
          </label>
          <label for="file-input" class="rail-btn cursor-pointer shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Choose files" aria-label="Choose files">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M6 3h5l3 3v11a1 1 0 01-1 1H6a1 1 0 01-1-1V4a1 1 0 011-1z" /><path stroke-linecap="round" stroke-linejoin="round" d="M11 3v3h3" /></svg>
            <span class="text-[10px] font-medium">Files</span>
          </label>
          <label for="zip-input" class="rail-btn cursor-pointer shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Open a ZIP" aria-label="Open a ZIP">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M4 7a2 2 0 012-2h2l1.5 2H14a2 2 0 012 2v5a2 2 0 01-2 2H6a2 2 0 01-2-2V7z" /><path stroke-linecap="round" d="M10 9v5M8.5 10.5L10 9l1.5 1.5" /></svg>
            <span class="text-[10px] font-medium">Open</span>
          </label>
        </div>
      </aside>

      <main class="order-1 md:order-2 flex-1 min-w-0 flex flex-col p-4 md:p-6">
      <div class="mx-auto w-full max-w-6xl flex flex-col gap-4">
      <div class="ui-panel p-4 flex items-center justify-between gap-3">
        <div class="flex items-center gap-2">
          <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M4 7a2 2 0 012-2h2l1.5 2H14a2 2 0 012 2v5a2 2 0 01-2 2H6a2 2 0 01-2-2V7z" /><path stroke-linecap="round" d="M10 9v5M8.5 10.5L10 9l1.5 1.5" /></svg>
          <h1 class="text-sm font-bold text-[var(--ink-1)]">ZIP Archiver</h1>
        </div>
        <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <section class="ui-panel flex flex-col min-h-0">
          <div class="flex items-center justify-between px-4 pt-3 pb-2">
            <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Create archive</span>
            <span class="text-xs font-mono text-[var(--ink-3)]">{{ selectedFiles.length }} queued</span>
          </div>

          <div class="px-4 pb-4 flex flex-col gap-3">
            <label for="folder-input" class="ui-dropzone p-4 flex justify-between items-center gap-2">
              <div class="flex items-center gap-3">
                <svg class="h-5 w-5 shrink-0 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M3 6a1 1 0 011-1h3l1.5 2H16a1 1 0 011 1v7a1 1 0 01-1 1H4a1 1 0 01-1-1V6z" /></svg>
                <div>
                  <div class="font-semibold text-sm text-[var(--ink-1)]">Choose folder</div>
                  <div class="text-[var(--ink-3)] text-xs">Nested files are preserved</div>
                </div>
              </div>
              <span class="ui-button-secondary !py-1.5 !px-3 text-xs whitespace-nowrap">Browse</span>
            </label>
            <input id="folder-input" type="file" webkitdirectory multiple class="hidden" @change="onFolderChange">

            <label for="file-input" class="ui-dropzone p-4 flex justify-between items-center gap-2">
              <div class="flex items-center gap-3">
                <svg class="h-5 w-5 shrink-0 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M6 3h5l3 3v11a1 1 0 01-1 1H6a1 1 0 01-1-1V4a1 1 0 011-1z" /><path stroke-linecap="round" stroke-linejoin="round" d="M11 3v3h3" /></svg>
                <div>
                  <div class="font-semibold text-sm text-[var(--ink-1)]">Choose files</div>
                  <div class="text-[var(--ink-3)] text-xs">Combines with folder selection</div>
                </div>
              </div>
              <span class="ui-button-secondary !py-1.5 !px-3 text-xs whitespace-nowrap">Browse</span>
            </label>
            <input id="file-input" type="file" multiple class="hidden" @change="onFileChange">

            <!-- Queued files list -->
            <div v-if="selectedFiles.length" class="rounded-[var(--radius-sm)] overflow-hidden" style="border: 1px solid var(--border)">
              <div class="max-h-40 overflow-auto divide-y" style="border-color: var(--border)">
                <div v-for="file in selectedFiles" :key="file.webkitRelativePath || file.name" class="flex items-center justify-between gap-2 px-3 py-1.5" style="background: var(--surface-1)">
                  <span class="text-xs font-mono truncate text-[var(--ink-2)]">{{ file.webkitRelativePath || file.name }}</span>
                  <span class="text-xs font-mono shrink-0 text-[var(--ink-3)]">{{ prettySize(file.size) }}</span>
                </div>
              </div>
            </div>

            <button
              class="ui-button w-full"
              :disabled="compressing"
              @click="buildZipFromSelectedFiles"
            >
              {{ compressing ? 'Compressing...' : 'Create ZIP' }}
            </button>

            <div class="w-full h-1.5 rounded-none overflow-hidden transition-opacity duration-150" style="background: var(--surface-2); border: 1px solid var(--border)" :class="createProgress === 0 ? 'opacity-0' : 'opacity-100'">
              <div class="h-full transition-all duration-150" style="background: var(--accent)" :style="{ width: `${createProgress}%` }" />
            </div>
            <div class="font-mono text-xs text-[var(--ink-3)] min-h-5">{{ createStatus || selectedFilesSummary }}</div>
          </div>
        </section>

        <section class="ui-panel flex flex-col min-h-0">
          <div class="flex items-center justify-between px-4 pt-3 pb-2">
            <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Inspect archive</span>
            <span v-if="zipEntries.length" class="text-xs font-mono text-[var(--ink-3)]">{{ zipEntries.length }} entries</span>
          </div>

          <div class="px-4 pb-4 flex flex-col gap-3">
            <label
              for="zip-input"
              class="ui-dropzone p-4 flex items-center justify-center gap-2"
              @drop="onZipDrop"
              @dragover="onZipDragOver"
            >
              <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M10 13V4m0 9l-3-3m3 3l3-3M4 16h12" /></svg>
              <span class="font-semibold text-sm text-[var(--ink-1)]">Drop ZIP or click to browse</span>
              <input id="zip-input" type="file" accept=".zip,application/zip" class="hidden" @change="onZipFileChange">
            </label>

            <div class="font-mono text-xs text-[var(--ink-3)] min-h-5">{{ openStatus }}</div>

            <div class="rounded-[var(--radius-sm)] overflow-hidden" style="border: 1px solid var(--border)">
              <div v-if="!zipEntries.length" class="p-4 text-sm text-[var(--ink-3)]" style="background: var(--surface-1)">No archive opened yet.</div>
              <ul v-else class="max-h-96 overflow-auto divide-y" style="border-color: var(--border)">
                <li v-for="entry in zipEntries" :key="entry.name" class="p-2.5 flex items-center justify-between gap-3" style="background: var(--surface-1)">
                  <div class="min-w-0 flex items-center gap-3">
                    <img
                      v-if="entry.isImage && entry.previewUrl"
                      :src="entry.previewUrl"
                      :alt="entry.name"
                      class="w-9 h-9 rounded-[var(--radius-sm)] object-cover shrink-0"
                      style="border: 1px solid var(--border)"
                    >
                    <svg v-else class="w-9 h-9 shrink-0 p-2 rounded-[var(--radius-sm)] text-[var(--ink-3)]" style="background: var(--surface-2)" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M6 3h5l3 3v11a1 1 0 01-1 1H6a1 1 0 01-1-1V4a1 1 0 011-1z" /></svg>
                    <div class="min-w-0">
                      <p class="text-xs font-mono truncate text-[var(--ink-1)]">{{ entry.name }}</p>
                      <p class="text-xs font-mono text-[var(--ink-3)]">{{ prettySize(entry.size) }}</p>
                    </div>
                  </div>
                  <div class="flex items-center gap-1.5 shrink-0">
                    <button
                      class="ui-button-secondary px-2.5 py-1 text-xs"
                      @click="openEntry(entry)"
                    >
                      Open
                    </button>
                    <button
                      class="ui-button px-2.5 py-1 text-xs"
                      @click="downloadBytes(entry.name.split('/').pop() || entry.name, entry.bytes, entry.mime)"
                    >
                      Save
                    </button>
                  </div>
                </li>
              </ul>
            </div>
          </div>
        </section>
      </div>
      </div>
    </main>
    </div>

    <!-- Bottom status strip -->
    <div class="shrink-0 border-t px-5 py-2 flex items-center gap-4 text-xs font-mono" style="border-color: var(--border); background: var(--surface-1); color: var(--ink-3)">
      <span class="truncate">{{ createStatus || selectedFilesSummary }}</span>
      <span class="truncate">{{ openStatus }}</span>
      <span v-if="zipEntries.length" class="ml-auto shrink-0">{{ zipEntries.length }} entries</span>
    </div>

      <div v-if="modalEntry" class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4">
        <div class="ui-panel max-w-4xl w-full max-h-[90vh] overflow-auto relative" style="background: var(--surface-1)">
          <button @click="closeModal" class="ui-button-secondary absolute top-3 right-3 rounded-none p-2 !px-2 !py-2">✕</button>
          <div class="p-4 flex items-center justify-center">
            <template v-if="modalEntry && modalEntry.mime.startsWith('video/')">
              <video v-if="modalUrl" :src="modalUrl" controls autoplay class="max-w-full max-h-[80vh] rounded-[var(--radius-sm)]"></video>
            </template>
            <template v-else>
              <img v-if="modalUrl" :src="modalUrl" :alt="modalEntry?.name" class="max-w-full max-h-[80vh] rounded-[var(--radius-sm)] object-contain" />
            </template>
          </div>
          <div class="p-3 text-sm text-[var(--ink-2)]" style="border-top: 1px solid var(--border)">{{ modalEntry?.name }}</div>
        </div>
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
