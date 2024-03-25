// @ts-nocheck

import * as monaco from "monaco-editor";
import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
import jsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";

// @ts-ignore
self.MonacoEnvironment = {
  getWorker(_: any, label: string) {
    if (label === "json") {
      return new jsonWorker();
    }
    return new editorWorker();
  },
};

import { language as mdxLanguage } from "monaco-editor/esm/vs/basic-languages/mdx/mdx.js";

monaco.languages.register({ id: "mdx" });

monaco.languages.registerCompletionItemProvider("mdx", {
  provideCompletionItems: (model, position) => {
    // 获取当前光标前的文本
    const word = model.getWordUntilPosition(position);
    const range = {
      startLineNumber: position.lineNumber,
      endLineNumber: position.lineNumber,
      startColumn: word.startColumn,
      endColumn: position.column,
    };

    // 这里可以根据 MDX 的语法规则和关键字来提供补全建议
    // 以下是一个简单的示例，只返回固定的建议列表
    const suggestions = [
      {
        label: "SELECT",
        kind: monaco.languages.CompletionItemKind.Keyword,
        insertText: "SELECT",
        range: range,
      },
      {
        label: "FROM",
        kind: monaco.languages.CompletionItemKind.Keyword,
        insertText: "FROM",
        range: range,
      },
      // ... 添加更多的补全建议
    ];

    return { suggestions };
  },
});
