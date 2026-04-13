<script setup lang="ts">
useHead({
  title: 'Image Tools | Local WASM',
  meta: [
    { name: 'description', content: 'In-browser image tools: crop, perspective crop, resize, compress, optimize. Everything stays on your device.' },
    { name: 'robots', content: 'index,follow' },
  ],
})

// WASM functions
let compress_image: any = null
let crop_image: any = null
let perspective_crop: any = null

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
let activeMode: 'crop' | 'perspective' | 'none' = 'crop'

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

const setStatus = (message?: string) => {
  if (statusEl) statusEl.textContent = message || ''
}

const setProcessingState = (isProcessing: boolean) => {
  if (!processBtn) return
  processBtn.disabled = isProcessing || !currentBytes
  processBtn.textContent = isProcessing ? 'Processing...' : 'Process'
  processBtn.setAttribute('aria-busy', isProcessing ? 'true' : 'false')
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
  updateCropOverlay()
  updatePerspectiveOverlay()
}

const handleFile = async (file: File) => {
  if (!file) return
  const heicCandidate = isHeicFile(file)
  setStatus(heicCandidate ? 'Converting HEIC...' : 'Loading image...')
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
    setStatus(messages.join(' '))
    if (downloadBtn) downloadBtn.disabled = false
    if (processBtn) processBtn.disabled = false
    if (resetBtn) resetBtn.disabled = false
  } catch (err) {
    setStatus(err instanceof Error ? err.message : String(err))
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

const setMode = (mode: 'crop' | 'perspective' | 'none', options: any = {}) => {
  if (!mode || !['crop', 'perspective', 'none'].includes(mode)) return
  const changed = activeMode !== mode
  activeMode = mode
  updateModeButtons()
  updateCropOverlay()
  updatePerspectiveOverlay()
  if (changed && currentBytes && !options.silent) {
    if (mode === 'crop') {
      setStatus('Crop mode active. Drag the handles to adjust.')
    } else if (mode === 'perspective') {
      setStatus('Perspective mode active. Move the four corners.')
    } else {
      setStatus('Overlays hidden. Select a mode to adjust again.')
    }
  }
}

// ==================== Processing ====================
const runPipeline = async () => {
  if (!currentBytes) {
    setStatus('Add an image first.')
    return
  }
  setProcessingState(true)
  setStatus('Preparing WebAssembly module...')
  try {
    await ensureWasm()
    let bytes = currentBytes
    if (activeMode === 'crop') {
      if (!currentDims) throw new Error('Crop needs image dimensions')
      const x = Math.max(0, Math.round(cropRect.x * currentDims.width))
      const y = Math.max(0, Math.round(cropRect.y * currentDims.height))
      const w = Math.max(1, Math.round(cropRect.w * currentDims.width))
      const h = Math.max(1, Math.round(cropRect.h * currentDims.height))
      setStatus('Cropping...')
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
      setStatus('Applying perspective crop...')
      bytes = perspective_crop(bytes, pts, width, height)
    }
    const { format, quality } = getOutputSettings()
    setStatus('Compressing...')
    bytes = compress_image(bytes, quality, format)
    currentBytes = bytes
    await updatePreview(bytes)
    setMode('none', { silent: true })
    setStatus('Done. Preview updated.')
  } catch (err) {
    console.error(err)
    setStatus(err instanceof Error ? err.message : String(err))
  } finally {
    setProcessingState(false)
  }
}

const downloadResult = () => {
  if (!currentBytes) return
  const format = formatSelect?.value ?? 'png'
  const blob = new Blob([bytesToBlobPart(currentBytes)], { type: `image/${format}` })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  const ext = format === 'jpeg' ? 'jpg' : format
  a.href = url
  a.download = `image-tools-${Date.now()}.${ext}`
  document.body.appendChild(a)
  a.click()
  a.remove()
  URL.revokeObjectURL(url)
}

const resetImage = async () => {
  if (!originalBytes) return
  currentBytes = originalBytes
  cropRect = { x: 0, y: 0, w: 1, h: 1 }
  perspPoints = { tl: { x: 0, y: 0 }, tr: { x: 1, y: 0 }, br: { x: 1, y: 1 }, bl: { x: 0, y: 1 } }
  setMode('crop', { silent: true })
  await updatePreview(currentBytes)
  setStatus('Back to the original file.')
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
  <div class="min-h-[calc(100vh-48px)] bg-[#FFFDF5] text-black flex flex-col">
    <!-- Main Workspace -->
    <section class="flex-1 p-6 overflow-y-auto">
      <div class="max-w-7xl mx-auto">
        <div class="mb-6 neo-shell bg-[#FFE8A3] p-4">
          <h2 class="text-xl font-black uppercase tracking-wide">Image Editor</h2>
          <p class="text-sm mt-1">Edit locally in your browser. Nothing is uploaded to a server.</p>
          <div class="mt-3 grid grid-cols-2 md:grid-cols-4 gap-2 text-xs font-semibold">
            <div class="border-2 border-black rounded-lg bg-white px-3 py-2">1. Upload</div>
            <div class="border-2 border-black rounded-lg bg-white px-3 py-2">2. Adjust Crop</div>
            <div class="border-2 border-black rounded-lg bg-white px-3 py-2">3. Process</div>
            <div class="border-2 border-black rounded-lg bg-white px-3 py-2">4. Download</div>
          </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <!-- Preview Card (Left on desktop) -->
          <div class="lg:order-1">
            <div class="neo-shell bg-white p-6 h-full">
              <div class="mb-3">
                <h3 class="text-base font-black uppercase tracking-wide">Preview</h3>
                <p class="text-sm text-black/80">Use handles directly on the image to choose the exact area you want.</p>
              </div>
              <div class="preview-stage mb-4 border-4 border-black rounded-lg overflow-hidden bg-[#FFF7D6]"
                id="previewStage">
                <img id="preview" alt="Preview" class="w-full h-auto"
                  src="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 1280 720'%3E%3Crect width='1280' height='720' fill='%23FFF7D6'/%3E%3Crect x='60' y='60' width='1160' height='600' rx='20' fill='%23FFFFFF' stroke='%23000000' stroke-width='8'/%3E%3Crect x='130' y='135' width='180' height='180' rx='16' fill='%23FFD93D' stroke='%23000000' stroke-width='6'/%3E%3Cpath d='M170 260l45-55 35 40 25-30 45 45' stroke='%23000000' stroke-width='10' fill='none' stroke-linecap='round' stroke-linejoin='round'/%3E%3Ccircle cx='250' cy='180' r='16' fill='%23000000'/%3E%3Ctext x='380' y='220' fill='%23000000' font-size='56' font-family='monospace' font-weight='700'%3EDrop or browse an image%3C/text%3E%3Ctext x='380' y='288' fill='%23333333' font-size='34' font-family='monospace'%3EEverything runs locally on your device%3C/text%3E%3C/svg%3E" />
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
              <div class="text-black text-sm mb-3">Use the crop box or adjust the four perspective points to refine
                the selection.</div>
              <div id="meta" class="font-mono text-xs text-black"></div>
              <div id="stats" class="font-mono text-xs text-cyan-300/80 mt-2 min-h-5" aria-live="polite"></div>
            </div>
          </div>

          <!-- Controls (Right on desktop) -->
          <div class="lg:order-2">
            <div class="space-y-4">
              <!-- Upload Dropzone -->
              <label id="dropzone"
                class="block border-4 border-dashed border-black rounded-xl p-6 bg-white cursor-pointer transition-all hover:-translate-y-0.5"
                aria-label="Upload image by click or drag and drop">
                <div class="mb-3">
                  <div class="font-black uppercase tracking-wide text-base mb-1">Drop an image or browse</div>
                  <div class="text-black text-sm">Supported: PNG, JPEG, WebP, HEIC. Tip: start with Crop mode for quick framing.</div>
                </div>
                <span
                  class="inline-block px-4 py-2 bg-[#FFD93D] border-2 border-black text-black rounded-lg text-sm font-medium transition-colors">Browse</span>
                <p class="mt-3 text-xs text-black/70">Shortcuts: Ctrl/Cmd+O upload, Ctrl/Cmd+Enter process, Ctrl/Cmd+S download.</p>
              </label>
              <input id="fileInput" type="file" accept="image/*,.heic,.heif" class="hidden" />

              <!-- Mode Switch -->
              <div>
                <p class="text-xs font-semibold uppercase tracking-wide mb-2">Adjustment Mode</p>
                <div class="flex gap-2">
                <button id="modeCrop" data-mode="crop"
                  class="px-4 py-2 rounded-lg font-semibold text-sm border-2 border-black bg-[#FF6B6B] text-black transition-colors active">Crop</button>
                <button id="modePerspective" data-mode="perspective"
                  class="px-4 py-2 rounded-lg font-semibold text-sm border-2 border-black bg-[#C4B5FD] text-black transition-colors">Perspective
                  Crop</button>
                </div>
              </div>

              <!-- Process & Download -->
              <div class="space-y-3">
                <div class="flex flex-col gap-2">
                  <button id="processBtn"
                    class="neo-button w-full bg-[#FF6B6B] text-black">Process</button>
                  <button id="downloadBtn"
                    class="neo-button w-full bg-[#FFD93D] text-black">Download</button>
                  <button id="resetBtn"
                    class="w-full px-4 py-2.5 border-4 border-black bg-white text-black rounded-lg font-semibold text-sm transition-colors">Reset</button>
                </div>

                <!-- Compression Settings -->
                <div class="bg-[#FFFDF5] border-4 border-black rounded-lg p-4 space-y-3">
                  <div id="qualityRow" class="flex items-center gap-4">
                    <label for="qualityInput"
                      class="text-sm font-semibold text-black whitespace-nowrap">Quality (JPEG)</label>
                    <input id="qualityInput" type="range" min="10" max="100" value="80"
                      class="flex-1 h-2 bg-white border-2 border-black rounded-full cursor-pointer" aria-label="Compression quality">
                    <span id="qualityValue" class="font-mono text-xs text-black min-w-12">80%</span>
                  </div>
                  <div class="flex items-center gap-4">
                    <label for="formatSelect" class="text-sm font-semibold text-black">Output</label>
                    <div class="relative flex-1">
                      <select id="formatSelect"
                        class="w-full px-3 py-2 bg-white border-4 border-black rounded-lg text-black text-sm font-medium appearance-none pr-8 cursor-pointer transition-colors">
                        <option value="png">PNG</option>
                        <option value="jpeg">JPEG</option>
                        <option value="webp" selected>WebP</option>
                      </select>
                      <svg class="absolute right-2 top-2.5 w-5 h-5 text-black pointer-events-none"
                        fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd"
                          d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                          clip-rule="evenodd" />
                      </svg>
                    </div>
                  </div>
                  <p class="text-xs text-black/70">Choose output format first, then adjust quality if JPEG is selected.</p>
                  <div id="status" class="font-mono text-xs text-black min-h-5"></div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<style scoped>
/* Tool-specific interactive selectors - preserved from original */
#previewStage {
  position: relative;
  max-width: 100%;
  height: auto;
  background-color: #fff7d6;
  background-image: radial-gradient(#f3e2a9 1px, transparent 1px);
  background-size: 14px 14px;
}

#modeCrop.active,
#modePerspective.active {
  background: #000;
  color: #fff;
  box-shadow: 4px 4px 0 #000;
}

#modeCrop:not(.active),
#modePerspective:not(.active) {
  opacity: 0.9;
}

