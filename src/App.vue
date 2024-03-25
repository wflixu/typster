<script setup lang="ts">
import MdxEditor from './pages/markdown/MdxEditor.vue'
import TypstEditor from './pages/typst/TypstEditor.vue'
</script>

<template>
  <main>
   <!-- <MdxEditor/> -->
   <TypstEditor/>
  </main>
</template>

<style scoped>

main {
  height: 100vh;
  background-color: goldenrod;
}
</style>
