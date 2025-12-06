<template>
    <main :class="{ 'expand': systemStore.showSidebar }">
        <div class="sidebar" v-show="systemStore.showSidebar">
            <Sidebar />
        </div>
        <div class="title">
            <TitleBar />
        </div>
        <div class="editor">
            <TypstEditor />
        </div>
        <div class="status">
            <StatusBar />
        </div>
    </main>
</template>

<script setup lang="ts">
import TypstEditor from '../typst/TypstEditor.vue'
import Sidebar from './Sidebar.vue';
import TitleBar from './TitleBar.vue'
import StatusBar from './StatusBar.vue';
import { useSystemStoreHook } from '../../store/store';

const systemStore = useSystemStoreHook()


</script>

<style scoped>
/* 默认布局 - 侧边栏隐藏 */
main {
    height: 100vh;
    width: 100%;
    display: grid;
    grid-template-columns: 1fr;
    /* 单列布局 */
    grid-template-rows: 32px 1fr 24px;
    /* 标题栏 + 编辑器 + 状态栏 */
}

/* 侧边栏展开布局 */
main.expand {
    grid-template-columns: 300px 1fr;
    /* 侧边栏 300px + 主内容区自适应 */
    grid-template-rows: 32px 1fr 24px;
    /* 保持相同的三行结构 */
    grid-template-areas:
        "sidebar title"
        "sidebar editor"
        "sidebar status";

    /* 侧边栏跨越所有行 */
    .sidebar {
        grid-area: sidebar;
        /* 展开时指定到侧边栏区域 */
    }

    .title {
        grid-area: title;
        /* 展开时指定到标题栏区域 */
    }

    .editor {
        grid-area: editor;
        /* 展开时指定到编辑器区域 */
       overflow-y: auto;
    }

    .status {
        grid-area: status;
        /* 展开时指定到状态栏区域 */
    }
}

/* 侧边栏样式 - 仅在展开时显示 */
.sidebar {
    background-color: #f8f9fa;
    border-right: 1px solid #e1e5e9;
}


</style>