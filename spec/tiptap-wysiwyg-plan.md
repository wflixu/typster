# 实现计划：基于 Tiptap 的 Typst WYSIWYG 编辑器

## 上下文

Typster 项目当前使用 `@tiptap/markdown` 扩展提供基础的 Markdown 编辑功能。目标是将其改造为完整的 Typst WYSIWYG 编辑器，提供类似 Typora 的所见即所得体验，同时支持实时 PDF 预览和 Typst 原生数学公式。

### 当前实现状态

**已实现：**
- Tiptap 编辑器基础框架（[TypstEditor.vue](src/pages/typst/TypstEditor.vue)）
- Markdown 格式支持（通过 `@tiptap/markdown`）
- 完整的 Typst 0.14.0 后端编译系统（[typst.rs](src-tauri/src/cmds/typst.rs)）
- PNG 渲染和 base64 编码返回
- 文件系统和安全验证机制
- Pinia 状态管理（[store.ts](src/store/store.ts)）

**缺失的核心功能：**
- 自定义 Tiptap 扩展系统（完全缺失）
- Typst 特定节点和标记（heading, math, code, lists 等）
- Typst 数学公式系统（完全缺失）
- 双向转换（Typst ↔ ProseMirror 文档）
- 实时预览功能
- 自动补全集成

### 用户需求确认

1. **实现策略**：完整的 Typst 原生支持（构建自定义扩展系统）
2. **数学公式**：使用 Typst 数学语法（`#$x^2$#`），而非 LaTeX/KaTeX
3. **预览模式**：混合模式（WYSIWYG 和分栏预览可切换）

## 推荐方案

采用分阶段的方式构建完整的 Typst 扩展系统，每个阶段都可独立测试和验证。

### Phase 1: 核心扩展系统架构（3-5天）

建立扩展系统基础设施，实现基础的 Typst 语法支持。

#### 需要创建的文件

1. **[src/components/tiptap-editor/extensions/index.ts](src/components/tiptap-editor/extensions/index.ts)** - 扩展导出入口
2. **[src/components/tiptap-editor/extensions/TypstExtension.ts](src/components/tiptap-editor/extensions/TypstExtension.ts)** - 基础扩展类
3. **[src/components/tiptap-editor/extensions/nodes/TypstHeading.ts](src/components/tiptap-editor/extensions/nodes/TypstHeading.ts)** - 标题节点（`=`, `==`, `===` 等）
4. **[src/components/tiptap-editor/extensions/nodes/TypstParagraph.ts](src/components/tiptap-editor/extensions/nodes/TypstParagraph.ts)** - 段落节点
5. **[src/components/tiptap-editor/extensions/nodes/TypstCodeBlock.ts](src/components/tiptap-editor/extensions/nodes/TypstCodeBlock.ts)** - 代码块节点
6. **[src/components/tiptap-editor/extensions/marks/TypstStrong.ts](src/components/tiptap-editor/extensions/marks/TypstStrong.ts)** - 粗体标记（`*text*`）
7. **[src/components/tiptap-editor/extensions/marks/TypstEmph.ts](src/components/tiptap-editor/extensions/marks/TypstEmph.ts)** - 斜体标记（`_text_`）
8. **[src/components/tiptap-editor/extensions/TypstParser.ts](src/components/tiptap-editor/extensions/TypstParser.ts)** - Typst → ProseMirror 解析器
9. **[src/components/tiptap-editor/extensions/TypstSerializer.ts](src/components/tiptap-editor/extensions/TypstSerializer.ts)** - ProseMirror → Typst 序列化器

#### 需要修改的文件

1. **[src/pages/typst/TypstEditor.vue](src/pages/typst/TypstEditor.vue)** - 集成新扩展系统

#### 实现要点

**TypstHeading.ts 示例：**
```typescript
import { Node, mergeAttributes } from '@tiptap/core'

export const TypstHeading = Node.create({
  name: 'heading',
  group: 'block',
  content: 'inline*',
  defining: true,

  addAttributes() {
    return {
      level: { default: 1 },
      typstSyntax: { default: null } // 保存原始 Typst 语法
    }
  },

  parseDOM() {
    return [1, 2, 3, 4, 5, 6].map(level => ({
      tag: `h${level}`,
      attrs: { level }
    }))
  },

  renderHTML({ node, HTMLAttributes }) {
    return [`h${node.attrs.level}`, mergeAttributes(HTMLAttributes), 0]
  },

  addInputRules() {
    // Typst 语法: = == === 等
    return [
      // '= ' → h1, '== ' → h2, etc.
    ]
  }
})
```

