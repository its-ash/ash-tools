<script setup lang="ts">
useHead({
  title: 'Free Compound Interest & Savings Goal Calculator',
  meta: [
    { name: 'description', content: 'See how your savings grow with compound interest, or find the monthly deposit you need to reach a goal. Free, private, and offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/savings/' },
    { property: 'og:title', content: 'Free Compound Interest & Savings Goal Calculator' },
    { property: 'og:description', content: 'Plan savings growth and goals privately in your browser.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/savings/' }],
})

useToolSchema({
  name: 'Compound Interest & Savings Goal Calculator',
  description: 'Calculate compound interest growth and the monthly savings needed to reach a goal.',
  url: 'https://ash-tools.store/savings/',
  category: 'FinanceApplication',
})

const CURRENCIES = ['INR', 'USD', 'EUR', 'GBP', 'JPY', 'CAD', 'AUD']
const DEFAULT_CURRENCY = 'INR'
const COMPOUNDING = [
  { n: 1, label: 'Yearly' },
  { n: 4, label: 'Quarterly' },
  { n: 12, label: 'Monthly' },
  { n: 365, label: 'Daily' },
]
const MAX_YEARS = 100
const MAX_RATE = 100
const MAX_AMOUNT = 1e12

type Mode = 'growth' | 'goal'

const mode = ref<Mode>('growth')
const currency = ref(DEFAULT_CURRENCY)
const initial = ref(50000)
const monthly = ref(5000)
const rate = ref(7)
const years = ref(15)
const compounding = ref(12)
const goal = ref(2500000)

const clamp = (v: unknown, min: number, max: number) => {
  const n = Number(v)
  return Number.isFinite(n) ? Math.min(max, Math.max(min, n)) : min
}

const inputs = computed(() => ({
  p: clamp(initial.value, 0, MAX_AMOUNT),
  c: clamp(monthly.value, 0, MAX_AMOUNT),
  r: clamp(rate.value, 0, MAX_RATE) / 100,
  y: Math.round(clamp(years.value, 1, MAX_YEARS)),
  n: compounding.value,
  t: clamp(goal.value, 0, MAX_AMOUNT),
}))

const errors = computed(() => {
  const e: string[] = []
  if (!(Number(years.value) >= 1 && Number(years.value) <= MAX_YEARS)) e.push(`Years must be between 1 and ${MAX_YEARS}.`)
  if (!(Number(rate.value) >= 0 && Number(rate.value) <= MAX_RATE)) e.push(`Rate must be between 0 and ${MAX_RATE}%.`)
  if ([initial.value, monthly.value, goal.value].some((v) => Number(v) < 0)) e.push('Amounts cannot be negative.')
  return e
})

const monthlyRate = computed(() => (1 + inputs.value.r / inputs.value.n) ** (inputs.value.n / 12) - 1)

const requiredMonthly = computed(() => {
  const { p, t, y } = inputs.value
  const i = monthlyRate.value
  const N = y * 12
  const g = (1 + i) ** N
  const need = i ? (t - p * g) * i / (g - 1) : (t - p) / N
  return Math.max(0, need)
})

const contribution = computed(() => (mode.value === 'goal' ? requiredMonthly.value : inputs.value.c))

const schedule = computed(() => {
  const { p, y } = inputs.value
  const i = monthlyRate.value
  const c = contribution.value
  let bal = p
  const rows: { year: number, balance: number, contributed: number, interest: number }[] = []
  for (let m = 1; m <= y * 12; m++) {
    bal = bal * (1 + i) + c
    if (m % 12 === 0) {
      const contributed = p + c * m
      rows.push({ year: m / 12, balance: bal, contributed, interest: bal - contributed })
    }
  }
  return rows
})

const final = computed(() => schedule.value.at(-1) ?? { year: 0, balance: inputs.value.p, contributed: inputs.value.p, interest: 0 })
const maxBalance = computed(() => Math.max(1, ...schedule.value.map((r) => r.balance)))

const milestones = computed(() => {
  const marks = new Set([1, 5, 10, 20, 30, inputs.value.y].filter((y) => y <= inputs.value.y))
  return schedule.value.filter((r) => marks.has(r.year))
})