.crop-box {
  position: absolute;
  top: 0;
  left: 0;
  border: 2px solid rgba(34, 211, 238, 0.6);
  background: rgba(34, 211, 238, 0.05);
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
  background: rgba(0, 0, 0, 0.7);
  color: #22d3ee;
  font-family: var(--font-mono);
  font-size: 12px;
  padding: 4px 8px;
  border-radius: 4px;
  white-space: nowrap;
  z-index: 20;
}

.handle {
  position: absolute;
  width: 24px;
  height: 24px;
  background: #22d3ee;
  border: 2px solid rgba(0, 0, 0, 0.5);
  box-shadow: 0 0 10px rgba(34, 211, 238, 0.4), 0 0 2px rgba(0, 0, 0, 0.8);
  border-radius: 50%;
  cursor: pointer;
  z-index: 25;
  transition: transform 0.15s ease, background-color 0.15s ease;
}

.handle:hover {
  transform: scale(1.1);
  background: #67e8f9;
}

.handle:active {
  transform: scale(0.95);
}

.handle.nw {
  top: -12px;
  left: -12px;
  cursor: nwse-resize;
}

.handle.ne {
  top: -12px;
  right: -12px;
  cursor: nesw-resize;
}

.handle.sw {
  bottom: -12px;
  left: -12px;
  cursor: nesw-resize;
}

.handle.se {
  bottom: -12px;
  right: -12px;
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
  stroke: rgba(34, 211, 238, 0.7);
  stroke-width: 2;
  pointer-events: none;
}

.persp-handle {
  position: absolute;
  width: 24px;
  height: 24px;
  background: #22d3ee;
  border: 2px solid rgba(0, 0, 0, 0.5);
  box-shadow: 0 0 10px rgba(34, 211, 238, 0.4), 0 0 2px rgba(0, 0, 0, 0.8);
  border-radius: 50%;
  cursor: pointer;
  z-index: 30;
  transform: translate(-50%, -50%);
  transition: transform 0.15s ease, background-color 0.15s ease;
}

.persp-handle:hover {
  transform: translate(-50%, -50%) scale(1.1);
  background: #67e8f9;
}

.persp-handle:active {
  transform: translate(-50%, -50%) scale(0.95);
}

#dropzone.drop-active {
  border-color: #000;
  background: #ffe8a3;
  transform: translateY(-2px);
}
</style>
