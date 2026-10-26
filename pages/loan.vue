<script setup lang="ts">
useHead({
  title: 'Free Loan Repayment Calculator | Monthly Payment & Amortization',
  meta: [
    { name: 'description', content: 'Find your monthly loan payment, total interest, and payoff date. See how extra payments save money. Free, private, and offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/loan/' },
    { property: 'og:title', content: 'Free Loan Repayment Calculator' },
    { property: 'og:description', content: 'Monthly payment, total interest, and payoff date in your browser.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/loan/' }],
})

useToolSchema({
  name: 'Loan Repayment Calculator',
  description: 'Calculate monthly loan payments, total interest, payoff date, and savings from extra payments.',
  url: 'https://ash-tools.store/loan/',
  category: 'FinanceApplication',
})

const CURRENCIES = ['INR', 'USD', 'EUR', 'GBP', 'JPY', 'CAD', 'AUD']
const DEFAULT_CURRENCY = 'INR'
const MAX_MONTHS = 600
const MAX_RATE = 100
const MAX_AMOUNT = 1e12

const amount = ref(2500000)
const rate = ref(8.5)
const termYears = ref(20)
const termMonths = ref(0)
const extra = ref(0)
const start = ref('')
const currency = ref(DEFAULT_CURRENCY)

const num = (v: unknown) => (Number.isFinite(Number(v)) ? Number(v) : 0)

const months = computed(() => Math.round(num(termYears.value) * 12 + num(termMonths.value)))
const principal = computed(() => Math.min(MAX_AMOUNT, Math.max(0, num(amount.value))))
const monthlyRate = computed(() => Math.min(MAX_RATE, Math.max(0, num(rate.value))) / 100 / 12)
const extraPay = computed(() => Math.max(0, num(extra.value)))

const errors = computed(() => {
  const e: string[] = []
  if (!(principal.value > 0)) e.push('Enter a loan amount above zero.')
  if (!(months.value >= 1 && months.value <= MAX_MONTHS)) e.push(`Term must be between 1 month and ${MAX_MONTHS / 12} years.`)
  if (num(rate.value) < 0 || num(rate.value) > MAX_RATE) e.push(`Rate must be between 0 and ${MAX_RATE}%.`)
  if (num(extra.value) < 0) e.push('Extra payment cannot be negative.')
  return e
})

const valid = computed(() => !errors.value.length)

const payment = computed(() => {
  if (!valid.value) return 0
  const i = monthlyRate.value
  const n = months.value
  return i ? principal.value * i / (1 - (1 + i) ** -n) : principal.value / n
})

type Year = { year: number, principal: number, interest: number, balance: number }

const simulate = (extraAmount: number) => {
  const i = monthlyRate.value
  let bal = principal.value
  let interestTotal = 0
  let count = 0
  const years: Year[] = []
  let yp = 0
  let yi = 0
  while (bal > 0.005 && count < MAX_MONTHS) {
    const interest = bal * i
    const pay = Math.min(bal + interest, payment.value + extraAmount)
    const princ = pay - interest
    bal -= princ
    interestTotal += interest
    yp += princ
    yi += interest
    count++
    if (count % 12 === 0 || bal <= 0.005) {
      years.push({ year: Math.ceil(count / 12), principal: yp, interest: yi, balance: Math.max(0, bal) })
      yp = 0
      yi = 0
    }
  }
  return { months: count, interest: interestTotal, years }
}

const plan = computed(() => (valid.value ? simulate(extraPay.value) : { months: 0, interest: 0, years: [] as Year[] }))
const base = computed(() => (valid.value ? simulate(0) : { months: 0, interest: 0, years: [] as Year[] }))

const totalPaid = computed(() => principal.value + plan.value.interest)
const interestShare = computed(() => (totalPaid.value > 0 ? (plan.value.interest / totalPaid.value) * 100 : 0))
const savedInterest = computed(() => Math.max(0, base.value.interest - plan.value.interest))
const savedMonths = computed(() => Math.max(0, base.value.months - plan.value.months))
const maxYearPay = computed(() => Math.max(1, ...plan.value.years.map((y) => y.principal + y.interest)))

const startDate = computed(() => {
  const [y, m] = (start.value || '').split('-').map(Number)
  return y && m ? new Date(y, m - 1, 1) : new Date()
})

const payoff = computed(() => {
  if (!plan.value.months) return ''
  const d = new Date(startDate.value)
  d.setMonth(d.getMonth() + plan.value.months - 1)
  return new Intl.DateTimeFormat(undefined, { month: 'long', year: 'numeric' }).format(d)
})

