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
  <header class="sticky top-0 z-50 border-4 border-black bg-[#FFFDF5] shadow-[8px_8px_0px_0px_#000]">
    <div class="min-h-16 flex items-center justify-between px-3 py-2 md:px-6">
      <div class="flex items-center gap-6">
        <NuxtLink
          to="/"
          class="inline-flex items-center gap-2 border-4 border-black bg-[#FFD93D] px-3 py-1 font-black uppercase tracking-wide shadow-[4px_4px_0px_0px_#000] transition-transform duration-100 ease-linear hover:-translate-y-0.5"
        >
          <span class="inline-block h-3 w-3 border-2 border-black rounded-full bg-[#FF6B6B]"></span>
          <span class="hidden sm:inline">Ash Tools</span>
        </NuxtLink>

        <nav class="hidden md:flex items-center gap-2">
          <NuxtLink
            v-for="link in normalTools"
            :key="link.href"
            :to="link.href"
            class="nav-link border-4 border-transparent px-3 py-1.5 text-xs font-bold uppercase tracking-wide transition-all duration-100 ease-linear hover:border-black hover:bg-[#FF6B6B] hover:shadow-[4px_4px_0px_0px_#000]"
          >
            {{ link.name }}
          </NuxtLink>
        </nav>
      </div>

      <div class="flex items-center gap-4">
        <nav class="hidden md:flex items-center gap-2">
          <NuxtLink
            v-for="link in devTools"
            :key="link.href"
            :to="link.href"
            class="nav-link border-4 border-transparent px-3 py-1.5 text-xs font-bold uppercase tracking-wide transition-all duration-100 ease-linear hover:border-black hover:bg-[#C4B5FD] hover:shadow-[4px_4px_0px_0px_#000]"
          >
            {{ link.name }}
          </NuxtLink>
        </nav>

        <button
          @click="isMenuOpen = !isMenuOpen"
          class="md:hidden h-12 w-12 border-4 border-black bg-white p-2 shadow-[4px_4px_0px_0px_#000] transition-all duration-100 ease-linear active:translate-x-0.5 active:translate-y-0.5 active:shadow-none"
          aria-label="Toggle menu"
        >
          <div class="w-6 h-5 flex flex-col justify-between">
            <span :class="['h-1 bg-black transition-all duration-100 ease-linear', isMenuOpen && 'rotate-45 translate-y-2']"></span>
            <span :class="['h-1 bg-black transition-all duration-100 ease-linear', isMenuOpen && 'opacity-0']"></span>
            <span :class="['h-1 bg-black transition-all duration-100 ease-linear', isMenuOpen && '-rotate-45 -translate-y-2']"></span>
          </div>
        </button>
      </div>
    </div>

    <Transition
      enter-active-class="transition-all duration-200 ease-out"
      leave-active-class="transition-all duration-200 ease-out"
      enter-from-class="opacity-0 -translate-y-2"
      leave-to-class="opacity-0 -translate-y-2"
    >
      <nav
        v-if="isMenuOpen"
        class="md:hidden border-t-4 border-black bg-[#FFFDF5] px-3 py-3 overflow-y-auto max-h-[calc(100vh-4rem)]"
      >
        <div class="mb-2 inline-block border-4 border-black bg-[#FFD93D] px-3 py-1 text-xs font-black uppercase tracking-[0.2em] shadow-[4px_4px_0px_0px_#000]">Tools</div>
        <NuxtLink
          v-for="link in normalTools"
          :key="link.href"
          :to="link.href"
          class="nav-link mb-2 block border-4 border-black bg-white px-4 py-3 text-sm font-bold uppercase tracking-wide shadow-[4px_4px_0px_0px_#000] transition-all duration-100 ease-linear active:translate-x-0.5 active:translate-y-0.5 active:shadow-none"
          @click="isMenuOpen = false"
        >
          {{ link.name }}
        </NuxtLink>
        
        <div class="mb-2 mt-4 inline-block border-4 border-black bg-[#C4B5FD] px-3 py-1 text-xs font-black uppercase tracking-[0.2em] shadow-[4px_4px_0px_0px_#000]">Dev Tools</div>
        <NuxtLink
          v-for="link in devTools"
          :key="link.href"
          :to="link.href"
          class="nav-link mb-2 block border-4 border-black bg-white px-4 py-3 text-sm font-bold uppercase tracking-wide shadow-[4px_4px_0px_0px_#000] transition-all duration-100 ease-linear active:translate-x-0.5 active:translate-y-0.5 active:shadow-none"
          @click="isMenuOpen = false"
        >
          {{ link.name }}
        </NuxtLink>
      </nav>
    </Transition>
  </header>
</template>
