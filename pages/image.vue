<script setup lang="ts">
definePageMeta({ layout: 'tools' })

useHead({
  title: 'Free Image Editor | Crop, Resize, Rotate & Filters in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser image editor. Crop, perspective crop, rotate, resize, adjust color, apply filters, and add text overlays. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/image/' },
    { property: 'og:title', content: 'Free Image Editor | Crop, Resize, Rotate & Filters' },
    { property: 'og:description', content: 'Edit images privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/image/' }],
})

useToolSchema({
  name: 'Image Editor',
  description: 'Crop, resize, rotate, adjust, and filter images locally in your browser.',
  url: 'https://ash-tools.store/image/',
  category: 'MultimediaApplication',
})

// WASM functions
let compress_image: any = null
let crop_image: any = null
let perspective_crop: any = null
let rotate_image: any = null
let resize_image: any = null

// State variables
let wasmReadyPromise: Promise<void> | null = null
let currentBytes: Uint8Array | null = null
let originalBytes: Uint8Array | null = null
let currentDims: { width: number; height: number } | null = null
let previewUrl: string | null = null
let cropRect = { x: 0, y: 0, w: 1, h: 1 }
let perspPoints = {
  tl: { x: 0, y: 0 },
  tr: { x: 1, y: 0 },
  br: { x: 1, y: 1 },
  bl: { x: 0, y: 1 },
}
type EditorMode = 'crop' | 'perspective' | 'rotate' | 'resize' | 'adjust' | 'filters' | 'text' | 'layers' | 'none'
let activeMode: EditorMode = 'crop'

// Rotate/flip: applied cumulatively to the working image on Process.
const rotateState = ref({ degrees: 0, flipH: false, flipV: false })

// Resize: target dimensions for the next Process, defaults populated from currentDims.
const resizeState = ref({ width: 0, height: 0, lockAspect: true, filter: 'lanczos3' })
let resizeAspect = 1

// Adjust: live CSS-filter preview, baked into the export via canvas at Process/Download time.
const adjustState = ref({ brightness: 0, contrast: 0, saturation: 0, temperature: 0, sharpness: 0 })
const DEFAULT_ADJUST = { brightness: 0, contrast: 0, saturation: 0, temperature: 0, sharpness: 0 }

// Filters: one preset active at a time, alongside the Adjust sliders. CSS-only preview,
// baked into the export via canvas alongside Adjust.
type FilterPreset = 'none' | 'grayscale' | 'sepia' | 'blur' | 'vivid' | 'fade'
const activeFilter = ref<FilterPreset>('none')
const FILTER_CSS: Record<FilterPreset, string> = {
  none: '',
  grayscale: 'grayscale(1)',
  sepia: 'sepia(0.65)',
  blur: 'blur(1.5px)',
  vivid: 'saturate(1.7) contrast(1.1)',
  fade: 'brightness(1.1) saturate(0.7) contrast(0.9)',
}

// Text layers: draggable captions composited onto the export via canvas.
type TextLayer = {
  id: string
  text: string
  x: number // 0-1, fraction of image width, center anchor
  y: number // 0-1, fraction of image height, center anchor
  size: number // px, relative to a 1000px-wide reference frame
  color: string
  visible: boolean
}
const textLayers = ref<TextLayer[]>([])
const selectedLayerId = ref<string | null>(null)
let layerIdCounter = 0

const adjustCssFilter = computed(() => {
  const a = adjustState.value
  const parts = [
    `brightness(${1 + a.brightness / 100})`,
    `contrast(${1 + a.contrast / 100})`,
    `saturate(${1 + a.saturation / 100})`,
  ]
  if (a.temperature !== 0) {
    // Approximate warmth via hue-rotate is wrong direction-wise for a "temperature" feel,
    // so approximate with sepia+hue-rotate blend kept subtle.
    parts.push(`sepia(${Math.min(0.35, Math.abs(a.temperature) / 200)})`)
    if (a.temperature < 0) parts.push('hue-rotate(180deg)')
  }
  if (FILTER_CSS[activeFilter.value]) parts.push(FILTER_CSS[activeFilter.value])
  return parts.join(' ')
})

const hasAdjustments = computed(() => {
  const a = adjustState.value
  return a.brightness !== 0 || a.contrast !== 0 || a.saturation !== 0 || a.temperature !== 0 || a.sharpness !== 0 || activeFilter.value !== 'none'
})

// DOM refs
let fileInput: HTMLInputElement | null = null
let dropzone: HTMLElement | null = null
let statusEl: HTMLElement | null = null
let previewImg: HTMLImageElement | null = null
let metaEl: HTMLElement | null = null
let processBtn: HTMLButtonElement | null = null
let downloadBtn: HTMLButtonElement | null = null
let resetBtn: HTMLButtonElement | null = null
let modeCropBtn: HTMLButtonElement | null = null
let modePerspectiveBtn: HTMLButtonElement | null = null
let cropBox: HTMLElement | null = null
let cropSizeEl: HTMLElement | null = null
let perspOverlay: HTMLElement | null = null
let perspSvg: SVGSVGElement | null = null
let perspPolygon: SVGPolygonElement | null = null
let perspectiveHandleElements: Element[] = []
let perspectiveHandles: Record<string, Element> = {}
let formatSelect: HTMLSelectElement | null = null
let qualityRow: HTMLElement | null = null
let qualityInput: HTMLInputElement | null = null
let qualityValue: HTMLElement | null = null
let statsEl: HTMLElement | null = null

// Reactive UI state (mirrors the plain-DOM state above; drives step indicator,
// control-rail gating, and status styling only - never touched by the WASM pipeline).
const hasImage = ref(false)
const isProcessing = ref(false)
const activeStep = ref(1)
const statusKind = ref<'idle' | 'busy' | 'success' | 'error'>('idle')
const activeModeDisplay = ref<EditorMode>('crop')

let projectedSizeTimer: ReturnType<typeof setTimeout> | null = null
let projectedSizeToken = 0
const cleanupFns: Array<() => void> = []

type LoadedImageBytes = {
  bytes: Uint8Array
  dimensions?: { width: number; height: number }
  note?: string
}

// HEIC detection constants
const HEIC_MIME_TYPES = new Set([
  'image/heic',
  'image/heif',
  'image/heic-sequence',
  'image/heif-sequence',
])
const HEIC_EXTENSION = /\.hei[cf]$/i

// ==================== WASM Loading ====================
const ensureWasm = async () => {
  if (!wasmReadyPromise) {
    wasmReadyPromise = (async () => {
      try {
        const module = await import('../public/image/pkg/image_tools.js')
        compress_image = module.compress_image
        crop_image = module.crop_image
        perspective_crop = module.perspective_crop
        rotate_image = module.rotate_image
        resize_image = module.resize_image
        if (module.default && typeof module.default === 'function') {
          await module.default()
        }
      } catch (error) {
        console.error('Failed to load WASM module:', error)
        throw error
      }
    })()
  }
  return wasmReadyPromise
}

// ==================== Utility Functions ====================
const prettySize = (bytes: number): string => {
  if (bytes < 1024) return `${bytes} B`
  const kb = bytes / 1024
  if (kb < 1024) return `${kb.toFixed(1)} KB`
  return `${(kb / 1024).toFixed(2)} MB`
}

const setDownloadLabel = (value?: number | string | null) => {
  if (!downloadBtn) return
  if (typeof value === 'number' && Number.isFinite(value)) {
    downloadBtn.textContent = `Download (${prettySize(value)})`
    return
  }
  if (typeof value === 'string') {
    downloadBtn.textContent = value
    return
  }
  downloadBtn.textContent = 'Download'
}

const setStatus = (message?: string, kind: 'idle' | 'busy' | 'success' | 'error' = 'idle') => {
  if (statusEl) statusEl.textContent = message || ''
  statusKind.value = kind
}

const setProcessingState = (processing: boolean) => {
  isProcessing.value = processing
  if (!processBtn) return
  processBtn.disabled = processing || !currentBytes
  processBtn.textContent = processing ? 'Processing...' : 'Process'
  processBtn.setAttribute('aria-busy', processing ? 'true' : 'false')
}

const updateStats = (currentSize?: number) => {
  if (!statsEl) return
  if (!originalBytes || !currentSize) {
    statsEl.textContent = ''
    return
  }
  if (currentSize === originalBytes.length) {
    statsEl.textContent = `Original ${prettySize(originalBytes.length)} • No size change yet`
    return
  }
  const ratio = ((originalBytes.length - currentSize) / originalBytes.length) * 100
  if (ratio >= 0) {
    statsEl.textContent = `Original ${prettySize(originalBytes.length)} -> Current ${prettySize(currentSize)} (${ratio.toFixed(1)}% smaller)`
    return
  }
  statsEl.textContent = `Original ${prettySize(originalBytes.length)} -> Current ${prettySize(currentSize)} (${Math.abs(ratio).toFixed(1)}% larger)`
}

const clamp01 = (value: number): number => Math.min(1, Math.max(0, value))

// ==================== Resize helpers ====================
const onResizeWidthChange = (value: number) => {
  const w = Math.max(1, Math.round(value) || 1)
  resizeState.value.width = w
  if (resizeState.value.lockAspect && resizeAspect) {
    resizeState.value.height = Math.max(1, Math.round(w / resizeAspect))
  }
}

const onResizeHeightChange = (value: number) => {
  const h = Math.max(1, Math.round(value) || 1)
  resizeState.value.height = h
  if (resizeState.value.lockAspect && resizeAspect) {
    resizeState.value.width = Math.max(1, Math.round(h * resizeAspect))
  }
}

// ==================== Text layers ====================
const addTextLayer = () => {
  const id = `layer-${++layerIdCounter}`
  textLayers.value.push({
    id,
    text: 'Caption',
    x: 0.5,
    y: 0.85,
    size: 48,
    color: '#ffffff',
    visible: true,
  })
  selectedLayerId.value = id
}

const removeTextLayer = (id: string) => {
  textLayers.value = textLayers.value.filter((l) => l.id !== id)
  if (selectedLayerId.value === id) selectedLayerId.value = null
}

const selectedLayer = computed(() => textLayers.value.find((l) => l.id === selectedLayerId.value) || null)

