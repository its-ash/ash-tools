<template>
  <div class="neo-canvas min-h-screen bg-[#FFFDF5] text-black flex flex-col">
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
