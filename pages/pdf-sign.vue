<script setup lang="ts">
definePageMeta({ layout: 'tools' })

useHead({
  title: 'PDF Sign | Ash Tools',
  meta: [
    { name: 'description', content: 'Free online PDF signer. Draw or upload your signature and place it anywhere on a PDF. Runs fully offline in your browser.' },
    { name: 'keywords', content: 'pdf sign, sign pdf online, esign pdf, draw signature, add signature to pdf, offline pdf signer' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/pdf-sign/' },
    { property: 'og:title', content: 'PDF Sign | Ash Tools' },
    { property: 'og:description', content: 'Sign PDF files locally in your browser. No uploads, no tracking.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/pdf-sign/' }],
})

useToolSchema({
  name: 'PDF Sign',
  description: 'Draw or upload a signature and place it on PDF pages locally in your browser.',
  url: 'https://ash-tools.store/pdf-sign/',
  category: 'BusinessApplication',
})

const PDFJS = 'https://cdn.jsdelivr.net/npm/pdfjs-dist@3.11.174/build/pdf.min.js'
const PDFJS_WORKER = 'https://cdn.jsdelivr.net/npm/pdfjs-dist@3.11.174/build/pdf.worker.min.js'
const PDFLIB = 'https://cdn.jsdelivr.net/npm/pdf-lib@1.17.1/dist/pdf-lib.min.js'
const MAX_PDF = 100 * 1024 * 1024
const MAX_IMG = 10 * 1024 * 1024
const RENDER_W = 900
const PAD_W = 1200
const PAD_H = 400
const COLORS = ['#111111', '#1d4ed8', '#b91c1c']

type Page = { url: string, ratio: number }
type Placement = { id: string, page: number, x: number, y: number, w: number, h: number }
type Drag = { id: string, mode: 'move' | 'resize', sx: number, sy: number, ox: number, oy: number, ow: number, oh: number, pw: number, ph: number }

const fileName = ref('')
const pdfBytes = ref<Uint8Array | null>(null)
const pages = ref<Page[]>([])
const loading = ref(false)
const saving = ref(false)
const status = ref('Upload a PDF to get started.')
const mode = ref<'create' | 'upload'>('create')
const signUrl = ref<string | null>(null)
const signRatio = ref(3)
const penColor = ref(COLORS[0])
const penWidth = ref(3)
const removeBg = ref(true)
const placements = ref<Placement[]>([])
const selected = ref<string | null>(null)
const pad = ref<HTMLCanvasElement | null>(null)
const padEmpty = ref(true)
const padOpen = ref(false)
const spaceDraw = ref(false)
let rawUpload: HTMLImageElement | null = null
let drawing = false
let last: { x: number, y: number } | null = null
let drag: Drag | null = null

const loadScript = (src: string, key: string) => new Promise<any>((res, rej) => {
  const w = window as any
  if (w[key]) return res(w[key])
  const s = document.createElement('script')
  s.src = src
  s.onload = () => w[key] ? res(w[key]) : rej(new Error(`${key} failed to load`))
  s.onerror = () => rej(new Error(`Failed to load ${src}`))
  document.head.appendChild(s)
})

const getPdfJs = async () => {
  const lib = await loadScript(PDFJS, 'pdfjsLib')
  lib.GlobalWorkerOptions.workerSrc = PDFJS_WORKER
  return lib
}

const uid = () => crypto.randomUUID()

const onPdf = async (file?: File | null) => {
  if (!file) return
  if (file.type !== 'application/pdf' && !file.name.toLowerCase().endsWith('.pdf')) return void (status.value = 'Please choose a PDF file.')
  if (file.size > MAX_PDF) return void (status.value = 'PDF is larger than 100 MB.')
  loading.value = true
  status.value = 'Rendering pages…'
  try {
    const bytes = new Uint8Array(await file.arrayBuffer())
    const lib = await getPdfJs()
    const doc = await lib.getDocument({ data: bytes.slice() }).promise
    const out: Page[] = []
    for (let i = 1; i <= doc.numPages; i++) {
      const p = await doc.getPage(i)
      const base = p.getViewport({ scale: 1 })
      const vp = p.getViewport({ scale: RENDER_W / base.width })
      const c = document.createElement('canvas')
      c.width = vp.width
      c.height = vp.height
      await p.render({ canvasContext: c.getContext('2d')!, viewport: vp }).promise
      out.push({ url: c.toDataURL('image/jpeg', 0.85), ratio: vp.height / vp.width })
    }
    await doc.destroy()
    pages.value = out
    pdfBytes.value = bytes
    fileName.value = file.name
    placements.value = []
    selected.value = null
    status.value = `${out.length} page${out.length > 1 ? 's' : ''} loaded. Create or upload a signature, then click a page to place it.`
  } catch (e: any) {
    status.value = e?.name === 'PasswordException' ? 'Password-protected PDFs are not supported.' : `Could not open PDF: ${e?.message ?? e}`
  } finally {
    loading.value = false
  }
}

const onPdfInput = (e: Event) => {
  const el = e.target as HTMLInputElement
  onPdf(el.files?.[0])
  el.value = ''
}

const onDrop = (e: DragEvent) => {
  e.preventDefault()
  onPdf(e.dataTransfer?.files?.[0])
}

const clearAll = () => {
  pages.value = []
  pdfBytes.value = null
  fileName.value = ''
  placements.value = []
  selected.value = null
  status.value = 'Upload a PDF to get started.'
}

// Signature pad
const initPad = () => {
  const c = pad.value
  if (!c) return
  const dpr = window.devicePixelRatio || 1
  c.width = PAD_W * dpr
  c.height = PAD_H * dpr
  const ctx = c.getContext('2d')!
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0)
  ctx.lineCap = 'round'
  ctx.lineJoin = 'round'
  padEmpty.value = true
}