const bytesToBlobPart = (bytes: Uint8Array): ArrayBuffer => {
  const cloned = new Uint8Array(bytes.byteLength)
  cloned.set(bytes)
  return cloned.buffer
}

const addListener = (
  target: EventTarget,
  eventName: string,
  handler: EventListenerOrEventListenerObject,
  options?: boolean | AddEventListenerOptions
) => {
  target.addEventListener(eventName, handler, options)
  cleanupFns.push(() => target.removeEventListener(eventName, handler, options))
}

// ==================== Orientation Normalization ====================
const normalizeImageOrientation = (bytes: Uint8Array): Promise<{ bytes: Uint8Array; dims: { width: number; height: number } }> =>
  new Promise((resolve, reject) => {
    const blob = new Blob([bytesToBlobPart(bytes)])
    const url = URL.createObjectURL(blob)
    const img = new Image()
    img.onload = async () => {
      const w = img.naturalWidth
      const h = img.naturalHeight
      URL.revokeObjectURL(url)
      try {
        // Drawing to canvas applies EXIF orientation, giving normalized pixel data
        const canvas = document.createElement('canvas')
        canvas.width = w
        canvas.height = h
        const ctx = canvas.getContext('2d')
        if (!ctx) throw new Error('Canvas context unavailable')
        ctx.drawImage(img, 0, 0)
        const normalizedBlob = await canvasToBlob(canvas, 'image/png')
        resolve({ bytes: new Uint8Array(await normalizedBlob.arrayBuffer()), dims: { width: w, height: h } })
      } catch (err) {
        reject(err)
      }
    }
    img.onerror = () => {
      URL.revokeObjectURL(url)
      reject(new Error('Failed to load image for orientation normalization'))
    }
    img.src = url
  })

// ==================== File Reading ====================
const readFileAsBytes = (file: File): Promise<Uint8Array> =>
  new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onerror = () => reject(new Error(`Failed to read ${file.name}`))
    reader.onload = () => resolve(new Uint8Array(reader.result as ArrayBuffer))
    reader.readAsArrayBuffer(file)
  })

const extractDimensions = (
  bytes: Uint8Array
): Promise<{ width: number; height: number }> =>
  new Promise((resolve, reject) => {
    const blob = new Blob([bytesToBlobPart(bytes)])
    const url = URL.createObjectURL(blob)
    const img = new Image()
    img.onload = () => {
      resolve({ width: img.naturalWidth, height: img.naturalHeight })
      URL.revokeObjectURL(url)
    }
    img.onerror = () => {
      URL.revokeObjectURL(url)
      reject(new Error('Unable to read image dimensions'))
    }
    img.src = url
  })

// ==================== HEIC Handling ====================
const isHeicFile = (file: File): boolean => {
  if (!file) return false
  const type = (file.type || '').toLowerCase()
  if (type && HEIC_MIME_TYPES.has(type)) return true
  return HEIC_EXTENSION.test(file.name || '')
}

const canvasToBlob = (canvas: HTMLCanvasElement, type: string): Promise<Blob> =>
  new Promise((resolve, reject) => {
    canvas.toBlob((blob) => {
      if (blob) {
        resolve(blob)
      } else {
        reject(new Error('Failed to convert canvas to blob'))
      }
    }, type)
  })

const drawSourceToPngBytes = async (
  source: ImageBitmap | OffscreenCanvas | any,
  width: number,
  height: number
): Promise<Uint8Array> => {
  if (typeof OffscreenCanvas !== 'undefined') {
    const canvas = new OffscreenCanvas(width, height)
    const ctx = canvas.getContext('2d')
    if (!ctx) throw new Error('Unable to initialise canvas context')
    ctx.drawImage(source, 0, 0, width, height)
    source.close?.()
    const blob = await canvas.convertToBlob({ type: 'image/png' })
    return new Uint8Array(await blob.arrayBuffer())
  }

  const canvas = document.createElement('canvas')
  canvas.width = width
  canvas.height = height
  const ctx = canvas.getContext('2d')
  if (!ctx) throw new Error('Unable to initialise canvas context')
  ctx.drawImage(source, 0, 0, width, height)
  source.close?.()
  const blob = await canvasToBlob(canvas, 'image/png')
  return new Uint8Array(await blob.arrayBuffer())
}

const convertHeicWithImageDecoder = async (
  buffer: ArrayBuffer,
  type: string
): Promise<LoadedImageBytes | null> => {
  if (typeof (globalThis as any).ImageDecoder === 'undefined') return null
  try {
    const decoder = new (globalThis as any).ImageDecoder({
      data: new Uint8Array(buffer),
      type,
    })
    const { image } = await decoder.decode()
    const width = image.displayWidth || image.codedWidth
    const height = image.displayHeight || image.codedHeight
    const bytes = await drawSourceToPngBytes(image, width, height)
    decoder.close?.()
    return { bytes, dimensions: { width, height } }
  } catch (err) {
    console.warn('ImageDecoder HEIC conversion failed', err)
    return null
  }
}

const convertHeicWithBitmap = async (
  buffer: ArrayBuffer,
  type: string
): Promise<LoadedImageBytes | null> => {
  if (typeof createImageBitmap === 'undefined') return null
  try {
    const blob = new Blob([buffer], { type })
    const bitmap = await createImageBitmap(blob)
    const bytes = await drawSourceToPngBytes(bitmap, bitmap.width, bitmap.height)
    return { bytes, dimensions: { width: bitmap.width, height: bitmap.height } }
  } catch (err) {
    console.warn('ImageBitmap HEIC conversion failed', err)
    return null
  }
}

const convertHeicFile = async (file: File): Promise<LoadedImageBytes | null> => {
  const type = (file.type || 'image/heic').toLowerCase()
  const buffer = await file.arrayBuffer()

  const viaDecoder = await convertHeicWithImageDecoder(buffer, type)
  if (viaDecoder) {
    return { ...viaDecoder, note: 'Converted HEIC to PNG for editing.' }
  }

  const viaBitmap = await convertHeicWithBitmap(buffer, type)
  if (viaBitmap) {
    return { ...viaBitmap, note: 'Converted HEIC to PNG for editing.' }
  }

  return null
}

const loadImageBytes = async (file: File): Promise<LoadedImageBytes | null> => {
  if (!file) return null
  if (isHeicFile(file)) {
    const converted = await convertHeicFile(file)
    if (!converted) {
      throw new Error('Unable to convert HEIC. Please try a different browser or format.')
    }
    return converted
  }
  const bytes = await readFileAsBytes(file)
  return { bytes }
}

// ==================== Preview & Display ====================
const updatePreview = async (bytes: Uint8Array, providedDims?: any) => {
  if (previewUrl) URL.revokeObjectURL(previewUrl)
  const blob = new Blob([bytesToBlobPart(bytes)])
  previewUrl = URL.createObjectURL(blob)
  if (previewImg) previewImg.src = previewUrl
  const dims = providedDims ?? (await extractDimensions(bytes).catch(() => null))
  currentDims = dims
  const dimText = dims ? `${dims.width}×${dims.height}px` : null
  if (metaEl) {
    metaEl.textContent = dimText
      ? `${dimText} • ${prettySize(bytes.length)}`
      : prettySize(bytes.length)
  }
  cancelProjectedSize()
  setDownloadLabel(bytes.length)
  updateStats(bytes.length)
  updateCropOverlay()
  updatePerspectiveOverlay()
}

const setDefaultsFromDims = (dims: any) => {
  if (!dims) return
  cropRect = { x: 0, y: 0, w: 1, h: 1 }
  perspPoints = { tl: { x: 0, y: 0 }, tr: { x: 1, y: 0 }, br: { x: 1, y: 1 }, bl: { x: 0, y: 1 } }
  rotateState.value = { degrees: 0, flipH: false, flipV: false }
  resizeState.value = { width: dims.width, height: dims.height, lockAspect: true, filter: 'lanczos3' }
  resizeAspect = dims.width / dims.height
  adjustState.value = { ...DEFAULT_ADJUST }
  activeFilter.value = 'none'
  textLayers.value = []
  selectedLayerId.value = null
  updateCropOverlay()
  updatePerspectiveOverlay()
}

const handleFile = async (file: File) => {
  if (!file) return
  const heicCandidate = isHeicFile(file)
  setStatus(heicCandidate ? 'Converting HEIC...' : 'Loading image...', 'busy')
  try {
    const loaded = await loadImageBytes(file)
    if (!loaded || !loaded.bytes) throw new Error('Unable to read image bytes')
    let bytes = loaded.bytes
    let dims: { width: number; height: number } | null = loaded.dimensions || null

    // Normalize EXIF orientation so the preview and WASM crop both see the same pixels.
    // HEIC files are already normalized via their canvas conversion path.
    if (!heicCandidate) {
      try {
        const normalized = await normalizeImageOrientation(bytes)
        bytes = normalized.bytes
        dims = normalized.dims
      } catch (err) {
        console.warn('Orientation normalization failed, using raw bytes', err)
        if (!dims) dims = await extractDimensions(bytes)
      }
    } else {
      if (!dims) dims = await extractDimensions(bytes)
    }

    originalBytes = bytes
    currentBytes = bytes
    setMode('crop', { silent: true })
    setDefaultsFromDims(dims)
    await updatePreview(bytes, dims)
    updateQualityLabel()
    const messages = []
    if (loaded.note) messages.push(loaded.note)
    messages.push('Image ready. Choose a mode, adjust, then hit Process.')
    setStatus(messages.join(' '), 'success')
    if (downloadBtn) downloadBtn.disabled = false
    if (processBtn) processBtn.disabled = false
    if (resetBtn) resetBtn.disabled = false
    hasImage.value = true
    activeStep.value = 2
  } catch (err) {
    setStatus(err instanceof Error ? err.message : String(err), 'error')
  }
}

// ==================== Quality & Format ====================
const updateQualityLabel = () => {
  if (!qualityValue || !qualityInput) return
  qualityValue.textContent = `${qualityInput.value}%`
}

const getOutputSettings = () => {
  const format = formatSelect?.value ?? 'png'
  const sliderValue = Number(qualityInput?.value ?? 80)
  const quality = Math.max(10, Math.min(100, Math.round(Number.isFinite(sliderValue) ? sliderValue : 80)))
  return { format, quality }
}

