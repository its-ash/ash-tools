<script setup lang="ts">
const normalTools = [
  { name: 'Video', href: '/video/' },
  { name: 'Image', href: '/image/' },
  { name: 'ZIP', href: '/zip/' },
  { name: 'PDF', href: '/pdf/' },
  { name: 'WebLLM', href: '/webllm/' },
  // { name: 'Resume', href: '/resume/' },
]

const devTools = [
  { name: 'Speed', href: '/speed/' },
  { name: 'Regex', href: '/regexp/' },
  { name: 'Sandbox', href: '/sandbox/' },
]

const isMenuOpen = ref(false)
</script>

<template>
  <header class="sticky top-0 z-50 h-12 bg-slate-950/80 border-b border-slate-700 backdrop-blur-md">
    <div class="h-full flex items-center justify-between px-4 md:px-6">
      <div class="flex items-center gap-6">
        <!-- Logo -->
        <NuxtLink to="/" class="flex items-center gap-2 font-bold text-lg text-white hover:text-slate-200 transition-colors">
          <span class="inline-block w-2 h-2 bg-gradient-to-r from-blue-400 to-cyan-400 rounded-full"></span>
          <span class="hidden sm:inline">Ash Tools</span>
        </NuxtLink>

        <!-- Normal Tools Navigation -->
        <nav class="hidden md:flex items-center gap-1">
          <NuxtLink
            v-for="link in normalTools"
            :key="link.href"
            :to="link.href"
            class="nav-link px-3 py-1.5 text-xs font-medium text-slate-300 hover:text-white hover:bg-slate-800/50 rounded-lg transition-all"
          >
            {{ link.name }}
          </NuxtLink>
        </nav>
      </div>

      <div class="flex items-center gap-4">
        <!-- Dev Tools Navigation -->
        <nav class="hidden md:flex items-center gap-1">
          <NuxtLink
            v-for="link in devTools"
            :key="link.href"
            :to="link.href"
            class="nav-link px-3 py-1.5 text-xs font-medium text-slate-400 hover:text-white hover:bg-slate-800/50 rounded-lg transition-all"
          >
            {{ link.name }}
          </NuxtLink>
        </nav>

        <!-- Mobile Menu Button -->
        <button
          @click="isMenuOpen = !isMenuOpen"
          class="md:hidden p-2 hover:bg-slate-800 rounded-lg transition-colors"
          aria-label="Toggle menu"
        >
          <div class="w-6 h-5 flex flex-col justify-between">
            <span :class="['h-0.5 bg-slate-300 transition-all', isMenuOpen && 'rotate-45 translate-y-2']"></span>
            <span :class="['h-0.5 bg-slate-300 transition-all', isMenuOpen && 'opacity-0']"></span>
            <span :class="['h-0.5 bg-slate-300 transition-all', isMenuOpen && '-rotate-45 -translate-y-2']"></span>
          </div>
        </button>
      </div>
    </div>

    <!-- Mobile Navigation -->
    <Transition
      enter-active-class="transition-all duration-200"
      leave-active-class="transition-all duration-200"
      enter-from-class="opacity-0 -translate-y-2"
      leave-to-class="opacity-0 -translate-y-2"
    >
      <nav
        v-if="isMenuOpen"
        class="md:hidden absolute top-12 left-0 right-0 bg-slate-900/95 border-b border-slate-700 backdrop-blur-md py-2 overflow-y-auto max-h-[calc(100vh-3rem)]"
      >
        <div class="px-4 py-2 text-xs font-semibold text-slate-500 uppercase tracking-wider">Tools</div>
        <NuxtLink
          v-for="link in normalTools"
          :key="link.href"
          :to="link.href"
          class="nav-link block px-6 py-2 text-sm text-slate-300 hover:text-white hover:bg-slate-800 transition-colors"
          @click="isMenuOpen = false"
        >
          {{ link.name }}
        </NuxtLink>
        
        <div class="px-4 py-2 mt-2 border-t border-slate-800 text-xs font-semibold text-slate-500 uppercase tracking-wider">Dev Tools</div>
        <NuxtLink
          v-for="link in devTools"
          :key="link.href"
          :to="link.href"
          class="nav-link block px-6 py-2 text-sm text-slate-300 hover:text-white hover:bg-slate-800 transition-colors"
          @click="isMenuOpen = false"
        >
          {{ link.name }}
        </NuxtLink>
      </nav>
    </Transition>
  </header>
</template>
