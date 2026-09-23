<script setup lang="ts">
import qrcode from 'qrcode-generator'

definePageMeta({ layout: 'tools' })

useHead({
  title: 'Free QR Code Generator | Create QR codes in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser QR code generator. Turn text, URLs, or Wi-Fi details into a downloadable QR code locally. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/qr/' },
    { property: 'og:title', content: 'Free QR Code Generator | Create QR codes in your browser' },
    { property: 'og:description', content: 'Generate QR codes privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/qr/' }],
})

useToolSchema({
  name: 'QR Code Generator',
  description: 'Create QR codes from text or links, with an optional center logo, locally in your browser.',
  url: 'https://ash-tools.store/qr/',
  category: 'UtilitiesApplication',
})

type ErrorCorrectionLevel = 'L' | 'M' | 'Q' | 'H'

const MAX_QR_CHARS = 2953
const DEFAULT_SIZE = 512

const text = ref('https://ash-tools.store')
const ecLevel = ref<ErrorCorrectionLevel>('M')
const size = ref(DEFAULT_SIZE)
const foreground = ref('#211c16')
const background = ref('#ffffff')
const backgroundAlpha = ref(100) // percent; lets the background be transparent for overlay use
const status = ref('Enter text or a URL to generate a QR code.')
const isError = ref(false)

const canvasRef = ref<HTMLCanvasElement | null>(null)
const svgMarkup = ref('')

const hexToRgbTriplet = (hex: string): [number, number, number] => {
  const clean = hex.replace('#', '')
  const r = parseInt(clean.slice(0, 2), 16) || 0
  const g = parseInt(clean.slice(2, 4), 16) || 0
  const b = parseInt(clean.slice(4, 6), 16) || 0
  return [r, g, b]
}

const backgroundRgba = computed(() => {
  const [r, g, b] = hexToRgbTriplet(background.value)
  return `rgba(${r}, ${g}, ${b}, ${(backgroundAlpha.value / 100).toFixed(2)})`
})

// Center logo: an uploaded image drawn over the middle of the QR. Forces
// high error-correction so the code still scans with part of it covered.
const logoDataUrl = ref<string | null>(null)
const logoImg = ref<HTMLImageElement | null>(null)
const logoSizePct = ref(22) // logo width as % of the QR's pixel size
const ecLevelBeforeLogo = ref<ErrorCorrectionLevel>('M')

const readFileAsDataUrl = (file: File): Promise<string> =>
  new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onerror = () => reject(new Error('Failed to read file'))
    reader.onload = () => resolve(reader.result as string)
    reader.readAsDataURL(file)
  })

// Average color of the image, downsampled to a tiny canvas for speed.
const extractDominantColor = (img: HTMLImageElement): string => {
  const sample = document.createElement('canvas')
  sample.width = 32
  sample.height = 32
  const ctx = sample.getContext('2d', { willReadFrequently: true })
  if (!ctx) return foreground.value
  ctx.drawImage(img, 0, 0, 32, 32)
  const { data } = ctx.getImageData(0, 0, 32, 32)
  let r = 0, g = 0, b = 0, count = 0
  for (let i = 0; i < data.length; i += 4) {
    const alpha = data[i + 3]
    if (alpha < 32) continue // skip near-transparent pixels
    r += data[i]; g += data[i + 1]; b += data[i + 2]
    count += 1
  }
  if (!count) return foreground.value
  r = Math.round(r / count); g = Math.round(g / count); b = Math.round(b / count)
  // Darken if the average is too light to keep contrast against a white background.
  const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255
  if (luminance > 0.75) {
    const scale = 0.6
    r = Math.round(r * scale); g = Math.round(g * scale); b = Math.round(b * scale)
  }
  const toHex = (n: number) => n.toString(16).padStart(2, '0')
  return `#${toHex(r)}${toHex(g)}${toHex(b)}`
}

