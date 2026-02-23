# 实现计划：基于 Tiptap 的 Typst WYSIWYG 编辑器

> 本计划基于 [spec/editor.md](editor.md) 的技术方案，详细说明了如何将 Typster 从基础 Markdown 编辑器改造为完整的 Typst WYSIWYG 编辑器。

## 上下文

### 背景

参考 [spec/editor.md](editor.md) 中的技术方案，本项目旨在将 Typster 从基础的 Markdown 编辑器改造为完整的 Typst WYSIWYG 编辑器。核心思路是利用 Tiptap 底层的 ProseMirror 提供的强大文档模型和扩展机制，精确映射 Typst 的抽象语法树（AST），并实现双向转换（源码 ↔ 模型）。

### 技术架构

```
┌─────────────────────────────────────────────────────────────┐
│              前端编辑器 (Tiptap/ProseMirror)                  │
├─────────────────────────────────────────────────────────────┤
│  ProseMirror 文档模型（自定义 Schema）                        │
│  Tiptap 扩展（Typst 节点/标记）                               │
│  用户交互（点击、输入、拖拽）                                 │
└─────────────────────────────────────────────────────────────┘
           ↓                              ↓
  导出为 Typst 源码              导入 Typst 源码
           ↓                              ↓
┌─────────────────────────────────────────────────────────────┐
│                     双向转换层                                │
│  Typst 解析器（后端编译）     Typst 序列化器（JS 实现）       │
└─────────────────────────────────────────────────────────────┘
           ↓
┌─────────────────────────────────────────────────────────────┐
│                   渲染与预览                                  │
│  Typst 编译器（Rust 后端）    →    PDF/Canvas 预览           │
└─────────────────────────────────────────────────────────────┘
```

### 用户需求确认

1. **实现策略**：完整的 Typst 原生支持（构建自定义扩展系统）
2. **数学公式**：使用 Typst 数学语法（`#$x^2$#`），而非 LaTeX/KaTeX
3. **预览模式**：混合模式（WYSIWYG 和分栏预览可切换）

## 核心设计原理

### ProseMirror Schema 映射 Typst 文档结构

根据 [spec/editor.md](editor.md) 的设计，Typst 文档元素需要精确映射为 ProseMirror 的**节点（Node）**和**标记（Mark）**：

**块级节点（Node）：**
- `heading` - 标题（`=`, `==`, `===` 等）
- `paragraph` - 段落
- `codeBlock` - 代码块
- `bulletList` - 无序列表
- `orderedList` - 有序列表
- `table` - 表格
- `figure` - 图片和图表
- `mathBlock` - 块级公式（`$#[ ... ]#`）

**行内标记（Mark）：**
- `strong` - 粗体（`*text*`）
- `emph` - 斜体（`_text_`）
- `raw` - 行内代码（`` `code` ``）
- `link` - 链接（`#link("url")[text]`）
- `mathInline` - 行内公式（`#$...$#`）

### 双向转换策略

**解析器（Typst → ProseMirror）：**
- Phase 1: 使用正则表达式解析常见 Typst 语法
- Phase 2+: 考虑使用后端 Typst 编译器返回 AST
- 长期: 评估 typst.ts WASM 方案

**序列化器（ProseMirror → Typst）：**
- 遍历 ProseMirror 文档树
- 每个节点/标记实现 `toTypst()` 方法
- 保持源码可读性（合理换行、缩进）

### Tiptap 扩展设计模式

每个 Typst 元素对应一个 Tiptap 扩展，包含：
1. Schema 定义
2. parseDOM/toDOM
3. addCommands
4. addKeyboardShortcuts
5. addInputRules
6. toTypst（通过 addStorage）

## 实施方案

### Phase 1: 核心扩展系统架构（3-5天）

**目标**：建立扩展系统基础设施，实现基础的 Typst 语法支持。

#### 需要创建的文件

**扩展系统核心：**
- [src/components/tiptap-editor/extensions/index.ts](../src/components/tiptap-editor/extensions/index.ts)
- [src/components/tiptap-editor/extensions/TypstExtension.ts](../src/components/tiptap-editor/extensions/TypstExtension.ts)
- [src/components/tiptap-editor/extensions/TypstParser.ts](../src/components/tiptap-editor/extensions/TypstParser.ts)
- [src/components/tiptap-editor/extensions/TypstSerializer.ts](../src/components/tiptap-editor/extensions/TypstSerializer.ts)

**节点扩展：**
- [src/components/tiptap-editor/extensions/nodes/TypstHeading.ts](../src/components/tiptap-editor/extensions/nodes/TypstHeading.ts)
- [src/components/tiptap-editor/extensions/nodes/TypstParagraph.ts](../src/components/tiptap-editor/extensions/nodes/TypstParagraph.ts)
- [src/components/tiptap-editor/extensions/nodes/TypstCodeBlock.ts](../src/components/tiptap-editor/extensions/nodes/TypstCodeBlock.ts)

**标记扩展：**
- [src/components/tiptap-editor/extensions/marks/TypstStrong.ts](../src/components/tiptap-editor/extensions/marks/TypstStrong.ts)
- [src/components/tiptap-editor/extensions/marks/TypstEmph.ts](../src/components/tiptap-editor/extensions/marks/TypstEmph.ts)
- [src/components/tiptap-editor/extensions/marks/TypstRaw.ts](../src/components/tiptap-editor/extensions/marks/TypstRaw.ts)

#### 测试验证

