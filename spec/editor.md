

### 🎯 目标：用 Tiptap 构建 Typst 的 WYSIWYG 编辑器

Tiptap 本身是为富文本（HTML/Markdown）设计的，但它的底层 ProseMirror 提供了强大的**文档模型**和**扩展机制**，理论上可以表示任意结构化内容——包括 Typst 的文档。因此，我们可以将 Tiptap 改造为一个 Typst 编辑器，核心是让 ProseMirror 的文档模型能够精确映射 Typst 的抽象语法树（AST），并实现双向转换（源码 ↔ 模型）。

下图展示了整体架构：

```mermaid
flowchart LR
    subgraph “前端编辑器 (Tiptap/ProseMirror)”
        A[“ProseMirror 文档模型<br>（自定义 Schema）”]
        B[“Tiptap 扩展<br>（Typst 节点/标记）”]
        C[“用户交互<br>（点击、输入、拖拽）”]
    end

    subgraph “双向转换层”
        D[“Typst 解析器<br>（WASM/JS）”]
        E[“Typst 序列化器<br>（WASM/JS）”]
    end

    subgraph “渲染与预览”
        F[“Typst 编译器<br>（WASM）”]
        G[“PDF/Canvas 预览”]
    end

    C --> B --> A
    A -- “导出为 Typst 源码” --> E --> F --> G
    G -. “点击预览定位源码” .-> A
    F -. “编译结果用于验证” .-> E
    D -- “导入 Typst 源码” --> A
```

---

### 🧱 第一步：设计 ProseMirror Schema 映射 Typst 文档结构

Typst 的文档元素需要映射为 ProseMirror 的**节点（Node）**和**标记（Mark）**。

- **节点**：对应块级元素，如 `heading`、`paragraph`、`list`、`figure`、`table`、`math`（块级公式）、`code block` 等。
- **标记**：对应行内样式或修饰，如 `strong`、`emph`、`raw`（行内代码）、`link`、`math`（行内公式）等。

例如 Typst 源码：
```typst
= 一级标题
在 *Typst* 中，你可以写 $x^2$ 这样的公式。
```
可以映射为 ProseMirror 文档：
```json
{
  "type": "doc",
  "content": [
    { "type": "heading", "attrs": { "level": 1 }, "content": [{ "type": "text", "text": "一级标题" }] },
    { "type": "paragraph", "content": [
        { "type": "text", "text": "在 " },
        { "type": "text", "marks": [{ "type": "strong" }], "text": "Typst" },
        { "type": "text", "text": " 中，你可以写 " },
        { "type": "inline_math", "attrs": { "source": "x^2" } },
        { "type": "text", "text": " 这样的公式。" }
    ]}
  ]
}
```

**关键点**：ProseMirror 的 Schema 需要严格定义哪些节点允许包含哪些内容（例如标题内不能有块级节点），这需要仔细研究 Typst 的语法规则。

---

### 🔄 第二步：实现双向转换（Typst 源码 ↔ ProseMirror 文档）

这是最核心的工作，需要两个方向的处理：

#### 1. 解析器：Typst 源码 → ProseMirror 文档
Typst 本身是用 Rust 写的，我们可以将其解析器编译为 **WebAssembly (WASM)**，在前端直接调用。这样能保证解析结果与官方一致，且性能良好。

- **方法**：使用 `wasm-pack` 将 `typst-syntax` crate 编译为 WASM，暴露一个函数，输入源码字符串，输出一个 JSON 格式的抽象语法树（AST）。然后在前端编写一个转换器，将这个 AST 转换为符合 ProseMirror Schema 的文档节点。
- **备选方案**：如果不想依赖 WASM，也可以用 JavaScript 重新实现一个 Typst 解析器（但工作量巨大，且难以保证完全兼容）。

#### 2. 序列化器：ProseMirror 文档 → Typst 源码
我们需要将 ProseMirror 文档遍历并输出为 Typst 源码。每个节点和标记都需知道如何渲染自己为 Typst 语法。

- **实现方式**：可以借鉴 Tiptap 的 `Markdown` 扩展思路，为每个自定义节点实现 `toTypst` 方法。在 Tiptap 中，可以通过扩展的 `addStorage` 或自定义插件来注入序列化逻辑。
- **格式保留**：序列化时要保持源码的可读性（比如合理换行、缩进），并且处理好特殊字符的转义（如 `#`、`$` 等）。

---

### 🔌 第三步：创建 Tiptap 扩展

为每个 Typst 元素创建一个 Tiptap 扩展，这些扩展需要：

