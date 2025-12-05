<template>
  <div class="hybrid-editor">
    <!-- Editor Mode Toggle -->
    <div class="editor-toolbar">
      <div class="mode-toggle">
        <button
          @click="setMode('wysiwyg')"
          :class="{ active: currentMode === 'wysiwyg' }"
          class="mode-btn"
        >
          <span class="mode-icon">📝</span>
          Visual Mode
        </button>
        <button
          @click="setMode('source')"
          :class="{ active: currentMode === 'source' }"
          class="mode-btn"
        >
          <span class="mode-icon">💻</span>
          Source Mode
        </button>
      </div>
    </div>

    <!-- Editor Container -->
    <div class="editor-container">
      <!-- Monaco Editor (Source Mode) -->
      <MonacoEditor
        v-if="currentMode === 'source'"
        :path="path"
        :root="root"
        @change="handleMonacoChange"
        @compiled="handleCompiled"
        ref="monacoRef"
      />

      <!-- Tiptap Editor (WYSIWYG Mode) -->
      <TiptapEditor
        v-if="currentMode === 'wysiwyg'"
        :content="editorContent"
        :path="path"
        :root="root"
        @change="handleTiptapChange"
        @compiled="handleCompiled"
        ref="tiptapRef"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, computed } from 'vue'
import MonacoEditor from '../MonacoEditor.vue'
import TiptapEditor from '../tiptap-editor/TiptapEditor.vue'
import { TypstCompileResult } from '../../pages/typst/interface'

type EditorMode = 'wysiwyg' | 'source'

// Props
interface Props {
  path: string
  root: string
  initialMode?: EditorMode
}

const props = withDefaults(defineProps<Props>(), {
  initialMode: 'wysiwyg'
})

// Emits
interface Emits {
  (e: 'change', content: string): void
  (e: 'compiled', data: TypstCompileResult): void
  (e: 'mode-changed', mode: EditorMode): void
}

const emit = defineEmits<Emits>()

// Refs
const currentMode = ref<EditorMode>(props.initialMode)
const editorContent = ref<string>('')
const monacoRef = ref()
const tiptapRef = ref()

// Computed
const isWysiwygMode = computed(() => currentMode.value === 'wysiwyg')

// Methods
const setMode = (mode: EditorMode) => {
  if (currentMode.value === mode) return

  // Sync content before switching
  if (currentMode.value === 'source' && monacoRef.value) {
    // Get content from Monaco before switching
    const monacoEditor = monacoRef.value.editor
    if (monacoEditor) {
      const model = monacoEditor.getModel()
      if (model) {
        editorContent.value = model.getValue()
      }
    }
  }

  currentMode.value = mode
  emit('mode-changed', mode)
}

const handleMonacoChange = (content: string) => {
  editorContent.value = content
  emit('change', content)
}

const handleTiptapChange = (content: string) => {
  editorContent.value = content
  emit('change', content)
}

const handleCompiled = (data: TypstCompileResult) => {
  emit('compiled', data)
}

const syncContentBetweenEditors = () => {
  // This will be called when switching modes to ensure content is synchronized
  if (currentMode.value === 'wysiwyg' && tiptapRef.value) {
    // Update Tiptap with Monaco content
    tiptapRef.value.setContent(editorContent.value)
  } else if (currentMode.value === 'source' && monacoRef.value) {
    // Update Monaco with Tiptap content
    const monacoEditor = monacoRef.value.editor
    if (monacoEditor) {
      const model = monacoEditor.getModel()
      if (model) {
        model.setValue(editorContent.value)
      }
    }
  }
}

// Lifecycle
onMounted(() => {
  // Set initial mode
  setMode(props.initialMode)
})

watch(currentMode, (newMode, oldMode) => {
  if (newMode !== oldMode) {
    syncContentBetweenEditors()
  }
})

// Expose methods for parent components
defineExpose({
  setMode,
  getCurrentMode: () => currentMode.value,
  getContent: () => editorContent.value,
  setContent: (content: string) => {
    editorContent.value = content
    syncContentBetweenEditors()
  }
})
</script>

<style scoped>
.hybrid-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--surface-ground);
}

.editor-toolbar {
  display: flex;
  justify-content: flex-end;
  align-items: center;
  padding: 8px 16px;
  background: var(--surface-card);
  border-bottom: 1px solid var(--surface-border);
  min-height: 48px;
}

.mode-toggle {
  display: flex;
  background: var(--surface-100);
  border-radius: 8px;
  padding: 4px;
  gap: 4px;
}

.mode-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  border: none;
  background: transparent;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-color-secondary);
  transition: all 0.2s ease;
}

.mode-btn:hover {
  background: var(--surface-200);
  color: var(--text-color);
}

.mode-btn.active {
  background: var(--primary-color);
  color: white;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.mode-icon {
  font-size: 16px;
}

.editor-container {
  flex: 1;
  height: calc(100% - 48px);
  position: relative;
}

/* Ensure editors take full height */
.editor-container :deep(.monaco-editor) {
  height: 100%;
}

.editor-container :deep(.tiptap-editor) {
  height: 100%;
}
</style>