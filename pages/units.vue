<script setup lang="ts">
definePageMeta({ layout: 'tools' })

useHead({
  title: 'Free Unit Converter | Length, Weight, Temperature & more in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser unit converter for length, weight, temperature, data size, and time. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/units/' },
    { property: 'og:title', content: 'Free Unit Converter | Length, Weight, Temperature & more' },
    { property: 'og:description', content: 'Convert units privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/units/' }],
})

useToolSchema({
  name: 'Unit Converter',
  description: 'Convert length, weight, temperature, data size, and time units locally in your browser.',
  url: 'https://ash-tools.store/units/',
  category: 'UtilitiesApplication',
})

type Category = 'length' | 'weight' | 'temperature' | 'data' | 'time'

type UnitDef = { key: string; label: string; toBase: (v: number) => number; fromBase: (v: number) => number }

const categories: Record<Category, { label: string; units: UnitDef[] }> = {
  length: {
    label: 'Length',
    units: [
      { key: 'mm', label: 'Millimeters', toBase: (v) => v / 1000, fromBase: (v) => v * 1000 },
      { key: 'cm', label: 'Centimeters', toBase: (v) => v / 100, fromBase: (v) => v * 100 },
      { key: 'm', label: 'Meters', toBase: (v) => v, fromBase: (v) => v },
      { key: 'km', label: 'Kilometers', toBase: (v) => v * 1000, fromBase: (v) => v / 1000 },
      { key: 'in', label: 'Inches', toBase: (v) => v * 0.0254, fromBase: (v) => v / 0.0254 },
      { key: 'ft', label: 'Feet', toBase: (v) => v * 0.3048, fromBase: (v) => v / 0.3048 },
      { key: 'yd', label: 'Yards', toBase: (v) => v * 0.9144, fromBase: (v) => v / 0.9144 },
      { key: 'mi', label: 'Miles', toBase: (v) => v * 1609.344, fromBase: (v) => v / 1609.344 },
    ],
  },
  weight: {
    label: 'Weight',
    units: [
      { key: 'mg', label: 'Milligrams', toBase: (v) => v / 1e6, fromBase: (v) => v * 1e6 },
      { key: 'g', label: 'Grams', toBase: (v) => v / 1000, fromBase: (v) => v * 1000 },
      { key: 'kg', label: 'Kilograms', toBase: (v) => v, fromBase: (v) => v },
      { key: 't', label: 'Metric tons', toBase: (v) => v * 1000, fromBase: (v) => v / 1000 },
      { key: 'oz', label: 'Ounces', toBase: (v) => v * 0.0283495, fromBase: (v) => v / 0.0283495 },
      { key: 'lb', label: 'Pounds', toBase: (v) => v * 0.453592, fromBase: (v) => v / 0.453592 },
    ],
  },
  temperature: {
    label: 'Temperature',
    units: [
      { key: 'c', label: 'Celsius', toBase: (v) => v, fromBase: (v) => v },
      { key: 'f', label: 'Fahrenheit', toBase: (v) => ((v - 32) * 5) / 9, fromBase: (v) => (v * 9) / 5 + 32 },
      { key: 'k', label: 'Kelvin', toBase: (v) => v - 273.15, fromBase: (v) => v + 273.15 },
    ],
  },
  data: {
    label: 'Data size',
    units: [
      { key: 'b', label: 'Bytes', toBase: (v) => v, fromBase: (v) => v },
      { key: 'kb', label: 'Kilobytes', toBase: (v) => v * 1024, fromBase: (v) => v / 1024 },
      { key: 'mb', label: 'Megabytes', toBase: (v) => v * 1024 ** 2, fromBase: (v) => v / 1024 ** 2 },
      { key: 'gb', label: 'Gigabytes', toBase: (v) => v * 1024 ** 3, fromBase: (v) => v / 1024 ** 3 },
      { key: 'tb', label: 'Terabytes', toBase: (v) => v * 1024 ** 4, fromBase: (v) => v / 1024 ** 4 },
    ],
  },
  time: {
    label: 'Time',
    units: [
      { key: 'ms', label: 'Milliseconds', toBase: (v) => v / 1000, fromBase: (v) => v * 1000 },
      { key: 's', label: 'Seconds', toBase: (v) => v, fromBase: (v) => v },
      { key: 'min', label: 'Minutes', toBase: (v) => v * 60, fromBase: (v) => v / 60 },
      { key: 'h', label: 'Hours', toBase: (v) => v * 3600, fromBase: (v) => v / 3600 },
      { key: 'd', label: 'Days', toBase: (v) => v * 86400, fromBase: (v) => v / 86400 },
      { key: 'wk', label: 'Weeks', toBase: (v) => v * 604800, fromBase: (v) => v / 604800 },
    ],
  },
}

const category = ref<Category>('length')
const fromUnit = ref('m')
const toUnit = ref('ft')
const fromValue = ref('1')

const currentUnits = computed(() => categories[category.value].units)

const setDefaultUnits = () => {
  const units = currentUnits.value
  fromUnit.value = units[0]?.key || ''
  toUnit.value = units[1]?.key || units[0]?.key || ''
}

watch(category, setDefaultUnits)

const result = computed(() => {
  const units = currentUnits.value
  const from = units.find((u) => u.key === fromUnit.value)
  const to = units.find((u) => u.key === toUnit.value)
  const num = parseFloat(fromValue.value)
  if (!from || !to || Number.isNaN(num)) return null
  const base = from.toBase(num)
  return to.fromBase(base)
})