- [ ] 输入 `= 标题` 自动转换为一级标题
- [ ] 输入 `== 副标题` 自动转换为二级标题
- [ ] 输入 `*粗体*` 显示为粗体
- [ ] 输入 `_斜体_` 显示为斜体
- [ ] 保存时生成正确的 Typst 语法
- [ ] 加载现有 Typst 文件正确解析

---

### Phase 2: Typst 数学公式系统（5-7天）

**目标**：实现 Typst 原生数学公式的编辑和渲染。

#### 需要创建的文件

- [src/components/tiptap-editor/extensions/nodes/TypstMathInline.ts](../src/components/tiptap-editor/extensions/nodes/TypstMathInline.ts)
- [src/components/tiptap-editor/extensions/nodes/TypstMathBlock.ts](../src/components/tiptap-editor/extensions/nodes/TypstMathBlock.ts)
- [src/components/math-renderer/MathNodeView.ts](../src/components/math-renderer/MathNodeView.ts)
- [src/components/math-renderer/MathRenderer.vue](../src/components/math-renderer/MathRenderer.vue)
- [src/components/math-renderer/MathCache.ts](../src/components/math-renderer/MathCache.ts)

#### 需要修改的文件

- [src-tauri/src/cmds/typst.rs](../src-tauri/src/cmds/typst.rs) - 添加 `typst_compile_math` 命令

#### 测试验证

- [ ] 输入 `#$x^2$#` 自动转换为数学节点
- [ ] 数学公式正确渲染为 PNG
- [ ] 点击公式可编辑
- [ ] 编辑后按 Esc 或点击外部重新渲染
- [ ] 支持行内公式和块级公式（`$#[ ... ]#`）

---

### Phase 3: 增强 Typst 语法支持（4-6天）

**目标**：添加代码块、列表、表格、图片等高级语法。

#### 需要创建的文件

- [src/components/tiptap-editor/extensions/nodes/TypstBulletList.ts](../src/components/tiptap-editor/extensions/nodes/TypstBulletList.ts)
- [src/components/tiptap-editor/extensions/nodes/TypstOrderedList.ts](../src/components/tiptap-editor/extensions/nodes/TypstOrderedList.ts)
- [src/components/tiptap-editor/extensions/nodes/TypstTable.ts](../src/components/tiptap-editor/extensions/nodes/TypstTable.ts)
- [src/components/tiptap-editor/extensions/nodes/TypstFigure.ts](../src/components/tiptap-editor/extensions/nodes/TypstFigure.ts)

#### 测试验证

- [ ] 输入 `- item` 创建无序列表
- [ ] 输入 `+ item` 创建有序列表
- [ ] 支持嵌套列表
- [ ] 表格正确序列化
- [ ] 图片插入功能

---

### Phase 4: 实时预览系统（4-5天）

**目标**：实现混合预览模式（WYSIWYG + 分栏预览可切换）。

#### 需要创建的文件

- [src/components/preview/PreviewPanel.vue](../src/components/preview/PreviewPanel.vue)
- [src/components/preview/PreviewController.ts](../src/components/preview/PreviewController.ts)
- [src/components/preview/WysiwygRenderer.ts](../src/components/preview/WysiwygRenderer.ts)

#### 需要修改的文件

- [src/pages/typst/TypstEditor.vue](../src/pages/typst/TypstEditor.vue)
- [src/store/store.ts](../src/store/store.ts)

#### 测试验证

- [ ] 切换到分栏模式显示预览
- [ ] 编辑内容时预览自动更新（防抖）
- [ ] 手动刷新预览
- [ ] 导出 PDF 功能
- [ ] 编译错误正确显示

---

### Phase 5: 智能编辑功能（3-4天）

**目标**：实现自动补全、语法高亮、错误提示等。

#### 需要创建的文件

- [src/components/autocomplete/TypstAutocomplete.vue](../src/components/autocomplete/TypstAutocomplete.vue)
- [src/components/syntax-highlight/TypstHighlight.ts](../src/components/syntax-highlight/TypstHighlight.ts)

#### 需要修改的文件

- [src-tauri/src/cmds/typst.rs](../src-tauri/src/cmds/typst.rs) - 实现 `typst_autocomplete`

#### 测试验证

- [ ] 输入 `#` 显示可用函数列表
- [ ] 键盘导航选择补全项
- [ ] Tab 或 Enter 确认选择
- [ ] 语法错误实时提示

---

## 技术挑战与风险

### 1. Typst 可编程性的表示

**挑战**：如何表示 `#let`、函数调用等编程元素？

**解决方案**：保留为原始代码节点，不尝试求值

### 2. 性能问题

**挑战**：频繁编译可能较慢

**解决方案**：缓存、防抖、增量编译

### 3. 双向映射的复杂性

**挑战**：撤销/重做的同步

**解决方案**：单一数据源（ProseMirror 文档）

### 4. 解析准确性

**挑战**：正则表达式无法处理复杂嵌套

**解决方案**：分阶段实施，逐步增强

## 预估时间

| 阶段 | 时间 | 依赖 | 里程碑 |
|------|------|------|--------|
| Phase 1 | 3-5天 | 无 | 基础语法可编辑 |
| Phase 2 | 5-7天 | Phase 1 | 数学公式可渲染 |
| Phase 3 | 4-6天 | Phase 1 | 高级语法支持 |
| Phase 4 | 4-5天 | Phase 1,2 | 实时预览 |
| Phase 5 | 3-4天 | Phase 1 | 智能编辑 |
| **总计** | **19-27天** | | **3-4 周** |

## 参考

- [spec/editor.md](editor.md) - 技术方案详细说明
- [typst.ts](https://github.com/Myriad-Dreamin/typst.ts) - Typst WASM 绑定
