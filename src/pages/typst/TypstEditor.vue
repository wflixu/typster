<template>
    <div class="typst-wysiwyg-editor">
        <!-- Typora-style Title Bar -->
        <div class="title-bar">
            <div class="title-bar-left">
                <span class="document-title">Untitled</span>
            </div>
            <div class="title-bar-right">
                <button class="toolbar-button" @click="saveDocument">Save</button>
            </div>
        </div>

        <!-- Main Editor Area -->
        <div class="editor-container">
            <!-- Tiptap Editor -->

            <editor-content :editor="editor" class="tiptap-editor" />


            <!-- Status Bar -->
            <div class="status-bar">
                <div class="status-left">
                    <span class="word-count">{{ wordCount }} words</span>
                    <span class="char-count">{{ charCount }} characters</span>
                </div>
                <div class="status-right">
                    <span class="cursor-position">Line {{ cursorLine }}, Col {{ cursorCol }}</span>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import { Markdown } from '@tiptap/markdown'

// UI State
const wordCount = ref(0)
const charCount = ref(0)
const cursorLine = ref(1)
const cursorCol = ref(1)

// Editor - 简化配置，专注基础文本编辑
const editor = useEditor({
    content: '<p>开始编写你的文档...</p>',
    extensions: [StarterKit, Markdown],
    contentType: 'markdown', // parse initial content as Markdown
    onUpdate: () => {
        updateStatistics()
        updateCursorPosition()
    },
    onSelectionUpdate: () => {
        updateCursorPosition()
    },
})

// 简单的编辑器工具函数
const updateStatistics = () => {
    if (!editor.value) return

    const text = editor.value.getText()
    const words = text.trim().split(/\s+/).filter(word => word.length > 0)
    wordCount.value = words.length
    charCount.value = text.length
}

const updateCursorPosition = () => {
    if (!editor.value) return

    const { from } = editor.value.state.selection
    const resolvedPos = editor.value.state.doc.resolve(from)
    cursorLine.value = resolvedPos.index(0) + 1
    cursorCol.value = resolvedPos.index(1) + 1
}

const saveDocument = () => {
    console.log('Save document - not implemented yet')
}

onMounted(() => {
    updateStatistics()
})

onUnmounted(() => {
    editor.value?.destroy()
})
</script>

<style scoped>
.typst-wysiwyg-editor {
    height: 100vh;
    display: flex;
    flex-direction: column;
    background: #ffffff;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    overflow: hidden;
}

/* Title Bar - Typora Style */
.title-bar {
    height: 36px;
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
}

.window-controls {
    display: flex;
    gap: 8px;
}

.window-control {
    width: 16px;
    height: 16px;
    border-radius: 50%;
    border: none;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 10px;
    cursor: pointer;
    transition: all 0.2s ease;
    color: #fff;
    font-weight: bold;
}

.window-control.minimize {
    background: #febc2e;
}

.window-control.maximize {
    background: #28ca42;
}

.window-control.close {
    background: #ff5f57;
}

.title-bar-right {
    display: flex;
    align-items: center;
    gap: 8px;
}

.toolbar-button {
    padding: 4px 12px;
    background: #007acc;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
}

.toolbar-button:hover {
    background: #005a9e;
}

/* Editor Container */
.editor-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
}


.tiptap-editor {
    min-height: 100%;
    line-height: 1.6;
    font-size: 16px;
    color: #2c3e50;
    padding: 40px 60px;
    overflow-y: auto;

    :deep(.tiptap) {
        outline: none;
        min-height: 100%;
        width: 100%;
    }
}


/* Status Bar */
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