const onLogoChange = async (event: Event) => {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return

  try {
    const dataUrl = await readFileAsDataUrl(file)
    const img = new Image()
    await new Promise<void>((resolve, reject) => {
      img.onload = () => resolve()
      img.onerror = () => reject(new Error('Could not load image'))
      img.src = dataUrl
    })

    logoDataUrl.value = dataUrl
    logoImg.value = img
    foreground.value = extractDominantColor(img)

    if (ecLevel.value !== 'H') {
      ecLevelBeforeLogo.value = ecLevel.value
      ecLevel.value = 'H'
    }

    render()
  } catch (error) {
    isError.value = true
    status.value = error instanceof Error ? error.message : 'Could not load image.'
  } finally {
    input.value = ''
  }
}

const removeLogo = () => {
  logoDataUrl.value = null
  logoImg.value = null
  ecLevel.value = ecLevelBeforeLogo.value
  render()
}

const render = () => {
  const value = text.value.trim()
  if (!value) {
    isError.value = false
    status.value = 'Enter text or a URL to generate a QR code.'
    svgMarkup.value = ''
    const ctx = canvasRef.value?.getContext('2d')
    if (ctx && canvasRef.value) ctx.clearRect(0, 0, canvasRef.value.width, canvasRef.value.height)
    return
  }

  if (value.length > MAX_QR_CHARS) {
    isError.value = true
    status.value = `Text too long (${value.length} chars). Max is ${MAX_QR_CHARS} for QR codes.`
    return
  }

  try {
    const qr = qrcode(0, ecLevel.value)
    qr.addData(value)
    qr.make()

    const moduleCount = qr.getModuleCount()
    const cellSize = Math.max(1, Math.floor(size.value / moduleCount))
    const pixelSize = cellSize * moduleCount

    const canvas = canvasRef.value
    if (canvas) {
      canvas.width = pixelSize
      canvas.height = pixelSize
      const ctx = canvas.getContext('2d')
      if (ctx) {
        ctx.clearRect(0, 0, pixelSize, pixelSize)
        ctx.fillStyle = backgroundRgba.value
        ctx.fillRect(0, 0, pixelSize, pixelSize)
        ctx.fillStyle = foreground.value
        for (let row = 0; row < moduleCount; row += 1) {
          for (let col = 0; col < moduleCount; col += 1) {
            if (qr.isDark(row, col)) {
              ctx.fillRect(col * cellSize, row * cellSize, cellSize, cellSize)
            }
          }
        }

        if (logoImg.value) {
          const logoSize = pixelSize * (logoSizePct.value / 100)
          const backingSize = logoSize * 1.25
          const center = pixelSize / 2
          // The logo backing plate stays fully opaque even with a transparent
          // background, so the logo itself never shows QR modules through it.
          ctx.fillStyle = background.value
          ctx.fillRect(center - backingSize / 2, center - backingSize / 2, backingSize, backingSize)
          ctx.drawImage(logoImg.value, center - logoSize / 2, center - logoSize / 2, logoSize, logoSize)
        }
      }
    }

    const rects: string[] = []
    for (let row = 0; row < moduleCount; row += 1) {
      for (let col = 0; col < moduleCount; col += 1) {
        if (qr.isDark(row, col)) rects.push(`<rect x="${col}" y="${row}" width="1" height="1"/>`)
      }
    }
    let logoSvg = ''
    if (logoDataUrl.value) {
      const logoModules = moduleCount * (logoSizePct.value / 100)
      const backingModules = logoModules * 1.25
      const centerM = moduleCount / 2
      logoSvg = `<rect x="${centerM - backingModules / 2}" y="${centerM - backingModules / 2}" width="${backingModules}" height="${backingModules}" fill="${background.value}"/>` +
        `<image href="${logoDataUrl.value}" x="${centerM - logoModules / 2}" y="${centerM - logoModules / 2}" width="${logoModules}" height="${logoModules}" preserveAspectRatio="xMidYMid slice"/>`
    }
    svgMarkup.value = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${moduleCount} ${moduleCount}" shape-rendering="crispEdges"><rect width="${moduleCount}" height="${moduleCount}" fill="${backgroundRgba.value}"/><g fill="${foreground.value}">${rects.join('')}</g>${logoSvg}</svg>`

    isError.value = false
    status.value = logoImg.value
      ? `Generated with logo (${moduleCount}×${moduleCount} modules, EC ${ecLevel.value}).`
      : `Generated (${moduleCount}×${moduleCount} modules, ${value.length} chars, EC ${ecLevel.value}).`
  } catch (error) {
    isError.value = true
    status.value = error instanceof Error ? error.message : 'Could not generate QR code.'
    svgMarkup.value = ''
  }
}

