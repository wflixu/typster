<template>
    <div class="title-bar">
        <div class="title-bar-left">
        </div>
        <div class="title-bar-center">

        </div>
        <div class="title-bar-right">
            <!--  -->
            <i class="pi  cursor-pointer" :class="{
                'pi-angle-double-left': systemStore.showSidebar,
                'pi-angle-double-right': !systemStore.showSidebar,
            }" @click="() => systemStore.toggleShowSidebar()" style="font-size: 1rem"></i>
            <i class="pi pi-cog cursor-pointer" style="font-size: 1rem"></i>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useSystemStoreHook } from '../../store/store';


const systemStore = useSystemStoreHook()


</script>

<style scoped>
/* Title Bar - Typora Style */
.title-bar {
    height: 32px;
    background: #fafafa;
    border-bottom: 1px solid #e1e5e9;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 16px;
    user-select: none;
    flex-shrink: 0;
}

.title-bar-left {
    display: flex;
    align-items: center;
    gap: 8px;
}




.document-title {
    font-size: 14px;
    color: #666;
    font-weight: 500;
}

.title-bar-center {
    flex: 1;
    display: flex;
    justify-content: center;
    align-items: center;
}


.title-bar-right {
    display: flex;
    align-items: center;
    gap: 8px;
}
</style>