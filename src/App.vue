<script setup lang="ts">
import PageLoading from "./components/PageLoading.vue";
import { useSystemStoreHook } from "./store/store";

const systemStore = useSystemStoreHook();
</script>

<template>
    <PageLoading v-if="systemStore.loading"></PageLoading>
    <router-view></router-view>
</template>

<style scoped></style>
