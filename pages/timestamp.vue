<script setup lang="ts">
definePageMeta({ layout: 'tools' })

useHead({
  title: 'Free Timestamp Converter | Unix, ISO 8601 & Human Dates in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser timestamp converter. Convert between Unix timestamps, ISO 8601, and human-readable dates. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/timestamp/' },
    { property: 'og:title', content: 'Free Timestamp Converter | Unix, ISO 8601 & Human Dates' },
    { property: 'og:description', content: 'Convert timestamps privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/timestamp/' }],
})

useToolSchema({
  name: 'Timestamp Converter',
  description: 'Convert between Unix timestamps, ISO 8601, and human-readable dates, locally in your browser.',
  url: 'https://ash-tools.store/timestamp/',
  category: 'DeveloperApplication',
})

const status = ref('Enter a timestamp or use "Now".')
const isError = ref(false)

const unixInput = ref(String(Math.floor(Date.now() / 1000)))
const unixUnit = ref<'seconds' | 'milliseconds'>('seconds')
const isoInput = ref(new Date().toISOString())

const currentDate = ref<Date | null>(new Date())

const nowTicker = ref(Date.now())
let tickHandle: ReturnType<typeof setInterval> | null = null

const setFromUnix = () => {
  const raw = parseFloat(unixInput.value)
  if (Number.isNaN(raw)) {
    isError.value = true
    status.value = 'Invalid Unix timestamp.'
    currentDate.value = null
    return
  }
  const ms = unixUnit.value === 'seconds' ? raw * 1000 : raw
  const date = new Date(ms)
  if (Number.isNaN(date.getTime())) {
    isError.value = true
    status.value = 'Timestamp out of range.'
    currentDate.value = null
    return
  }
  currentDate.value = date
  isoInput.value = date.toISOString()
  isError.value = false
  status.value = 'Converted from Unix timestamp.'
}

const setFromIso = () => {
  const date = new Date(isoInput.value)
  if (Number.isNaN(date.getTime())) {
    isError.value = true
    status.value = 'Invalid date/ISO string.'
    currentDate.value = null
    return
  }
  currentDate.value = date
  unixInput.value = String(unixUnit.value === 'seconds' ? Math.floor(date.getTime() / 1000) : date.getTime())
  isError.value = false
  status.value = 'Converted from ISO date.'
}

const setNow = () => {
  const date = new Date()
  currentDate.value = date
  isoInput.value = date.toISOString()
  unixInput.value = String(unixUnit.value === 'seconds' ? Math.floor(date.getTime() / 1000) : date.getTime())
  isError.value = false
  status.value = 'Set to current time.'
}

watch(unixUnit, () => {
  if (currentDate.value) {
    unixInput.value = String(unixUnit.value === 'seconds' ? Math.floor(currentDate.value.getTime() / 1000) : currentDate.value.getTime())
  }
})

const pad = (n: number) => String(n).padStart(2, '0')

