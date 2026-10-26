<script setup lang="ts">
useHead({
  title: 'Free Time Zone Converter with Overlap View | Meeting Planner',
  meta: [
    { name: 'description', content: 'Convert times between time zones and see shared working hours at a glance. Free, private, and works offline in your browser.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/timezone/' },
    { property: 'og:title', content: 'Free Time Zone Converter with Overlap View' },
    { property: 'og:description', content: 'Compare time zones and find hours that work for everyone.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/timezone/' }],
})

useToolSchema({
  name: 'Time Zone Converter',
  description: 'Convert times between time zones and find overlapping working hours.',
  url: 'https://ash-tools.store/timezone/',
})

const STORAGE_KEY = 'timezone-zones'
const DEFAULT_ZONES = ['America/New_York', 'Europe/London', 'Asia/Kolkata', 'Asia/Tokyo']
const FALLBACK_ZONES = [
  'UTC', 'America/Los_Angeles', 'America/Denver', 'America/Chicago', 'America/New_York', 'America/Sao_Paulo',
  'Europe/London', 'Europe/Berlin', 'Europe/Paris', 'Europe/Moscow', 'Africa/Cairo', 'Africa/Lagos',
  'Asia/Dubai', 'Asia/Kolkata', 'Asia/Singapore', 'Asia/Shanghai', 'Asia/Tokyo', 'Australia/Sydney', 'Pacific/Auckland',
]
const MAX_ZONES = 10
const HOUR_MS = 3_600_000

type Parts = { y: number, mo: number, d: number, h: number, mi: number, s: number }

const ready = ref(false)
const allZones = ref<string[]>(FALLBACK_ZONES)
const zones = ref<string[]>([])
const when = ref('')
const workStart = ref(9)
const workEnd = ref(18)
const newZone = ref('')
const status = ref('')