const updateQualityVisibility = () => {
  const isJpeg = (formatSelect?.value ?? '') === 'jpeg'
  if (qualityRow) qualityRow.style.display = isJpeg ? 'grid' : 'none'
  if (qualityInput) qualityInput.disabled = !isJpeg
  if (qualityValue) qualityValue.style.display = isJpeg ? 'inline' : 'none'
}

const handleQualityChange = () => {
  updateQualityLabel()
  requestProjectedDownloadSize()
}

// ==================== Projected Size ====================
const cancelProjectedSize = () => {
  projectedSizeToken += 1
  if (projectedSizeTimer) {
    clearTimeout(projectedSizeTimer)
    projectedSizeTimer = null
  }
}

const requestProjectedDownloadSize = () => {
  if (!currentBytes) {
    cancelProjectedSize()
    setDownloadLabel(null)
    updateStats(undefined)
    return
  }
  const { format, quality } = getOutputSettings()
  const token = ++projectedSizeToken
  if (projectedSizeTimer) clearTimeout(projectedSizeTimer)
  setDownloadLabel('Download (estimating…)')
  projectedSizeTimer = setTimeout(async () => {
    try {
      await ensureWasm()
      if (!compress_image) throw new Error('WASM not loaded')
      const projected = compress_image(currentBytes!, quality, format)
      if (projectedSizeToken !== token) return
      setDownloadLabel(projected.length)
    } catch (err) {
      if (projectedSizeToken !== token) return
      console.warn('Failed to estimate compressed size', err)
      setDownloadLabel(null)
    } finally {
      if (projectedSizeToken === token) projectedSizeTimer = null
    }
  }, 180)
}

// ==================== Crop Overlay ====================
const updateCropSizeLabel = () => {
  if (!cropSizeEl) return
  if (!currentDims || activeMode !== 'crop') {
    cropSizeEl.style.display = 'none'
    cropSizeEl.textContent = ''
    return
  }
  const width = Math.max(1, Math.round(cropRect.w * currentDims.width))
  const height = Math.max(1, Math.round(cropRect.h * currentDims.height))
  cropSizeEl.textContent = `${width}×${height}px`
  cropSizeEl.style.display = 'block'
}

const updateCropOverlay = () => {
  if (!previewImg || !cropBox) return
  if (!currentDims || activeMode !== 'crop') {
    cropBox.style.display = 'none'
    updateCropSizeLabel()
    return
  }
  const displayW = previewImg.clientWidth
  const displayH = previewImg.clientHeight
  if (!displayW || !displayH) {
    cropBox.style.display = 'none'
    updateCropSizeLabel()
    return
  }
  cropBox.style.display = 'block'
  cropBox.style.left = `${cropRect.x * displayW}px`
  cropBox.style.top = `${cropRect.y * displayH}px`
  cropBox.style.width = `${cropRect.w * displayW}px`
  cropBox.style.height = `${cropRect.h * displayH}px`
  updateCropSizeLabel()
}

const attachCropInteractions = () => {
  if (!cropBox || !previewImg) return
  let activeHandle: string | null = null
  let startRect: any = null
  let startClient: any = null

  const stop = () => {
    activeHandle = null
    startRect = null
    startClient = null
    document.removeEventListener('pointermove', onMove)
    document.removeEventListener('pointerup', stop)
    document.removeEventListener('pointercancel', stop)
  }

  const onMove = (event: PointerEvent) => {
    if (!activeHandle || !startRect || !currentDims) return
    const displayW = previewImg!.clientWidth
    const displayH = previewImg!.clientHeight
    if (!displayW || !displayH) return
    const dx = (event.clientX - startClient.x) / displayW
    const dy = (event.clientY - startClient.y) / displayH
    let { x, y, w, h } = startRect
    const minW = 4 / displayW
    const minH = 4 / displayH
    const applyClamp = () => {
      x = clamp01(x)
      y = clamp01(y)
      w = Math.max(minW, Math.min(1 - x, w))
      h = Math.max(minH, Math.min(1 - y, h))
    }
    switch (activeHandle) {
      case 'move':
        x += dx
        y += dy
        x = clamp01(x)
        y = clamp01(y)
        x = Math.min(x, 1 - w)
        y = Math.min(y, 1 - h)
        break
      case 'nw':
        x += dx
        y += dy
        w -= dx
        h -= dy
        applyClamp()
        break
      case 'ne':
        y += dy
        w += dx
        h -= dy
        applyClamp()
        break
      case 'sw':
        x += dx
        w -= dx
        h += dy
        applyClamp()
        break
      case 'se':
        w += dx
        h += dy
        applyClamp()
        break
    }
    cropRect = { x, y, w, h }
    updateCropOverlay()
    event.preventDefault()
  }

  const onDown = (event: PointerEvent) => {
    if (!currentDims || activeMode !== 'crop') return
    const target = event.target as HTMLElement
    if (!target.dataset.handle && target !== cropBox) return
    activeHandle = target.dataset.handle || 'move'
    startRect = { ...cropRect }
    startClient = { x: event.clientX, y: event.clientY }
    event.preventDefault()
    document.addEventListener('pointermove', onMove)
    document.addEventListener('pointerup', stop)
    document.addEventListener('pointercancel', stop)
  }

  addListener(cropBox, 'pointerdown', onDown as EventListener)
}

// ==================== Perspective Overlay ====================
const updatePerspectiveOverlay = () => {
  if (!perspOverlay || !perspSvg || !perspPolygon) return
  if (!currentDims || activeMode !== 'perspective') {
    perspOverlay.style.display = 'none'
    return
  }
  const rect = previewImg!.getBoundingClientRect()
  const displayW = rect.width
  const displayH = rect.height
  if (!displayW || !displayH) {
    perspOverlay.style.display = 'none'
    return
  }
  perspOverlay.style.display = 'block'
  perspOverlay.style.width = `${displayW}px`
  perspOverlay.style.height = `${displayH}px`
  perspSvg.setAttribute('width', `${displayW}`)
  perspSvg.setAttribute('height', `${displayH}`)
  perspSvg.setAttribute('viewBox', `0 0 ${displayW} ${displayH}`)
  const pointsPx = {
    tl: { x: perspPoints.tl.x * displayW, y: perspPoints.tl.y * displayH },
    tr: { x: perspPoints.tr.x * displayW, y: perspPoints.tr.y * displayH },
    br: { x: perspPoints.br.x * displayW, y: perspPoints.br.y * displayH },
    bl: { x: perspPoints.bl.x * displayW, y: perspPoints.bl.y * displayH },
  }
  const polygonPoints = `${pointsPx.tl.x},${pointsPx.tl.y} ${pointsPx.tr.x},${pointsPx.tr.y} ${pointsPx.br.x},${pointsPx.br.y} ${pointsPx.bl.x},${pointsPx.bl.y}`
  perspPolygon.setAttribute('points', polygonPoints)
  Object.entries(perspectiveHandles).forEach(([key, el]) => {
    const pos = pointsPx[key as keyof typeof pointsPx]
    if (!el || !pos) return
    const htmlEl = el as HTMLElement
    htmlEl.style.left = `${pos.x}px`
    htmlEl.style.top = `${pos.y}px`
  })
}

const attachPerspectiveInteractions = () => {
  if (!perspectiveHandleElements.length) return
  let activeHandle: string | null = null
  let activeElement: Element | null = null

  const stop = (event?: PointerEvent) => {
    if (activeElement && event) {
      (activeElement as HTMLElement).releasePointerCapture?.(event.pointerId)
    }
    activeHandle = null
    activeElement = null
    document.removeEventListener('pointermove', onMove)
    document.removeEventListener('pointerup', stop)
    document.removeEventListener('pointercancel', stop)
  }

  const updateFromEvent = (event: PointerEvent) => {
    if (!currentDims || !activeHandle) return
    const rect = previewImg!.getBoundingClientRect()
    const displayW = rect.width
    const displayH = rect.height
    if (!displayW || !displayH) return
    const x = clamp01((event.clientX - rect.left) / displayW)
    const y = clamp01((event.clientY - rect.top) / displayH)
    perspPoints = { ...perspPoints, [activeHandle]: { x, y } }
    updatePerspectiveOverlay()
  }

  const onMove = (event: PointerEvent) => {
    if (!activeHandle) return
    updateFromEvent(event)
  }

  const onDown = (event: PointerEvent) => {
    if (!currentDims || activeMode !== 'perspective') return
    const key = (event.target as HTMLElement).dataset.point
    if (!key) return
    activeHandle = key
    activeElement = event.target as Element
    event.preventDefault()
      ; (event.target as HTMLElement).setPointerCapture?.(event.pointerId)
    updateFromEvent(event)
    document.addEventListener('pointermove', onMove)
    document.addEventListener('pointerup', stop)
    document.addEventListener('pointercancel', stop)
  }

  perspectiveHandleElements.forEach((element) => {
    addListener(element, 'pointerdown', onDown as EventListener)
  })
}

// ==================== Mode Management ====================
const updateModeButtons = () => {
  if (modeCropBtn) {
    modeCropBtn.classList.toggle('active', activeMode === 'crop')
    modeCropBtn.setAttribute('aria-pressed', activeMode === 'crop' ? 'true' : 'false')
  }
  if (modePerspectiveBtn) {
    modePerspectiveBtn.classList.toggle('active', activeMode === 'perspective')
    modePerspectiveBtn.setAttribute('aria-pressed', activeMode === 'perspective' ? 'true' : 'false')
  }
}

const EDITOR_MODES: EditorMode[] = ['crop', 'perspective', 'rotate', 'resize', 'adjust', 'filters', 'text', 'layers', 'none']