const formats = computed(() => {
  const date = currentDate.value
  if (!date) return null
  return {
    iso: date.toISOString(),
    utc: date.toUTCString(),
    local: date.toString(),
    localeDate: date.toLocaleDateString(undefined, { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' }),
    localeTime: date.toLocaleTimeString(),
    unixSeconds: Math.floor(date.getTime() / 1000),
    unixMs: date.getTime(),
    sqlDatetime: `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())} ${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}`,
    relative: relativeTime(date),
  }
})

function relativeTime(date: Date) {
  const diffMs = date.getTime() - nowTicker.value
  const diffSec = Math.round(diffMs / 1000)
  const abs = Math.abs(diffSec)
  const units: [number, string][] = [[60, 'second'], [60, 'minute'], [24, 'hour'], [7, 'day'], [4.345, 'week'], [12, 'month'], [Infinity, 'year']]
  let value = abs
  let unitLabel = 'second'
  let divisor = 1
  for (const [amount, name] of units) {
    if (value < amount) { unitLabel = name; break }
    value = Math.floor(value / amount)
    divisor *= amount
  }
  const rtf = new Intl.RelativeTimeFormat(undefined, { numeric: 'auto' })
  const roundedValue = Math.round(abs / divisor) || 0
  return rtf.format(diffSec < 0 ? -roundedValue : roundedValue, unitLabel as Intl.RelativeTimeFormatUnit)
}

const copyValue = async (value: string | number) => {
  try {
    await navigator.clipboard.writeText(String(value))
    status.value = 'Copied to clipboard.'
  } catch {
    status.value = 'Could not copy — select and copy manually.'
  }
}

onMounted(() => {
  setFromUnix()
  tickHandle = setInterval(() => { nowTicker.value = Date.now() }, 1000)
})

onUnmounted(() => {
  if (tickHandle) clearInterval(tickHandle)
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
          <button class="rail-btn shrink-0 w-14 h-14 rounded-none flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Set to now" aria-label="Set to now" @click="setNow">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="10" cy="10" r="7" /><path stroke-linecap="round" stroke-linejoin="round" d="M10 6v4l3 2" /></svg>
            <span class="text-[10px] font-medium">Now</span>
          </button>
        </div>
      </aside>

      <main class="order-1 md:order-2 flex-1 min-w-0 flex flex-col p-4 md:p-6">
      <div class="mx-auto w-full max-w-4xl flex flex-col gap-4">
        <div class="ui-panel p-4 flex items-center justify-between gap-3">
          <div class="flex items-center gap-2">
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="10" cy="10" r="7" /><path stroke-linecap="round" stroke-linejoin="round" d="M10 6v4l3 2" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">Timestamp Converter</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <section class="ui-panel p-4 flex flex-col gap-3">
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
            <label class="flex flex-col gap-1">
              <span class="text-xs font-semibold text-[var(--ink-3)]">Unix timestamp</span>
              <div class="flex gap-2">
                <input v-model="unixInput" type="text" inputmode="numeric" class="ui-input font-mono text-sm flex-1" @input="setFromUnix">
                <select v-model="unixUnit" class="ui-input text-xs !w-auto" @change="setFromUnix">
                  <option value="seconds">sec</option>
                  <option value="milliseconds">ms</option>
                </select>
              </div>
            </label>
            <label class="flex flex-col gap-1">
              <span class="text-xs font-semibold text-[var(--ink-3)]">ISO 8601 / date string</span>
              <input v-model="isoInput" type="text" class="ui-input font-mono text-sm" @input="setFromIso">
            </label>
          </div>
          <button class="ui-button self-start !py-1.5 !px-4 text-xs" @click="setNow">Use current time</button>
        </section>

        <section v-if="formats" class="ui-panel p-4">
          <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Formats</span>
          <div class="mt-3 rounded-none overflow-hidden divide-y border" style="border-color: var(--border)">
            <button v-for="row in [
              { label: 'ISO 8601', value: formats.iso },
              { label: 'UTC', value: formats.utc },
              { label: 'Local', value: formats.local },
              { label: 'Readable date', value: formats.localeDate },
              { label: 'Readable time', value: formats.localeTime },
              { label: 'SQL datetime', value: formats.sqlDatetime },
              { label: 'Unix (seconds)', value: formats.unixSeconds },
              { label: 'Unix (ms)', value: formats.unixMs },
              { label: 'Relative', value: formats.relative },
            ]" :key="row.label" class="w-full flex items-center justify-between gap-3 px-3 py-2 text-left hover:opacity-80" style="background: var(--surface-1)" @click="copyValue(row.value)">
              <span class="text-xs font-semibold w-32 shrink-0 text-[var(--ink-3)]">{{ row.label }}</span>
              <span class="text-xs font-mono truncate flex-1 text-[var(--ink-1)]">{{ row.value }}</span>
            </button>
          </div>
        </section>
      </div>
      </main>
    </div>

    <div class="shrink-0 border-t px-5 py-2 flex items-center gap-3 text-xs font-mono" style="border-color: var(--border); background: var(--surface-1); color: var(--ink-3)">
      <span class="status-dot w-2 h-2 rounded-full inline-block shrink-0" :class="isError ? 'err' : 'ok'"></span>
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
