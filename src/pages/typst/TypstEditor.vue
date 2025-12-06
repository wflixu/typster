<template>
    <div class="typst-editor">
        <editor-content :editor="editor" class="tiptap-editor" />
    </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import { Markdown } from '@tiptap/markdown';
import { useSystemStoreHook } from '../../store/store';

const systemStore = useSystemStoreHook();


// Editor - 简化配置，专注基础文本编辑
const editor = useEditor({
    content: '<h1>Untitled Document</h1><p>Start writing your Typst document here...</p>',
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
    systemStore.setEditingInfo({
        wordCount: words.length,
        charCount: text.length,
    })

}

const updateCursorPosition = () => {
    if (!editor.value) return

    const { from } = editor.value.state.selection
    const resolvedPos = editor.value.state.doc.resolve(from)
    systemStore.setEditingInfo({
        cursorLine: resolvedPos.index(0) + 1,
        cursorCol: resolvedPos.index(1) + 1,
    })
}



onMounted(() => {
    updateStatistics()
})

onUnmounted(() => {
    editor.value?.destroy()
})
</script>

<style scoped>
.typst-editor {
    background: #ffffff;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    overflow: hidden;
}


/* Editor Container */
.editor-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
}

.tiptap-editor {
    flex: 1;
    line-height: 1.6;
    font-size: 16px;
    color: #2c3e50;
    padding: 40px 60px;
    overflow-y: auto;
    background: #ffffff;

    :deep(.tiptap) {
        outline: none;
        min-height: 100%;
        width: 100%;
        max-width: 900px;
        margin: 0 auto;
        padding: 20px 0;
    }

    :deep(.tiptap p) {
        margin-bottom: 16px;
        line-height: 1.8;
    }

    :deep(.tiptap h1) {
        font-size: 2em;
        font-weight: 600;
        margin-top: 32px;
        margin-bottom: 16px;
        line-height: 1.3;
        color: #1a1a1a;
    }

    :deep(.tiptap h2) {
        font-size: 1.5em;
        font-weight: 600;
        margin-top: 24px;
        margin-bottom: 12px;
        line-height: 1.4;
        color: #1a1a1a;
    }

    :deep(.tiptap h3) {
        font-size: 1.25em;
        font-weight: 600;
        margin-top: 20px;
        margin-bottom: 10px;
        line-height: 1.4;
        color: #1a1a1a;
    }
}
</style>