const setMode = (mode: EditorMode, options: any = {}) => {
  if (!mode || !EDITOR_MODES.includes(mode)) return
  const changed = activeMode !== mode
  activeMode = mode
  activeModeDisplay.value = mode
  if (mode === 'resize' && currentDims && !resizeState.value.width) {
    resizeState.value.width = currentDims.width
    resizeState.value.height = currentDims.height
    resizeAspect = currentDims.width / currentDims.height
  }
  updateModeButtons()
  updateCropOverlay()
  updatePerspectiveOverlay()
  if (changed && currentBytes && !options.silent) {
    if (mode === 'crop') {
      setStatus('Crop mode active. Drag the handles to adjust.')
      activeStep.value = 2
    } else if (mode === 'perspective') {
      setStatus('Perspective mode active. Move the four corners.')
      activeStep.value = 2
    } else if (mode === 'rotate') {
      setStatus('Rotate mode active. Rotate or flip, then hit Process.')
      activeStep.value = 2
    } else if (mode === 'resize') {
      setStatus('Resize mode active. Set dimensions, then hit Process.')
      activeStep.value = 2
    } else if (mode === 'adjust' || mode === 'filters' || mode === 'text' || mode === 'layers') {
      setStatus(`${mode[0].toUpperCase()}${mode.slice(1)} panel active.`)
    } else {
      setStatus('Overlays hidden. Select a mode to adjust again.')
    }
  }
}

// ==================== Canvas Composite (Adjust / Filters / Text) ====================
// Bakes the live CSS-filter preview and any text layers onto the working image via
// canvas. Sharpness has no direct CSS filter equivalent, so it's approximated with an
// unsharp-mask-style convolution pass only when non-zero (kept cheap: single pass).
const applyUnsharpMask = (ctx: CanvasRenderingContext2D, width: number, height: number, amount: number) => {
  if (amount <= 0) return
  const src = ctx.getImageData(0, 0, width, height)
  const out = ctx.createImageData(width, height)
  const s = src.data
  const o = out.data
  const strength = Math.min(1, amount / 100) * 0.6
  const idx = (x: number, y: number) => (y * width + x) * 4
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const c = idx(x, y)
      for (let ch = 0; ch < 3; ch++) {
        const center = s[c + ch]
        const up = s[idx(x, Math.max(0, y - 1)) + ch]
        const down = s[idx(x, Math.min(height - 1, y + 1)) + ch]
        const left = s[idx(Math.max(0, x - 1), y) + ch]
        const right = s[idx(Math.min(width - 1, x + 1), y) + ch]
        const blur = (up + down + left + right + center) / 5
        const sharpened = center + (center - blur) * strength * 4
        o[c + ch] = Math.max(0, Math.min(255, sharpened))
      }
      o[c + 3] = s[c + 3]
    }
  }
  ctx.putImageData(out, 0, 0)
}

const compositeForExport = async (bytes: Uint8Array, dims: { width: number; height: number }): Promise<Uint8Array> => {
  if (!hasAdjustments.value && textLayers.value.every((l) => !l.visible || !l.text.trim())) {
    return bytes
  }
  const blob = new Blob([bytesToBlobPart(bytes)])
  const url = URL.createObjectURL(blob)
  try {
    const img = await new Promise<HTMLImageElement>((resolve, reject) => {
      const el = new Image()
      el.onload = () => resolve(el)
      el.onerror = () => reject(new Error('Failed to load image for compositing'))
      el.src = url
    })
    const canvas = document.createElement('canvas')
    canvas.width = dims.width
    canvas.height = dims.height
    const ctx = canvas.getContext('2d')
    if (!ctx) throw new Error('Canvas context unavailable')
    ctx.filter = adjustCssFilter.value || 'none'
    ctx.drawImage(img, 0, 0, dims.width, dims.height)
    ctx.filter = 'none'
    if (adjustState.value.sharpness > 0) {
      applyUnsharpMask(ctx, dims.width, dims.height, adjustState.value.sharpness)
    }
    // Text layers, in list order (later layers draw on top).
    for (const layer of textLayers.value) {
      if (!layer.visible || !layer.text.trim()) continue
      const px = layer.x * dims.width
      const py = layer.y * dims.height
      const fontSize = (layer.size / 1000) * dims.width
      ctx.font = `700 ${fontSize}px -apple-system, BlinkMacSystemFont, sans-serif`
      ctx.fillStyle = layer.color
      ctx.textAlign = 'center'
      ctx.textBaseline = 'middle'
      ctx.fillText(layer.text, px, py)
    }
    const outBlob = await canvasToBlob(canvas, 'image/png')
    return new Uint8Array(await outBlob.arrayBuffer())
  } finally {
    URL.revokeObjectURL(url)
  }
}

// ==================== Processing ====================
const runPipeline = async () => {
  if (!currentBytes) {
    setStatus('Add an image first.', 'error')
    return
  }
  setProcessingState(true)
  activeStep.value = 3
  setStatus('Preparing WebAssembly module...', 'busy')
  try {
    await ensureWasm()
    let bytes = currentBytes
    if (activeMode === 'crop') {
      if (!currentDims) throw new Error('Crop needs image dimensions')
      const x = Math.max(0, Math.round(cropRect.x * currentDims.width))
      const y = Math.max(0, Math.round(cropRect.y * currentDims.height))
      const w = Math.max(1, Math.round(cropRect.w * currentDims.width))
      const h = Math.max(1, Math.round(cropRect.h * currentDims.height))
      setStatus('Cropping...', 'busy')
      bytes = crop_image(bytes, x, y, w, h)
    } else if (activeMode === 'perspective') {
      if (!currentDims) throw new Error('Perspective crop needs image dimensions')
      const { width, height } = currentDims
      const pts = new Float32Array([
        perspPoints.tl.x * width,
        perspPoints.tl.y * height,
        perspPoints.tr.x * width,
        perspPoints.tr.y * height,
        perspPoints.br.x * width,
        perspPoints.br.y * height,
        perspPoints.bl.x * width,
        perspPoints.bl.y * height,
      ])
      setStatus('Applying perspective crop...', 'busy')
      bytes = perspective_crop(bytes, pts, width, height)
    } else if (activeMode === 'rotate') {
      const { degrees, flipH, flipV } = rotateState.value
      if (degrees !== 0 || flipH || flipV) {
        setStatus('Rotating...', 'busy')
        bytes = rotate_image(bytes, degrees, flipH, flipV)
      }
    } else if (activeMode === 'resize') {
      const w = Math.max(1, Math.round(resizeState.value.width))
      const h = Math.max(1, Math.round(resizeState.value.height))
      if (!currentDims || w !== currentDims.width || h !== currentDims.height) {
        setStatus('Resizing...', 'busy')
        bytes = resize_image(bytes, w, h, resizeState.value.filter)
      }
    }
    const postOpDims = await extractDimensions(bytes).catch(() => currentDims)
    if (postOpDims && (hasAdjustments.value || textLayers.value.length)) {
      setStatus('Applying adjustments...', 'busy')
      bytes = await compositeForExport(bytes, postOpDims)
    }
    const { format, quality } = getOutputSettings()
    setStatus('Compressing...', 'busy')
    bytes = compress_image(bytes, quality, format)
    currentBytes = bytes
    await updatePreview(bytes)
    setMode('none', { silent: true })
    setStatus('Done. Preview updated.', 'success')
    activeStep.value = 4
  } catch (err) {
    console.error(err)
    setStatus(err instanceof Error ? err.message : String(err), 'error')
    activeStep.value = 2
  } finally {
    setProcessingState(false)
  }
}

const downloadResult = async () => {
  if (!currentBytes) return
  const { format, quality } = getOutputSettings()
  setStatus('Preparing download...', 'busy')
  try {
    await ensureWasm()
    if (!compress_image) throw new Error('WASM not loaded')
    // If Adjust/Filters/Text haven't been baked in via Process yet, composite them now so
    // the download always matches what's shown in the live preview.
    let sourceBytes = currentBytes
    if (hasAdjustments.value || textLayers.value.some((l) => l.visible && l.text.trim())) {
      const dims = currentDims ?? (await extractDimensions(sourceBytes).catch(() => null))
      if (dims) sourceBytes = await compositeForExport(sourceBytes, dims)
    }
    // Always export through the same compress step used for the size estimate,
    // so the downloaded file matches the format/quality shown next to the button.
    const bytes = compress_image(sourceBytes, quality, format)
    const blob = new Blob([bytesToBlobPart(bytes)], { type: `image/${format}` })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    const ext = format === 'jpeg' ? 'jpg' : format
    a.href = url
    a.download = `image-tools-${Date.now()}.${ext}`
    document.body.appendChild(a)
    a.click()
    a.remove()
    URL.revokeObjectURL(url)
    setDownloadLabel(bytes.length)
    setStatus(`Downloaded. ${prettySize(bytes.length)}.`, 'success')
    activeStep.value = 4
  } catch (err) {
    setStatus(err instanceof Error ? err.message : String(err), 'error')
  }
}

const resetImage = async () => {
  if (!originalBytes) return
  currentBytes = originalBytes
  cropRect = { x: 0, y: 0, w: 1, h: 1 }
  perspPoints = { tl: { x: 0, y: 0 }, tr: { x: 1, y: 0 }, br: { x: 1, y: 1 }, bl: { x: 0, y: 1 } }
  rotateState.value = { degrees: 0, flipH: false, flipV: false }
  adjustState.value = { ...DEFAULT_ADJUST }
  activeFilter.value = 'none'
  textLayers.value = []
  selectedLayerId.value = null
  setMode('crop', { silent: true })
  const dims = await extractDimensions(currentBytes).catch(() => currentDims)
  if (dims) {
    resizeState.value = { width: dims.width, height: dims.height, lockAspect: true, filter: 'lanczos3' }
    resizeAspect = dims.width / dims.height
  }
  await updatePreview(currentBytes)
  setStatus('Back to the original file.')
  activeStep.value = 2
}

// ==================== Dropzone ====================
const bindDropzone = () => {
  if (!dropzone) return

  const prevent = (event: DragEvent) => {
    event.preventDefault()
    event.stopPropagation()
  }

  const activate = () => dropzone!.classList.add('drop-active')
  const deactivate = () => dropzone!.classList.remove('drop-active')

  ;['dragenter', 'dragover', 'dragleave', 'drop'].forEach((eventName) => {
    addListener(dropzone!, eventName, prevent as EventListener, false)
    addListener(document.body, eventName, prevent as EventListener, false)
  })

  ;['dragenter', 'dragover'].forEach((eventName) => {
    addListener(dropzone!, eventName, activate as EventListener, false)
  })

  ;['dragleave', 'drop'].forEach((eventName) => {
    addListener(dropzone!, eventName, deactivate as EventListener, false)
  })

  const onDrop = (event: DragEvent) => {
    const [file] = event.dataTransfer?.files || []
    if (file) handleFile(file)
  }

  const onClick = () => {
    if (fileInput) fileInput.click()
  }

  addListener(dropzone, 'drop', onDrop as EventListener)
  addListener(dropzone, 'click', onClick as EventListener)
}