const formatters = new Map<string, Intl.DateTimeFormat>()
const partsIn = (ms: number, tz: string): Parts => {
  let f = formatters.get(tz)
  if (!f) {
    f = new Intl.DateTimeFormat('en-US', { timeZone: tz, hourCycle: 'h23', year: 'numeric', month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit', second: '2-digit' })
    formatters.set(tz, f)
  }
  const p = Object.fromEntries(f.formatToParts(ms).map((x) => [x.type, Number(x.value)]))
  return { y: p.year, mo: p.month, d: p.day, h: p.hour % 24, mi: p.minute, s: p.second }
}

const offsetMin = (tz: string, ms: number) => {
  const p = partsIn(ms, tz)
  return Math.round((Date.UTC(p.y, p.mo - 1, p.d, p.h, p.mi, p.s) - Math.floor(ms / 1000) * 1000) / 60000)
}

const zonedToMs = (value: string, tz: string) => {
  const [date, time = '00:00'] = value.split('T')
  const [y, m, d] = date.split('-').map(Number)
  const [h, mi] = time.split(':').map(Number)
  const guess = Date.UTC(y, m - 1, d, h, mi)
  const first = guess - offsetMin(tz, guess) * 60000
  return guess - offsetMin(tz, first) * 60000
}

const pad = (n: number) => String(n).padStart(2, '0')
const toInput = (ms: number, tz: string) => {
  const p = partsIn(ms, tz)
  return `${p.y}-${pad(p.mo)}-${pad(p.d)}T${pad(p.h)}:${pad(p.mi)}`
}

const isValidZone = (tz: string) => {
  try {
    new Intl.DateTimeFormat('en-US', { timeZone: tz })
    return true
  } catch {
    return false
  }
}

const cityOf = (tz: string) => tz.split('/').pop()!.replace(/_/g, ' ')
const fmtOffset = (min: number) => {
  const sign = min < 0 ? '-' : '+'
  const a = Math.abs(min)
  return `UTC${sign}${Math.floor(a / 60)}${a % 60 ? `:${pad(a % 60)}` : ''}`
}
const fmtDiff = (min: number) => {
  if (!min) return 'Same time'
  const sign = min < 0 ? '-' : '+'
  const a = Math.abs(min)
  return `${sign}${Math.floor(a / 60)}h${a % 60 ? ` ${a % 60}m` : ''}`
}

const refZone = computed(() => zones.value[0] ?? 'UTC')
const instant = computed(() => ready.value && when.value ? zonedToMs(when.value, refZone.value) : 0)

const cards = computed(() => {
  if (!ready.value) return []
  const refOff = offsetMin(refZone.value, instant.value)
  return zones.value.map((tz) => {
    const off = offsetMin(tz, instant.value)
    return {
      tz,
      city: cityOf(tz),
      time: new Intl.DateTimeFormat(undefined, { timeZone: tz, hour: '2-digit', minute: '2-digit' }).format(instant.value),
      date: new Intl.DateTimeFormat(undefined, { timeZone: tz, weekday: 'short', month: 'short', day: 'numeric' }).format(instant.value),
      offset: fmtOffset(off),
      diff: fmtDiff(off - refOff),
    }
  })
})

const inWork = (h: number) => workStart.value === workEnd.value
  ? true
  : workStart.value < workEnd.value ? h >= workStart.value && h < workEnd.value : h >= workStart.value || h < workEnd.value

const dayHours = computed(() => {
  if (!ready.value || !when.value) return []
  const start = zonedToMs(`${when.value.slice(0, 10)}T00:00`, refZone.value)
  return Array.from({ length: 24 }, (_, i) => start + i * HOUR_MS)
})

const rows = computed(() => zones.value.map((tz) => ({
  tz,
  city: cityOf(tz),
  cells: dayHours.value.map((ms) => {
    const p = partsIn(ms, tz)
    return { ms, h: p.h, mi: p.mi, work: inWork(p.h) }
  }),
})))

const overlap = computed(() => dayHours.value.map((_, i) => rows.value.every((r) => r.cells[i]?.work)))

const selectedCol = computed(() => {
  if (!dayHours.value.length) return -1
  return dayHours.value.findIndex((ms) => instant.value >= ms && instant.value < ms + HOUR_MS)
})

const overlapSummary = computed(() => {
  const ranges: string[] = []
  let start = -1
  overlap.value.forEach((ok, i) => {
    if (ok && start < 0) start = i
    if ((!ok || i === overlap.value.length - 1) && start >= 0) {
      const end = ok ? i + 1 : i
      const fmt = (idx: number) => new Intl.DateTimeFormat(undefined, { timeZone: refZone.value, hour: '2-digit', minute: '2-digit' }).format(dayHours.value[0] + idx * HOUR_MS)
      ranges.push(`${fmt(start)} to ${fmt(end)}`)
      start = -1
    }
  })
  return ranges
})

const pickCell = (ms: number) => { when.value = toInput(ms, refZone.value) }
const setNow = () => { when.value = toInput(Date.now(), refZone.value) }

const addZone = () => {
  const tz = newZone.value.trim()
  if (!tz) return
  const match = allZones.value.find((z) => z.toLowerCase() === tz.toLowerCase()) ?? (isValidZone(tz) ? tz : '')
  if (!match) {
    status.value = `"${tz}" is not a known time zone.`
    return
  }
  if (zones.value.includes(match)) {
    status.value = `${cityOf(match)} is already in the list.`
    return
  }
  if (zones.value.length >= MAX_ZONES) {
    status.value = `You can compare up to ${MAX_ZONES} zones.`
    return
  }
  zones.value.push(match)
  newZone.value = ''
  status.value = ''
}

const removeZone = (tz: string) => {
  if (zones.value.length <= 1) return
  const keep = instant.value
  zones.value = zones.value.filter((z) => z !== tz)
  when.value = toInput(keep, refZone.value)
}

const makeBase = (tz: string) => {
  const keep = instant.value
  zones.value = [tz, ...zones.value.filter((z) => z !== tz)]
  when.value = toInput(keep, tz)
}

const resetZones = () => {
  const local = Intl.DateTimeFormat().resolvedOptions().timeZone || 'UTC'
  zones.value = [...new Set([local, ...DEFAULT_ZONES])].slice(0, MAX_ZONES)
  setNow()
}

watch(zones, (z) => { if (ready.value) localStorage.setItem(STORAGE_KEY, JSON.stringify(z)) }, { deep: true })

onMounted(() => {
  if (typeof Intl.supportedValuesOf === 'function') allZones.value = Intl.supportedValuesOf('timeZone')
  const local = Intl.DateTimeFormat().resolvedOptions().timeZone || 'UTC'
  let saved: string[] = []
  try {
    const parsed = JSON.parse(localStorage.getItem(STORAGE_KEY) || '[]')
    if (Array.isArray(parsed)) saved = parsed.filter((z): z is string => typeof z === 'string' && isValidZone(z))
  } catch {
    saved = []
  }
  zones.value = saved.length ? saved.slice(0, MAX_ZONES) : [...new Set([local, ...DEFAULT_ZONES])]
  ready.value = true
  setNow()
})
</script>

<template>
  <div class="flex-1 flex flex-col">
    <ClientOnly>
      <Teleport to="#page-actions">
        <div class="flex items-center gap-1">
          <NuxtLink to="/" class="rail-btn text-[var(--ink-3)]" title="Back to home" aria-label="Back to home">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6"><path stroke-linecap="round" stroke-linejoin="round" d="M11.5 4L6 9.5l5.5 5.5" /></svg>
            <span>Home</span>
          </NuxtLink>
          <button class="rail-btn text-[var(--ink-3)]" title="Set to now" aria-label="Set to now" @click="setNow">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="10" cy="10" r="7" /><path stroke-linecap="round" stroke-linejoin="round" d="M10 6v4l3 2" /></svg>
            <span>Now</span>
          </button>
          <button class="rail-btn text-[var(--ink-3)]" title="Reset zones" aria-label="Reset zones" @click="resetZones">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M4 10a6 6 0 1 0 1.8-4.3M4 4v3h3" /></svg>
            <span>Reset</span>
          </button>
        </div>
      </Teleport>
    </ClientOnly>

    <main class="flex-1 min-w-0 flex flex-col p-4 md:p-6">
      <div class="mx-auto w-full max-w-6xl flex flex-col gap-4">
        <div class="ui-panel px-4 flex items-center justify-between gap-3">
          <div id="page-actions" class="order-first flex flex-wrap items-center gap-1"></div>
          <div class="flex items-center gap-2">
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="10" cy="10" r="7" /><path stroke-linecap="round" d="M3 10h14M10 3c2 2 2.8 4.4 2.8 7S12 15 10 17c-2-2-2.8-4.4-2.8-7S8 5 10 3z" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">Time Zone Converter</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <section class="ui-panel p-4 grid grid-cols-1 md:grid-cols-[1.4fr_1fr_1.4fr] gap-4 items-end">
          <label class="flex flex-col gap-1.5">
            <span class="text-xs font-semibold text-[var(--ink-2)]">Time in {{ cityOf(refZone) }}</span>
            <input v-model="when" type="datetime-local" class="ui-input font-mono" :disabled="!ready">
          </label>
          <div class="flex flex-col gap-1.5">
            <span class="text-xs font-semibold text-[var(--ink-2)]">Work hours</span>
            <div class="flex items-center gap-2">
              <select v-model.number="workStart" class="ui-input font-mono" aria-label="Work start hour">
                <option v-for="h in 24" :key="h" :value="h - 1">{{ pad(h - 1) }}:00</option>
              </select>
              <span class="text-xs text-[var(--ink-3)]">to</span>
              <select v-model.number="workEnd" class="ui-input font-mono" aria-label="Work end hour">
                <option v-for="h in 24" :key="h" :value="h % 24">{{ pad(h % 24) }}:00</option>
              </select>
            </div>
          </div>
          <form class="flex flex-col gap-1.5" @submit.prevent="addZone">
            <label for="tz-add" class="text-xs font-semibold text-[var(--ink-2)]">Add a time zone</label>
            <div class="flex items-center gap-2">
              <input id="tz-add" v-model="newZone" list="tz-list" class="ui-input" placeholder="Europe/Berlin" autocomplete="off">
              <datalist id="tz-list">
                <option v-for="z in allZones" :key="z" :value="z" />
              </datalist>
              <button type="submit" class="ui-button shrink-0">Add</button>
            </div>
          </form>
          <p v-if="status" class="md:col-span-3 text-xs text-[var(--danger)]" role="alert">{{ status }}</p>
        </section>

        <section class="grid grid-cols-[repeat(auto-fill,minmax(min(100%,13rem),1fr))] gap-3">
          <template v-if="ready">
            <article v-for="(c, i) in cards" :key="c.tz" class="ui-panel p-4 flex flex-col gap-1" :class="i === 0 && 'ring-2 ring-[var(--c-life)]'">
              <div class="flex items-center justify-between gap-2">
                <span class="text-sm font-bold text-[var(--ink-1)] truncate" :title="c.tz">{{ c.city }}</span>
                <span class="text-[11px] font-mono text-[var(--ink-3)] shrink-0">{{ c.offset }}</span>
              </div>
              <span class="text-3xl font-extrabold tracking-tight font-mono text-[var(--ink-1)]">{{ c.time }}</span>
              <span class="text-xs text-[var(--ink-3)]">{{ c.date }} · {{ i === 0 ? 'Base zone' : c.diff }}</span>
              <div class="mt-2 flex items-center gap-1.5">
                <button v-if="i !== 0" class="ui-button-secondary !py-1 !px-2.5 text-xs" @click="makeBase(c.tz)">Set as base</button>
                <button v-if="zones.length > 1" class="ui-button-secondary !py-1 !px-2.5 text-xs" :aria-label="`Remove ${c.city}`" @click="removeZone(c.tz)">Remove</button>
              </div>
            </article>
          </template>
          <template v-else>
            <div v-for="n in 4" :key="n" class="ui-panel h-36 animate-pulse bg-[var(--surface-2)]" />
          </template>
        </section>

        <section class="ui-panel p-4 flex flex-col gap-3">
          <div class="flex flex-wrap items-baseline justify-between gap-2">
            <h2 class="text-sm font-bold">Overlap view</h2>
            <p class="text-xs text-[var(--ink-2)]">
              <template v-if="overlapSummary.length">Shared work hours in {{ cityOf(refZone) }}: <strong class="text-[var(--ink-1)]">{{ overlapSummary.join(', ') }}</strong></template>
              <template v-else-if="ready">No shared work hours on this day. Try wider work hours.</template>
            </p>
          </div>

          <div class="overflow-x-auto">
            <div class="grid min-w-[760px] gap-y-1" style="grid-template-columns: 7.5rem repeat(24, minmax(0, 1fr))">
              <template v-for="r in rows" :key="r.tz">
                <span class="sticky left-0 z-10 bg-[var(--surface-1)] pr-2 text-xs font-semibold text-[var(--ink-2)] truncate self-center" :title="r.tz">{{ r.city }}</span>
                <button
                  v-for="(cell, i) in r.cells"
                  :key="cell.ms"
                  type="button"
                  class="tz-cell"
                  :class="[cell.work && 'is-work', i === selectedCol && 'is-selected']"
                  :title="`${r.city} ${pad(cell.h)}:${pad(cell.mi)}`"
                  @click="pickCell(cell.ms)"
                >
                  {{ cell.h }}
                </button>
              </template>
              <span class="sticky left-0 z-10 bg-[var(--surface-1)] pr-2 text-xs font-bold text-[var(--ink-1)] self-center">Everyone</span>
              <span
                v-for="(ok, i) in overlap"
                :key="i"
                class="tz-cell"
                :class="[ok && 'is-overlap', i === selectedCol && 'is-selected']"
                aria-hidden="true"
              />
            </div>
          </div>
          <p class="text-xs text-[var(--ink-3)]">Each column is one hour in {{ cityOf(refZone) }}. Click a cell to jump to that hour.</p>
        </section>
      </div>
    </main>
  </div>
</template>

<style scoped>
.tz-cell {
  height: 2rem;
  margin-inline: 1px;
  border-radius: 6px;
  background: var(--surface-2);
  color: var(--ink-3);
  font-family: var(--font-mono);
  font-size: 0.6875rem;
  display: grid;
  place-items: center;
  transition: background-color 120ms ease, box-shadow 120ms ease;
}

.tz-cell.is-work {
  background: color-mix(in oklab, var(--c-life) 22%, var(--surface-1));
  color: var(--ink-1);
}

.tz-cell.is-overlap {
  background: var(--c-life);
}

.tz-cell.is-selected {
  box-shadow: inset 0 0 0 2px var(--accent);
}

@media (hover: hover) and (pointer: fine) {
  button.tz-cell:hover {
    box-shadow: inset 0 0 0 2px var(--border-strong);
  }
}
</style>
