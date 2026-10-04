import tailwindcss from '@tailwindcss/vite'

export default defineNuxtConfig({
  ssr: true,

  devtools: { enabled: false },

  experimental: {
    appManifest: false,
  },

  app: {
    head: {
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1, maximum-scale=5',
      htmlAttrs: {
        lang: 'en'
      },
      meta: [
        // Basic SEO
        {
          name: 'description',
          content: 'Ash Tools is a free, privacy-first toolkit of 20+ browser utilities for media editing, PDF and ZIP workflows, and developer tasks. Everything runs locally on your device—no uploads, no tracking, no servers.'
        },
        {
          name: 'keywords',
          content: 'video compressor, image editor, pdf merger, zip compressor, code sandbox, json formatter, regex tester, qr code generator, uuid generator, local processing, offline tools, privacy tools, browser tools'
        },
        {
          name: 'author',
          content: 'Ashvini Jangid'
        },
        {
          name: 'creator',
          content: 'Ashvini Jangid'
        },
        {
          name: 'theme-color',
          content: '#faf8f5'
        },
        {
          name: 'color-scheme',
          content: 'light'
        },
        // Robots
        {
          name: 'robots',
          content: 'index, follow, max-snippet:-1, max-image-preview:large, max-video-preview:-1'
        },
        // Apple
        {
          name: 'apple-mobile-web-app-capable',
          content: 'yes'
        },
        {
          name: 'apple-mobile-web-app-status-bar-style',
          content: 'black-translucent'
        },
        {
          name: 'apple-mobile-web-app-title',
          content: 'Ash Tools'
        },
        // Microsoft
        {
          name: 'msapplication-TileColor',
          content: '#faf8f5'
        },
        // Open Graph
        {
          property: 'og:type',
          content: 'website'
        },
        {
          property: 'og:title',
          content: 'Ash Tools | Private Browser Tools for Media, Docs, and Dev'
        },
        {
          property: 'og:description',
          content: 'Ash Tools is a free, privacy-first toolkit of 20+ browser utilities for media editing, PDF and ZIP workflows, and developer tasks. No uploads, no tracking, no servers.'
        },
        {
          property: 'og:url',
          content: 'https://ash-tools.store/'
        },
        {
          property: 'og:image',
          content: 'https://ash-tools.store/og-image.png'
        },
        {
          property: 'og:image:width',
          content: '1200'
        },
        {
          property: 'og:image:height',
          content: '630'
        },
        {
          property: 'og:locale',
          content: 'en_US'
        },
        // Twitter Card
        {
          name: 'twitter:card',
          content: 'summary_large_image'
        },
        {
          name: 'twitter:title',
          content: 'Ash Tools | Private Browser Tools for Media, Docs, and Dev'
        },
        {
          name: 'twitter:description',
          content: 'Ash Tools is a free, privacy-first toolkit of 20+ browser utilities for media editing, PDF and ZIP workflows, and developer tasks. No uploads, no tracking, no servers.'
        },
        {
          name: 'twitter:image',
          content: 'https://ash-tools.store/twitter-image.png'
        },
        {
          name: 'twitter:creator',
          content: '@ashvinijangid'
        },
        // Additional SEO
        {
          name: 'revisit-after',
          content: '7 days'
        },
        {
          name: 'language',
          content: 'English'
        },
        {
          'http-equiv': 'X-UA-Compatible',
          content: 'IE=edge'
        }
      ],
      link: [
        {
          rel: 'icon',
          href: '/favicon.png',
          type: 'image/png',
        },
        {
          rel: 'apple-touch-icon',
          href: '/apple-touch-icon.png'
        },
        {
          rel: 'manifest',
          href: '/manifest.json'
        }
      ],
      script: [
        // Google Tag Manager (gtag.js)
        {
          src: 'https://www.googletagmanager.com/gtag/js?id=G-GVC5N24NVF',
          async: true,
        },
        {
          innerHTML: `
            window.dataLayer = window.dataLayer || [];
            function gtag(){dataLayer.push(arguments);}
            gtag('js', new Date());
            gtag('config', 'G-GVC5N24NVF');
          `,
        },
        // Google AdSense
        {
          src: 'https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=ca-pub-9899193996105785',
          async: true,
          crossorigin: 'anonymous',
        }
      ]
    },
    baseURL: '/',
    buildAssetsDir: 'assets',
  },

  routeRules: {
    '/**': { prerender: true }
  },

  nitro: {
    preset: 'static',
    prerender: {
      crawlLinks: true,
      routes: [
        '/', '/image', '/zip', '/video', '/sandbox', '/pdf', '/pdf-sign',
        '/json', '/csv', '/regex', '/encode', '/color', '/qr',
        '/markdown', '/uuid', '/diff', '/units', '/timestamp',
        '/text', '/cron', '/base64-image', '/linkedin',
        '/sitemap.xml', '/robots.txt',
      ]
    },
    output: {
      publicDir: './docs'
    },
    storage: {
      db: { driver: 'memory' }
    }
  },

  css: ['~/files/css/main.css'],

  vite: {
    plugins: [tailwindcss()],
    assetsInclude: ['**/*.wasm'],
    base: '/',
  },

  modules: [
    '@vite-pwa/nuxt'
  ],

  pwa: {
    manifest: {
      name: 'Ash Tools',
      short_name: 'Ash Tools',
      description: 'Free, privacy-first browser tools for media, docs, and developer workflows. Runs locally, no uploads.',
      theme_color: '#faf8f5',
      background_color: '#faf8f5',
      display: 'standalone',
      orientation: 'portrait',
      icons: [
        {
          src: 'pwa-192x192.png',
          sizes: '192x192',
          type: 'image/png'
        },
        {
          src: 'pwa-512x512.png',
          sizes: '512x512',
          type: 'image/png'
        },
        {
          src: 'pwa-512x512.png',
          sizes: '512x512',
          type: 'image/png',
          purpose: 'any maskable'
        }
      ]
    },
    workbox: {
      navigateFallback: null,
      globPatterns: ['**/*.{js,css,html,png,svg,ico,wasm}'],
      maximumFileSizeToCacheInBytes: 64 * 1024 * 1024 // 64 MiB
    },
    client: {
      installPrompt: true,
      periodicSyncForUpdates: 3600
    },
    devOptions: {
      enabled: false,
      type: 'module'
    }
  },

  compatibilityDate: '2024-11-01',
})