const bindKeyboardShortcuts = () => {
  const onKeyDown = (event: KeyboardEvent) => {
    const cmdOrCtrl = event.metaKey || event.ctrlKey
    if (cmdOrCtrl && event.key.toLowerCase() === 'o') {
      event.preventDefault()
      fileInput?.click()
      return
    }
    if (cmdOrCtrl && event.key === 'Enter') {
      event.preventDefault()
      if (!processBtn?.disabled) runPipeline()
      return
    }
    if (cmdOrCtrl && event.key.toLowerCase() === 's' && currentBytes) {
      event.preventDefault()
      downloadResult()
    }
  }

  addListener(window, 'keydown', onKeyDown as EventListener)
}

// ==================== Initialization ====================
const initializeTooling = () => {
  fileInput = document.getElementById('fileInput') as HTMLInputElement
  dropzone = document.getElementById('dropzone')
  statusEl = document.getElementById('status')
  previewImg = document.getElementById('preview') as HTMLImageElement
  metaEl = document.getElementById('meta')
  processBtn = document.getElementById('processBtn') as HTMLButtonElement
  downloadBtn = document.getElementById('downloadBtn') as HTMLButtonElement
  resetBtn = document.getElementById('resetBtn') as HTMLButtonElement
  modeCropBtn = document.getElementById('modeCrop') as HTMLButtonElement
  modePerspectiveBtn = document.getElementById('modePerspective') as HTMLButtonElement
  cropBox = document.getElementById('cropBox')
  cropSizeEl = document.getElementById('cropSize')
  perspOverlay = document.getElementById('perspOverlay')
  perspSvg = document.getElementById('perspSvg') as unknown as SVGSVGElement
  perspPolygon = document.getElementById('perspPolygon') as unknown as SVGPolygonElement
  perspectiveHandleElements = Array.from(document.querySelectorAll('.persp-handle'))
  perspectiveHandles = perspectiveHandleElements.reduce(
    (acc, el) => {
      const key = el.getAttribute('data-point')
      if (key) acc[key] = el
      return acc
    },
    {} as Record<string, Element>
  )
  formatSelect = document.getElementById('formatSelect') as HTMLSelectElement
  qualityRow = document.getElementById('qualityRow')
  qualityInput = document.getElementById('qualityInput') as HTMLInputElement
  qualityValue = document.getElementById('qualityValue')
  statsEl = document.getElementById('stats')

  if (fileInput) {
    addListener(fileInput, 'change', ((event: Event) => {
      const [file] = (event.target as HTMLInputElement).files || []
      if (file) handleFile(file)
    }) as EventListener)
  }

  if (processBtn) addListener(processBtn, 'click', runPipeline as EventListener)
  if (downloadBtn) addListener(downloadBtn, 'click', downloadResult as EventListener)
  if (resetBtn) addListener(resetBtn, 'click', resetImage as EventListener)

  if (modeCropBtn) {
    addListener(modeCropBtn, 'click', ((event: Event) => {
      event.preventDefault()
      setMode('crop')
    }) as EventListener)
  }

  if (modePerspectiveBtn) {
    addListener(modePerspectiveBtn, 'click', ((event: Event) => {
      event.preventDefault()
      setMode('perspective')
    }) as EventListener)
  }

  if (qualityInput) {
    addListener(qualityInput, 'input', handleQualityChange as EventListener)
    addListener(qualityInput, 'change', handleQualityChange as EventListener)
  }

  if (formatSelect) {
    addListener(formatSelect, 'change', (() => {
      updateQualityVisibility()
      requestProjectedDownloadSize()
    }) as EventListener)
  }

  bindDropzone()
  bindKeyboardShortcuts()
  setStatus('Drop an image or browse to get started.')
  setProcessingState(false)
  if (downloadBtn) downloadBtn.disabled = true
  if (resetBtn) resetBtn.disabled = true
  setDownloadLabel(null)
  updateStats(undefined)
  updateModeButtons()
  updateQualityLabel()
  updateQualityVisibility()
  updateCropOverlay()
  updatePerspectiveOverlay()
  attachCropInteractions()
  attachPerspectiveInteractions()

  addListener(window, 'resize', (() => {
    updateCropOverlay()
    updatePerspectiveOverlay()
  }) as EventListener)

  if (previewImg) {
    addListener(previewImg, 'load', (() => {
      requestAnimationFrame(() => {
        updateCropOverlay()
        updatePerspectiveOverlay()
      })
    }) as EventListener)
  }
}

onMounted(async () => {
  await nextTick()
  initializeTooling()
})

onUnmounted(() => {
  cancelProjectedSize()
  cleanupFns.splice(0).forEach((cleanup) => cleanup())
  if (previewUrl) {
    URL.revokeObjectURL(previewUrl)
    previewUrl = null
  }
})
</script>