const padPos = (e: PointerEvent) => {
  const r = pad.value!.getBoundingClientRect()
  return { x: (e.clientX - r.left) * PAD_W / r.width, y: (e.clientY - r.top) * PAD_H / r.height }
}

const padDown = (e: PointerEvent) => {
  drawing = true
  pad.value!.setPointerCapture(e.pointerId)
  last = padPos(e)
  padMove(e)
}

const padMove = (e: PointerEvent) => {
  if (!drawing && !spaceDraw.value) return
  if (!last) return void (last = padPos(e))
  const ctx = pad.value!.getContext('2d')!
  const p = padPos(e)
  ctx.strokeStyle = penColor.value
  ctx.lineWidth = penWidth.value * (e.pressure && e.pointerType === 'pen' ? 0.5 + e.pressure : 1)
  ctx.beginPath()
  ctx.moveTo(last.x, last.y)
  ctx.lineTo(p.x + 0.01, p.y)
  ctx.stroke()
  last = p
  padEmpty.value = false
}

const padUp = () => {
  drawing = false
  last = null
}

const openPad = async () => {
  padOpen.value = true
  await nextTick()
  initPad()
}

const closePad = () => {
  padOpen.value = false
  spaceDraw.value = false
  padUp()
}

const onPadKey = (e: KeyboardEvent) => {
  if (!padOpen.value) return
  if (e.key === 'Escape') return closePad()
  if (e.code !== 'Space' || (e.target as HTMLElement)?.closest('textarea, input:not([type=range])')) return
  e.preventDefault()
  if (e.type === 'keyup') {
    spaceDraw.value = false
    last = null
  } else if (!e.repeat) {
    spaceDraw.value = true
    last = null
  }
}

const clearPad = () => {
  const c = pad.value
  if (!c) return
  c.getContext('2d')!.clearRect(0, 0, PAD_W, PAD_H)
  padEmpty.value = true
}

const trim = (src: HTMLCanvasElement) => {
  const ctx = src.getContext('2d')!
  const { width: w, height: h } = src
  const d = ctx.getImageData(0, 0, w, h).data
  let x0 = w, y0 = h, x1 = -1, y1 = -1
  for (let y = 0; y < h; y++) for (let x = 0; x < w; x++) {
    if (d[(y * w + x) * 4 + 3] > 8) {
      x0 = Math.min(x0, x); x1 = Math.max(x1, x)
      y0 = Math.min(y0, y); y1 = Math.max(y1, y)
    }
  }
  if (x1 < 0) return null
  const m = 4
  x0 = Math.max(0, x0 - m); y0 = Math.max(0, y0 - m)
  x1 = Math.min(w - 1, x1 + m); y1 = Math.min(h - 1, y1 + m)
  const out = document.createElement('canvas')
  out.width = x1 - x0 + 1
  out.height = y1 - y0 + 1
  out.getContext('2d')!.drawImage(src, x0, y0, out.width, out.height, 0, 0, out.width, out.height)
  return out
}

