<template>
  <div class="tiptap-editor" ref="editorRef">
    <editor-content :editor="editor" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from 'vue'
import { Editor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import { Bold } from '@tiptap/extension-bold'
import { Italic } from '@tiptap/extension-italic'
import { Document } from '@tiptap/extension-document'
import { Paragraph } from '@tiptap/extension-paragraph'
import { Text } from '@tiptap/extension-text'
import { Heading } from '@tiptap/extension-heading'
import { History } from '@tiptap/extension-history'
import { invoke } from '@tauri-apps/api/core'
import { throttle, debounce } from 'radash'
import { TypstCompileResult } from '../../pages/typst/interface'
import { relativePath } from '../../shared/util'

// Props
interface Props {
  content: string
  path: string
  root: string
}

const props = defineProps<Props>()

// Emits
interface Emits {
  (e: 'change', content: string): void
  (e: 'compiled', data: TypstCompileResult): void
}

const emit = defineEmits<Emits>()

// Refs
const editorRef = ref<HTMLElement>()
let editor: Editor | undefined = undefined

// Methods
const initializeEditor = () => {
  editor = new Editor({
    element: editorRef.value!,
    extensions: [
      StarterKit,
      Bold,
      Italic,
      Document,
      Paragraph,
      Text,
      Heading.configure({
        levels: [1, 2, 3, 4, 5, 6],
      }),
      History,
    ],
    content: props.content || '',
    onUpdate: ({ editor }) => {
      const content = editor.getHTML()
      emit('change', content)
      handleCompileThrottle()
      handleSaveDebounce()
    },
  })
}

const setContent = (content: string) => {
  if (editor) {
    editor.commands.setContent(content)
  }
}

const handleCompile = async () => {
  if (!editor) return

  // Convert HTML content back to Typst format
  const typstContent = await convertHtmlToTypst(editor.getHTML())

  if (!typstContent) return

  try {
    const path = relativePath(props.root, props.path)
    const result = await invoke<TypstCompileResult>('typst_compile_doc', {
      path,
      content: typstContent
    })
    emit('compiled', result)
  } catch (error) {
    console.error('Compilation error:', error)
  }
}

const handleSave = async () => {
  if (!editor) return

  try {
    // Convert HTML content back to Typst format
    const typstContent = await convertHtmlToTypst(editor.getHTML())

    if (!typstContent) return

    const path = relativePath(props.root, props.path)
    await invoke("fs_write_file_text", {
      path,
      content: typstContent
    })

    await handleCompile()
  } catch (error) {
    console.error('Save error:', error)
  }
}

// Convert HTML from Tiptap back to Typst markup
const convertHtmlToTypst = async (html: string): Promise<string> => {
  // Basic conversion - this will be enhanced later
  return html
    .replace(/<h1[^>]*>(.*?)<\/h1>/gi, '= $1')
    .replace(/<h2[^>]*>(.*?)<\/h2>/gi, '== $1')
    .replace(/<h3[^>]*>(.*?)<\/h3>/gi, '=== $1')
    .replace(/<h4[^>]*>(.*?)<\/h4>/gi, '==== $1')
    .replace(/<h5[^>]*>(.*?)<\/h5>/gi, '===== $1')
    .replace(/<h6[^>]*>(.*?)<\/h6>/gi, '====== $1')
    .replace(/<strong[^>]*>(.*?)<\/strong>/gi, '*$1*')
    .replace(/<b[^>]*>(.*?)<\/b>/gi, '*$1*')
    .replace(/<em[^>]*>(.*?)<\/em>/gi, '_$1_')
    .replace(/<i[^>]*>(.*?)<\/i>/gi, '_$1_')
    .replace(/<p[^>]*>(.*?)<\/p>/gi, '$1\n')
    .replace(/<br\s*\/?>/gi, '\n')
    .replace(/<[^>]*>/g, '') // Remove any remaining HTML tags
    .replace(/&nbsp;/g, ' ')
    .replace(/&amp;/g, '&')
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>')
    .replace(/&quot;/g, '"')
    .trim()
}

// Throttled and debounced handlers
const handleCompileThrottle = throttle({ interval: 1000 }, handleCompile)
const handleSaveDebounce = debounce({ delay: 500 }, handleSave)

// Lifecycle
onMounted(() => {
  initializeEditor()
})

onBeforeUnmount(() => {
  if (editor) {
    editor.destroy()
  }
})

// Watch for content changes from parent
watch(() => props.content, (newContent) => {
  if (editor && newContent !== editor.getHTML()) {
    setContent(newContent)
  }
})

// Expose methods
defineExpose({
  setContent,
  getEditor: () => editor,
  getHTML: () => editor?.getHTML() || '',
})
</script>

<style scoped>
.tiptap-editor {
  height: 100%;
  padding: 16px;
  box-sizing: border-box;
  overflow-y: auto;
}

/* Tiptap editor styles */
:deep(.ProseMirror) {
  height: 100%;
  outline: none;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  font-size: 16px;
  line-height: 1.6;
  color: var(--text-color);
}

:deep(.ProseMirror h1) {
  font-size: 2em;
  font-weight: bold;
  margin: 1em 0 0.5em 0;
  line-height: 1.2;
}

:deep(.ProseMirror h2) {
  font-size: 1.5em;
  font-weight: bold;
  margin: 1em 0 0.5em 0;
  line-height: 1.3;
}

:deep(.ProseMirror h3) {
  font-size: 1.25em;
  font-weight: bold;
  margin: 1em 0 0.5em 0;
  line-height: 1.4;
}

:deep(.ProseMirror p) {
  margin: 0.5em 0;
}

:deep(.ProseMirror strong) {
  font-weight: bold;
}

:deep(.ProseMirror em) {
  font-style: italic;
}

:deep(.ProseMirror:focus) {
  outline: none;
}

/* Selection and cursor styles */
:deep(.ProseMirror-selectednode) {
  outline: 2px solid var(--primary-color);
  outline-offset: 2px;
}
</style>