const duration = (m: number) => {
  const y = Math.floor(m / 12)
  const r = m % 12
  return [y && `${y} ${y === 1 ? 'year' : 'years'}`, r && `${r} ${r === 1 ? 'month' : 'months'}`].filter(Boolean).join(' ') || '0 months'
}

const locale = computed(() => (currency.value === 'INR' ? 'en-IN' : undefined))
const money = computed(() => new Intl.NumberFormat(locale.value, { style: 'currency', currency: currency.value, maximumFractionDigits: 0 }))
const moneyExact = computed(() => new Intl.NumberFormat(locale.value, { style: 'currency', currency: currency.value, maximumFractionDigits: 2 }))
const fmt = (n: number) => money.value.format(n)

onMounted(() => {
  const d = new Date()
  start.value = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}`
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
        </div>
      </Teleport>
    </ClientOnly>

    <main class="flex-1 min-w-0 flex flex-col p-4 md:p-6">
      <div class="mx-auto w-full max-w-6xl flex flex-col gap-4">
        <div class="ui-panel px-4 flex items-center justify-between gap-3">
          <div id="page-actions" class="order-first flex flex-wrap items-center gap-1"></div>
          <div class="flex items-center gap-2">
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="2.5" y="5" width="15" height="10" rx="1.5" /><path stroke-linecap="round" d="M2.5 8.5h15M5.5 12h4" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">Loan Calculator</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-[340px_minmax(0,1fr)] gap-4 items-start">
          <section class="ui-panel p-4 flex flex-col gap-4">
            <label class="flex flex-col gap-1.5">
              <span class="text-xs font-semibold text-[var(--ink-2)]">Loan amount</span>
              <input v-model.number="amount" type="number" min="0" step="any" inputmode="decimal" class="ui-input font-mono">
            </label>
            <label class="flex flex-col gap-1.5">
              <span class="text-xs font-semibold text-[var(--ink-2)]">Yearly interest rate (%)</span>
              <input v-model.number="rate" type="number" min="0" :max="MAX_RATE" step="0.05" inputmode="decimal" class="ui-input font-mono">
            </label>
            <div class="grid grid-cols-2 gap-3">
              <label class="flex flex-col gap-1.5">
                <span class="text-xs font-semibold text-[var(--ink-2)]">Term (years)</span>
                <input v-model.number="termYears" type="number" min="0" max="50" step="1" inputmode="numeric" class="ui-input font-mono">
              </label>
              <label class="flex flex-col gap-1.5">
                <span class="text-xs font-semibold text-[var(--ink-2)]">+ months</span>
                <input v-model.number="termMonths" type="number" min="0" max="11" step="1" inputmode="numeric" class="ui-input font-mono">
              </label>
            </div>
            <label class="flex flex-col gap-1.5">
              <span class="text-xs font-semibold text-[var(--ink-2)]">Extra payment each month</span>
              <input v-model.number="extra" type="number" min="0" step="any" inputmode="decimal" class="ui-input font-mono">
              <span class="text-xs text-[var(--ink-3)]">Optional. Goes straight to the principal.</span>
            </label>
            <div class="grid grid-cols-2 gap-3">
              <label class="flex flex-col gap-1.5">
                <span class="text-xs font-semibold text-[var(--ink-2)]">First payment</span>
                <input v-model="start" type="month" class="ui-input font-mono">
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
              <div v-if="!valid" class="py-10 text-center text-sm text-[var(--ink-3)]">Fix the fields on the left to see your payment.</div>
              <template v-else>
                <div class="flex flex-wrap items-center gap-6">
                  <div class="flex-1 min-w-[14rem]">
                    <span class="text-xs font-semibold text-[var(--ink-3)]">Monthly payment</span>
                    <p class="mt-1 text-4xl font-extrabold tracking-tight text-[var(--c-life)]">{{ moneyExact.format(payment + extraPay) }}</p>
                    <p v-if="extraPay" class="mt-1 text-sm text-[var(--ink-2)]">{{ moneyExact.format(payment) }} required + {{ moneyExact.format(extraPay) }} extra</p>
                    <p class="mt-1 text-sm text-[var(--ink-2)]">Paid off in <strong class="text-[var(--ink-1)]">{{ payoff }}</strong> ({{ duration(plan.months) }})</p>
                  </div>
                  <div
                    class="relative h-28 w-28 shrink-0 rounded-full"
                    :style="{ background: `conic-gradient(var(--c-media) 0 ${interestShare}%, color-mix(in oklab, var(--c-life) 70%, var(--surface-2)) ${interestShare}% 100%)` }"
                    role="img"
                    :aria-label="`Interest is ${Math.round(interestShare)}% of the total you pay`"
                  >
                    <div class="absolute inset-3 flex flex-col items-center justify-center rounded-full bg-[var(--surface-1)] text-center">
                      <span class="text-lg font-extrabold leading-none">{{ Math.round(interestShare) }}%</span>
                      <span class="text-[10px] text-[var(--ink-3)]">interest</span>
                    </div>
                  </div>
                </div>
                <dl class="mt-5 grid grid-cols-2 sm:grid-cols-3 gap-4 border-t border-[var(--border)] pt-4">
                  <div>
                    <dt class="flex items-center gap-1.5 text-xs text-[var(--ink-3)]"><span class="h-2.5 w-2.5 rounded-sm bg-[color-mix(in_oklab,var(--c-life)_70%,var(--surface-2))]" aria-hidden="true" />Principal</dt>
                    <dd class="text-lg font-bold">{{ fmt(principal) }}</dd>
                  </div>
                  <div>
                    <dt class="flex items-center gap-1.5 text-xs text-[var(--ink-3)]"><span class="h-2.5 w-2.5 rounded-sm bg-[var(--c-media)]" aria-hidden="true" />Total interest</dt>
                    <dd class="text-lg font-bold">{{ fmt(plan.interest) }}</dd>
                  </div>
                  <div>
                    <dt class="text-xs text-[var(--ink-3)]">Total paid</dt>
                    <dd class="text-lg font-bold">{{ fmt(totalPaid) }}</dd>
                  </div>
                </dl>
                <p v-if="extraPay && savedInterest > 0" class="mt-4 rounded-[var(--radius-md)] bg-[color-mix(in_oklab,var(--c-life)_12%,var(--surface-1))] px-4 py-3 text-sm">
                  The extra payment saves <strong>{{ fmt(savedInterest) }}</strong> in interest and ends the loan <strong>{{ duration(savedMonths) }}</strong> sooner.
                </p>
              </template>
            </section>

            <section v-if="valid" class="ui-panel p-5">
              <h2 class="text-sm font-bold">What you pay each year</h2>
              <div class="mt-4 flex h-44 items-end gap-[2px]" role="img" aria-label="Yearly principal and interest payments">
                <div
                  v-for="y in plan.years"
                  :key="y.year"
                  class="flex-1 min-w-[2px] flex flex-col justify-end h-full"
                  :title="`Year ${y.year}: ${fmt(y.principal)} principal, ${fmt(y.interest)} interest`"
                >
                  <div class="rounded-t-[3px] bg-[var(--c-media)]" :style="{ height: `${(y.interest / maxYearPay) * 100}%` }" />
                  <div class="bg-[color-mix(in_oklab,var(--c-life)_70%,var(--surface-2))]" :style="{ height: `${(y.principal / maxYearPay) * 100}%` }" />
                </div>
              </div>
              <div class="mt-1 flex justify-between text-[11px] font-mono text-[var(--ink-3)]">
                <span>Year 1</span>
                <span>Year {{ plan.years.length }}</span>
              </div>

              <details class="mt-5 group">
                <summary class="cursor-pointer select-none text-sm font-semibold text-[var(--accent)]">Show yearly schedule</summary>
                <div class="mt-3 max-h-80 overflow-auto rounded-[var(--radius-md)] border border-[var(--border)]">
                  <table class="w-full text-sm">
                    <thead class="sticky top-0 bg-[var(--surface-2)] text-xs text-[var(--ink-3)]">
                      <tr>
                        <th class="px-3 py-2 text-left font-semibold">Year</th>
                        <th class="px-3 py-2 text-right font-semibold">Principal</th>
                        <th class="px-3 py-2 text-right font-semibold">Interest</th>
                        <th class="px-3 py-2 text-right font-semibold">Balance left</th>
                      </tr>
                    </thead>
                    <tbody class="font-mono">
                      <tr v-for="y in plan.years" :key="y.year" class="odd:bg-[var(--surface-1)] even:bg-[color-mix(in_oklab,var(--surface-2)_50%,var(--surface-1))]">
                        <td class="px-3 py-1.5">{{ y.year }}</td>
                        <td class="px-3 py-1.5 text-right">{{ fmt(y.principal) }}</td>
                        <td class="px-3 py-1.5 text-right">{{ fmt(y.interest) }}</td>
                        <td class="px-3 py-1.5 text-right">{{ fmt(y.balance) }}</td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </details>
            </section>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>
