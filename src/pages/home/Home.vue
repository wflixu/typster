<template>
    <main :class="{'expand': expand}">
        <SidebarToggle v-model:expand="expand" />
        <Sidebar v-show="expand"/>
        <TypstEditor />
    </main>
</template>

<script setup lang="ts">
import TypstEditor from '../typst/TypstEditor.vue'
import SidebarToggle from './SidebarToggle.vue'
import Sidebar from './Sidebar.vue';
import { ref } from 'vue';

const expand = ref(true);



</script>

<style scoped>
main {
    background-color: gold;
    border: 1px solid red;
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