const downloadPng = () => {
  if (!canvasRef.value) return
  canvasRef.value.toBlob((blob) => {
    if (!blob) return
    const url = URL.createObjectURL(blob)
    const anchor = document.createElement('a')
    anchor.href = url
    anchor.download = 'qrcode.png'
    document.body.appendChild(anchor)
    anchor.click()
    anchor.remove()
    URL.revokeObjectURL(url)
  }, 'image/png')
}

const downloadSvg = () => {
  if (!svgMarkup.value) return
  const blob = new Blob([svgMarkup.value], { type: 'image/svg+xml' })
  const url = URL.createObjectURL(blob)
  const anchor = document.createElement('a')
  anchor.href = url
  anchor.download = 'qrcode.svg'
  document.body.appendChild(anchor)
  anchor.click()
  anchor.remove()
  URL.revokeObjectURL(url)
}

const clearAll = () => {
  text.value = ''
  removeLogo()
  render()
}

watch([text, ecLevel, size, foreground, background, backgroundAlpha, logoSizePct], render)
onMounted(render)
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
          <button class="rail-btn shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Clear" aria-label="Clear" @click="clearAll">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M5 6h10M8 6V4h4v2m-6 0l1 10h6l1-10" /></svg>
            <span class="text-[10px] font-medium">Clear</span>
          </button>
        </div>
      </aside>

      <main class="order-1 md:order-2 flex-1 min-w-0 flex flex-col p-4 md:p-6">
      <div class="mx-auto w-full max-w-6xl flex flex-col gap-4">
        <div class="ui-panel p-4 flex items-center justify-between gap-3">
          <div class="flex items-center gap-2">
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="3" y="3" width="6" height="6" rx="1" /><rect x="11" y="3" width="6" height="6" rx="1" /><rect x="3" y="11" width="6" height="6" rx="1" /><path stroke-linecap="round" d="M12 12h2v2h-2zM16 12h1M12 16h1M16 16h1" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">QR Code Generator</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Content</span>
              <span class="text-xs font-mono text-[var(--ink-3)]">{{ text.length }}/{{ MAX_QR_CHARS }}</span>
            </div>
            <div class="px-4 pb-4 flex flex-col gap-3">
              <textarea
                v-model="text"
                spellcheck="false"
                placeholder="https://example.com or any text..."
                class="ui-input font-mono text-xs w-full min-h-[120px] resize-none"
              />

              <div class="grid grid-cols-2 gap-3">
                <label class="flex flex-col gap-1">
                  <span class="text-xs font-semibold text-[var(--ink-3)]">Error correction</span>
                  <select v-model="ecLevel" class="ui-input text-sm">
                    <option value="L">L — 7% recovery</option>
                    <option value="M">M — 15% recovery</option>
                    <option value="Q">Q — 25% recovery</option>
                    <option value="H">H — 30% recovery</option>
                  </select>
                </label>
                <label class="flex flex-col gap-1">
                  <span class="text-xs font-semibold text-[var(--ink-3)]">Size (px)</span>
                  <select v-model.number="size" class="ui-input text-sm">
                    <option :value="256">256</option>
                    <option :value="512">512</option>
                    <option :value="1024">1024</option>
                    <option :value="2048">2048</option>
                  </select>
                </label>
              </div>

              <div class="grid grid-cols-2 gap-3">
                <label class="flex flex-col gap-1">
                  <span class="text-xs font-semibold text-[var(--ink-3)]">Foreground</span>
                  <div class="flex items-center gap-2">
                    <input v-model="foreground" type="color" class="h-9 w-9 shrink-0 rounded-[var(--radius-sm)] cursor-pointer" style="border: 1px solid var(--border-strong)">
                    <input v-model="foreground" class="ui-input font-mono text-xs">
                  </div>
                </label>
                <label class="flex flex-col gap-1">
                  <span class="text-xs font-semibold text-[var(--ink-3)]">Background</span>
                  <div class="flex items-center gap-2">
                    <input v-model="background" type="color" class="h-9 w-9 shrink-0 rounded-[var(--radius-sm)] cursor-pointer" style="border: 1px solid var(--border-strong)">
                    <input v-model="background" class="ui-input font-mono text-xs">
                  </div>
                </label>
              </div>

              <div>
                <div class="flex items-center justify-between mb-1">
                  <span class="text-xs font-semibold text-[var(--ink-3)]">Background opacity</span>
                  <span class="text-xs font-mono tabular-nums text-[var(--ink-3)]">{{ backgroundAlpha }}%</span>
                </div>
                <input v-model.number="backgroundAlpha" type="range" min="0" max="100" step="1" class="w-full">
                <p v-if="backgroundAlpha < 60" class="mt-1.5 text-[11px] leading-relaxed" style="color: var(--accent)">
                  Low opacity may prevent scanners from reading the code reliably, especially over busy backgrounds.
                </p>
              </div>

              <div class="flex flex-col gap-2 pt-1" style="border-top: 1px solid var(--border)">
                <span class="text-xs font-semibold text-[var(--ink-3)]">Center logo</span>

                <div v-if="!logoDataUrl" class="flex items-center gap-2">
                  <label class="ui-button-secondary flex-1 !py-2 text-xs text-center cursor-pointer">
                    Upload image
                    <input type="file" accept="image/*" class="hidden" @change="onLogoChange">
                  </label>
                </div>

                <div v-else class="flex flex-col gap-2">
                  <div class="flex items-center gap-3">
                    <img :src="logoDataUrl" alt="Logo preview" class="h-10 w-10 shrink-0 rounded-[var(--radius-sm)] object-cover" style="border: 1px solid var(--border-strong)">
                    <div class="min-w-0 flex-1">
                      <p class="text-xs font-medium text-[var(--ink-1)] truncate">Logo applied</p>
                      <p class="text-[11px] text-[var(--ink-3)]">Color picked from image · EC forced to H</p>
                    </div>
                    <button type="button" class="ui-button-secondary !py-1.5 !px-2.5 text-xs shrink-0" @click="removeLogo">Remove</button>
                  </div>
                  <label class="flex flex-col gap-1">
                    <span class="flex items-center justify-between text-[11px] text-[var(--ink-3)]">
                      <span>Logo size</span>
                      <span class="font-mono">{{ logoSizePct }}%</span>
                    </span>
                    <input v-model.number="logoSizePct" type="range" min="12" max="30" step="1" class="w-full">
                  </label>
                </div>
              </div>
            </div>
          </section>

          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Preview</span>
            </div>
            <div class="px-4 pb-4 flex-1 flex flex-col items-center justify-center gap-4">
              <div class="p-3 rounded-[var(--radius-md)]" style="background: var(--surface-2)">
                <canvas ref="canvasRef" class="max-w-full h-auto rounded-[var(--radius-sm)]" style="width: 260px; height: 260px" />
              </div>
              <div class="flex gap-2 w-full">
                <button class="ui-button-secondary flex-1 !py-2 text-xs" :disabled="!svgMarkup" @click="downloadSvg">Download SVG</button>
                <button class="ui-button flex-1 !py-2 text-xs" :disabled="!svgMarkup" @click="downloadPng">Download PNG</button>
              </div>
            </div>
          </section>
        </div>
      </div>
      </main>
    </div>

    <div class="shrink-0 border-t px-5 py-2 flex items-center gap-4 text-xs font-mono" style="border-color: var(--border); background: var(--surface-1); color: var(--ink-3)">
      <span class="truncate" :class="isError ? 'text-red-600' : ''">{{ status }}</span>
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
