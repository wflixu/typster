<template>
    <div class="typst-editor">
        <editor-content :editor="editor" class="tiptap-editor" />
    </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import { Markdown } from '@tiptap/markdown';
import { useSystemStoreHook } from '../../store/store';
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';
import { showSaveChanges, showFileError } from '../../utils/dialog-utils';

const systemStore = useSystemStoreHook();

// 文件保存状态
const isSaving = ref(false);
const hasUnsavedChanges = ref(false);

// Editor - 配置编辑器，正确使用@tiptap/markdown
const editor = useEditor({
    content: 'default content',
    contentType: 'markdown',
    extensions: [
        StarterKit,
        Markdown.configure({
            html: false, // 不使用HTML输入
            transformPastedText: true, // 自动转换粘贴的文本为Markdown
            transformCopiedText: false, // 复制时不转换
            breaks: true, // 支持换行符
        })
    ],
    onUpdate: () => {
        updateStatistics()
        updateCursorPosition()
        hasUnsavedChanges.value = true
    },
    onSelectionUpdate: () => {
        updateCursorPosition()
    },
    editorProps: {
        attributes: {
            spellcheck: 'false',
        },
    },
})

// 文件操作函数
const loadFileContent = async (filePath: string) => {
    if (!filePath || !editor.value) return;

    try {
        console.log('Loading file:', filePath);
        const content = await readTextFile(filePath);

        // 直接设置内容，让Markdown扩展自动解析
        editor.value.commands.setContent(content, {
            contentType: 'markdown',
            emitUpdate: true,
        });
        hasUnsavedChanges.value = false;

        // 立即更新统计信息
        updateStatistics();
        updateCursorPosition();

        console.log('File loaded successfully, content:', content.substring(0, 100) + '...');
    } catch (error) {
        console.error('Failed to load file:', error);
        await showFileError('读取文件失败', error);
    }
};

const saveCurrentFile = async () => {
    if (!systemStore.editingFilePath || !editor.value || isSaving.value) return;

    isSaving.value = true;
    try {
        // 使用 @tiptap/markdown 扩展的 getMarkdown() 方法获取 Markdown 内容
        const markdownContent = editor.value.getMarkdown();
        console.log('Saving markdown content:', markdownContent.substring(0, 100) + '...');
        await writeTextFile(systemStore.editingFilePath, markdownContent);
        hasUnsavedChanges.value = false;
        console.log('File saved successfully');
    } catch (error) {
        console.error('Failed to save file:', error);
        await showFileError('保存文件失败', error);
    } finally {
        isSaving.value = false;
    }
};


// 自动保存功能 (防抖)
let saveTimeout: number | null = null;
const autoSave = () => {
    if (saveTimeout) {
        clearTimeout(saveTimeout);
    }
    saveTimeout = setTimeout(() => {
        if (hasUnsavedChanges.value) {
            saveCurrentFile();
        }
    }, 60000); // 60秒后自动保存（1分钟）
};

// 统计函数
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
    const doc = editor.value.state.doc
    const text = doc.textBetween(0, from)
    const lines = text.split('\n')

    systemStore.setEditingInfo({
        cursorLine: lines.length,
        cursorCol: lines[lines.length - 1].length + 1,
    })
}

// 监听 editingFilePath 变化
watch(
    () => systemStore.editingFilePath,
    (newPath, oldPath) => {
        if (newPath !== oldPath && newPath) {
            // 检查是否有未保存的更改
            if (hasUnsavedChanges.value && oldPath) {
                showSaveChanges(oldPath, async () => {
                    await saveCurrentFile();
                    loadFileContent(newPath);
                }, () => {
                    loadFileContent(newPath);
                });
            } else {
                loadFileContent(newPath);
            }
        } else if (!newPath) {
            // 清空编辑器
            editor.value?.commands.setContent('');
            hasUnsavedChanges.value = false;
        }
    },
);

// 监听编辑器初始化完成，然后加载文件
watch(editor, (newEditor) => {
    if (newEditor && systemStore.editingFilePath) {
        // 编辑器初始化完成且有文件路径，加载文件
        loadFileContent(systemStore.editingFilePath);
    }
}, { immediate: true });

// 监听编辑器内容变化，触发自动保存
watch(hasUnsavedChanges, (newValue) => {
    if (newValue) {
        autoSave();
    }
});

// 暴露保存函数供外部使用
defineExpose({
    saveFile: saveCurrentFile,
    hasUnsavedChanges: () => hasUnsavedChanges.value
});
// 监听窗口关闭事件
const handleBeforeUnload = (event: BeforeUnloadEvent) => {
    if (hasUnsavedChanges.value) {
        event.preventDefault();
        // 现代浏览器不需要设置returnValue
    }
};
onMounted(() => {
    updateStatistics()
    window.addEventListener('beforeunload', handleBeforeUnload);
})

onUnmounted(() => {
    editor.value?.destroy()
    window.removeEventListener('beforeunload', handleBeforeUnload);
    if (saveTimeout) {
        clearTimeout(saveTimeout);
    }
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