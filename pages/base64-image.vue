<script setup lang="ts">
definePageMeta({ layout: 'tools' })

useHead({
  title: 'Free Image to Base64 Converter | Data URIs in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser image to Base64 converter. Drop an image and get a copyable data URI, or convert Base64 back to a file. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/base64-image/' },
    { property: 'og:title', content: 'Free Image to Base64 Converter | Data URIs' },
    { property: 'og:description', content: 'Convert images to Base64 privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/base64-image/' }],
})

useToolSchema({
  name: 'Image to Base64 Converter',
  description: 'Convert images to Base64 data URIs and back, locally in your browser.',
  url: 'https://ash-tools.store/base64-image/',
  category: 'DeveloperApplication',
})

type ImageInfo = { name: string; type: string; size: number; width: number; height: number; dataUrl: string }

const imageInfo = ref<ImageInfo | null>(null)
const status = ref('Drop an image or click to browse.')
const isError = ref(false)
const includeDataUriPrefix = ref(true)
const cssFormat = ref(false)

const decodeInput = ref('')
const decodeError = ref('')
const decodePreview = ref<string | null>(null)

const prettySize = (bytes: number) => {
  if (bytes < 1024) return `${bytes} B`
  const kb = bytes / 1024
  if (kb < 1024) return `${kb.toFixed(1)} KB`
  return `${(kb / 1024).toFixed(2)} MB`
}

const outputText = computed(() => {
  if (!imageInfo.value) return ''
  const base64Only = imageInfo.value.dataUrl.split(',')[1] || ''
  if (cssFormat.value) return `url("${imageInfo.value.dataUrl}")`
  return includeDataUriPrefix.value ? imageInfo.value.dataUrl : base64Only
})

const outputSize = computed(() => new Blob([outputText.value]).size)

const loadImage = (file: File) => {
  if (!file.type.startsWith('image/')) {
    isError.value = true
    status.value = 'Please select an image file.'
    return
  }
  const reader = new FileReader()
  reader.onload = () => {
    const dataUrl = String(reader.result || '')
    const img = new Image()
    img.onload = () => {
      imageInfo.value = { name: file.name, type: file.type, size: file.size, width: img.naturalWidth, height: img.naturalHeight, dataUrl }
      isError.value = false
      status.value = `Loaded ${file.name} (${prettySize(file.size)}).`
    }
    img.onerror = () => {
      isError.value = true
      status.value = 'Could not read image dimensions.'
    }
    img.src = dataUrl
  }
  reader.onerror = () => {
    isError.value = true
    status.value = 'Could not read the file.'
  }
  reader.readAsDataURL(file)
}

const onFileChange = (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (file) loadImage(file)
}

const onDrop = (event: DragEvent) => {
  event.preventDefault()
  const file = event.dataTransfer?.files?.[0]
  if (file) loadImage(file)
}

const onDragOver = (event: DragEvent) => {
  event.preventDefault()
}

const copyOutput = async () => {
  if (!outputText.value) return
  try {
    await navigator.clipboard.writeText(outputText.value)
    status.value = 'Copied to clipboard.'
  } catch {
    status.value = 'Could not copy — select and copy manually.'
  }
}

const downloadText = () => {
  if (!outputText.value) return
  const blob = new Blob([outputText.value], { type: 'text/plain' })
  const url = URL.createObjectURL(blob)
  const anchor = document.createElement('a')
  anchor.href = url
  anchor.download = `${imageInfo.value?.name || 'image'}.base64.txt`
  document.body.appendChild(anchor)
  anchor.click()
  anchor.remove()
  URL.revokeObjectURL(url)
}

const clearImage = () => {
  imageInfo.value = null
  isError.value = false
  status.value = 'Drop an image or click to browse.'
}

// --- Base64 to image (decode) ---
const decodeToImage = () => {
  decodeError.value = ''
  decodePreview.value = null
  const raw = decodeInput.value.trim()
  if (!raw) return
  try {
    const dataUrl = raw.startsWith('data:') ? raw : `data:image/png;base64,${raw}`
    const img = new Image()
    img.onload = () => { decodePreview.value = dataUrl }
    img.onerror = () => { decodeError.value = 'Could not decode as an image — check the Base64 string.' }
    img.src = dataUrl
  } catch {
    decodeError.value = 'Invalid Base64 input.'
  }
}

const downloadDecoded = () => {
  if (!decodePreview.value) return
  const anchor = document.createElement('a')
  anchor.href = decodePreview.value
  anchor.download = 'decoded-image.png'
  document.body.appendChild(anchor)
  anchor.click()
  anchor.remove()
}
</script>

