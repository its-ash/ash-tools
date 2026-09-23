<template>
  <div class="min-h-screen bg-[var(--surface-0)] text-[var(--ink-1)] flex flex-col">
    <Header />
    <main class="flex-1 px-4 py-4 md:px-6 md:py-6">
      <slot />
    </main>
    <Footer />
  </div>
</template>

<style>
/* Global Tailwind CSS is loaded from assets/css/main.css */
</style>
