<template>
    <div class="sidebar-toggle">
        <Button size="small" variant="text" @click="back" icon="pi pi-th-large">
        </Button>
        <Button size="small" variant="text" @click="onToggle" :icon="toggleIcon">
           
        </Button>
    </div>
</template>

<script setup lang="ts">

import { useRouter } from 'vue-router';
import { computed } from 'vue';
import { useSystemStoreHook } from '../../store/store';

const systemStore = useSystemStoreHook();

const router = useRouter();
const back = () => {
    systemStore.selectProject(null);
    router.push('/project')

}
const toggleIcon = computed(() => {
    return systemStore.showSidebar ? 'pi pi-angle-double-left' : 'pi pi-angle-double-right';
})

const onToggle = () => {
   systemStore.toggleShowSidebar()
}

</script>

<style scoped>
.sidebar-toggle {
    display: inline-flex;
    justify-content: center;
    height: 36px;
    align-items: center;
    z-index: 100;
}

</style>