<template>
  <div class="min-h-screen flex flex-col bg-[var(--surface-0)] text-[var(--ink-1)]">

    <!-- Top Bar -->
    <header class="h-16 shrink-0 border-b border-[var(--border)] flex items-center justify-between px-4 md:px-6 sticky top-0 z-30 bg-[var(--surface-1)]">
      <div class="flex items-center gap-3 md:gap-8 min-w-0">
        <div class="flex items-center gap-2 shrink-0">
          <NuxtLink to="/" class="ide-tool-btn flex items-center justify-center h-8 w-8 rounded-none shrink-0" title="Back to home" aria-label="Back to home">
            <svg width="16" height="16" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6"><path stroke-linecap="round" stroke-linejoin="round" d="M11.5 4L6 9.5l5.5 5.5" /></svg>
          </NuxtLink>
          <div class="w-8 h-8 rounded-none flex items-center justify-center shrink-0 bg-[var(--ink-1)]">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M2.25 15.75l5.159-5.159a2.25 2.25 0 013.182 0l5.159 5.159m-1.5-1.5l1.409-1.409a2.25 2.25 0 013.182 0l2.909 2.909M3 6h18M3 6a2.25 2.25 0 00-2.25 2.25v11.25A2.25 2.25 0 003 21.75h18a2.25 2.25 0 002.25-2.25V8.25A2.25 2.25 0 0021 6H3z" /></svg>
          </div>
          <span class="font-bold text-[15px] tracking-tight hidden sm:block">Image Editor</span>
        </div>
        <span class="hidden lg:block text-sm font-mono truncate text-[var(--ink-3)]">local · offline · nothing uploaded</span>
      </div>

      <div class="flex items-center gap-1.5 md:gap-2 shrink-0">
        <button id="resetBtn" :disabled="!hasImage" class="ui-button-secondary hidden sm:flex h-9 px-3.5 items-center gap-1.5 text-sm disabled:opacity-40 disabled:pointer-events-none">
          <span>Reset</span>
        </button>
        <button id="downloadBtn" :disabled="!hasImage" class="ui-button h-9 px-4 text-sm font-bold flex items-center gap-2 disabled:opacity-40 disabled:pointer-events-none">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="7 10 12 15 17 10"></polyline><line x1="12" y1="15" x2="12" y2="3"></line></svg>
          <span>Download</span>
        </button>
      </div>
    </header>

    <div class="flex flex-1 min-h-0 flex-col md:flex-row">

      <!-- Left rail: mode switch (real feature set: crop / perspective) -->
      <aside class="order-2 md:order-1 shrink-0 border-t md:border-t-0 md:border-r border-[var(--border)] md:w-[76px] w-full bg-[var(--surface-1)]">
        <div class="flex md:flex-col items-center gap-1 p-2 md:py-4 overflow-x-auto md:overflow-visible" :class="(!hasImage || isProcessing) && 'opacity-40 pointer-events-none'">
          <button id="modeCrop" data-mode="crop"
            class="rail-btn ide-tool-btn active shrink-0 w-14 h-14 rounded-none flex flex-col items-center justify-center gap-1" title="Crop" aria-label="Crop">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"><path d="M6 2v14a2 2 0 0 0 2 2h14"></path><path d="M18 22V8a2 2 0 0 0-2-2H2"></path></svg>
            <span class="text-[10px] font-medium">Crop</span>
          </button>
          <button id="modePerspective" data-mode="perspective"
            class="rail-btn ide-tool-btn shrink-0 w-14 h-14 rounded-none flex flex-col items-center justify-center gap-1" title="Perspective" aria-label="Perspective">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"><path d="M4 6l6-2 8 2 2-1v14l-2 1-8-2-6 2V6z"></path></svg>
            <span class="text-[10px] font-medium leading-tight text-center">Perspective</span>
          </button>
          <button type="button" @click="setMode('rotate')"
            class="rail-btn ide-tool-btn shrink-0 w-14 h-14 rounded-none flex flex-col items-center justify-center gap-1"
            :class="activeModeDisplay === 'rotate' && 'active'" title="Rotate" aria-label="Rotate">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-3.02-6.71"></path><polyline points="21 3 21 9 15 9"></polyline></svg>
            <span class="text-[10px] font-medium">Rotate</span>
          </button>
          <button type="button" @click="setMode('resize')"
            class="rail-btn ide-tool-btn shrink-0 w-14 h-14 rounded-none flex flex-col items-center justify-center gap-1"
            :class="activeModeDisplay === 'resize' && 'active'" title="Resize" aria-label="Resize">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"><polyline points="15 3 21 3 21 9"></polyline><polyline points="9 21 3 21 3 15"></polyline><line x1="21" y1="3" x2="14" y2="10"></line><line x1="3" y1="21" x2="10" y2="14"></line></svg>
            <span class="text-[10px] font-medium">Resize</span>
          </button>
          <div class="w-8 border-t md:border-t-0 md:w-8 md:h-px my-1 border-[var(--border)]"></div>
          <button type="button" @click="setMode('adjust')"
            class="rail-btn ide-tool-btn shrink-0 w-14 h-14 rounded-none flex flex-col items-center justify-center gap-1"
            :class="activeModeDisplay === 'adjust' && 'active'" :disabled="!hasImage" title="Adjust" aria-label="Adjust">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"><line x1="4" y1="21" x2="4" y2="14"></line><line x1="4" y1="10" x2="4" y2="3"></line><line x1="12" y1="21" x2="12" y2="12"></line><line x1="12" y1="8" x2="12" y2="3"></line><line x1="20" y1="21" x2="20" y2="16"></line><line x1="20" y1="12" x2="20" y2="3"></line><line x1="1" y1="14" x2="7" y2="14"></line><line x1="9" y1="8" x2="15" y2="8"></line><line x1="17" y1="16" x2="23" y2="16"></line></svg>
            <span class="text-[10px] font-medium">Adjust</span>
          </button>
          <button type="button" @click="setMode('filters')"
            class="rail-btn ide-tool-btn shrink-0 w-14 h-14 rounded-none flex flex-col items-center justify-center gap-1"
            :class="activeModeDisplay === 'filters' && 'active'" :disabled="!hasImage" title="Filters" aria-label="Filters">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"><circle cx="9" cy="9" r="6"></circle><circle cx="15" cy="15" r="6" opacity="0.5"></circle></svg>
            <span class="text-[10px] font-medium">Filters</span>
          </button>
          <button type="button" @click="setMode('text')"
            class="rail-btn ide-tool-btn shrink-0 w-14 h-14 rounded-none flex flex-col items-center justify-center gap-1"
            :class="activeModeDisplay === 'text' && 'active'" :disabled="!hasImage" title="Text" aria-label="Text">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 7 4 4 20 4 20 7"></polyline><line x1="9" y1="20" x2="15" y2="20"></line><line x1="12" y1="4" x2="12" y2="20"></line></svg>
            <span class="text-[10px] font-medium">Text</span>
          </button>
          <button type="button" @click="setMode('layers')"
            class="rail-btn ide-tool-btn shrink-0 w-14 h-14 rounded-none flex flex-col items-center justify-center gap-1"
            :class="activeModeDisplay === 'layers' && 'active'" :disabled="!hasImage" title="Layers" aria-label="Layers">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 2 7 12 12 22 7 12 2"></polygon><polyline points="2 17 12 22 22 17"></polyline><polyline points="2 12 12 17 22 12"></polyline></svg>
            <span class="text-[10px] font-medium">Layers</span>
          </button>
        </div>
      </aside>

      <!-- Main Canvas -->
      <main class="order-1 md:order-2 flex-1 min-w-0 flex flex-col bg-[var(--surface-2)]">
        <!-- Canvas toolbar -->
        <div class="h-12 shrink-0 flex items-center justify-between px-4 md:px-8 border-b border-[var(--border)]">
          <div id="meta" class="text-sm mono text-[var(--ink-3)]"></div>
          <div id="stats" class="text-xs mono text-[var(--ink-3)]" aria-live="polite"></div>
        </div>

        <!-- Canvas stage -->
        <div class="flex-1 flex items-center justify-center p-6 md:p-12 overflow-auto">
          <div class="preview-stage relative w-full max-w-[720px] rounded-none border border-[var(--border)]" id="previewStage">
            <div class="preview-frame">
              <img id="preview" alt="Preview" class="w-full h-auto rounded-none"
                :style="{ filter: adjustCssFilter, transform: `rotate(${rotateState.degrees}deg) scaleX(${rotateState.flipH ? -1 : 1}) scaleY(${rotateState.flipV ? -1 : 1})`, transition: 'transform 200ms cubic-bezier(0.23, 1, 0.32, 1)' }"
                src="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 1280 720'%3E%3Cdefs%3E%3ClinearGradient id='g' x1='0' y1='0' x2='1' y2='1'%3E%3Cstop offset='0%25' stop-color='%23e8dfce'/%3E%3Cstop offset='28%25' stop-color='%23cbb896'/%3E%3Cstop offset='52%25' stop-color='%239a8768'/%3E%3Cstop offset='74%25' stop-color='%235c6b56'/%3E%3Cstop offset='100%25' stop-color='%2333443a'/%3E%3C/linearGradient%3E%3C/defs%3E%3Crect width='1280' height='720' fill='url(%23g)'/%3E%3Ccircle cx='980' cy='160' r='120' fill='%23f6dfa1' opacity='0.7'/%3E%3Ctext x='120' y='340' fill='%23fdfaf3' font-size='44' font-family='-apple-system,sans-serif' font-weight='600'%3EDrop an image or browse%3C/text%3E%3Ctext x='120' y='384' fill='%23eee7d6' font-size='22' font-family='-apple-system,sans-serif'%3EEverything runs locally on your device%3C/text%3E%3C/svg%3E" />
              <!-- Text layer overlay (live preview only; baked into export via canvas) -->
              <div v-if="hasImage" class="absolute inset-0 pointer-events-none rounded-none overflow-hidden">
                <div
                  v-for="layer in textLayers"
                  :key="layer.id"
                  v-show="layer.visible && layer.text.trim()"
                  class="absolute font-bold whitespace-nowrap"
                  :style="{ left: `${layer.x * 100}%`, top: `${layer.y * 100}%`, fontSize: `${layer.size / 12}px`, color: layer.color, transform: 'translate(-50%, -50%)', textShadow: '0 1px 3px rgba(0,0,0,0.4)' }"
                >{{ layer.text }}</div>
              </div>
              <!-- Crop Box (tool-specific markup) -->
              <div id="cropBox" class="crop-box">
                <div id="cropSize" class="crop-size" aria-live="polite"></div>
                <span class="handle nw" data-handle="nw"></span>
                <span class="handle ne" data-handle="ne"></span>
                <span class="handle sw" data-handle="sw"></span>
                <span class="handle se" data-handle="se"></span>
              </div>
              <!-- Perspective Overlay (tool-specific SVG) -->
              <div id="perspOverlay" class="persp-overlay">
                <svg id="perspSvg" class="persp-svg" xmlns="http://www.w3.org/2000/svg">
                  <polygon id="perspPolygon" class="persp-polygon"></polygon>
                </svg>
                <span class="persp-handle" data-point="tl"></span>
                <span class="persp-handle" data-point="tr"></span>
                <span class="persp-handle" data-point="br"></span>
                <span class="persp-handle" data-point="bl"></span>
              </div>
            </div>
          </div>
        </div>
      </main>

      <!-- Right Panel -->
      <aside class="order-3 shrink-0 w-full md:w-[300px] border-t md:border-t-0 md:border-l border-[var(--border)] flex flex-col max-h-[60vh] md:max-h-none overflow-y-auto bg-[var(--surface-1)]">
        <div class="p-4 md:p-6 flex flex-col gap-6">

          <!-- Upload card: dropzone/fileInput stay permanently mounted (drag/drop + Ctrl/Cmd+O
               listeners are bound to them once on mount), only the presentation shrinks once
               an image is loaded. -->
          <section class="ui-panel p-4">
            <h2 v-if="!hasImage" class="text-[11px] font-bold uppercase tracking-wider mb-3 text-[var(--ink-3)]">Open an image</h2>
            <label id="dropzone"
              class="ui-dropzone flex items-center gap-2 cursor-pointer"
              :class="hasImage ? 'flex-row justify-center p-3' : 'flex-col justify-center text-center gap-2 p-6'"
              aria-label="Upload image by click or drag and drop">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" class="text-[var(--ink-3)]" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><path d="M12 3v12"></path><path d="m7 8 5-5 5 5"></path><path d="M5 17v2a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2v-2"></path></svg>
              <span class="text-sm font-medium">{{ hasImage ? 'Open a different image' : 'Drop image or browse' }}</span>
              <span v-if="!hasImage" class="text-xs mono text-[var(--ink-3)]">PNG, JPEG, WebP, HEIC</span>
            </label>
            <input id="fileInput" type="file" accept="image/*,.heic,.heif" class="hidden" />
            <p v-if="!hasImage" class="mt-3 text-xs text-[var(--ink-3)]">Shortcuts: Ctrl/Cmd+O upload, Ctrl/Cmd+Enter process, Ctrl/Cmd+S download.</p>
          </section>

          <!-- Mode card: crop / perspective / rotate / resize -->
          <section v-if="['crop', 'perspective', 'rotate', 'resize', 'none'].includes(activeModeDisplay)" class="ui-panel p-4" :class="!hasImage && 'opacity-40 pointer-events-none'">
            <div class="flex items-center justify-between mb-4">
              <h2 class="text-[11px] font-bold uppercase tracking-wider text-[var(--ink-3)]">
                {{ activeModeDisplay === 'perspective' ? 'Perspective' : activeModeDisplay === 'rotate' ? 'Rotate' : activeModeDisplay === 'resize' ? 'Resize' : 'Crop' }}
              </h2>
              <span class="text-xs mono text-[var(--ink-3)]">{{ activeModeDisplay === 'none' ? 'none' : activeModeDisplay }}</span>
            </div>

            <p v-if="activeModeDisplay === 'perspective'" class="text-xs leading-relaxed mb-4 text-[var(--ink-2)]">
              Drag the four corners to straighten a document or whiteboard shot.
            </p>
            <p v-else-if="activeModeDisplay === 'crop' || activeModeDisplay === 'none'" class="text-xs leading-relaxed mb-4 text-[var(--ink-2)]">
              Drag the handles on the canvas to choose the exact area you want.
            </p>

            <!-- Rotate controls -->
            <div v-if="activeModeDisplay === 'rotate'" class="flex flex-col gap-3 mb-4">
              <div class="flex items-center gap-2">
                <button type="button" @click="rotateState.degrees = ((rotateState.degrees - 90) % 360 + 360) % 360"
                  class="ui-button-secondary flex-1 h-10 flex items-center justify-center gap-2 text-sm">
                  <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 1 0 3.02-6.71"></path><polyline points="3 3 3 9 9 9"></polyline></svg>
                  90° Left
                </button>
                <button type="button" @click="rotateState.degrees = (rotateState.degrees + 90) % 360"
                  class="ui-button-secondary flex-1 h-10 flex items-center justify-center gap-2 text-sm">
                  <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-3.02-6.71"></path><polyline points="21 3 21 9 15 9"></polyline></svg>
                  90° Right
                </button>
              </div>
              <div class="flex items-center gap-2">
                <button type="button" @click="rotateState.flipH = !rotateState.flipH"
                  :class="rotateState.flipH ? 'ui-button' : 'ui-button-secondary'" class="flex-1 h-10 flex items-center justify-center gap-2 text-sm">
                  <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h3"></path><path d="M16 3h3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-3"></path><line x1="12" y1="2" x2="12" y2="22"></line></svg>
                  Flip H
                </button>
                <button type="button" @click="rotateState.flipV = !rotateState.flipV"
                  :class="rotateState.flipV ? 'ui-button' : 'ui-button-secondary'" class="flex-1 h-10 flex items-center justify-center gap-2 text-sm">
                  <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 8V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v3"></path><path d="M21 16v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-3"></path><line x1="2" y1="12" x2="22" y2="12"></line></svg>
                  Flip V
                </button>
              </div>
              <p class="text-xs leading-relaxed mono text-[var(--ink-3)]">{{ rotateState.degrees }}° rotation{{ rotateState.flipH ? ', flipped horizontally' : '' }}{{ rotateState.flipV ? ', flipped vertically' : '' }}</p>
            </div>

            <!-- Resize controls -->
            <div v-else-if="activeModeDisplay === 'resize'" class="flex flex-col gap-3 mb-4">
              <div class="flex items-center gap-2">
                <div class="flex-1">
                  <label for="resizeWidth" class="text-xs mb-1 block text-[var(--ink-3)]">Width</label>
                  <input id="resizeWidth" type="number" min="1" :value="resizeState.width"
                    @input="onResizeWidthChange(Number(($event.target as HTMLInputElement).value))"
                    class="ui-input mono !h-9 !py-0">
                </div>
                <button type="button" @click="resizeState.lockAspect = !resizeState.lockAspect" title="Lock aspect ratio"
                  class="h-9 w-9 mt-5 rounded-none flex items-center justify-center shrink-0"
                  :class="resizeState.lockAspect ? 'ui-button' : 'ui-button-secondary'">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2"></rect><path d="M7 11V7a5 5 0 0 1 10 0v4"></path></svg>
                </button>
                <div class="flex-1">
                  <label for="resizeHeight" class="text-xs mb-1 block text-[var(--ink-3)]">Height</label>
                  <input id="resizeHeight" type="number" min="1" :value="resizeState.height"
                    @input="onResizeHeightChange(Number(($event.target as HTMLInputElement).value))"
                    class="ui-input mono !h-9 !py-0">
                </div>
              </div>
              <div class="relative">
                <select v-model="resizeState.filter" class="ui-input w-full !h-9 !py-0 text-sm font-bold appearance-none pl-3 pr-8 cursor-pointer">
                  <option value="lanczos3">Lanczos3 (sharp)</option>
                  <option value="catmullrom">CatmullRom (balanced)</option>
                  <option value="triangle">Triangle (soft)</option>
                  <option value="nearest">Nearest (pixelated)</option>
                </select>
                <svg class="absolute right-2.5 top-2.5 w-4 h-4 pointer-events-none text-[var(--ink-3)]" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
                </svg>
              </div>
            </div>
          </section>

          <!-- Process card: always mounted regardless of active panel -->
          <section class="ui-panel p-4" :class="!hasImage && 'opacity-40 pointer-events-none'">
            <button id="processBtn" class="ui-button w-full h-11 text-sm font-bold flex items-center justify-center gap-2">
              Process
            </button>
          </section>

          <!-- Adjust tab -->
          <section v-if="activeModeDisplay === 'adjust'" class="ui-panel p-4" :class="!hasImage && 'opacity-40 pointer-events-none'">
            <h2 class="text-[11px] font-bold uppercase tracking-wider mb-4 text-[var(--ink-3)]">Adjustments</h2>
            <div class="flex flex-col gap-4">
              <div v-for="field in [
                { key: 'brightness', label: 'Brightness' },
                { key: 'contrast', label: 'Contrast' },
                { key: 'saturation', label: 'Saturation' },
                { key: 'temperature', label: 'Temperature' },
                { key: 'sharpness', label: 'Sharpness' },
              ]" :key="field.key">
                <div class="flex items-center justify-between mb-2">
                  <label :for="field.key" class="text-sm text-[var(--ink-2)]">{{ field.label }}</label>
                  <span class="text-xs mono tabular-nums text-[var(--ink-3)]">{{ (adjustState as any)[field.key] > 0 ? '+' : '' }}{{ (adjustState as any)[field.key] }}</span>
                </div>
                <input :id="field.key" type="range" :min="field.key === 'sharpness' ? 0 : -100" max="100"
                  v-model.number="(adjustState as any)[field.key]" class="ui-range ide-range w-full">
              </div>
            </div>
            <button v-if="hasAdjustments" type="button" @click="adjustState = { ...DEFAULT_ADJUST }"
              class="mt-4 text-xs font-bold underline text-[var(--ink-3)]">Reset adjustments</button>
          </section>

          <!-- Filters tab -->
          <section v-if="activeModeDisplay === 'filters'" class="ui-panel p-4" :class="!hasImage && 'opacity-40 pointer-events-none'">
            <h2 class="text-[11px] font-bold uppercase tracking-wider mb-4 text-[var(--ink-3)]">Filters</h2>
            <div class="grid grid-cols-3 gap-2.5">
              <button v-for="f in [
                { key: 'none', label: 'None' },
                { key: 'grayscale', label: 'Grayscale' },
                { key: 'sepia', label: 'Sepia' },
                { key: 'blur', label: 'Blur' },
                { key: 'vivid', label: 'Vivid' },
                { key: 'fade', label: 'Fade' },
              ]" :key="f.key" type="button" @click="activeFilter = f.key as FilterPreset"
                class="chip-btn flex flex-col items-center gap-1.5 group" :aria-label="`${f.label} filter`">
                <span class="w-full aspect-square rounded-none relative overflow-hidden border"
                  :style="{ filter: FILTER_CSS[f.key as FilterPreset] || 'none', background: 'linear-gradient(160deg, #e8dfce 0%, #cbb896 28%, #9a8768 52%, #5c6b56 74%, #33443a 100%)', borderColor: 'var(--border)', outline: activeFilter === f.key ? '2px solid var(--accent)' : 'none', outlineOffset: '2px' }"></span>
                <span class="text-[11px] font-bold" :class="activeFilter === f.key ? 'text-[var(--ink-1)]' : 'text-[var(--ink-3)]'">{{ f.label }}</span>
              </button>
            </div>
          </section>

          <!-- Text tab -->
          <section v-if="activeModeDisplay === 'text'" class="ui-panel p-4" :class="!hasImage && 'opacity-40 pointer-events-none'">
            <div class="flex items-center justify-between mb-4">
              <h2 class="text-[11px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Text</h2>
              <button type="button" @click="addTextLayer" class="ui-button-secondary w-6 h-6 rounded-none flex items-center justify-center !p-0" aria-label="Add text layer">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
              </button>
            </div>

            <p v-if="!textLayers.length" class="text-xs leading-relaxed text-[var(--ink-3)]">No captions yet. Add one to overlay text on the export.</p>

            <div v-else class="flex flex-col gap-3">
              <button type="button" v-for="layer in textLayers" :key="layer.id" @click="selectedLayerId = layer.id"
                class="text-left rounded-none px-2.5 py-2 text-sm truncate"
                :class="selectedLayerId === layer.id ? 'bg-[var(--surface-2)] text-[var(--ink-1)]' : 'text-[var(--ink-2)]'">
                {{ layer.text || '(empty)' }}
              </button>

              <div v-if="selectedLayer" class="flex flex-col gap-3 pt-1 border-t border-[var(--border)]">
                <input type="text" v-model="selectedLayer.text" placeholder="Caption text" class="ui-input !h-9 !py-0 text-sm">
                <div class="flex items-center gap-2">
                  <input type="color" v-model="selectedLayer.color" class="h-9 w-9 rounded-none cursor-pointer shrink-0 border border-[var(--border)]">
                  <input type="range" min="16" max="160" v-model.number="selectedLayer.size" class="ui-range ide-range flex-1" aria-label="Text size">
                  <button type="button" @click="selectedLayer.visible = !selectedLayer.visible"
                    class="ui-button-secondary h-9 w-9 rounded-none flex items-center justify-center shrink-0 !p-0" :aria-label="selectedLayer.visible ? 'Hide layer' : 'Show layer'">
                    <svg v-if="selectedLayer.visible" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path><circle cx="12" cy="12" r="3"></circle></svg>
                    <svg v-else width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="1" y1="1" x2="23" y2="23"></line><path d="M10.58 10.58a2 2 0 0 0 2.83 2.83"></path><path d="M16.68 16.68A9.77 9.77 0 0 1 12 18c-7 0-11-8-11-8a18.5 18.5 0 0 1 4.06-5.06M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19"></path></svg>
                  </button>
                  <button type="button" @click="removeTextLayer(selectedLayer.id)"
                    class="ui-button-secondary h-9 w-9 rounded-none flex items-center justify-center shrink-0 !p-0 text-[var(--danger)]" aria-label="Delete layer">
                    <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"></path><path d="M10 11v6M14 11v6"></path></svg>
                  </button>
                </div>
                <div class="grid grid-cols-2 gap-3">
                  <div>
                    <label class="text-xs mb-1 block text-[var(--ink-3)]">Horizontal</label>
                    <input type="range" min="0" max="1" step="0.01" v-model.number="selectedLayer.x" class="ui-range ide-range w-full">
                  </div>
                  <div>
                    <label class="text-xs mb-1 block text-[var(--ink-3)]">Vertical</label>
                    <input type="range" min="0" max="1" step="0.01" v-model.number="selectedLayer.y" class="ui-range ide-range w-full">
                  </div>
                </div>
              </div>
            </div>
          </section>

          <!-- Layers tab -->
          <section v-if="activeModeDisplay === 'layers'" class="ui-panel p-4" :class="!hasImage && 'opacity-40 pointer-events-none'">
            <div class="flex items-center justify-between mb-4">
              <h2 class="text-[11px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Layers</h2>
              <button type="button" @click="addTextLayer(); setMode('text')" class="ui-button-secondary w-6 h-6 rounded-none flex items-center justify-center !p-0" aria-label="Add text layer">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
              </button>
            </div>

            <div class="flex flex-col gap-1.5">
              <div class="flex items-center gap-3 px-2.5 py-2 rounded-none bg-[var(--surface-2)] border border-[var(--border)]">
                <span class="w-9 h-9 rounded-none shrink-0 border border-[var(--border)]" style="background: linear-gradient(160deg, #e8dfce 0%, #cbb896 28%, #9a8768 52%, #5c6b56 74%, #33443a 100%)"></span>
                <span class="text-sm flex-1 min-w-0 truncate text-[var(--ink-1)]">Background</span>
              </div>
              <div v-for="layer in textLayers" :key="layer.id"
                class="flex items-center gap-3 px-2.5 py-2 rounded-none transition-colors cursor-pointer border border-transparent"
                :class="selectedLayerId === layer.id ? 'bg-[var(--surface-2)] border-[var(--border)]' : ''"
                @click="selectedLayerId = layer.id; setMode('text')">
                <span class="w-9 h-9 rounded-none shrink-0 flex items-center justify-center bg-[var(--surface-1)] border border-[var(--border)]">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" class="text-[var(--ink-2)]" stroke-width="1.7"><polyline points="4 7 4 4 20 4 20 7"></polyline><line x1="9" y1="20" x2="15" y2="20"></line><line x1="12" y1="4" x2="12" y2="20"></line></svg>
                </span>
                <span class="text-sm flex-1 min-w-0 truncate" :class="layer.visible ? 'text-[var(--ink-1)]' : 'text-[var(--ink-3)]'">{{ layer.text || '(empty)' }}</span>
                <button type="button" @click.stop="layer.visible = !layer.visible" class="shrink-0" :class="layer.visible ? 'text-[var(--ink-2)]' : 'text-[var(--ink-3)]'" :aria-label="layer.visible ? 'Hide layer' : 'Show layer'">
                  <svg v-if="layer.visible" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path><circle cx="12" cy="12" r="3"></circle></svg>
                  <svg v-else width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="1" y1="1" x2="23" y2="23"></line><path d="M10.58 10.58a2 2 0 0 0 2.83 2.83"></path><path d="M16.68 16.68A9.77 9.77 0 0 1 12 18c-7 0-11-8-11-8a18.5 18.5 0 0 1 4.06-5.06M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19"></path></svg>
                </button>
              </div>
              <p v-if="!textLayers.length" class="text-xs leading-relaxed px-2.5 text-[var(--ink-3)]">No layers yet besides the background.</p>
            </div>
          </section>

          <!-- Export card -->
          <section class="ui-panel p-4" :class="!hasImage && 'opacity-40 pointer-events-none'">
            <h2 class="text-[11px] font-bold uppercase tracking-wider mb-4 text-[var(--ink-3)]">Export</h2>

            <div id="qualityRow" class="mb-4">
              <div class="flex items-center justify-between mb-2">
                <label for="qualityInput" class="text-sm text-[var(--ink-2)]">Quality (JPEG)</label>
                <span id="qualityValue" class="text-xs mono tabular-nums text-[var(--ink-3)]">80%</span>
              </div>
              <input id="qualityInput" type="range" min="10" max="100" value="80" class="ui-range ide-range w-full" aria-label="Compression quality">
            </div>

            <div class="flex items-center gap-2 mb-4">
              <div class="relative flex-1">
                <select id="formatSelect" class="ui-input w-full !h-9 !py-0 text-sm font-bold appearance-none pl-3 pr-8 cursor-pointer">
                  <option value="png">PNG</option>
                  <option value="jpeg">JPEG</option>
                  <option value="webp" selected>WebP</option>
                </select>
                <svg class="absolute right-2.5 top-2.5 w-4 h-4 pointer-events-none text-[var(--ink-3)]" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
                </svg>
              </div>
            </div>
            <p class="text-xs text-[var(--ink-3)]">Choose output format first, then adjust quality if JPEG is selected.</p>
          </section>

          <!-- Status -->
          <section
            class="ui-panel flex items-start gap-2 px-3 py-2.5 transition-colors duration-150"
            :class="statusKind === 'error'
              ? 'bg-[var(--danger-soft)] text-[var(--danger)]'
              : statusKind === 'success'
                ? 'bg-[var(--success-soft)] text-[var(--success)]'
                : 'bg-[var(--surface-2)] text-[var(--ink-2)]'"
          >
            <span
              v-if="statusKind === 'busy'"
              class="mt-0.5 h-3 w-3 shrink-0 animate-spin rounded-full border-2 border-current border-t-transparent"
              aria-hidden="true"
            />
            <div id="status" class="mono text-xs leading-relaxed min-h-5"></div>
          </section>

        </div>
      </aside>

    </div>

    <!-- Status bar: upload -> adjust -> process -> download -->
    <footer class="shrink-0 border-t border-[var(--border)] h-9 flex items-center px-4 md:px-6 gap-4 text-[11px] mono bg-[var(--surface-1)]">
      <div
        v-for="(label, i) in ['Upload', 'Adjust', 'Process', 'Download']"
        :key="label"
        class="flex items-center gap-1.5"
        :class="activeStep >= i + 1 ? 'text-[var(--ink-1)]' : 'text-[var(--ink-3)]'"
      >
        <span
          class="status-dot w-2 h-2 shrink-0 inline-block rounded-full"
          :class="activeStep > i + 1 ? 'ok' : ''"
          :style="activeStep <= i + 1 ? { background: 'transparent', border: activeStep === i + 1 ? '1.5px solid var(--ink-1)' : '1.5px solid var(--ink-3)' } : {}"
        ></span>
        <span :class="activeStep >= i + 1 ? 'font-semibold' : ''">{{ label }}</span>
      </div>
    </footer>
  </div>
