<template>
    <div class="status-bar">
        <div class="status-left">
            <!-- 文件路径 -->
            <span v-if="systemStore.editingFilePath" class="file-path" :title="systemStore.editingFilePath">
                📄 {{ systemStore.editingFilePath }}
            </span>
            <span v-else class="no-file">无打开文件</span>

            <!-- 保存/加载状态 -->
            <span v-if="saveStatus" class="save-status" :class="saveStatus.type">
                {{ saveStatus.text }}
            </span>
        </div>
        <div class="status-center">
            <MoveBar />
        </div>
        <div class="status-right">
            <!-- 编辑器状态信息 -->
            <span class="word-count">{{ wordCount }} words</span>
            <span class="cursor-position">Ln {{ cursorLine }}, Col {{ cursorCol }}</span>

            <!-- 未保存状态指示器 -->
            <span v-if="hasUnsavedChanges" class="unsaved-indicator">● 未保存</span>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, toRefs } from 'vue';
import { useSystemStoreHook } from '../../store/store';
import MoveBar from '../../components/MoveBar.vue';

const systemStore = useSystemStoreHook()

// 移除 charCount，只保留需要的统计信息
const { wordCount, cursorCol, cursorLine } = toRefs(systemStore.editingInfo)

// 保存状态管理
const saveStatus = ref<{ type: 'saving' | 'saved' | 'loading' | 'error', text: string } | null>(null)
const hasUnsavedChanges = ref(false)

// 监听来自编辑器的事件
const handleSaveStatus = (event: CustomEvent) => {
    saveStatus.value = event.detail
    // 3秒后清除状态
    setTimeout(() => {
        saveStatus.value = null
    }, 3000)
}

const handleUnsavedChanges = (event: CustomEvent) => {
    hasUnsavedChanges.value = event.detail
}

// 获取文件名
const getFileName = (filePath: string): string => {
    return filePath.split('/').pop() || filePath
}


onMounted(() => {
    // 监听编辑器状态事件
    window.addEventListener('save-status', handleSaveStatus as EventListener)
    window.addEventListener('unsaved-changes', handleUnsavedChanges as EventListener)
})

onUnmounted(() => {
    window.removeEventListener('save-status', handleSaveStatus as EventListener)
    window.removeEventListener('unsaved-changes', handleUnsavedChanges as EventListener)
})

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
    gap: 12px;
    align-items: center;
}

.status-left span,
.status-right span {
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    font-size: 11px;
}

/* 文件路径样式 */
.file-path {
    color: #2c3e50;
    font-weight: 500;
    max-width: 300px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.no-file {
    color: #999;
    font-style: italic;
}

/* 保存状态样式 */
.save-status {
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 10px;
    font-weight: 500;
}

.save-status.saving {
    background: #007acc;
    color: white;
}

.save-status.saved {
    background: #28a745;
    color: white;
}

.save-status.loading {
    background: #6c757d;
    color: white;
}

.save-status.error {
    background: #dc3545;
    color: white;
}

/* 编辑器状态样式 */
.word-count {
    color: #666;
}

.cursor-position {
    color: #666;
}

.unsaved-indicator {
    color: #ff9500;
    font-weight: bold;
    animation: blink 2s ease-in-out infinite;
}

@keyframes blink {
    0%, 50% { opacity: 1; }
    51%, 100% { opacity: 0.3; }
}
</style>