const setSign = (c: HTMLCanvasElement | null) => {
  if (!c) return void (status.value = 'Signature is empty.')
  signUrl.value = c.toDataURL('image/png')
  signRatio.value = c.width / c.height
  status.value = 'Signature ready. Click on a page to place it.'
}

const useDrawn = () => {
  if (!pad.value) return
  setSign(trim(pad.value))
  closePad()
}

const processUpload = () => {
  if (!rawUpload) return
  const c = document.createElement('canvas')
  c.width = rawUpload.naturalWidth
  c.height = rawUpload.naturalHeight
  const ctx = c.getContext('2d')!
  ctx.drawImage(rawUpload, 0, 0)
  if (removeBg.value) {
    const img = ctx.getImageData(0, 0, c.width, c.height)
    const d = img.data
    for (let i = 0; i < d.length; i += 4) {
      const l = (d[i] + d[i + 1] + d[i + 2]) / 3
      d[i + 3] = l > 225 ? 0 : l > 180 ? Math.round(d[i + 3] * (225 - l) / 45) : d[i + 3]
    }
    ctx.putImageData(img, 0, 0)
  }
  setSign(trim(c))
}

const onSignUpload = (e: Event) => {
  const el = e.target as HTMLInputElement
  const f = el.files?.[0]
  el.value = ''
  if (!f) return
  if (!f.type.startsWith('image/')) return void (status.value = 'Please choose an image file.')
  if (f.size > MAX_IMG) return void (status.value = 'Image is larger than 10 MB.')
  const url = URL.createObjectURL(f)
  const img = new Image()
  img.onload = () => {
    rawUpload = img
    processUpload()
    URL.revokeObjectURL(url)
  }
  img.onerror = () => {
    status.value = 'Could not read image.'
    URL.revokeObjectURL(url)
  }
  img.src = url
}

watch(removeBg, processUpload)

// Placement
const onPageClick = (e: MouseEvent, page: number) => {
  if (drag) return
  if (!signUrl.value) return void (status.value = 'Create or upload a signature first.')
  const r = (e.currentTarget as HTMLElement).getBoundingClientRect()
  const w = 0.25
  const h = Math.min(0.5, (w / signRatio.value) / pages.value[page].ratio)
  const x = Math.min(Math.max((e.clientX - r.left) / r.width - w / 2, 0), 1 - w)
  const y = Math.min(Math.max((e.clientY - r.top) / r.height - h / 2, 0), 1 - h)
  const id = uid()
  placements.value.push({ id, page, x, y, w, h })
  selected.value = id
}

const startDrag = (e: PointerEvent, p: Placement, m: Drag['mode']) => {
  e.stopPropagation()
  e.preventDefault()
  const box = (e.currentTarget as HTMLElement).closest('[data-page]')!.getBoundingClientRect()
  selected.value = p.id
  drag = { id: p.id, mode: m, sx: e.clientX, sy: e.clientY, ox: p.x, oy: p.y, ow: p.w, oh: p.h, pw: box.width, ph: box.height }
  window.addEventListener('pointermove', onDrag)
  window.addEventListener('pointerup', endDrag, { once: true })
}

const onDrag = (e: PointerEvent) => {
  if (!drag) return
  const p = placements.value.find(i => i.id === drag!.id)
  if (!p) return
  const dx = (e.clientX - drag.sx) / drag.pw
  const dy = (e.clientY - drag.sy) / drag.ph
  if (drag.mode === 'move') {
    p.x = Math.min(Math.max(drag.ox + dx, 0), 1 - p.w)
    p.y = Math.min(Math.max(drag.oy + dy, 0), 1 - p.h)
  } else {
    const k = drag.oh / drag.ow
    const w = Math.min(Math.max(drag.ow + dx, 0.04), 1 - p.x, (1 - p.y) / k)
    p.w = w
    p.h = w * k
  }
}