**TypstParser.ts 核心逻辑：**
```typescript
export class TypstParser {
  parse(source: string): ProseMirrorNode {
    // 解析 Typst 语法为 ProseMirror 文档
    // 1. 按行分割
    // 2. 识别标题 (=, ==, ===)
    // 3. 识别粗体 (*text*) 和斜体 (_text_)
    // 4. 构建文档树
  }
}
```

**TypstSerializer.ts 核心逻辑：**
```typescript
export class TypstSerializer {
  serialize(doc: ProseMirrorNode): string {
    // 将 ProseMirror 文档序列化为 Typst 源码
    // heading → "= Title"
    // strong → "*text*"
    // emph → "_text_"
  }
}
```

#### 测试验证

- [ ] 输入 `= 标题` 自动转换为一级标题
- [ ] 输入 `== 副标题` 自动转换为二级标题
- [ ] 输入 `*粗体*` 显示为粗体
- [ ] 输入 `_斜体_` 显示为斜体
- [ ] 保存时生成正确的 Typst 语法
- [ ] 加载现有 Typst 文件正确解析

---

### Phase 2: Typst 数学公式系统（5-7天）

实现 Typst 原生数学公式的编辑和渲染。

#### 关键设计决策

由于选择使用 **Typst 数学语法**（而非 LaTeX/KaTeX），数学公式的渲染将依赖后端 Typst 编译器。

**渲染策略：**
1. 编辑器中显示 Typst 数学语法（如 `#$x^2$#`）作为占位符
2. 使用后端 `typst_compile_doc` 编译数学公式为 PNG
3. 在编辑器中显示渲染后的 PNG
4. 点击公式可编辑源码

#### 需要创建的文件

1. **[src/components/tiptap-editor/extensions/nodes/TypstMath.ts](src/components/tiptap-editor/extensions/nodes/TypstMath.ts)** - 数学公式节点
2. **[src/components/math-renderer/MathNodeView.ts](src/components/math-renderer/MathNodeView.ts)** - 数学公式节点视图
3. **[src/components/math-renderer/MathRenderer.vue](src/components/math-renderer/MathRenderer.vue)** - 数学公式渲染组件

#### TypstMath.ts 实现

```typescript
import { Node, mergeAttributes } from '@tiptap/core'

export const TypstMath = Node.create({
  name: 'math',
  group: 'inline',
  inline: true,
  atom: true,

  addAttributes() {
    return {
      typstSyntax: { default: '' }, // #$x^2$#
      rendered: { default: false }, // 是否已渲染
      imageData: { default: null }  // base64 PNG
    }
  },

  parseDOM() {
    return [{
      tag: 'span[data-type="math"]',
      getAttrs: node => ({
        typstSyntax: (node as HTMLElement).getAttribute('data-typst-syntax'),
        imageData: (node as HTMLElement).getAttribute('data-image-data')
      })
    }]
  },

  renderHTML({ HTMLAttributes }) {
    return ['span', mergeAttributes({
      'data-type': 'math',
      class: 'typst-math-inline'
    }, HTMLAttributes), 0]
  },

  addNodeView() {
    return ({ node, view, getPos }) => new MathNodeView(node, view, getPos)
  },

  addInputRules() {
    // Typst 数学语法: $#x^2$# 或 $#\sum#$
    return [
      // 识别 $#...$# 并转换为数学节点
    ]
  }
})
```

#### MathNodeView.ts 实现

