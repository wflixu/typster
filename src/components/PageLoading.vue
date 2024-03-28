<template>
    <div class="page-loading">
        <a-spin size="large" />
    </div>
</template>

<script setup lang="ts">

</script>

<style scoped>
.page-loading {
   position: fixed;
   left: 0;
   right: 0;
   top: 0;
   bottom: 0;
   z-index: 1000;
   background-color: rgba(110,110,110, 0.4);
   display: flex;
   justify-content: center;
   align-items: center;
}
</style>