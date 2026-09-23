<script setup lang="ts">
definePageMeta({ layout: 'tools' })

useHead({
  title: 'Free Color Picker & Converter | HEX, RGB, HSL in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser color picker and converter. Convert between HEX, RGB, and HSL, and generate palettes locally. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/color/' },
    { property: 'og:title', content: 'Free Color Picker & Converter | HEX, RGB, HSL' },
    { property: 'og:description', content: 'Pick and convert colors privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/color/' }],
})

useToolSchema({
  name: 'Color Picker & Converter',
  description: 'Convert between HEX, RGB, and HSL and generate palettes, locally in your browser.',
  url: 'https://ash-tools.store/color/',
  category: 'DeveloperApplication',
})

const DEFAULT_COLOR = '#bd5b38'

const hex = ref(DEFAULT_COLOR)
const status = ref('Pick a color or enter a value.')

const clamp = (n: number, min: number, max: number) => Math.min(max, Math.max(min, n))

const normalizeHex = (value: string) => {
  let clean = value.trim().replace(/^#/, '')
  if (clean.length === 3) clean = clean.split('').map((c) => c + c).join('')
  if (!/^[0-9a-fA-F]{6}$/.test(clean)) throw new Error('Invalid hex color.')
  return `#${clean.toLowerCase()}`
}

const hexToRgb = (value: string) => {
  const clean = normalizeHex(value).slice(1)
  return {
    r: parseInt(clean.slice(0, 2), 16),
    g: parseInt(clean.slice(2, 4), 16),
    b: parseInt(clean.slice(4, 6), 16),
  }
}

const rgbToHex = (r: number, g: number, b: number) =>
  `#${[r, g, b].map((c) => clamp(Math.round(c), 0, 255).toString(16).padStart(2, '0')).join('')}`

const rgbToHsl = (r: number, g: number, b: number) => {
  const rn = r / 255
  const gn = g / 255
  const bn = b / 255
  const max = Math.max(rn, gn, bn)
  const min = Math.min(rn, gn, bn)
  const l = (max + min) / 2
  const delta = max - min
  let h = 0
  let s = 0
  if (delta !== 0) {
    s = delta / (1 - Math.abs(2 * l - 1))
    if (max === rn) h = ((gn - bn) / delta) % 6
    else if (max === gn) h = (bn - rn) / delta + 2
    else h = (rn - gn) / delta + 4
    h *= 60
    if (h < 0) h += 360
  }
  return { h: Math.round(h), s: Math.round(s * 100), l: Math.round(l * 100) }
}

const hslToRgb = (h: number, s: number, l: number) => {
  const sn = s / 100
  const ln = l / 100
  const c = (1 - Math.abs(2 * ln - 1)) * sn
  const x = c * (1 - Math.abs(((h / 60) % 2) - 1))
  const m = ln - c / 2
  let rp = 0
  let gp = 0
  let bp = 0
  if (h < 60) { rp = c; gp = x; bp = 0 }
  else if (h < 120) { rp = x; gp = c; bp = 0 }
  else if (h < 180) { rp = 0; gp = c; bp = x }
  else if (h < 240) { rp = 0; gp = x; bp = c }
  else if (h < 300) { rp = x; gp = 0; bp = c }
  else { rp = c; gp = 0; bp = x }
  return { r: (rp + m) * 255, g: (gp + m) * 255, b: (bp + m) * 255 }
}

const rgb = computed(() => {
  try {
    return hexToRgb(hex.value)
  } catch {
    return { r: 0, g: 0, b: 0 }
  }
})

const hsl = computed(() => rgbToHsl(rgb.value.r, rgb.value.g, rgb.value.b))

const luminance = computed(() => {
  const srgb = [rgb.value.r, rgb.value.g, rgb.value.b].map((c) => {
    const cn = c / 255
    return cn <= 0.03928 ? cn / 12.92 : ((cn + 0.055) / 1.055) ** 2.4
  })
  return 0.2126 * srgb[0] + 0.7152 * srgb[1] + 0.0722 * srgb[2]
})

const contrastWithWhite = computed(() => ((1.0 + 0.05) / (luminance.value + 0.05)).toFixed(2))
const contrastWithBlack = computed(() => ((luminance.value + 0.05) / (0 + 0.05)).toFixed(2))

const hexInput = ref(DEFAULT_COLOR)
const rgbInputs = reactive({ r: 0, g: 0, b: 0 })
const hslInputs = reactive({ h: 0, s: 0, l: 0 })

const syncFromHex = () => {
  try {
    hex.value = normalizeHex(hexInput.value)
    const c = hexToRgb(hex.value)
    rgbInputs.r = c.r
    rgbInputs.g = c.g
    rgbInputs.b = c.b
    const h = rgbToHsl(c.r, c.g, c.b)
    hslInputs.h = h.h
    hslInputs.s = h.s
    hslInputs.l = h.l
    status.value = 'Color updated.'
  } catch (error) {
    status.value = error instanceof Error ? error.message : 'Invalid hex color.'
  }
}

const syncFromRgb = () => {
  rgbInputs.r = clamp(rgbInputs.r, 0, 255)
  rgbInputs.g = clamp(rgbInputs.g, 0, 255)
  rgbInputs.b = clamp(rgbInputs.b, 0, 255)
  hex.value = rgbToHex(rgbInputs.r, rgbInputs.g, rgbInputs.b)
  hexInput.value = hex.value
  const h = rgbToHsl(rgbInputs.r, rgbInputs.g, rgbInputs.b)
  hslInputs.h = h.h
  hslInputs.s = h.s
  hslInputs.l = h.l
  status.value = 'Color updated.'
}

const syncFromHsl = () => {
  hslInputs.h = clamp(hslInputs.h, 0, 360)
  hslInputs.s = clamp(hslInputs.s, 0, 100)
  hslInputs.l = clamp(hslInputs.l, 0, 100)
  const c = hslToRgb(hslInputs.h, hslInputs.s, hslInputs.l)
  rgbInputs.r = Math.round(c.r)
  rgbInputs.g = Math.round(c.g)
  rgbInputs.b = Math.round(c.b)
  hex.value = rgbToHex(c.r, c.g, c.b)
  hexInput.value = hex.value
  status.value = 'Color updated.'
}

const onNativePicker = (event: Event) => {
  const value = (event.target as HTMLInputElement).value
  hexInput.value = value
  syncFromHex()
}

// ==================== Visual picker (saturation/lightness square + hue strip) ====================
const svSquareRef = ref<HTMLDivElement | null>(null)
const hueStripRef = ref<HTMLDivElement | null>(null)

const updateFromSvPointer = (event: PointerEvent) => {
  const el = svSquareRef.value
  if (!el) return
  const rect = el.getBoundingClientRect()
  const x = clamp((event.clientX - rect.left) / rect.width, 0, 1)
  const y = clamp((event.clientY - rect.top) / rect.height, 0, 1)
  hslInputs.s = Math.round(x * 100)
  hslInputs.l = Math.round((1 - y) * 100)
  syncFromHsl()
}

const updateFromHuePointer = (event: PointerEvent) => {
  const el = hueStripRef.value
  if (!el) return
  const rect = el.getBoundingClientRect()
  const x = clamp((event.clientX - rect.left) / rect.width, 0, 1)
  hslInputs.h = Math.round(x * 360)
  syncFromHsl()
}

const makeDragHandler = (onMove: (event: PointerEvent) => void) => (downEvent: PointerEvent) => {
  downEvent.preventDefault()
  onMove(downEvent)
  const target = downEvent.currentTarget as HTMLElement
  target.setPointerCapture(downEvent.pointerId)
  const onPointerMove = (moveEvent: PointerEvent) => onMove(moveEvent)
  const onPointerUp = () => {
    target.releasePointerCapture(downEvent.pointerId)
    target.removeEventListener('pointermove', onPointerMove)
    target.removeEventListener('pointerup', onPointerUp)
  }
  target.addEventListener('pointermove', onPointerMove)
  target.addEventListener('pointerup', onPointerUp)
}

const onSvPointerDown = makeDragHandler(updateFromSvPointer)
const onHuePointerDown = makeDragHandler(updateFromHuePointer)

// Pure hue at full saturation/mid lightness, used as the SV square's base tint.
const hueBackground = computed(() => {
  const c = hslToRgb(hslInputs.h, 100, 50)
  return rgbToHex(c.r, c.g, c.b)
})

const rgbString = computed(() => `rgb(${rgbInputs.r}, ${rgbInputs.g}, ${rgbInputs.b})`)
const hslString = computed(() => `hsl(${hslInputs.h}, ${hslInputs.s}%, ${hslInputs.l}%)`)

const shades = computed(() => {
  const steps = [90, 75, 60, 45, 30, 15]
  return steps.map((l) => {
    const c = hslToRgb(hslInputs.h, hslInputs.s, l)
    return rgbToHex(c.r, c.g, c.b)
  })
})

const complementary = computed(() => {
  const h = (hslInputs.h + 180) % 360
  const c = hslToRgb(h, hslInputs.s, hslInputs.l)
  return rgbToHex(c.r, c.g, c.b)
})

const analogous = computed(() => {
  return [-30, 30].map((offset) => {
    const h = ((hslInputs.h + offset) % 360 + 360) % 360
    const c = hslToRgb(h, hslInputs.s, hslInputs.l)
    return rgbToHex(c.r, c.g, c.b)
  })
})

const copyValue = async (value: string) => {
  try {
    await navigator.clipboard.writeText(value)
    status.value = `Copied ${value}.`
  } catch {
    status.value = 'Could not copy — select and copy manually.'
  }
}

const applySwatch = (value: string) => {
  hexInput.value = value
  syncFromHex()
}

onMounted(() => {
  syncFromHex()
})
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
          <label for="native-picker" class="rail-btn cursor-pointer shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Pick a color" aria-label="Pick a color">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M4 16l1.5-4.5L13 4a1.4 1.4 0 012 2l-7.5 7.5L4 16z" /></svg>
            <span class="text-[10px] font-medium">Pick</span>
          </label>
          <input
            id="native-picker"
            type="color"
            :value="hex"
            class="absolute h-px w-px overflow-hidden opacity-0"
            style="clip: rect(0 0 0 0); clip-path: inset(50%)"
            @input="onNativePicker"
          >
        </div>
      </aside>

      <main class="order-1 md:order-2 flex-1 min-w-0 flex flex-col p-4 md:p-6">
      <div class="mx-auto w-full max-w-6xl flex flex-col gap-4">
        <div class="ui-panel p-4 flex items-center justify-between gap-3">
          <div class="flex items-center gap-2">
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="10" cy="10" r="6.5" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">Color Picker &amp; Converter</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-[280px_1fr] gap-4">
          <section class="ui-panel p-4 flex flex-col gap-3">
            <!-- Saturation / lightness square -->
            <div
              ref="svSquareRef"
              class="relative w-full h-40 rounded-[var(--radius-md)] shrink-0 cursor-crosshair touch-none select-none"
              :style="{ background: `linear-gradient(to top, #000, rgba(0,0,0,0)), linear-gradient(to right, #fff, ${hueBackground})` }"
              style="border: 1px solid var(--border)"
              @pointerdown="onSvPointerDown"
            >
              <span
                class="absolute h-4 w-4 -translate-x-1/2 -translate-y-1/2 rounded-full pointer-events-none"
                style="border: 2px solid #fff; outline: 1.5px solid var(--border)"
                :style="{ left: `${hslInputs.s}%`, top: `${100 - hslInputs.l}%`, background: hex }"
              />
            </div>

            <!-- Hue strip -->
            <div
              ref="hueStripRef"
              class="relative w-full h-4 rounded-[var(--radius-sm)] shrink-0 cursor-pointer touch-none select-none"
              style="background: linear-gradient(to right, #f00, #ff0, #0f0, #0ff, #00f, #f0f, #f00); border: 1px solid var(--border)"
              @pointerdown="onHuePointerDown"
            >
              <span
                class="absolute top-1/2 h-5 w-5 -translate-x-1/2 -translate-y-1/2 rounded-full pointer-events-none"
                style="border: 2px solid #fff; outline: 1.5px solid var(--border)"
                :style="{ left: `${(hslInputs.h / 360) * 100}%`, background: hueBackground }"
              />
            </div>

            <div
              class="w-full h-10 rounded-[var(--radius-sm)] shrink-0"
              :style="{ background: hex }"
              style="border: 1px solid var(--border)"
            />

            <label class="flex flex-col gap-1">
              <span class="text-xs font-semibold text-[var(--ink-3)]">HEX</span>
              <input v-model="hexInput" class="ui-input font-mono text-sm" placeholder="#bd5b38" @change="syncFromHex">
            </label>
            <div class="grid grid-cols-3 gap-2">
              <label class="flex flex-col gap-1">
                <span class="text-xs font-semibold text-[var(--ink-3)]">R</span>
                <input v-model.number="rgbInputs.r" type="number" min="0" max="255" class="ui-input font-mono text-sm !px-2" @change="syncFromRgb">
              </label>
              <label class="flex flex-col gap-1">
                <span class="text-xs font-semibold text-[var(--ink-3)]">G</span>
                <input v-model.number="rgbInputs.g" type="number" min="0" max="255" class="ui-input font-mono text-sm !px-2" @change="syncFromRgb">
              </label>
              <label class="flex flex-col gap-1">
                <span class="text-xs font-semibold text-[var(--ink-3)]">B</span>
                <input v-model.number="rgbInputs.b" type="number" min="0" max="255" class="ui-input font-mono text-sm !px-2" @change="syncFromRgb">
              </label>
            </div>
            <div class="grid grid-cols-3 gap-2">
              <label class="flex flex-col gap-1">
                <span class="text-xs font-semibold text-[var(--ink-3)]">H</span>
                <input v-model.number="hslInputs.h" type="number" min="0" max="360" class="ui-input font-mono text-sm !px-2" @change="syncFromHsl">
              </label>
              <label class="flex flex-col gap-1">
                <span class="text-xs font-semibold text-[var(--ink-3)]">S%</span>
                <input v-model.number="hslInputs.s" type="number" min="0" max="100" class="ui-input font-mono text-sm !px-2" @change="syncFromHsl">
              </label>
              <label class="flex flex-col gap-1">
                <span class="text-xs font-semibold text-[var(--ink-3)]">L%</span>
                <input v-model.number="hslInputs.l" type="number" min="0" max="100" class="ui-input font-mono text-sm !px-2" @change="syncFromHsl">
              </label>
            </div>
          </section>

          <section class="flex flex-col gap-4 min-w-0">
            <div class="ui-panel p-4">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Values</span>
              <div class="mt-3 grid grid-cols-1 sm:grid-cols-3 gap-2">
                <button class="ui-card p-3 text-left" @click="copyValue(hex)">
                  <div class="text-[10px] font-semibold uppercase text-[var(--ink-3)]">Hex</div>
                  <div class="font-mono text-sm text-[var(--ink-1)]">{{ hex }}</div>
                </button>
                <button class="ui-card p-3 text-left" @click="copyValue(rgbString)">
                  <div class="text-[10px] font-semibold uppercase text-[var(--ink-3)]">RGB</div>
                  <div class="font-mono text-sm text-[var(--ink-1)]">{{ rgbString }}</div>
                </button>
                <button class="ui-card p-3 text-left" @click="copyValue(hslString)">
                  <div class="text-[10px] font-semibold uppercase text-[var(--ink-3)]">HSL</div>
                  <div class="font-mono text-sm text-[var(--ink-1)]">{{ hslString }}</div>
                </button>
              </div>
              <div class="mt-3 flex flex-wrap gap-4 text-xs font-mono text-[var(--ink-3)]">
                <span>Contrast vs white: {{ contrastWithWhite }}:1</span>
                <span>Contrast vs black: {{ contrastWithBlack }}:1</span>
              </div>
            </div>

            <div class="ui-panel p-4">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Shades</span>
              <div class="mt-3 grid grid-cols-6 gap-2">
                <button
                  v-for="shade in shades"
                  :key="shade"
                  class="h-14 rounded-[var(--radius-sm)] transition-transform duration-150 hover:scale-105"
                  :style="{ background: shade }"
                  style="border: 1px solid var(--border)"
                  :title="shade"
                  @click="applySwatch(shade)"
                />
              </div>
            </div>

            <div class="ui-panel p-4">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Harmonies</span>
              <div class="mt-3 grid grid-cols-1 sm:grid-cols-2 gap-4">
                <div>
                  <div class="text-xs text-[var(--ink-3)] mb-2">Complementary</div>
                  <div class="flex gap-2">
                    <button class="h-12 flex-1 rounded-[var(--radius-sm)]" :style="{ background: hex }" style="border: 1px solid var(--border)" @click="applySwatch(hex)" />
                    <button class="h-12 flex-1 rounded-[var(--radius-sm)]" :style="{ background: complementary }" style="border: 1px solid var(--border)" :title="complementary" @click="applySwatch(complementary)" />
                  </div>
                </div>
                <div>
                  <div class="text-xs text-[var(--ink-3)] mb-2">Analogous</div>
                  <div class="flex gap-2">
                    <button class="h-12 flex-1 rounded-[var(--radius-sm)]" :style="{ background: analogous[0] }" style="border: 1px solid var(--border)" :title="analogous[0]" @click="applySwatch(analogous[0])" />
                    <button class="h-12 flex-1 rounded-[var(--radius-sm)]" :style="{ background: hex }" style="border: 1px solid var(--border)" @click="applySwatch(hex)" />
                    <button class="h-12 flex-1 rounded-[var(--radius-sm)]" :style="{ background: analogous[1] }" style="border: 1px solid var(--border)" :title="analogous[1]" @click="applySwatch(analogous[1])" />
                  </div>
                </div>
              </div>
            </div>
          </section>
        </div>
      </div>
      </main>
    </div>

    <div class="shrink-0 border-t px-5 py-2 flex items-center gap-4 text-xs font-mono" style="border-color: var(--border); background: var(--surface-1); color: var(--ink-3)">
      <span class="truncate">{{ status }}</span>
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
