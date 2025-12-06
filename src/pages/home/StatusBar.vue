<template>
    <div class="status-bar">
        <div class="status-left">
            <span class="word-count">{{ wordCount }} words</span>
            <span class="char-count">{{ charCount }} characters</span>
        </div>
        <div class="status-center">
            <MoveBar />
        </div>
        <div class="status-right">
            <span class="cursor-position">Line {{ cursorLine }}, Col {{ cursorCol }}</span>
        </div>
    </div>
</template>

<script setup lang="ts">
import { toRefs } from 'vue';
import { useWinMove } from '../../shared/move-hook';
import { useSystemStoreHook } from '../../store/store';
import MoveBar from '../../components/MoveBar.vue';


const systemStore = useSystemStoreHook()

const { wordCount, charCount, cursorCol, cursorLine } = toRefs(systemStore.editingInfo)

const { mousedownHandler,
    mouseupHandler,
    mousemoveHandler,
    mouseleaveHandler, } = useWinMove()

</script>

<style scoped>
.status-bar {
    height: 24px;
    background: #f8f9fa;
    border-top: 1px solid #e1e5e9;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 16px;
    font-size: 12px;
    color: #666;
    flex-shrink: 0;
}

.status-center {
    flex: 1;
    height: 24px;
}

.status-left,
.status-right {
    display: flex;
    gap: 16px;
    align-items: center;
}

.status-left span,
.status-right span {
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    font-size: 11px;
}
</style>