<template>
  <div class="min-h-screen bg-[var(--surface-0)] text-[var(--ink-1)] flex flex-col">
    <div class="flex-1 p-4 md:p-6 flex flex-col md:flex-row gap-4">
      <aside class="ui-panel shrink-0 md:w-[68px] w-full">
        <div class="flex md:flex-col items-center gap-1 p-2 md:py-3">
          <NuxtLink to="/" class="rail-btn shrink-0 w-14 h-14 rounded-[var(--radius-sm)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Back to home" aria-label="Back to home">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6"><path stroke-linecap="round" stroke-linejoin="round" d="M11.5 4L6 9.5l5.5 5.5" /></svg>
            <span class="text-[10px] font-medium">Home</span>
          </NuxtLink>
          <label for="b64-file-input" class="rail-btn cursor-pointer shrink-0 w-14 h-14 rounded-[var(--radius-sm)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Choose image" aria-label="Choose image">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="3" y="4" width="14" height="12" rx="1.5" /><circle cx="7.5" cy="8.5" r="1.5" /><path stroke-linecap="round" stroke-linejoin="round" d="M4 14l4-4 3 3 2-2 3 3" /></svg>
            <span class="text-[10px] font-medium">Image</span>
          </label>
          <input id="b64-file-input" type="file" accept="image/*" class="hidden" @change="onFileChange">
          <button class="rail-btn shrink-0 w-14 h-14 rounded-[var(--radius-sm)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Clear" aria-label="Clear" @click="clearImage">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M5 6h10M8 6V4h4v2m-6 0l1 10h6l1-10" /></svg>
            <span class="text-[10px] font-medium">Clear</span>
          </button>
        </div>
      </aside>

      <main class="flex-1 min-w-0 mx-auto w-full max-w-6xl flex flex-col gap-4">
        <div class="ui-panel p-4 flex items-center justify-between gap-3">
          <div class="flex items-center gap-2">
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="3" y="4" width="14" height="12" rx="1.5" /><circle cx="7.5" cy="8.5" r="1.5" /><path stroke-linecap="round" stroke-linejoin="round" d="M4 14l4-4 3 3 2-2 3 3" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">Image ⇄ Base64</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Image to Base64</span>
            </div>
            <div class="px-4 pb-4 flex flex-col gap-3">
              <label
                for="b64-file-input"
                class="ui-dropzone p-6 flex flex-col items-center justify-center gap-2 text-center"
                @drop="onDrop"
                @dragover="onDragOver"
              >
                <img v-if="imageInfo" :src="imageInfo.dataUrl" :alt="imageInfo.name" class="max-h-32 object-contain" style="border: 1px solid var(--border)">
                <svg v-else class="h-6 w-6 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="3" y="4" width="14" height="12" rx="1.5" /><circle cx="7.5" cy="8.5" r="1.5" /><path stroke-linecap="round" stroke-linejoin="round" d="M4 14l4-4 3 3 2-2 3 3" /></svg>
                <span class="font-semibold text-sm text-[var(--ink-1)]">{{ imageInfo ? imageInfo.name : 'Drop image or click to browse' }}</span>
                <span v-if="imageInfo" class="text-xs text-[var(--ink-3)]">{{ imageInfo.width }}×{{ imageInfo.height }} · {{ prettySize(imageInfo.size) }} · {{ imageInfo.type }}</span>
              </label>

              <div class="flex flex-wrap items-center gap-4">
                <label class="flex items-center gap-1.5 text-xs text-[var(--ink-2)]">
                  <input v-model="includeDataUriPrefix" type="checkbox" class="ui-range" :disabled="cssFormat"> Include data URI prefix
                </label>
                <label class="flex items-center gap-1.5 text-xs text-[var(--ink-2)]">
                  <input v-model="cssFormat" type="checkbox" class="ui-range"> CSS url() format
                </label>
              </div>

              <div class="relative">
                <pre class="ui-input font-mono text-xs w-full min-h-[100px] max-h-40 overflow-auto whitespace-pre-wrap break-all">{{ outputText || 'Base64 output will appear here.' }}</pre>
              </div>
              <div v-if="outputText" class="text-xs font-mono text-[var(--ink-3)]">{{ prettySize(outputSize) }} as text</div>

              <div class="flex gap-2">
                <button class="ui-button-secondary flex-1 !py-2 text-xs" :disabled="!outputText" @click="copyOutput">Copy</button>
                <button class="ui-button flex-1 !py-2 text-xs" :disabled="!outputText" @click="downloadText">Download .txt</button>
              </div>
            </div>
          </section>

          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Base64 to image</span>
            </div>
            <div class="px-4 pb-4 flex flex-col gap-3">
              <textarea
                v-model="decodeInput"
                spellcheck="false"
                placeholder="Paste a Base64 string or data URI..."
                class="ui-input font-mono text-xs w-full min-h-[100px] resize-none"
                @input="decodeToImage"
              />
              <div class="overflow-hidden flex items-center justify-center min-h-[140px]" style="border: 1px solid var(--border); background: var(--surface-2)">
                <img v-if="decodePreview" :src="decodePreview" alt="Decoded" class="max-h-40 object-contain">
                <span v-else class="text-sm text-[var(--ink-3)] p-4">{{ decodeError || 'Preview will appear here.' }}</span>
              </div>
              <button class="ui-button w-full !py-2 text-xs" :disabled="!decodePreview" @click="downloadDecoded">Download image</button>
            </div>
          </section>
        </div>
      </main>
    </div>

    <div class="shrink-0 border-t px-5 py-2 flex items-center gap-3 text-xs font-mono" style="border-color: var(--border); background: var(--surface-1); color: var(--ink-3)">
      <span class="status-dot w-2 h-2 shrink-0 inline-block" :class="isError ? 'err' : 'ok'"></span>
      <span class="truncate" :class="isError ? 'text-red-600' : ''">{{ status }}</span>
    </div>
  </div>
</template>

<style scoped>
.rail-btn {
  transition: background-color 140ms ease, color 140ms ease;
}

.rail-btn:hover {
  background: var(--surface-2);
  color: var(--ink-1);
}

.rail-btn:active {
  transform: scale(0.97);
}

@media (prefers-reduced-motion: reduce) {
  .rail-btn {
    transition: none !important;
  }
}
</style>
