<template>
  <div class="monaco-editor" ref="boxRef"></div>
</template>

<script setup lang="ts">
import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
import { onMounted, onUnmounted, ref, watch } from "vue";

const model = defineModel({ type: String, default: '' })

const boxRef = ref<HTMLElement>();
let monacoEditor: monaco.editor.IStandaloneCodeEditor | null = null;

onMounted(() => {
  console.log(boxRef.value);
  if (!boxRef.value) {
    return;
  }
  monacoEditor = monaco.editor.create(boxRef.value!, {
    value: model.value,
    language: "mdx",
    fontSize: 16,
    lineHeight: 32,
    scrollBeyondLastColumn: 2,
    minimap: { enabled: false },
  });
  monacoEditor.onDidChangeModelContent((ev) => {
    model.value = monacoEditor?.getValue() ?? "";
  });
  // // 使用浏览器原生 API 实现复制
  // monacoEditor.onCopy(function (e) {
  //   // 将选中的文本复制到剪贴板
  //   navigator.clipboard.writeText(e.selection.getSelectedText());
  // });

  // 使用编辑器事件实现粘贴
  monacoEditor.onDidPaste(function (e) {
    // 将剪贴板中的文本粘贴到编辑器中
    console.log(e)
    // e.selection.insertText(navigator.clipboard.readText());
  });
});

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
});
</script>

<style scoped>
.monaco-editor {
  padding: 16px 0px;
  width: 100%;
  height: 100%;
}
</style>
