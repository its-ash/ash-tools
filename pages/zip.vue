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
        const mime = isImage
          ? extension === 'svg'
            ? 'image/svg+xml'
            : `image/${extension === 'jpg' ? 'jpeg' : extension}`
          : 'application/octet-stream'

        return {
          name,
          size: data.length,
          isImage,
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
  <div class="min-h-[calc(100vh-48px)] bg-slate-950 text-slate-200 p-8">
    <main class="mx-auto w-full max-w-6xl grid grid-cols-1 lg:grid-cols-2 gap-6">
      <section class="bg-slate-900/80 border border-white/8 rounded-2xl p-7 backdrop-blur-md flex flex-col gap-4">
        <div class="inline-flex items-center gap-2 uppercase tracking-wide text-xs text-slate-500 before:content-[''] before:w-4 before:h-0.5 before:bg-gradient-to-r before:from-violet-500 before:to-cyan-400 before:rounded">
          Create ZIP
        </div>
        <h1 class="text-3xl font-bold tracking-tight leading-tight">Zip files and folders locally.</h1>
        <p class="text-slate-400 leading-relaxed text-sm">Choose a folder or files and create a ZIP archive in your browser.</p>

        <label for="folder-input" class="border border-dashed border-white/10 rounded-xl p-5 bg-gradient-to-br from-violet-500/8 to-cyan-400/5 cursor-pointer hover:border-violet-500/50 transition-all duration-150 hover:-translate-y-0.5 flex justify-between items-center gap-2">
          <div>
            <div class="font-semibold text-sm">Choose folder</div>
            <div class="text-slate-500 text-xs">Nested files are preserved.</div>
          </div>
          <span class="text-violet-500 font-semibold text-xs whitespace-nowrap">Browse folder</span>
        </label>
        <input id="folder-input" type="file" webkitdirectory multiple class="hidden" @change="onFolderChange">

        <label for="file-input" class="border border-dashed border-white/10 rounded-xl p-5 bg-gradient-to-br from-violet-500/8 to-cyan-400/5 cursor-pointer hover:border-violet-500/50 transition-all duration-150 hover:-translate-y-0.5 flex justify-between items-center gap-2">
          <div>
            <div class="font-semibold text-sm">Choose files</div>
            <div class="text-slate-500 text-xs">You can combine with folder files.</div>
          </div>
          <span class="text-violet-500 font-semibold text-xs whitespace-nowrap">Browse files</span>
        </label>
        <input id="file-input" type="file" multiple class="hidden" @change="onFileChange">

        <button
          class="w-full bg-gradient-to-r from-violet-500 to-cyan-400 text-slate-950 font-bold py-3 px-4 rounded-xl transition-all duration-150 hover:-translate-y-0.5 hover:shadow-lg disabled:opacity-50 disabled:cursor-not-allowed"
          :disabled="compressing"
          @click="buildZipFromSelectedFiles"
        >
          {{ compressing ? 'Compressing...' : 'Create ZIP' }}
        </button>

        <div class="font-mono text-xs text-slate-500 min-h-5">{{ createStatus }}</div>
        <div class="w-full h-2.5 rounded-full bg-white/5 border border-white/10 overflow-hidden" :class="createProgress === 0 ? 'opacity-0' : 'opacity-100'">
          <div class="h-full bg-gradient-to-r from-violet-500 to-cyan-400 transition-all duration-150" :style="{ width: `${createProgress}%` }" />
        </div>
        <div class="font-mono text-xs text-slate-500 min-h-5">{{ selectedFilesSummary }}</div>
      </section>

      <section class="bg-slate-900/80 border border-white/8 rounded-2xl p-7 backdrop-blur-md flex flex-col gap-4">
        <div class="inline-flex items-center gap-2 uppercase tracking-wide text-xs text-slate-500 before:content-[''] before:w-4 before:h-0.5 before:bg-gradient-to-r before:from-violet-500 before:to-cyan-400 before:rounded">
          Open ZIP
        </div>
        <h2 class="text-3xl font-bold tracking-tight leading-tight">Drop a ZIP to view files.</h2>
        <p class="text-slate-400 leading-relaxed text-sm">Inspect image and file entries, then download each file directly.</p>

        <label
          for="zip-input"
          class="border border-dashed border-white/10 rounded-xl p-6 bg-gradient-to-br from-violet-500/8 to-cyan-400/5 transition-all duration-150 flex flex-col items-center justify-center text-center gap-2"
          @drop="onZipDrop"
          @dragover="onZipDragOver"
        >
          <span class="font-semibold text-sm">Drop ZIP file here</span>
          <span class="text-slate-500 text-xs">or click to browse</span>
          <input id="zip-input" type="file" accept=".zip,application/zip" class="hidden" @change="onZipFileChange">
        </label>

        <div class="font-mono text-xs text-slate-500 min-h-5">{{ openStatus }}</div>
        <div v-if="zipInputFile" class="text-xs text-slate-400">Opened: {{ zipInputFile.name }}</div>

        <div class="max-h-[420px] overflow-auto rounded-xl border border-white/10 bg-slate-950/40">
          <div v-if="!zipEntries.length" class="p-4 text-sm text-slate-500">No files to display yet.</div>
          <ul v-else class="divide-y divide-white/10">
            <li v-for="entry in zipEntries" :key="entry.name" class="p-3 flex items-center justify-between gap-3">
              <div class="min-w-0 flex items-center gap-3">
                <img
                  v-if="entry.isImage && entry.previewUrl"
                  :src="entry.previewUrl"
                  :alt="entry.name"
                  class="w-12 h-12 rounded object-cover border border-white/10"
                >
                <div class="min-w-0">
                  <p class="text-sm font-medium truncate">{{ entry.name }}</p>
                  <p class="text-xs text-slate-500">{{ prettySize(entry.size) }}</p>
                </div>
              </div>
              <button
                class="shrink-0 px-3 py-1.5 text-xs font-semibold rounded-lg bg-white/10 hover:bg-white/20 transition-colors"
                @click="downloadBytes(entry.name.split('/').pop() || entry.name, entry.bytes)"
              >
                Download
              </button>
            </li>
          </ul>
        </div>
      </section>
    </main>
  </div>
</template>