</template>

<style scoped>
/* Lumen-style palette for this page only: literal hex values match the
   .superdesign/design_iterations/image_editor_1.html reference mockup,
   independent of the site-wide CSS variable system used on other tool pages. */

* { -webkit-font-smoothing: antialiased; }

#previewStage {
  position: relative;
  max-width: 100%;
  height: auto;
}

.preview-frame {
  position: relative;
}

#previewStage img {
  display: block;
}

/* Icon-rail tooltip, matching the shared rail-btn pattern used across tool pages */
.rail-btn {
  position: relative;
  transition: transform 140ms cubic-bezier(0.23, 1, 0.32, 1), background-color 140ms ease, color 140ms ease;
}

.rail-btn:active {
  transform: scale(0.94);
}

/* Tool rail buttons (left aside) */
.ide-tool-btn {
  color: var(--ink-3);
}

.ide-tool-btn:hover {
  background: var(--surface-2);
  color: var(--ink-1);
}

.ide-tool-btn.active,
#modeCrop.active,
#modePerspective.active {
  background: var(--ink-1);
  color: #ffffff;
}

.ide-tool-btn:disabled {
  opacity: 0.4;
  pointer-events: none;
}

/* Dropzone */
.ide-dropzone {
  transition: border-color 150ms ease, background-color 150ms ease;
}