const locale = computed(() => (currency.value === 'INR' ? 'en-IN' : undefined))
const money = computed(() => new Intl.NumberFormat(locale.value, { style: 'currency', currency: currency.value, maximumFractionDigits: 0 }))
const moneyExact = computed(() => new Intl.NumberFormat(locale.value, { style: 'currency', currency: currency.value, maximumFractionDigits: 2 }))
const fmt = (n: number) => money.value.format(n)

const interestShare = computed(() => final.value.balance > 0 ? Math.round((final.value.interest / final.value.balance) * 100) : 0)
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
        </div>
      </Teleport>
    </ClientOnly>

    <main class="flex-1 min-w-0 flex flex-col p-4 md:p-6">
      <div class="mx-auto w-full max-w-6xl flex flex-col gap-4">
        <div class="ui-panel px-4 flex items-center justify-between gap-3">
          <div id="page-actions" class="order-first flex flex-wrap items-center gap-1"></div>
          <div class="flex items-center gap-2">
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M3 16h14M5 13v3M9 10v6M13 7v9M17 4v12" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">Savings Calculator</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-[340px_minmax(0,1fr)] gap-4 items-start">
          <section class="ui-panel p-4 flex flex-col gap-4">
            <div class="grid grid-cols-2 gap-1 rounded-[var(--radius-md)] bg-[var(--surface-2)] p-1" role="tablist" aria-label="Calculator mode">
              <button
                v-for="m in [{ id: 'growth', label: 'Growth' }, { id: 'goal', label: 'Savings goal' }]"
                :key="m.id"
                type="button"
                role="tab"
                :aria-selected="mode === m.id"
                class="h-9 rounded-[var(--radius-sm)] text-sm font-semibold transition-colors duration-150"
                :class="mode === m.id ? 'bg-[var(--surface-1)] text-[var(--ink-1)] shadow-[var(--shadow-sm)]' : 'text-[var(--ink-3)] hover:text-[var(--ink-1)]'"
                @click="mode = m.id as Mode"
              >
                {{ m.label }}
              </button>
            </div>

            <label v-if="mode === 'goal'" class="flex flex-col gap-1.5">
              <span class="text-xs font-semibold text-[var(--ink-2)]">Savings goal</span>
              <input v-model.number="goal" type="number" min="0" step="any" inputmode="decimal" class="ui-input font-mono">
            </label>
            <label class="flex flex-col gap-1.5">
              <span class="text-xs font-semibold text-[var(--ink-2)]">Starting amount</span>
              <input v-model.number="initial" type="number" min="0" step="any" inputmode="decimal" class="ui-input font-mono">
            </label>
            <label v-if="mode === 'growth'" class="flex flex-col gap-1.5">
              <span class="text-xs font-semibold text-[var(--ink-2)]">Monthly deposit</span>
              <input v-model.number="monthly" type="number" min="0" step="any" inputmode="decimal" class="ui-input font-mono">
            </label>
            <div class="grid grid-cols-2 gap-3">
              <label class="flex flex-col gap-1.5">
                <span class="text-xs font-semibold text-[var(--ink-2)]">Yearly rate (%)</span>
                <input v-model.number="rate" type="number" min="0" :max="MAX_RATE" step="0.1" inputmode="decimal" class="ui-input font-mono">
              </label>
              <label class="flex flex-col gap-1.5">
                <span class="text-xs font-semibold text-[var(--ink-2)]">Years</span>
                <input v-model.number="years" type="number" min="1" :max="MAX_YEARS" step="1" inputmode="numeric" class="ui-input font-mono">
              </label>
            </div>
            <div class="grid grid-cols-2 gap-3">
              <label class="flex flex-col gap-1.5">
                <span class="text-xs font-semibold text-[var(--ink-2)]">Compounding</span>
                <select v-model.number="compounding" class="ui-input">
                  <option v-for="c in COMPOUNDING" :key="c.n" :value="c.n">{{ c.label }}</option>
                </select>
              </label>
              <label class="flex flex-col gap-1.5">
                <span class="text-xs font-semibold text-[var(--ink-2)]">Currency</span>
                <select v-model="currency" class="ui-input">
                  <option v-for="c in CURRENCIES" :key="c" :value="c">{{ c }}</option>
                </select>
              </label>
            </div>
            <ul v-if="errors.length" class="text-xs text-[var(--danger)] flex flex-col gap-1" role="alert">
              <li v-for="e in errors" :key="e">{{ e }}</li>
            </ul>
          </section>

          <div class="flex flex-col gap-4 min-w-0">
            <section class="ui-panel p-5">
              <template v-if="mode === 'goal'">
                <span class="text-xs font-semibold text-[var(--ink-3)]">Save each month</span>
                <p class="mt-1 text-4xl font-extrabold tracking-tight text-[var(--c-life)]">{{ moneyExact.format(requiredMonthly) }}</p>
                <p class="mt-1 text-sm text-[var(--ink-2)]">
                  <template v-if="requiredMonthly > 0">to reach {{ fmt(inputs.t) }} in {{ inputs.y }} {{ inputs.y === 1 ? 'year' : 'years' }}.</template>
                  <template v-else>Your starting amount reaches {{ fmt(inputs.t) }} on its own in {{ inputs.y }} years.</template>
                </p>
              </template>
              <template v-else>
                <span class="text-xs font-semibold text-[var(--ink-3)]">Balance after {{ inputs.y }} {{ inputs.y === 1 ? 'year' : 'years' }}</span>
                <p class="mt-1 text-4xl font-extrabold tracking-tight text-[var(--c-life)]">{{ fmt(final.balance) }}</p>
              </template>
              <dl class="mt-5 grid grid-cols-3 gap-4 border-t border-[var(--border)] pt-4">
                <div>
                  <dt class="text-xs text-[var(--ink-3)]">You put in</dt>
                  <dd class="text-lg font-bold">{{ fmt(final.contributed) }}</dd>
                </div>
                <div>
                  <dt class="text-xs text-[var(--ink-3)]">Interest earned</dt>
                  <dd class="text-lg font-bold">{{ fmt(final.interest) }}</dd>
                </div>
                <div>
                  <dt class="text-xs text-[var(--ink-3)]">From interest</dt>
                  <dd class="text-lg font-bold">{{ interestShare }}%</dd>
                </div>
              </dl>
            </section>

            <section class="ui-panel p-5">
              <div class="flex flex-wrap items-center justify-between gap-3">
                <h2 class="text-sm font-bold">Growth by year</h2>
                <div class="flex items-center gap-4 text-xs text-[var(--ink-2)]">
                  <span class="flex items-center gap-1.5"><span class="h-2.5 w-2.5 rounded-sm bg-[color-mix(in_oklab,var(--c-life)_35%,var(--surface-2))]" aria-hidden="true" />Deposits</span>
                  <span class="flex items-center gap-1.5"><span class="h-2.5 w-2.5 rounded-sm bg-[var(--c-life)]" aria-hidden="true" />Interest</span>
                </div>
              </div>
              <div class="mt-4 flex h-52 items-end gap-[2px]" role="img" :aria-label="`Balance grows to ${fmt(final.balance)} over ${inputs.y} years`">
                <div
                  v-for="r in schedule"
                  :key="r.year"
                  class="flex-1 min-w-[2px] flex flex-col justify-end h-full"
                  :title="`Year ${r.year}: ${fmt(r.balance)}`"
                >
                  <div class="rounded-t-[3px] bg-[var(--c-life)]" :style="{ height: `${(Math.max(0, r.interest) / maxBalance) * 100}%` }" />
                  <div class="bg-[color-mix(in_oklab,var(--c-life)_35%,var(--surface-2))]" :style="{ height: `${(Math.min(r.balance, r.contributed) / maxBalance) * 100}%` }" />
                </div>
              </div>
              <div class="mt-1 flex justify-between text-[11px] font-mono text-[var(--ink-3)]">
                <span>Year 1</span>
                <span>Year {{ inputs.y }}</span>
              </div>

              <div class="mt-5 grid grid-cols-2 sm:grid-cols-3 gap-2">
                <div v-for="m in milestones" :key="m.year" class="rounded-[var(--radius-md)] bg-[var(--surface-2)] p-3">
                  <span class="block text-xs text-[var(--ink-3)]">Year {{ m.year }}</span>
                  <span class="block text-base font-bold">{{ fmt(m.balance) }}</span>
                </div>
              </div>
            </section>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>
