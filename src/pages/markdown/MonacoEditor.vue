<template>
  <div class="monaco-editor" ref="boxRef"></div>
</template>

<script setup lang="ts">
import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
import { onMounted, onUnmounted, ref, watch } from "vue";
const props = defineProps({
  modelValue: String,
});
const emit = defineEmits<{
  "update:modelValue": [value: string];
}>();

const boxRef = ref<HTMLElement>();
let monacoEditor: monaco.editor.IStandaloneCodeEditor | null = null;

onMounted(() => {
  console.log(boxRef.value);
  if (!boxRef.value) {
    return;
  }
  monacoEditor = monaco.editor.create(boxRef.value!, {
    value: props.modelValue,
    language: "mdx",
    fontSize: 16,
    lineHeight: 32,
    scrollBeyondLastColumn: 2,
    minimap: { enabled: false },
  });
  monacoEditor.onDidChangeModelContent((ev) => {
    emit("update:modelValue", monacoEditor?.getValue() ?? "");
  });
});

watch(
  () => props.modelValue,
  async (val, old) => {
    if (monacoEditor) {
      if (val != monacoEditor.getValue()) {
        monacoEditor.setValue(val ?? "");
        await monacoEditor
          .getAction("monacoEditor.action.formatDocument")
          ?.run();
      }
    }
  }
);

onUnmounted(() => {
  if (monacoEditor) {
    monacoEditor.dispose();
  }
});
</script>

<style scoped>
.monaco-editor {
  padding: 16px 0px;
  /* background-color: black; */
  width: 100%;
  height: 100%;
}
</style>