const endDrag = () => {
  window.removeEventListener('pointermove', onDrag)
  setTimeout(() => (drag = null))
}

const removePlacement = (id: string) => {
  placements.value = placements.value.filter(p => p.id !== id)
  if (selected.value === id) selected.value = null
}

const onKey = (e: KeyboardEvent) => {
  if (padOpen.value || !selected.value || !['Delete', 'Backspace'].includes(e.key)) return
  if ((e.target as HTMLElement)?.closest('input, textarea')) return
  e.preventDefault()
  removePlacement(selected.value)
}

onMounted(() => {
  window.addEventListener('keydown', onKey)
  window.addEventListener('keydown', onPadKey)
  window.addEventListener('keyup', onPadKey)
})
onBeforeUnmount(() => {
  window.removeEventListener('keydown', onKey)
  window.removeEventListener('keydown', onPadKey)
  window.removeEventListener('keyup', onPadKey)
  window.removeEventListener('pointermove', onDrag)
})

const signUrlsById = new Map<string, string>()
watch(placements, ps => ps.forEach(p => !signUrlsById.has(p.id) && signUrl.value && signUrlsById.set(p.id, signUrl.value)), { deep: true })
const imgFor = (id: string) => signUrlsById.get(id) ?? signUrl.value ?? ''

