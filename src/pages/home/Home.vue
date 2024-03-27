<template>
    <main :class="{'expand': systemStore.showSidebar}">
        <MoveBar/>
        <Sidebar v-show="systemStore.showSidebar"/>
        <TypstEditor />
    </main>
</template>

<script setup lang="ts">
import TypstEditor from '../typst/TypstEditor.vue'
import Sidebar from './Sidebar.vue';
import MoveBar from './../../components/MoveBar.vue'
import { useSystemStoreHook } from '../../store/store';

const systemStore =  useSystemStoreHook()


</script>

<style scoped>
main {
    height: 100vh;
    display: grid;
    grid-template-columns:  1fr;
    grid-template-rows: 1fr;
}
main.expand {
    grid-template-columns: 300px 1fr;
    grid-template-rows: 1fr;
}
</style>