/* Range input (quality slider) */
.ide-range {
  -webkit-appearance: none;
  appearance: none;
  height: 2px;
  border-radius: 0;
  background: var(--border);
  outline: none;
}

.ide-range::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 14px;
  height: 14px;
  border-radius: 0;
  background: var(--ink-1);
  border: 2px solid #ffffff;
  box-shadow: 0 0 0 1px var(--border);
  cursor: pointer;
  transition: transform 140ms cubic-bezier(0.23, 1, 0.32, 1);
}

.ide-range::-webkit-slider-thumb:hover {
  transform: scale(1.15);
}

.ide-range::-moz-range-thumb {
  width: 14px;
  height: 14px;
  border-radius: 0;
  background: var(--ink-1);
  border: 2px solid #ffffff;
  box-shadow: 0 0 0 1px var(--border);
  cursor: pointer;
}

/* Crop overlay */
.crop-box {
  position: absolute;
  top: 0;
  left: 0;
  border: 2px solid var(--ink-1);
  background: rgba(10, 10, 10, 0.08);
  cursor: move;
  display: none;
  z-index: 10;
}

.crop-box.active {
  display: block;
}

.crop-size {
  position: absolute;
  top: -24px;
  left: 0;
  background: var(--ink-1);
  color: #ffffff;
  font-family: var(--font-mono, ui-monospace, monospace);
  font-size: 12px;
  padding: 4px 8px;
  border-radius: 0;
  white-space: nowrap;
  z-index: 20;
}

.handle {
  position: absolute;
  width: 16px;
  height: 16px;
  background: var(--ink-1);
  border: 2px solid #ffffff;
  box-shadow: 0 0 0 1px var(--border);
  border-radius: 0;
  cursor: pointer;
  z-index: 25;
  transition: transform 0.15s ease, background-color 0.15s ease;
}

.handle:hover {
  transform: scale(1.1);
  background: var(--accent-hover);
}

.handle:active {
  transform: scale(0.95);
}

.handle.nw {
  top: -9px;
  left: -9px;
  cursor: nwse-resize;
}

.handle.ne {
  top: -9px;
  right: -9px;
  cursor: nesw-resize;
}

.handle.sw {
  bottom: -9px;
  left: -9px;
  cursor: nesw-resize;
}

.handle.se {
  bottom: -9px;
  right: -9px;
  cursor: nwse-resize;
}

.persp-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: none;
  z-index: 10;
}

.persp-overlay.active {
  display: block;
}

.persp-svg {
  width: 100%;
  height: 100%;
  position: absolute;
  top: 0;
  left: 0;
}

.persp-polygon {
  fill: none;
  stroke: var(--ink-1);
  stroke-width: 2;
  pointer-events: none;
}

.persp-handle {
  position: absolute;
  width: 16px;
  height: 16px;
  background: var(--ink-1);
  border: 2px solid #ffffff;
  box-shadow: 0 0 0 1px var(--border);
  border-radius: 0;
  cursor: pointer;
  z-index: 30;
  transform: translate(-50%, -50%);
  transition: transform 0.15s ease, background-color 0.15s ease;
}

.persp-handle:hover {
  transform: translate(-50%, -50%) scale(1.1);
  background: var(--accent-hover);
}

.persp-handle:active {
  transform: translate(-50%, -50%) scale(0.95);
}

@media (prefers-reduced-motion: reduce) {
  .rail-btn,
  .ide-tool-btn,
  .handle,
  .persp-handle {
    transition: none !important;
  }
}
</style>
