<script setup lang="ts">
useHead({
  title: 'Free Recipe Scaler | Scale Ingredients to Any Serving Size',
  meta: [
    { name: 'description', content: 'Paste a recipe and scale every ingredient to any number of servings. Handles fractions like 1 1/2 and ½. Free, private, and offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/recipe/' },
    { property: 'og:title', content: 'Free Recipe Scaler | Scale Ingredients' },
    { property: 'og:description', content: 'Scale recipe ingredients to any serving size in your browser.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/recipe/' }],
})

useToolSchema({
  name: 'Recipe Scaler',
  description: 'Scale recipe ingredient amounts to any number of servings.',
  url: 'https://ash-tools.store/recipe/',
  category: 'LifestyleApplication',
})

const STORAGE_KEY = 'recipe-scaler'
const MAX_SERVINGS = 1000
const SAMPLE = `2 cups all-purpose flour
2 tbsp sugar
1 1/2 tsp baking powder
½ tsp salt
1 3/4 cups milk
2 eggs
3 tbsp melted butter
1-2 tsp vanilla extract
Butter for the pan`

const UNICODE: Record<string, number> = { '¼': 1 / 4, '½': 1 / 2, '¾': 3 / 4, '⅓': 1 / 3, '⅔': 2 / 3, '⅕': 1 / 5, '⅙': 1 / 6, '⅛': 1 / 8, '⅜': 3 / 8, '⅝': 5 / 8, '⅞': 7 / 8 }
const U = Object.keys(UNICODE).join('')
const AMOUNT = `\\d{1,3}(?:,\\d{3})+|\\d+\\s+\\d+\\s*\\/\\s*\\d+|\\d+\\s*\\/\\s*\\d+|\\d*\\.\\d+|\\d+(?:[.,]\\d+)?(?:\\s*[${U}])?|[${U}]`
const LINE_RE = new RegExp(`^(\\s*(?:[-*•]\\s+)?)(${AMOUNT})(?:(\\s*(?:-|–|to)\\s*)(${AMOUNT}))?(?=\\s|$|[^\\d])`)

const input = ref(SAMPLE)
const fromServings = ref(4)
const toServings = ref(8)
const useFractions = ref(true)
const copied = ref(false)

const parseAmount = (raw: string) => {
  const s = raw.trim()
  if (/^\d{1,3}(,\d{3})+$/.test(s)) return Number(s.replace(/,/g, ''))
  const uni = s.match(new RegExp(`^(\\d*)\\s*([${U}])$`))
  if (uni) return Number(uni[1] || 0) + UNICODE[uni[2]]
  const mixed = s.match(/^(\d+)\s+(\d+)\s*\/\s*(\d+)$/)
  if (mixed) return Number(mixed[1]) + Number(mixed[2]) / Number(mixed[3])
  const frac = s.match(/^(\d+)\s*\/\s*(\d+)$/)
  if (frac) return Number(frac[2]) ? Number(frac[1]) / Number(frac[2]) : NaN
  return Number(s.replace(',', '.'))
}

const gcd = (a: number, b: number): number => (b ? gcd(b, a % b) : a)

const formatAmount = (n: number) => {
  if (!Number.isFinite(n)) return ''
  if (useFractions.value && n < 20) {
    const whole = Math.floor(n)
    const rest = n - whole
    for (const den of [2, 3, 4, 8]) {
      const num = Math.round(rest * den)
      if (Math.abs(num / den - rest) < 0.02) {
        if (num === 0) return String(whole || 0)
        if (num === den) return String(whole + 1)
        const g = gcd(num, den)
        return `${whole ? `${whole} ` : ''}${num / g}/${den / g}`
      }
    }
  }
  return String(Number(n.toFixed(n >= 100 ? 0 : n >= 10 ? 1 : 2)))
}

const factor = computed(() => {
  const a = Number(fromServings.value)
  const b = Number(toServings.value)
  return a > 0 && b > 0 ? b / a : 1
})

const validServings = computed(() => [fromServings.value, toServings.value].every((v) => Number(v) > 0 && Number(v) <= MAX_SERVINGS))

const lines = computed(() => input.value.split('\n').map((line) => {
  const m = line.match(LINE_RE)
  if (!m || !line.trim()) return { original: line, scaled: line, changed: false, prefix: '', amount: '', rest: line }
  const [full, prefix, a1, sep, a2] = m
  const n1 = parseAmount(a1)
  const n2 = a2 ? parseAmount(a2) : NaN
  if (!Number.isFinite(n1)) return { original: line, scaled: line, changed: false, prefix: '', amount: '', rest: line }
  const amount = formatAmount(n1 * factor.value) + (a2 && Number.isFinite(n2) ? `${sep}${formatAmount(n2 * factor.value)}` : '')
  const rest = line.slice(full.length)
  return { original: line, scaled: `${prefix}${amount}${rest}`, changed: true, prefix, amount, rest }
}))

const scaledText = computed(() => lines.value.map((l) => l.scaled).join('\n'))
const changedCount = computed(() => lines.value.filter((l) => l.changed).length)

const setMultiplier = (m: number) => {
  const base = Number(fromServings.value) > 0 ? Number(fromServings.value) : 1
  toServings.value = Number((base * m).toFixed(2))
}

const copyScaled = async () => {
  try {
    await navigator.clipboard.writeText(scaledText.value)
    copied.value = true
    setTimeout(() => { copied.value = false }, 1500)
  } catch {
    copied.value = false
  }
}

const loadSample = () => { input.value = SAMPLE }
const clearAll = () => { input.value = '' }