```typescript
export class MathNodeView implements NodeView {
  node: ProseMirrorNode
  dom: HTMLElement
  isEditing: boolean

  constructor(node, view, getPos) {
    this.node = node
    this.isEditing = false
    this.dom = document.createElement('span')
    this.dom.className = 'typst-math-inline'
    this.update(node)
  }

  update(node) {
    if (node.type !== this.node.type) return false
    this.node = node
    this.render()
    return true
  }

  private async render() {
    const syntax = this.node.attrs.typstSyntax
    if (!this.isEditing && syntax) {
      // 调用后端编译数学公式
      const imageData = await this.compileMath(syntax)
      this.dom.innerHTML = `<img src="data:image/png;base64,${imageData}" />`
    } else {
      // 显示可编辑的文本框
      this.dom.innerHTML = `<textarea>${syntax}</textarea>`
    }
  }

  private async compileMath(syntax: string): Promise<string> {
    // 调用后端编译单个数学公式
    const result = await invoke('typst_compile_math', { syntax })
    return result.image // base64 PNG
  }

  selectNode() {
    this.isEditing = true
    this.render()
  }

  deselectNode() {
    this.isEditing = false
    this.render()
  }
}
```

#### 后端扩展

在 **[src-tauri/src/cmds/typst.rs](src-tauri/src/cmds/typst.rs)** 添加：

```rust
#[command]
pub async fn typst_compile_math(syntax: String) -> Result<MathRenderResult, AppError> {
    // 创建临时文件，仅包含数学公式
    // 调用 Typst 编译器
    // 返回 base64 编码的 PNG
}
```

#### 测试验证

- [ ] 输入 `#$x^2$#` 自动转换为数学节点
- [ ] 数学公式正确渲染为 PNG
- [ ] 点击公式可编辑
- [ ] 编辑后按 Esc 或点击外部重新渲染
- [ ] 支持行内公式和块级公式（`$#[ ... ]#`）

---

### Phase 3: 增强 Typst 语法支持（4-6天）

添加代码块、列表、表格、图片等高级语法。

#### 需要创建的文件

1. **[src/components/tiptap-editor/extensions/nodes/TypstBulletList.ts](src/components/tiptap-editor/extensions/nodes/TypstBulletList.ts)** - 无序列表（`- item`）
2. **[src/components/tiptap-editor/extensions/nodes/TypstOrderedList.ts](src/components/tiptap-editor/extensions/nodes/TypstOrderedList.ts)** - 有序列表（`+ item` 或 `1. item`）
3. **[src/components/tiptap-editor/extensions/nodes/TypstTable.ts](src/components/tiptap-editor/extensions/nodes/TypstTable.ts)** - 表格支持
4. **[src/components/tiptap-editor/extensions/nodes/TypstFigure.ts](src/components/tiptap-editor/extensions/nodes/TypstFigure.ts)** - 图片和图表

#### 实现要点

**列表输入规则：**
```typescript
// '- ' → bullet list item
// '+ ' 或 '1. ' → ordered list item
```

**TypstSerializer 增强：**
```typescript
serialize(doc: ProseMirrorNode): string {
  // 处理列表、表格、图片等
  // bulletList → "- item"
  // table → Typst 表格语法
}
```

#### 测试验证

- [ ] 输入 `- item` 创建无序列表
- [ ] 输入 `+ item` 创建有序列表
- [ ] 支持嵌套列表
- [ ] 表格正确序列化
- [ ] 图片插入功能（已有 [image-handler](src/utils/image-handler.ts)）

---

### Phase 4: 实时预览系统（4-5天）

实现分栏预览和 WYSIWYG 模式切换。

#### 需要创建的文件

1. **[src/components/preview/PreviewPanel.vue](src/components/preview/PreviewPanel.vue)** - 预览面板组件
2. **[src/components/preview/PreviewController.ts](src/components/preview/PreviewController.ts)** - 预览控制器

#### 需要修改的文件

1. **[src/pages/typst/TypstEditor.vue](src/pages/typst/TypstEditor.vue)** - 添加预览面板和模式切换
2. **[src/store/store.ts](src/store/store.ts)** - 添加预览模式状态

#### 实现要点

**预览模式状态：**
```typescript
// store.ts
const previewMode = ref<'wysiwyg' | 'split' | 'preview'>('wysiwyg')
```