const download = async () => {
  if (!pdfBytes.value || !placements.value.length) return
  saving.value = true
  status.value = 'Signing PDF…'
  try {
    const { PDFDocument, degrees } = await loadScript(PDFLIB, 'PDFLib')
    const doc = await PDFDocument.load(pdfBytes.value, { ignoreEncryption: true })
    const docPages = doc.getPages()
    const cache = new Map<string, any>()
    for (const p of placements.value) {
      const src = imgFor(p.id)
      if (!cache.has(src)) cache.set(src, await doc.embedPng(src))
      const img = cache.get(src)
      const page = docPages[p.page]
      const { x: bx, y: by, width: bw, height: bh } = page.getCropBox()
      const rot = ((page.getRotation().angle % 360) + 360) % 360
      const [vw, vh] = rot % 180 ? [bh, bw] : [bw, bh]
      const w = p.w * vw, h = p.h * vh
      const vx = p.x * vw, vy = vh - p.y * vh - h
      const pos = rot === 90 ? { x: bx + bw - vy, y: by + vx, r: 90 }
        : rot === 180 ? { x: bx + bw - vx, y: by + bh - vy, r: 180 }
        : rot === 270 ? { x: bx + vy, y: by + bh - vx, r: 270 }
        : { x: bx + vx, y: by + vy, r: 0 }
      page.drawImage(img, { x: pos.x, y: pos.y, width: w, height: h, rotate: degrees(pos.r) })
    }
    const out = await doc.save()
    const a = document.createElement('a')
    a.href = URL.createObjectURL(new Blob([out], { type: 'application/pdf' }))
    a.download = fileName.value.replace(/\.pdf$/i, '') + '-signed.pdf'
    a.click()
    setTimeout(() => URL.revokeObjectURL(a.href), 1000)
    status.value = 'Signed PDF downloaded.'
  } catch (e: any) {
    status.value = `Signing failed: ${e?.message ?? e}`
  } finally {
    saving.value = false
  }
}
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
          <label for="sign-pdf-input" class="rail-btn cursor-pointer shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Open PDF" aria-label="Open PDF">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M3 6a1 1 0 011-1h3l1.5 2H16a1 1 0 011 1v7a1 1 0 01-1 1H4a1 1 0 01-1-1V6z" /><path stroke-linecap="round" d="M3 8h14" /></svg>
            <span class="text-[10px] font-medium">Open</span>
          </label>
          <button v-if="pages.length" @click="clearAll" class="rail-btn shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Clear" aria-label="Clear">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18"></path><path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"></path></svg>
            <span class="text-[10px] font-medium">Clear</span>
          </button>
        </div>
      </aside>

      <div class="order-1 md:order-2 flex-1 flex justify-center p-3.5 min-w-0">
        <div class="w-full max-w-6xl flex flex-col gap-3.5">

          <header class="ui-panel p-4 flex items-center justify-between gap-3">
            <div class="flex items-center gap-2">
              <svg class="h-4 w-4 text-[var(--ink-3)]" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487z" /></svg>
              <h1 class="text-sm font-bold text-[var(--ink-1)]">PDF Sign</h1>
            </div>
            <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
          </header>

          <input id="sign-pdf-input" type="file" accept=".pdf,application/pdf" class="hidden" @change="onPdfInput">

          <div class="grid grid-cols-1 lg:grid-cols-3 gap-3.5 items-start">

            <section class="lg:col-span-2 ui-panel p-5 flex flex-col gap-4 min-w-0">
              <div class="flex items-center justify-between">
                <div class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Document</div>
                <span v-if="fileName" class="text-xs font-mono text-[var(--ink-3)] truncate max-w-[60%]">{{ fileName }}</span>
              </div>

              <label v-if="!pages.length" for="sign-pdf-input" class="ui-dropzone p-10 flex flex-col items-center justify-center text-center gap-2" @drop="onDrop" @dragover.prevent>
                <div class="w-12 h-12 flex items-center justify-center mb-1" style="background: var(--accent-soft); border: 1px solid var(--border)">
                  <svg class="w-6 h-6" style="color: var(--accent)" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4" /></svg>
                </div>
                <span class="font-semibold text-sm text-[var(--ink-1)]">{{ loading ? 'Loading…' : 'Drop a PDF here' }}</span>
                <span class="text-[var(--ink-3)] text-xs">or click to browse</span>
              </label>

              <div v-else class="flex flex-col gap-4 max-h-[78vh] overflow-y-auto pr-1">
                <div v-for="(pg, i) in pages" :key="i" class="flex flex-col gap-1">
                  <span class="text-[10px] font-mono text-[var(--ink-3)]">Page {{ i + 1 }}</span>
                  <div :data-page="i" class="relative w-full select-none border border-[var(--border)] bg-white"
                    :class="signUrl ? 'cursor-crosshair' : ''" :style="{ aspectRatio: `1 / ${pg.ratio}` }"
                    @click="onPageClick($event, i)">
                    <img :src="pg.url" :alt="`Page ${i + 1}`" class="absolute inset-0 w-full h-full pointer-events-none" draggable="false">
                    <div v-for="p in placements.filter(p => p.page === i)" :key="p.id"
                      class="absolute group touch-none cursor-move"
                      :class="selected === p.id ? 'outline outline-2 outline-[var(--accent)]' : 'hover:outline hover:outline-1 hover:outline-[var(--accent)]'"
                      :style="{ left: `${p.x * 100}%`, top: `${p.y * 100}%`, width: `${p.w * 100}%`, height: `${p.h * 100}%` }"
                      @click.stop="selected = p.id" @pointerdown="startDrag($event, p, 'move')">
                      <img :src="imgFor(p.id)" alt="Signature" class="w-full h-full object-contain pointer-events-none" draggable="false">
                      <button v-show="selected === p.id" @pointerdown.stop @click.stop="removePlacement(p.id)"
                        class="absolute -top-3 -right-3 w-6 h-6 rounded-full flex items-center justify-center text-xs text-white"
                        style="background: var(--danger)" title="Remove" aria-label="Remove signature">✕</button>
                      <span v-show="selected === p.id" @pointerdown="startDrag($event, p, 'resize')"
                        class="absolute -bottom-1.5 -right-1.5 w-3 h-3 cursor-nwse-resize" style="background: var(--accent)" aria-hidden="true"></span>
                    </div>
                  </div>
                </div>
              </div>
            </section>

            <section class="ui-panel p-5 flex flex-col gap-4 lg:sticky lg:top-3.5">
              <div class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Signature</div>

              <div class="grid grid-cols-2 border border-[var(--border)] rounded-[var(--radius-sm)] overflow-hidden text-xs font-semibold" role="tablist">
                <button v-for="m in (['create', 'upload'] as const)" :key="m" role="tab" :aria-selected="mode === m" @click="mode = m"
                  class="py-2 transition-colors capitalize"
                  :style="mode === m ? 'background: var(--accent); color: var(--surface-0)' : 'color: var(--ink-2)'">{{ m }} sign</button>
              </div>

              <div v-if="mode === 'create'" class="flex flex-col gap-3">
                <button @click="openPad" class="ui-dropzone p-6 flex flex-col items-center justify-center text-center gap-1 cursor-pointer">
                  <span class="font-semibold text-sm text-[var(--ink-1)]">Open signature pad</span>
                  <span class="text-[var(--ink-3)] text-xs">Draw with mouse, pen, or hold Space + move</span>
                </button>
              </div>

              <div v-else class="flex flex-col gap-3">
                <label for="sign-img-input" class="ui-dropzone p-6 flex flex-col items-center justify-center text-center gap-1 cursor-pointer">
                  <span class="font-semibold text-sm text-[var(--ink-1)]">Choose signature image</span>
                  <span class="text-[var(--ink-3)] text-xs">PNG, JPG, or WebP</span>
                  <input id="sign-img-input" type="file" accept="image/*" class="hidden" @change="onSignUpload">
                </label>
                <label class="flex items-center gap-2 text-xs text-[var(--ink-2)]">
                  <input v-model="removeBg" type="checkbox"> Remove white background
                </label>
              </div>

              <div v-if="signUrl" class="flex flex-col gap-1.5">
                <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Current</span>
                <div class="h-20 flex items-center justify-center bg-white border border-[var(--border)] rounded-[var(--radius-sm)] p-2">
                  <img :src="signUrl" alt="Current signature" class="max-h-full max-w-full object-contain">
                </div>
              </div>

              <p class="text-xs text-[var(--ink-3)]" aria-live="polite">{{ status }}</p>
              <p v-if="placements.length" class="text-xs font-mono text-[var(--ink-3)]">{{ placements.length }} signature{{ placements.length > 1 ? 's' : '' }} placed · drag to move, corner to resize, Del to remove</p>

              <button @click="download" :disabled="!placements.length || saving" class="ui-button py-2.5 text-sm font-semibold disabled:opacity-40">
                {{ saving ? 'Signing…' : 'Download signed PDF' }}
              </button>
            </section>
          </div>
        </div>
      </div>
    </div>

    <div v-if="padOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4" style="background: rgb(0 0 0 / 0.55)" @click.self="closePad">
      <div class="ui-panel w-full max-w-5xl p-5 flex flex-col gap-4" role="dialog" aria-modal="true" aria-label="Create signature">
        <div class="flex items-center justify-between gap-3">
          <h2 class="text-sm font-bold text-[var(--ink-1)]">Create signature</h2>
          <button @click="closePad" class="w-8 h-8 flex items-center justify-center text-[var(--ink-3)] hover:text-[var(--ink-1)]" aria-label="Close">✕</button>
        </div>
        <canvas ref="pad" class="w-full touch-none bg-white border border-dashed rounded-[var(--radius-sm)] cursor-crosshair"
          :style="{ aspectRatio: `${PAD_W} / ${PAD_H}`, borderColor: spaceDraw ? 'var(--accent)' : 'var(--border)' }"
          @pointerdown="padDown" @pointermove="padMove" @pointerup="padUp" @pointerleave="padUp" @pointercancel="padUp"></canvas>
        <p class="text-xs text-[var(--ink-3)]">Click and drag to draw, or hold <kbd class="ui-badge font-mono">Space</kbd> and move the mouse. {{ spaceDraw ? 'Drawing…' : '' }}</p>
        <div class="flex flex-wrap items-center justify-between gap-3">
          <div class="flex items-center gap-4">
            <div class="flex gap-1.5">
              <button v-for="c in COLORS" :key="c" @click="penColor = c" :aria-label="`Ink ${c}`"
                class="w-7 h-7 rounded-full border-2" :style="{ background: c, borderColor: penColor === c ? 'var(--accent)' : 'transparent' }"></button>
            </div>
            <label class="flex items-center gap-2 text-xs text-[var(--ink-3)]">
              Pen
              <input v-model.number="penWidth" type="range" min="1" max="12" class="ui-range w-28">
            </label>
          </div>
          <div class="flex gap-2">
            <button @click="clearPad" class="ui-button-secondary px-4 py-2 text-xs">Clear</button>
            <button @click="useDrawn" :disabled="padEmpty" class="ui-button px-4 py-2 text-xs disabled:opacity-40">Use signature</button>
          </div>
        </div>
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