onMounted(() => {
  try {
    const saved = JSON.parse(localStorage.getItem(STORAGE_KEY) || 'null')
    if (saved && typeof saved.input === 'string') {
      input.value = saved.input
      fromServings.value = Number(saved.from) || 4
      toServings.value = Number(saved.to) || 8
    }
  } catch {
    localStorage.removeItem(STORAGE_KEY)
  }
})

watch([input, fromServings, toServings], () => {
  localStorage.setItem(STORAGE_KEY, JSON.stringify({ input: input.value, from: fromServings.value, to: toServings.value }))
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
          <button class="rail-btn text-[var(--ink-3)]" title="Load sample" aria-label="Load sample" @click="loadSample">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M5 3h7l3 3v11H5zM12 3v3h3M8 10h4M8 13h4" /></svg>
            <span>Sample</span>
          </button>
          <button class="rail-btn text-[var(--ink-3)]" title="Clear" aria-label="Clear" @click="clearAll">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M5 6h10M8 6V4h4v2m-6 0l1 10h6l1-10" /></svg>
            <span>Clear</span>
          </button>
        </div>
      </Teleport>
    </ClientOnly>

    <main class="flex-1 min-w-0 flex flex-col p-4 md:p-6">
      <div class="mx-auto w-full max-w-6xl flex flex-col gap-4">
        <div class="ui-panel px-4 flex items-center justify-between gap-3">
          <div id="page-actions" class="order-first flex flex-wrap items-center gap-1"></div>
          <div class="flex items-center gap-2">
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M3 9h14a7 7 0 0 1-14 0zM8 3v3M12 3v3" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">Recipe Scaler</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <section class="ui-panel p-4 flex flex-wrap items-end gap-4">
          <label class="flex flex-col gap-1.5">
            <span class="text-xs font-semibold text-[var(--ink-2)]">Recipe makes</span>
            <input v-model.number="fromServings" type="number" min="0.25" :max="MAX_SERVINGS" step="any" inputmode="decimal" class="ui-input !w-28 font-mono">
          </label>
          <label class="flex flex-col gap-1.5">
            <span class="text-xs font-semibold text-[var(--ink-2)]">I want to make</span>
            <input v-model.number="toServings" type="number" min="0.25" :max="MAX_SERVINGS" step="any" inputmode="decimal" class="ui-input !w-28 font-mono">
          </label>
          <div class="flex flex-col gap-1.5">
            <span class="text-xs font-semibold text-[var(--ink-2)]">Quick scale</span>
            <div class="flex items-center gap-1.5">
              <button v-for="m in [0.5, 2, 3]" :key="m" class="ui-button-secondary !py-2 !px-3 text-xs" @click="setMultiplier(m)">{{ m === 0.5 ? '½' : m }}×</button>
            </div>
          </div>
          <label class="flex items-center gap-2 text-sm text-[var(--ink-2)] pb-2">
            <input v-model="useFractions" type="checkbox"> Show fractions
          </label>
          <div class="ml-auto text-right pb-1">
            <span class="block text-xs text-[var(--ink-3)]">Multiplier</span>
            <span class="text-2xl font-extrabold font-mono text-[var(--c-life)]">{{ Number(factor.toFixed(3)) }}×</span>
          </div>
          <p v-if="!validServings" class="w-full text-xs text-[var(--danger)]" role="alert">Servings must be between 0.25 and {{ MAX_SERVINGS }}.</p>
        </section>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <section class="ui-panel flex flex-col">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <label for="recipe-input" class="text-sm font-bold">Ingredients</label>
              <span class="text-xs text-[var(--ink-3)]">One per line</span>
            </div>
            <div class="px-4 pb-4 flex-1">
              <textarea
                id="recipe-input"
                v-model="input"
                spellcheck="false"
                placeholder="2 cups flour&#10;1 1/2 tsp salt&#10;3 eggs"
                class="ui-input font-mono text-sm w-full h-full min-h-[320px] resize-y leading-relaxed"
              />
            </div>
          </section>

          <section class="ui-panel flex flex-col">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <h2 class="text-sm font-bold">Scaled for {{ toServings || 0 }}</h2>
              <button class="ui-button !py-1.5 !px-3 text-xs" :disabled="!input.trim()" @click="copyScaled">{{ copied ? 'Copied' : 'Copy' }}</button>
            </div>
            <div class="px-4 pb-4 flex-1">
              <div v-if="!input.trim()" class="h-full min-h-[320px] grid place-items-center rounded-[var(--radius-md)] border border-dashed border-[var(--border-strong)] p-6 text-center text-sm text-[var(--ink-3)]">
                Paste ingredients on the left, or load the sample recipe.
              </div>
              <ul v-else class="min-h-[320px] divide-y divide-[var(--border)] rounded-[var(--radius-md)] border border-[var(--border)]">
                <li v-for="(l, i) in lines" :key="i" class="px-3 py-2 font-mono text-sm min-h-9">
                  <template v-if="l.changed">
                    <span>{{ l.prefix }}</span><mark class="rounded px-1 bg-[color-mix(in_oklab,var(--c-life)_18%,transparent)] text-[var(--ink-1)] font-bold">{{ l.amount }}</mark><span class="text-[var(--ink-2)]">{{ l.rest }}</span>
                  </template>
                  <span v-else class="text-[var(--ink-3)]">{{ l.original || ' ' }}</span>
                </li>
              </ul>
              <p v-if="input.trim()" class="mt-2 text-xs text-[var(--ink-3)]">{{ changedCount }} of {{ lines.filter((l) => l.original.trim()).length }} lines scaled. Lines with no amount stay the same.</p>
            </div>
          </section>
        </div>
      </div>
    </main>
  </div>
</template>