**PreviewPanel.vue：**
```vue
<template>
  <div class="preview-panel" :class="{ visible: isVisible }">
    <div class="preview-header">
      <button @click="refresh">刷新</button>
      <button @click="exportPDF">导出 PDF</button>
    </div>
    <div class="preview-content">
      <img v-for="page in pages" :src="page.image" />
    </div>
  </div>
</template>

<script setup lang="ts">
const compileDocument = async () => {
  const result = await invoke('typst_compile_doc', {
    path,
    content: getCurrentTypstSource()
  })
  pages.value = result[0] // PageData array
}
</script>
```

**后端已支持：**
- `typst_compile_doc` 已实现 PNG 渲染
- 返回 `PageData[]` 包含 base64 编码的图片
- 只需在前端显示这些图片

#### 测试验证

- [ ] 切换到分栏模式显示预览
- [ ] 编辑内容时预览自动更新（防抖）
- [ ] 手动刷新预览
- [ ] 导出 PDF 功能
- [ ] 编译错误正确显示

---

### Phase 5: 智能编辑功能（3-4天）

实现自动补全、语法高亮、错误提示等。

#### 需要创建的文件

1. **[src/components/autocomplete/TypstAutocomplete.vue](src/components/autocomplete/TypstAutocomplete.vue)** - 自动补全 UI
2. **[src/components/syntax-highlight/TypstHighlight.ts](src/components/syntax-highlight/TypstHighlight.ts)** - 语法高亮

#### 需要修改的文件

1. **[src-tauri/src/cmds/typst.rs](src-tauri/src/cmds/typst.rs)** - 实现 `typst_autocomplete`

#### 实现要点

**自动补全：**
```typescript
// 监听输入，调用后端自动补全
const completions = await invoke('typst_autocomplete', {
  path: filePath,
  position: cursorPosition
})
```

**后端实现：**
```rust
pub async fn typst_autocomplete(
    path: String,
    position: u32
) -> Result<Vec<TypstCompletion>, AppError> {
    // 使用 typst-ide 的自动补全功能
}
```

#### 测试验证

- [ ] 输入 `#` 显示可用函数列表
- [ ] 键盘导航选择补全项
- [ ] Tab 或 Enter 确认选择
- [ ] 语法错误实时提示

---

## 安装依赖

不需要额外安装前端依赖（不需要 KaTeX）。

后端可能需要添加：
```toml
# Cargo.toml
typst-ide = "0.14" # 用于自动补全
```

---

## 关键文件清单

### 需要创建的文件（按优先级）

**Phase 1 - 核心：**
- [src/components/tiptap-editor/extensions/TypstExtension.ts](src/components/tiptap-editor/extensions/TypstExtension.ts)
- [src/components/tiptap-editor/extensions/nodes/TypstHeading.ts](src/components/tiptap-editor/extensions/nodes/TypstHeading.ts)
- [src/components/tiptap-editor/extensions/nodes/TypstParagraph.ts](src/components/tiptap-editor/extensions/nodes/TypstParagraph.ts)
- [src/components/tiptap-editor/extensions/nodes/TypstCodeBlock.ts](src/components/tiptap-editor/extensions/nodes/TypstCodeBlock.ts)
- [src/components/tiptap-editor/extensions/marks/TypstStrong.ts](src/components/tiptap-editor/extensions/marks/TypstStrong.ts)
- [src/components/tiptap-editor/extensions/marks/TypstEmph.ts](src/components/tiptap-editor/extensions/marks/TypstEmph.ts)
- [src/components/tiptap-editor/extensions/TypstParser.ts](src/components/tiptap-editor/extensions/TypstParser.ts)
- [src/components/tiptap-editor/extensions/TypstSerializer.ts](src/components/tiptap-editor/extensions/TypstSerializer.ts)

**Phase 2 - 数学公式：**
- [src/components/tiptap-editor/extensions/nodes/TypstMath.ts](src/components/tiptap-editor/extensions/nodes/TypstMath.ts)
- [src/components/math-renderer/MathNodeView.ts](src/components/math-renderer/MathNodeView.ts)
- [src/components/math-renderer/MathRenderer.vue](src/components/math-renderer/MathRenderer.vue)

