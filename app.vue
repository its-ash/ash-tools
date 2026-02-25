<template>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>



<script setup>


function print() {
  let hue = Math.random() * 360;
  setTimeout(console.clear, 0);
  setTimeout(
    console.log.bind(
      console,
      '%c✨ Welcome to Ash Tools ✨',
      `color: hsl(${hue}, 100%, 50%); font-size: 5vw; font-weight: bold;`
    )
  );
}


if (!process.dev) {
  print();
  setTimeout(print, 1000)
}




</script>