- **定义 Schema**：在扩展中声明节点或标记的结构和属性。
- **配置 `parseDOM`**：由于我们使用自定义解析器，通常不需要 HTML 解析，但为了编辑器的内部视图（如用户输入的文本），我们可能需要支持从 HTML 粘贴等场景。更重要的可能是配置 `toDOM`，以便在编辑区域展示一个“占位”视图（例如公式节点可以用 KaTeX 渲染一个预览）。
- **实现命令**：为扩展定义编辑器命令，例如 `toggleHeading`、`insertInlineMath`。
- **添加快捷键**：例如 `Ctrl+B` 插入粗体标记。
- **输入规则**：支持在输入时触发转换，比如输入 `= ` 后按空格自动变成标题。

```javascript
// 示例：Heading 扩展
import { Node } from '@tiptap/core'

export const TypstHeading = Node.create({
  name: 'heading',
  group: 'block',
  content: 'inline*',
  defining: true,

  addAttributes() {
    return { level: { default: 1 } }
  },

  parseDOM() { return [] }, // 不从 HTML 解析，因为我们通过自定义解析器导入
  toDOM(node) {
    return ['h' + node.attrs.level, 0] // 在编辑器内以普通标题显示
  },

  // 序列化到 Typst 源码的方法（需通过其他方式集成）
  addStorage() {
    return {
      toTypst: (node) => '='.repeat(node.attrs.level) + ' ' + node.textContent + '\n'
    }
  },

  addCommands() {
    return {
      setHeading: (attrs) => ({ commands }) => commands.toggleNode('paragraph', 'heading', attrs)
    }
  },

  addKeyboardShortcuts() {
    return {
      'Mod-Alt-1': () => this.editor.commands.setHeading({ level: 1 }),
      // ...
    }
  },

  addInputRules() {
    return [
      // 当用户输入 "= " + 空格 时变成一级标题
      new RegExp('^= $', 'i'), // 简化示例
    ]
  }
})
```

---

### 👁️ 第四步：实现实时预览与交互

真正的 WYSIWYG 需要让用户看到接近最终排版的效果。有以下几种策略：

1. **直接渲染**：在编辑器内部使用 HTML+CSS 模拟 Typst 的样式（例如标题用 `<h1>`，公式用 KaTeX）。这种方法容易实现，但与最终 PDF 可能有差异。
2. **集成 Typst 编译器**：将 Typst 编译器也编译为 WASM，每次编辑后生成 PDF 或 SVG，在另一个面板预览。这是最准确的方式，但需要考虑性能优化（防抖、增量编译）。
3. **混合模式**：编辑器内显示简化的样式，同时提供实时 PDF 预览窗口。用户点击预览中的元素可以跳转到源码（利用我们之前讨论的“双向映射”）。

实现“点击预览定位源码”需要前端与编译器协作：预览渲染时，每个元素携带其在源码中的位置信息（通过 `Span`），当用户点击时，前端通过某种协议（如 LSP）将这个位置传递给编辑器，编辑器高亮对应的源码范围。

---

### 🧩 现有资源与挑战

- **已有探索**：社区已经有一些尝试，例如 [typst.ts](https://github.com/Myriad-Dreamin/typst.ts) 项目，将 Typst 编译器编译为 WASM，并提供了与 Monaco Editor 的集成。你可以借鉴其 WASM 绑定。
- **挑战**：
    - Typst 具有可编程性（如 `#let` 定义变量、函数调用），如何在 ProseMirror 模型中表示这些动态内容？可能需要将某些代码块保留为“原始代码节点”，或者设计更复杂的节点类型来存储求值后的结果。
    - 性能问题：每次编辑都重新编译整个文档可能较慢，需要实现增量编译。
    - 双向映射的复杂性：需要将 ProseMirror 的事务与源码变更同步，确保撤销/重做等操作同时作用于模型和源码。

---

### 🚀 总结

拓展 Tiptap 实现 Typst 的 WYSIWYG 是一个宏伟的目标，但技术上是可行的。核心步骤可以概括为：

1. **设计 ProseMirror Schema** 精确映射 Typst AST。
2. **实现解析器/序列化器**（利用 WASM 或 JS 实现）完成源码与模型的互转。
3. **创建 Tiptap 扩展** 封装每个 Typst 元素，提供交互命令。
4. **集成 Typst 编译器** 实现高质量预览，并逐步完善双向映射。

这需要前端、编译原理和 Rust/WASM 的交叉知识。如果你对其中某个环节特别感兴趣，我们可以深入讨论具体实现细节。