**Phase 3 - 高级语法：**
- [src/components/tiptap-editor/extensions/nodes/TypstBulletList.ts](src/components/tiptap-editor/extensions/nodes/TypstBulletList.ts)
- [src/components/tiptap-editor/extensions/nodes/TypstOrderedList.ts](src/components/tiptap-editor/extensions/nodes/TypstOrderedList.ts)
- [src/components/tiptap-editor/extensions/nodes/TypstTable.ts](src/components/tiptap-editor/extensions/nodes/TypstTable.ts)
- [src/components/tiptap-editor/extensions/nodes/TypstFigure.ts](src/components/tiptap-editor/extensions/nodes/TypstFigure.ts)

**Phase 4 - 预览：**
- [src/components/preview/PreviewPanel.vue](src/components/preview/PreviewPanel.vue)
- [src/components/preview/PreviewController.ts](src/components/preview/PreviewController.ts)

**Phase 5 - 智能编辑：**
- [src/components/autocomplete/TypstAutocomplete.vue](src/components/autocomplete/TypstAutocomplete.vue)
- [src/components/syntax-highlight/TypstHighlight.ts](src/components/syntax-highlight/TypstHighlight.ts)

### 需要修改的文件

- [src/pages/typst/TypstEditor.vue](src/pages/typst/TypstEditor.vue) - 集成新扩展
- [src/store/store.ts](src/store/store.ts) - 添加预览模式状态
- [src-tauri/src/cmds/typst.rs](src-tauri/src/cmds/typst.rs) - 添加数学公式编译和自动补全

---

## 验证测试

### Phase 1 验证
```bash
# 1. 启动开发服务器
pnpm start

# 2. 测试基础语法
# 输入 "= 标题" → 自动转为 h1
# 输入 "== 副标题" → 自动转为 h2
# 输入 "*粗体*" → 显示粗体
# 输入 "_斜体_" → 显示斜体

# 3. 保存并重新加载
# 验证文件保存为正确的 Typst 语法
# 验证重新加载后正确解析
```

### Phase 2 验证
```bash
# 1. 测试数学公式
# 输入 "#$x^2$#" → 转换为数学节点并渲染
# 点击公式 → 显示编辑框
# 修改并按 Esc → 重新渲染

# 2. 测试块级公式
# 输入 "$#[\sum_{i=1}^{n} i]#" → 渲染为块级公式
```

### Phase 3 验证
```bash
# 测试列表、表格、图片
# - item → 无序列表
# + item → 有序列表
# 粘贴图片 → 插入图片语法
```

### Phase 4 验证
```bash
# 1. 测试模式切换
# 点击工具栏按钮 → 切换到分栏模式
# 验证预览面板显示

# 2. 测试实时预览
# 编辑内容 → 预览自动更新（1秒延迟）

# 3. 测试 PDF 导出
# 点击导出按钮 → 生成 PDF 文件
```

### Phase 5 验证
```bash
# 测试自动补全
# 输入 "#" → 显示补全列表
# 使用键盘导航 → 选择补全项
```

---

## 风险和注意事项

### 技术风险

1. **Typst 数学公式渲染性能**
   - 风险：每个公式都需要后端编译，可能较慢
   - 缓解：实现缓存机制，相同公式不重复编译

2. **双向转换准确性**
   - 风险：正则表达式解析可能无法处理复杂嵌套
   - 缓解：逐步增强解析器，优先处理常见场景

3. **大文档性能**
   - 风险：大量数学公式或复杂文档可能导致性能问题
   - 缓解：虚拟滚动、增量渲染

### 兼容性风险

1. **Markdown 模式破坏**
   - 风险：新扩展可能影响现有 Markdown 功能
   - 缓解：通过文件扩展名（.md vs .typ）选择不同解析器

2. **状态管理复杂度**
   - 风险：预览模式、编辑模式可能导致状态混乱
   - 缓解：清晰的状态管理，使用 Pinia store

---

## 预估时间

| 阶段 | 时间 | 依赖 |
|------|------|------|
| Phase 1: 核心扩展系统 | 3-5天 | 无 |
| Phase 2: 数学公式系统 | 5-7天 | Phase 1 |
| Phase 3: 增强语法支持 | 4-6天 | Phase 1 |
| Phase 4: 实时预览 | 4-5天 | Phase 1, Phase 2 |
| Phase 5: 智能编辑 | 3-4天 | Phase 1 |
| **总计** | **19-27天** | |

建议按顺序实施，每个阶段完成后进行测试验证。