const formattedResult = computed(() => {
  if (result.value === null) return ''
  if (!Number.isFinite(result.value)) return 'Invalid'
  const abs = Math.abs(result.value)
  if (abs !== 0 && (abs < 0.0001 || abs > 1e12)) return result.value.toExponential(6)
  return parseFloat(result.value.toFixed(8)).toString()
})

const swap = () => {
  const temp = fromUnit.value
  fromUnit.value = toUnit.value
  toUnit.value = temp
}

const copyResult = async () => {
  if (!formattedResult.value) return
  try {
    await navigator.clipboard.writeText(formattedResult.value)
  } catch {
    // clipboard unavailable; no-op
  }
}

const quickTable = computed(() => {
  const units = currentUnits.value
  const from = units.find((u) => u.key === fromUnit.value)
  const num = parseFloat(fromValue.value)
  if (!from || Number.isNaN(num)) return []
  const base = from.toBase(num)
  return units
    .filter((u) => u.key !== fromUnit.value)
    .map((u) => ({ label: u.label, value: u.fromBase(base) }))
})
</script>

<template>
  <div class="min-h-screen bg-[var(--surface-0)] text-[var(--ink-1)] flex flex-col">
    <div class="flex-1 flex flex-col md:flex-row">
      <aside class="order-2 md:order-1 shrink-0 border-t md:border-t-0 md:border-r md:w-[76px] w-full" style="background: var(--surface-1); border-color: var(--border)">
        <div class="flex md:flex-col items-center gap-1 p-2 md:py-4 overflow-x-auto md:overflow-visible">
          <NuxtLink to="/" class="rail-btn shrink-0 w-14 h-14 rounded-none flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Back to home" aria-label="Back to home">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6"><path stroke-linecap="round" stroke-linejoin="round" d="M11.5 4L6 9.5l5.5 5.5" /></svg>
            <span class="text-[10px] font-medium">Home</span>
          </NuxtLink>
        </div>
      </aside>

      <main class="order-1 md:order-2 flex-1 min-w-0 flex flex-col p-4 md:p-6">
      <div class="mx-auto w-full max-w-4xl flex flex-col gap-4">
        <div class="ui-panel p-4 flex items-center justify-between gap-3">
          <div class="flex items-center gap-2">
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M4 7h9M13 7l-2.5-2.5M13 7l-2.5 2.5M16 13H7M7 13l2.5-2.5M7 13l2.5 2.5" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">Unit Converter</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <div class="ui-panel p-3 flex flex-wrap items-center gap-1 rounded-none">
          <button
            v-for="(cat, key) in categories"
            :key="key"
            class="px-3 py-1.5 text-xs font-semibold rounded-none transition-colors duration-150"
            :class="category === key ? 'ui-button !py-1.5 !px-3' : 'text-[var(--ink-3)] hover:text-[var(--ink-1)]'"
            @click="category = key as Category"
          >
            {{ cat.label }}
          </button>
        </div>

        <section class="ui-panel p-4">
          <div class="grid grid-cols-1 sm:grid-cols-[1fr_auto_1fr] gap-3 items-end">
            <label class="flex flex-col gap-1">
              <span class="text-xs font-semibold text-[var(--ink-3)]">From</span>
              <select v-model="fromUnit" class="ui-input text-sm">
                <option v-for="u in currentUnits" :key="u.key" :value="u.key">{{ u.label }}</option>
              </select>
              <input v-model="fromValue" type="text" inputmode="decimal" class="ui-input font-mono text-sm mt-1" placeholder="Enter value">
            </label>

            <button class="ui-button-secondary !p-2.5 rounded-none self-center mb-1" title="Swap units" aria-label="Swap units" @click="swap">
              <svg class="h-4 w-4" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M7 4L4 7l3 3M4 7h12M13 16l3-3-3-3M16 13H4" /></svg>
            </button>

            <label class="flex flex-col gap-1">
              <span class="text-xs font-semibold text-[var(--ink-3)]">To</span>
              <select v-model="toUnit" class="ui-input text-sm">
                <option v-for="u in currentUnits" :key="u.key" :value="u.key">{{ u.label }}</option>
              </select>
              <button class="ui-input font-mono text-sm mt-1 text-left truncate" style="cursor: text" @click="copyResult">
                {{ formattedResult || '—' }}
              </button>
            </label>
          </div>
        </section>

        <section v-if="quickTable.length" class="ui-panel p-4">
          <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Quick reference</span>
          <div class="mt-3 grid grid-cols-2 sm:grid-cols-3 gap-2">
            <div v-for="row in quickTable" :key="row.label" class="p-2.5 rounded-none border" style="background: var(--surface-2); border-color: var(--border)">
              <div class="text-[10px] text-[var(--ink-3)]">{{ row.label }}</div>
              <div class="text-sm font-mono font-semibold text-[var(--ink-1)] truncate">{{ Number.isFinite(row.value) ? parseFloat(row.value.toFixed(6)) : '—' }}</div>
            </div>
          </div>
        </section>
      </div>
      </main>
    </div>

    <div class="shrink-0 border-t px-5 py-2 flex items-center gap-3 text-xs font-mono" style="border-color: var(--border); background: var(--surface-1); color: var(--ink-3)">
      <span class="status-dot ok w-2 h-2 rounded-full inline-block shrink-0"></span>
      <span class="truncate">Click the result to copy it.</span>
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
