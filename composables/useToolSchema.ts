// Injects WebApplication JSON-LD structured data for a tool page, so search
// engines can understand it's a free, browser-based utility (helps rich
// results and knowledge panels). Call once per tool page's <script setup>.
export const useToolSchema = (options: {
  name: string
  description: string
  url: string
  category?: string
}) => {
  const schema = {
    '@context': 'https://schema.org',
    '@type': 'WebApplication',
    name: options.name,
    description: options.description,
    url: options.url,
    applicationCategory: options.category ?? 'UtilitiesApplication',
    operatingSystem: 'Any (runs in browser)',
    offers: {
      '@type': 'Offer',
      price: '0',
      priceCurrency: 'USD',
    },
    isAccessibleForFree: true,
    browserRequirements: 'Requires a modern browser with JavaScript enabled.',
  }

  useHead({
    script: [
      {
        type: 'application/ld+json',
        innerHTML: JSON.stringify(schema),
      },
    ],
  })
}
