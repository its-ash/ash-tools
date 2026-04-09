<script setup lang="ts">
useHead({
  title: 'Free File Compress | Local ZIP in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser ZIP compressor. Select folders or files and create archives locally on your device. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:title', content: 'Free File Compress | Local ZIP in your browser' },
    { property: 'og:description', content: 'Zip folders and files privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/zip/' }],
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
  <div class="min-h-[calc(100vh-48px)] bg-[#FFFDF5] text-black p-8">
    <main class="mx-auto w-full max-w-6xl grid grid-cols-1 lg:grid-cols-2 gap-6">
      <section class="neo-shell bg-white p-7 flex flex-col gap-4">
        <div class="inline-flex items-center gap-2 uppercase tracking-wide text-xs text-black before:content-[''] before:w-4 before:h-0.5 before:bg-black before:rounded">
          Create ZIP
        </div>
        <h1 class="text-3xl font-bold tracking-tight leading-tight">Zip files and folders locally.</h1>
        <p class="text-black leading-relaxed text-sm">Choose a folder or files and create a ZIP archive in your browser.</p>

        <label for="folder-input" class="border-4 border-dashed border-black rounded-xl p-5 bg-[#FFFDF5] cursor-pointer transition-all duration-150 hover:-translate-y-0.5 flex justify-between items-center gap-2">
          <div>
            <div class="font-semibold text-sm">Choose folder</div>
            <div class="text-black text-xs">Nested files are preserved.</div>
          </div>
          <span class="text-black font-semibold text-xs whitespace-nowrap">Browse folder</span>
        </label>
        <input id="folder-input" type="file" webkitdirectory multiple class="hidden" @change="onFolderChange">

        <label for="file-input" class="border-4 border-dashed border-black rounded-xl p-5 bg-[#FFFDF5] cursor-pointer transition-all duration-150 hover:-translate-y-0.5 flex justify-between items-center gap-2">
          <div>
            <div class="font-semibold text-sm">Choose files</div>
            <div class="text-black text-xs">You can combine with folder files.</div>
          </div>
          <span class="text-black font-semibold text-xs whitespace-nowrap">Browse files</span>
        </label>
        <input id="file-input" type="file" multiple class="hidden" @change="onFileChange">

        <button
          class="neo-button w-full bg-[#FF6B6B] text-black disabled:opacity-50 disabled:cursor-not-allowed"
          :disabled="compressing"
          @click="buildZipFromSelectedFiles"
        >
          {{ compressing ? 'Compressing...' : 'Create ZIP' }}
        </button>

        <div class="font-mono text-xs text-black min-h-5">{{ createStatus }}</div>
        <div class="w-full h-2.5 rounded-full bg-white border-2 border-black overflow-hidden" :class="createProgress === 0 ? 'opacity-0' : 'opacity-100'">
          <div class="h-full bg-linear-to-r from-violet-500 to-cyan-400 transition-all duration-150" :style="{ width: `${createProgress}%` }" />
        </div>
        <div class="font-mono text-xs text-black min-h-5">{{ selectedFilesSummary }}</div>
      </section>

      <section class="neo-shell bg-white p-7 flex flex-col gap-4">
        <div class="inline-flex items-center gap-2 uppercase tracking-wide text-xs text-black before:content-[''] before:w-4 before:h-0.5 before:bg-black before:rounded">
          Open ZIP
        </div>
        <h2 class="text-3xl font-bold tracking-tight leading-tight">Drop a ZIP to view files.</h2>
        <p class="text-black leading-relaxed text-sm">Inspect image and file entries, then download each file directly.</p>

        <label
          for="zip-input"
          class="border-4 border-dashed border-black rounded-xl p-6 bg-[#FFFDF5] transition-all duration-150 flex flex-col items-center justify-center text-center gap-2"
          @drop="onZipDrop"
          @dragover="onZipDragOver"
        >
          <span class="font-semibold text-sm">Drop ZIP file here</span>
          <span class="text-black text-xs">or click to browse</span>
          <input id="zip-input" type="file" accept=".zip,application/zip" class="hidden" @change="onZipFileChange">
        </label>

        <div class="font-mono text-xs text-black min-h-5">{{ openStatus }}</div>
        <div v-if="zipInputFile" class="text-xs text-black">Opened: {{ zipInputFile.name }}</div>

        <div class="max-h-105 overflow-auto rounded-xl border-4 border-black bg-white">
          <div v-if="!zipEntries.length" class="p-4 text-sm text-black">No files to display yet.</div>
          <ul v-else class="divide-y divide-black">
            <li v-for="entry in zipEntries" :key="entry.name" class="p-3 flex items-center justify-between gap-3">
              <div class="min-w-0 flex items-center gap-3">
                <img
                  v-if="entry.isImage && entry.previewUrl"
                  :src="entry.previewUrl"
                  :alt="entry.name"
                  class="w-12 h-12 rounded object-cover border-2 border-black"
                >
                <div class="min-w-0">
                  <p class="text-sm font-medium truncate">{{ entry.name }}</p>
                  <p class="text-xs text-black">{{ prettySize(entry.size) }}</p>
                </div>
              </div>
                <div class="flex items-center gap-2">
                <button
                  class="shrink-0 px-3 py-1.5 text-xs font-semibold border-2 border-black bg-[#FFD93D] transition-colors"
                  @click="openEntry(entry)"
                >
                  Open
                </button>
                <button
                  class="shrink-0 px-3 py-1.5 text-xs font-semibold border-2 border-black bg-[#C4B5FD] transition-colors"
                  @click="downloadBytes(entry.name.split('/').pop() || entry.name, entry.bytes, entry.mime)"
                >
                  Download
                </button>
              </div>
            </li>
          </ul>
        </div>
      </section>
    </main>
    
      <div v-if="modalEntry" class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4">
        <div class="bg-white border-4 border-black rounded-xl max-w-4xl w-full max-h-[90vh] overflow-auto relative">
          <button @click="closeModal" class="absolute top-3 right-3 text-black bg-[#FF6B6B] border-2 border-black rounded-full p-2">✕</button>
          <div class="p-4 flex items-center justify-center">
            <template v-if="modalEntry && modalEntry.mime.startsWith('video/')">
              <video v-if="modalUrl" :src="modalUrl" controls autoplay class="max-w-full max-h-[80vh] rounded"></video>
            </template>
            <template v-else>
              <img v-if="modalUrl" :src="modalUrl" :alt="modalEntry?.name" class="max-w-full max-h-[80vh] rounded object-contain" />
            </template>
          </div>
          <div class="p-3 border-t-4 border-black text-sm text-black">{{ modalEntry?.name }}</div>
        </div>
      </div>
  </div>
</template>

