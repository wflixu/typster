<template>
  <div class="editor" ref="boxRef"></div>
</template>

<script setup lang="ts">
import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
import { PropType, onMounted, onUnmounted, ref, watch } from "vue";
import { IMode } from "../pages/typst/interface";

const model = defineModel({ type: String, default: '' })

const props = defineProps({
  mode: String as PropType<IMode>
})

const boxRef = ref<HTMLElement>();
let monacoEditor: monaco.editor.IStandaloneCodeEditor | null = null;

const resizeObserver = new ResizeObserver((entries) => {
  for (const entry of entries) {
    const contentBoxSize = entry.contentBoxSize;
    if (contentBoxSize && monacoEditor) {
      monacoEditor.layout({ width: contentBoxSize[0].inlineSize!, height: contentBoxSize[0].blockSize });
    }
  }
});


onMounted(() => {
  if (!boxRef.value) {
    return;
  }
  monacoEditor = monaco.editor.create(boxRef.value!, {
    value: model.value,
    language: "typst",
    fontSize: 16,
    lineHeight: 28,
    scrollBeyondLastColumn: 2,
    minimap: { enabled: false },
  });
  monacoEditor.onDidChangeModelContent((ev) => {
    
    model.value = monacoEditor?.getValue() ?? "";
  });

  resizeObserver.observe(boxRef.value!)

});

watch(() => props.mode, () => {
  if (monacoEditor) {
    monacoEditor.layout()
  }
})



watch(
  () => model.value,
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

  resizeObserver.disconnect();
});
</script>

<style scoped>
.editor {
  box-sizing: border-box;
  padding: 16px 0px;
  height: 100%;
  overflow: hidden;
}
</style>
