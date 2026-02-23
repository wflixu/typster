# 实现计划：基于 Tiptap 的 Typst WYSIWYG 编辑器

## 上下文

### 背景

参考 [spec/editor.md](spec/editor.md) 中的技术方案，本项目旨在将 Typster 从基础的 Markdown 编辑器改造为完整的 Typst WYSIWYG 编辑器。核心思路是利用 Tiptap 底层的 ProseMirror 提供的强大文档模型和扩展机制，精确映射 Typst 的抽象语法树（AST），并实现双向转换（源码 ↔ 模型）。

### 技术架构

```
┌─────────────────────────────────────────────────────────────┐
│              前端编辑器 (Tiptap/ProseMirror)                  │
├─────────────────────────────────────────────────────────────┤
│  ProseMirror 文档模型（自定义 Schema）                        │
│  Tiptap 扩展（Typst 节点/标记）                               │
│  用户交互（点击、输入、拖拽）                                 │
└─────────────────────────────────────────────────────────────┘
           ↓ IPC Commands          ↑ IPC Results
┌─────────────────────────────────────────────────────────────┐
│                   Rust 后端 (Tauri)                          │
├─────────────────────────────────────────────────────────────┤
│  typst-syntax   → 解析 Typst 源码为 AST                      │
│  typst-ide      → 自动补全、诊断                             │
│  typst-render   → 渲染公式和文档                            │
│  typst-svg      → 生成 SVG（数学公式）                      │
│  typst-pdf      → 导出 PDF                                  │
│  typst-html     → 生成 HTML（可选）                         │
└─────────────────────────────────────────────────────────────┘
           ↓
┌─────────────────────────────────────────────────────────────┐
│                   前端渲染与显示                              │
│  SVG/PNG 图片    → 数学公式、文档预览                         │
│  诊断信息        → 错误提示                                  │
│  补全列表        → 自动补全                                  │
└─────────────────────────────────────────────────────────────┘
```

**前后端分工：**

| 功能 | 前端 (Vue + Tiptap) | 后端 (Rust + Typst crates) |
|------|---------------------|---------------------------|
| **编辑器交互** | ProseMirror 文档模型<br>用户输入处理<br>UI 渲染 | - |
| **Typst 解析** | 将 AST 转换为 ProseMirror 文档 | `typst-syntax` 解析源码 |
| **数学公式** | 显示渲染后的 SVG<br>Click-to-edit 交互 | `typst-render` + `typst-svg` 渲染 |
| **自动补全** | 显示补全列表<br>处理用户选择 | `typst-ide` 生成补全 |
| **诊断** | 显示错误/警告<br>标记错误位置 | `typst-ide` 提供诊断 |
| **PDF 导出** | 触发导出命令 | `typst-pdf` 生成 PDF |
| **实时预览** | 显示预览面板<br>缩放、导航 | `typst-render` 渲染页面 |

**使用的 Typst Crates：**

```toml
# Cargo.toml (已安装)
typst = "0.14.0"           # 核心库
typst-syntax = "0.14.0"    # 解析器
typst-ide = "0.14.0"       # IDE 功能（补全、诊断）
typst-render = "0.14.0"    # 渲染
typst-svg = "0.14.0"       # SVG 导出
typst-pdf = "0.14.0"       # PDF 导出
typst-html = "0.14.0"      # HTML 导出（可选）
```

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
- ProseMirror Schema 精确映射 Typst AST
- Typst 特定节点和标记（heading, math, code, lists, figure, table）
- Typst 数学公式系统（完全缺失）
- 双向转换（Typst ↔ ProseMirror 文档）
- 实时预览功能
- 自动补全集成

### 用户需求确认

1. **实现策略**：完整的 Typst 原生支持（构建自定义扩展系统）
2. **数学公式**：使用 Typst 数学语法（`#$x^2$#`），而非 LaTeX/KaTeX
3. **预览模式**：混合模式（WYSIWYG 和分栏预览可切换）

### 参考资源

- **typst.ts**: [https://github.com/Myriad-Dreamin/typst.ts](https://github.com/Myriad-Dreamin/typst.ts) - Typst 编译器 WASM 绑定（可选参考）

## 核心设计原理

### ProseMirror Schema 映射 Typst 文档结构

根据 [spec/editor.md](spec/editor.md) 的设计，Typst 文档元素需要精确映射为 ProseMirror 的**节点（Node）**和**标记（Mark）**：

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

**示例映射：**

Typst 源码：
```typst
= 一级标题
在 *Typst* 中，你可以写 $#x^2$# 这样的公式。
```

ProseMirror 文档：
```json
{
  "type": "doc",
  "content": [
    {
      "type": "heading",
      "attrs": { "level": 1, "typstSyntax": "= 一级标题" },
      "content": [{ "type": "text", "text": "一级标题" }]
    },
    {
      "type": "paragraph",
      "attrs": { "typstSyntax": "在 *Typst* 中，你可以写 $#x^2$# 这样的公式。" },
      "content": [
        { "type": "text", "text": "在 " },
        { "type": "text", "marks": [{ "type": "strong" }], "text": "Typst" },
        { "type": "text", "text": " 中，你可以写 " },
        { "type": "mathInline", "attrs": { "typstSyntax": "#$x^2$#" } },
        { "type": "text", "text": " 这样的公式。" }
      ]
    }
  ]
}
```

### 双向转换策略（充分利用 Rust 后端）

**核心原则**：前端专注于编辑器交互，后端 Rust 处理所有 Typst 特定的逻辑。

**解析器（Typst → ProseMirror）：**

使用后端 `typst-syntax` crate 解析 Typst 源码为 AST，然后通过 IPC 传递给前端。

**后端实现（Rust）：**
```rust
use typst_syntax::{parse, FileId, Source, VirtualPath};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TypstAstNode {
    pub node_type: String,
    pub attrs: std::collections::HashMap<String, String>,
    pub content: String,
    pub children: Vec<TypstAstNode>,
}

#[tauri::command]
pub async fn typst_parse_source(source: String) -> Result<Vec<TypstAstNode>, String> {
    let id = FileId::new(None, VirtualPath::new("main.typ".to_string()));
    let source = Source::new(id, &source);

    // 使用 typst-syntax 解析
    let root = parse(&source);

    // 转换为前端可用的格式
    let ast_nodes = convert_to_frontend_ast(root);
    Ok(ast_nodes)
}

fn convert_to_frontend_ast(node: typst_syntax::SyntaxNode) -> TypstAstNode {
    // 将 Typst AST 节点转换为前端可用的格式
    // ...
}
```

**前端使用：**
```typescript
// 前端调用后端解析
const ast = await invoke<TypstAstNode[]>('typst_parse_source', {
  source: typstContent
})

// 将 AST 转换为 ProseMirror 文档
const doc = astToProseMirror(ast, schema)
```

**序列化器（ProseMirror → Typst）：**
- **前端**：遍历 ProseMirror 文档树，生成 Typst 源码
- **后端验证**：可选地使用 `typst-syntax` 验证生成的源码

### Tiptap 扩展设计模式

每个 Typst 元素对应一个 Tiptap 扩展，包含：
1. **Schema 定义** - 声明节点/标记结构和属性
2. **parseDOM** - HTML 解析（用于粘贴等场景）
3. **toDOM** - 在编辑器内渲染视图
4. **addCommands** - 编辑命令（如 `toggleHeading`, `insertMath`）
5. **addKeyboardShortcuts** - 快捷键（如 `Mod-Alt-1`）
6. **addInputRules** - 输入规则（如 `= ` → 标题）
7. **toTypst** - 序列化方法（存储在扩展的 storage 中）

## 实施方案

采用分阶段的方式构建完整的 Typst 扩展系统，每个阶段都可独立测试和验证。

### Phase 1: 核心扩展系统架构（3-5天）

**目标**：建立扩展系统基础设施，实现基础的 Typst 语法支持。

#### 技术要点

基于 [spec/editor.md](spec/editor.md) 的扩展设计模式，每个扩展需要：
1. **定义 Schema**：声明节点/标记结构和属性
2. **parseDOM/toDOM**：HTML 交互（用于粘贴等场景）
3. **addCommands**：编辑命令
4. **addKeyboardShortcuts**：快捷键
5. **addInputRules**：输入时自动转换
6. **toTypst**：序列化方法（通过 addStorage）

#### 需要创建的文件

**扩展系统核心：**
1. **[src/components/tiptap-editor/extensions/index.ts](src/components/tiptap-editor/extensions/index.ts)** - 扩展导出入口
2. **[src/components/tiptap-editor/extensions/TypstExtension.ts](src/components/tiptap-editor/extensions/TypstExtension.ts)** - 基础扩展类
3. **[src/components/tiptap-editor/extensions/TypstParser.ts](src/components/tiptap-editor/extensions/TypstParser.ts)** - Typst → ProseMirror 解析器
4. **[src/components/tiptap-editor/extensions/TypstSerializer.ts](src/components/tiptap-editor/extensions/TypstSerializer.ts)** - ProseMirror → Typst 序列化器

**节点扩展：**
5. **[src/components/tiptap-editor/extensions/nodes/TypstHeading.ts](src/components/tiptap-editor/extensions/nodes/TypstHeading.ts)** - 标题节点（`=`, `==`, `===` 等）
6. **[src/components/tiptap-editor/extensions/nodes/TypstParagraph.ts](src/components/tiptap-editor/extensions/nodes/TypstParagraph.ts)** - 段落节点
7. **[src/components/tiptap-editor/extensions/nodes/TypstCodeBlock.ts](src/components/tiptap-editor/extensions/nodes/TypstCodeBlock.ts)** - 代码块节点

**标记扩展：**
8. **[src/components/tiptap-editor/extensions/marks/TypstStrong.ts](src/components/tiptap-editor/extensions/marks/TypstStrong.ts)** - 粗体标记（`*text*`）
9. **[src/components/tiptap-editor/extensions/marks/TypstEmph.ts](src/components/tiptap-editor/extensions/marks/TypstEmph.ts)** - 斜体标记（`_text_`）
10. **[src/components/tiptap-editor/extensions/marks/TypstRaw.ts](src/components/tiptap-editor/extensions/marks/TypstRaw.ts)** - 行内代码标记

#### 需要修改的文件

1. **[src/pages/typst/TypstEditor.vue](src/pages/typst/TypstEditor.vue)** - 集成新扩展系统
2. **[src/store/store.ts](src/store/store.ts)** - 添加编辑器模式状态

#### 实现要点

**1. TypstHeading.ts 实现（参考 spec/editor.md）：**

```typescript
import { Node, mergeAttributes } from '@tiptap/core'
import { textInputRule } from '@tiptap/pm/inputrules'

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

  // 序列化到 Typst 源码
  addStorage() {
    return {
      toTypst: (node: any) => {
        const level = node.attrs.level
        const text = node.textContent
        return '='.repeat(level) + ' ' + text + '\n'
      }
    }
  },

  addCommands() {
    return {
      setHeading: (attrs) => ({ commands }) => {
        return commands.toggleNode('paragraph', 'heading', attrs)
      }
    }
  },

  addKeyboardShortcuts() {
    return {
      'Mod-Alt-1': () => this.editor.commands.setHeading({ level: 1 }),
      'Mod-Alt-2': () => this.editor.commands.setHeading({ level: 2 }),
      'Mod-Alt-3': () => this.editor.commands.setHeading({ level: 3 }),
    }
  },

  addInputRules() {
    return [
      // Typst 语法: '= ' → h1, '== ' → h2, etc.
      textInputRule({
        find: /^= $/,
        handler: ({ state, range }) => {
          const tr = state.tr.replaceWith(
            range.from,
            range.to,
            state.schema.nodes.heading.create({ level: 1 })
          )
          return tr
        }
      }),
      textInputRule({
        find: /^== $/,
        handler: ({ state, range }) => {
          const tr = state.tr.replaceWith(
            range.from,
            range.to,
            state.schema.nodes.heading.create({ level: 2 })
          )
          return tr
        }
      }),
      // ... 其他级别
    ]
  }
})
```

**2. TypstParser.ts 核心实现：**

```typescript
import { Node as ProseMirrorNode } from '@tiptap/pm/model'

export class TypstParser {
  private schema: any

  constructor(schema: any) {
    this.schema = schema
  }

  /**
   * 解析 Typst 源码为 ProseMirror 文档
   *
   * Phase 1: 使用正则表达式解析常见语法
   * Phase 2+: 考虑使用后端 AST 或 WASM 解析
   */
  parse(source: string): ProseMirrorNode {
    const lines = source.split('\n')
    const nodes: any[] = []

    for (const line of lines) {
      const node = this.parseLine(line)
      if (node) {
        nodes.push(node)
      }
    }

    return this.schema.node('doc', null, nodes)
  }

  private parseLine(line: string): any | null {
    if (!line.trim()) return null

    // 标题: =, ==, ===
    const headingMatch = line.match(/^(=+)\s+(.+)$/)
    if (headingMatch) {
      const level = headingMatch[1].length
      const text = headingMatch[2]
      return this.schema.node('heading',
        { level, typstSyntax: line },
        [this.schema.text(text)]
      )
    }

    // 粗体和斜体: *text*, _text_
    const textNodes = this.parseInlineFormatting(line)
    return this.schema.node('paragraph',
      { typstSyntax: line },
      textNodes.length > 0 ? textNodes : [this.schema.text(line)]
    )
  }

  private parseInlineFormatting(line: string): any[] {
    // 解析行内标记（strong, emph, raw）
    // 简化实现，实际需要更复杂的解析
    const nodes: any[] = []
    const regex = /\*([^*]+)\*/g
    let match
    let lastIndex = 0

    while ((match = regex.exec(line)) !== null) {
      // 添加前置文本
      if (match.index > lastIndex) {
        nodes.push(this.schema.text(line.slice(lastIndex, match.index)))
      }
      // 添加粗体文本
      nodes.push(this.schema.text(match[1], [this.schema.marks['strong'].create()]))
      lastIndex = match.index + match[0].length
    }

    if (lastIndex < line.length) {
      nodes.push(this.schema.text(line.slice(lastIndex)))
    }

    return nodes
  }
}
```

**3. TypstSerializer.ts 核心实现：**

```typescript
import { Node as ProseMirrorNode } from '@tiptap/pm/model'

export class TypstSerializer {
  /**
   * 将 ProseMirror 文档序列化为 Typst 源码
   *
   * 遍历文档树，调用每个节点的 toTypst() 方法
   */
  serialize(doc: ProseMirrorNode): string {
    const lines: string[] = []

    doc.descendants((node) => {
      if (node.type.name === 'heading') {
        const level = node.attrs.level
        const text = node.textContent
        lines.push('='.repeat(level) + ' ' + text)
      } else if (node.type.name === 'paragraph') {
        const text = this.serializeInlineContent(node)
        if (text.trim()) {
          lines.push(text)
        }
      } else if (node.type.name === 'codeBlock') {
        lines.push('```')
        lines.push(node.textContent)
        lines.push('```')
      }
    })

    return lines.join('\n\n')
  }

  private serializeInlineContent(node: ProseMirrorNode): string {
    let result = ''

    node.forEach((child) => {
      if (child.isText) {
        let text = child.text

        // 应用标记
        if (child.marks) {
          for (const mark of child.marks) {
            if (mark.type.name === 'strong') {
              text = `*${text}*`
            } else if (mark.type.name === 'em') {
              text = `_${text}_`
            } else if (mark.type.name === 'code') {
              text = `` `${text}` ``
            }
          }
        }

        result += text
      }
    })

    return result
  }
}
```

**4. TypstExtension.ts 基础扩展：**

```typescript
import { Extension } from '@tiptap/core'

export const TypstExtension = Extension.create({
  name: 'typstExtension',

  addOptions() {
    return {
      mode: 'typst' // 'markdown' | 'typst' | 'hybrid'
    }
  },

  addGlobalAttributes() {
    return [
      {
        types: ['heading', 'paragraph', 'codeBlock'],
        attributes: {
          typstSyntax: {
            default: null,
            parseHTML: element => element.getAttribute('data-typst-syntax'),
            renderHTML: attributes => {
              if (!attributes.typstSyntax) return {}
              return {
                'data-typst-syntax': attributes.typstSyntax,
              }
            },
          },
        },
      },
    ]
  },

  addCommands() {
    return {
      setTypstMode: (mode: 'markdown' | 'typst') => () => {
        this.options.mode = mode
        return true
      },
    }
  },
})
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

**目标**：实现 Typst 原生数学公式的编辑和渲染。

#### 技术方案（充分利用 Rust 后端）

**使用 `typst-render` 和 `typst-svg` crates：**

1. **行内公式**：使用 SVG 渲染（可缩放，文件小）
2. **块级公式**：使用 SVG 或 PNG 渲染
3. **后端统一渲染**：所有数学公式由后端渲染，前端只负责显示

**后端实现（Rust）：**

```rust
use typst_render::{render, RenderContext};
use typst_svg::SvgModule;
use typst_syntax::{parse, FileId, Source, VirtualPath};
use base64::{Engine as _, engine::general_purpose::STANDARD};

#[derive(Serialize, Deserialize)]
pub struct MathRenderResult {
    pub svg_data: String,  // base64 编码的 SVG
    pub width: f64,
    pub height: f64,
}

#[tauri::command]
pub async fn typst_compile_math(syntax: String, display_mode: bool) -> Result<MathRenderResult, String> {
    // 构造完整的 Typst 文档
    let full_source = if display_mode {
        format!("$#{}#$", syntax)
    } else {
        format!("#${}$#", syntax)
    };

    // 解析源码
    let id = FileId::new(None, VirtualPath::new("math.typ".to_string()));
    let source = Source::new(id, &full_source);
    let root = parse(&source);

    // 创建渲染上下文
    let mut ctx = RenderContext::new();

    // 添加 SVG 模块
    ctx.add::<SvgModule>();

    // 渲染
    let output = render(&root, &ctx).map_err(|e| e.to_string())?;

    // 获取 SVG 数据
    let svg_data = output.pages.first()
        .and_then(|page| page.frame().svg())
        .ok_or("Failed to render SVG")?;

    // 编码为 base64
    let svg_base64 = STANDARD.encode(svg_data);

    Ok(MathRenderResult {
        svg_data: svg_base64,
        width: output.pages.first().map(|p| p.width()).unwrap_or(0.0),
        height: output.pages.first().map(|p| p.height()).unwrap_or(0.0),
    })
}
```

**前端实现（Vue）：**

```typescript
// MathRenderer.vue
const renderMath = async (syntax: string, displayMode: boolean) => {
  const result = await invoke<MathRenderResult>('typst_compile_math', {
    syntax,
    displayMode
  })

  // 直接使用 SVG（可缩放，清晰）
  return `data:image/svg+xml;base64,${result.svg_data}`
}
```

**优势：**
- ✅ SVG 格式可缩放，不损失清晰度
- ✅ 文件比 PNG 小
- ✅ 可以选择文本（SVG 内嵌文本）
- ✅ 后端统一处理，前端简化

#### 需要创建的文件

1. **[src/components/tiptap-editor/extensions/nodes/TypstMath.ts](src/components/tiptap-editor/extensions/nodes/TypstMath.ts)** - 数学公式节点
2. **[src/components/math-renderer/MathNodeView.ts](src/components/math-renderer/MathNodeView.ts)** - 数学公式节点视图
3. **[src/components/math-renderer/MathRenderer.vue](src/components/math-renderer/MathRenderer.vue)** - 数学公式渲染组件

#### TypstMathInline.ts 实现

```typescript
import { Node, mergeAttributes } from '@tiptap/core'
import { textInputRule } from '@tiptap/pm/inputrules'
import { MathNodeView } from '../../math-renderer/MathNodeView'

export const TypstMathInline = Node.create({
  name: 'mathInline',
  group: 'inline',
  inline: true,
  atom: true,

  addAttributes() {
    return {
      typstSyntax: { default: '' }, // #$x^2$#
      imageData: { default: null }  // base64 PNG (缓存)
    }
  },

  parseDOM() {
    return [{
      tag: 'span[data-type="math-inline"]',
      getAttrs: node => ({
        typstSyntax: (node as HTMLElement).getAttribute('data-typst-syntax'),
        imageData: (node as HTMLElement).getAttribute('data-image-data')
      })
    }]
  },

  renderHTML({ HTMLAttributes }) {
    return ['span', mergeAttributes({
      'data-type': 'math-inline',
      class: 'typst-math-inline'
    }, HTMLAttributes), 0]
  },

  addNodeView() {
    return ({ node, view, getPos }) => new MathNodeView(node, view, getPos, false)
  },

  addInputRules() {
    return [
      // Typst 行内数学语法: $#...$#
      textInputRule({
        find: /#\$[^$]+\$/,
        handler: ({ state, range, match }) => {
          const syntax = match[0]
          const tr = state.tr.replaceWith(
            range.from,
            range.to,
            state.schema.nodes.mathInline.create({
              typstSyntax: syntax,
              imageData: null
            })
          )
          return tr
        }
      })
    ]
  }
})
```

#### TypstMathBlock.ts 实现

```typescript
import { Node, mergeAttributes } from '@tiptap/core'
import { textblockTypeInputRule } from '@tiptap/pm/inputrules'
import { MathNodeView } from '../../math-renderer/MathNodeView'

export const TypstMathBlock = Node.create({
  name: 'mathBlock',
  group: 'block',
  content: 'text*',
  marks: '',
  atom: true,

  addAttributes() {
    return {
      typstSyntax: { default: '' }, // $#\n...\n$#
      imageData: { default: null }  // base64 PNG (缓存)
    }
  },

  parseDOM() {
    return [{
      tag: 'div[data-type="math-block"]',
      getAttrs: node => ({
        typstSyntax: (node as HTMLElement).getAttribute('data-typst-syntax'),
        imageData: (node as HTMLElement).getAttribute('data-image-data')
      })
    }]
  },

  renderHTML({ HTMLAttributes }) {
    return ['div', mergeAttributes({
      'data-type': 'math-block',
      class: 'typst-math-block'
    }, HTMLAttributes), 0]
  },

  addNodeView() {
    return ({ node, view, getPos }) => new MathNodeView(node, view, getPos, true)
  },

  addInputRules() {
    return [
      // Typst 块级数学语法: $#\n...\n$#
      textblockTypeInputRule(
        /#\$$/,
        this.type,
        (match) => ({
          typstSyntax: match[0] || '#$'
        })
      )
    ]
  }
})
```

#### MathNodeView.ts 实现（Click-to-Edit, Blur-to-Render）

```typescript
import { NodeView } from '@tiptap/core'
import { Node as ProseMirrorNode } from '@tiptap/pm/model'
import { EditorView } from '@tiptap/pm/view'
import { invoke } from '@tauri-apps/api/core'
import { mathCache } from './MathCache'

export class MathNodeView implements NodeView {
  node: ProseMirrorNode
  view: EditorView
  getPos: boolean | (() => number)
  dom: HTMLElement
  contentDOM: HTMLElement | null
  isEditing: boolean
  isBlock: boolean

  constructor(node: ProseMirrorNode, view: EditorView, getPos: boolean | (() => number), isBlock: boolean) {
    this.node = node
    this.view = view
    this.getPos = getPos
    this.isBlock = isBlock
    this.isEditing = false

    // 创建 DOM 元素
    this.dom = document.createElement(isBlock ? 'div' : 'span')
    this.dom.className = isBlock ? 'typst-math-block' : 'typst-math-inline'
    this.contentDOM = null

    this.render()
  }

  update(node: ProseMirrorNode) {
    if (node.type !== this.node.type) return false
    this.node = node
    this.render()
    return true
  }

  private async render() {
    const syntax = this.node.attrs.typstSyntax

    if (this.isEditing) {
      // 编辑模式：显示可编辑的文本框
      this.showEditor()
    } else {
      // 渲染模式：显示编译后的数学公式
      await this.showRenderedMath(syntax)
    }
  }

  private showEditor() {
    const syntax = this.node.attrs.typstSyntax
    const textarea = document.createElement('textarea')
    textarea.value = syntax
    textarea.className = 'math-editor'
    textarea.spellcheck = false

    // 失去焦点时重新渲染
    textarea.addEventListener('blur', () => {
      this.updateSyntax(textarea.value)
      this.isEditing = false
      this.render()
    })

    // Esc 键退出编辑
    textarea.addEventListener('keydown', (e) => {
      if (e.key === 'Escape') {
        this.isEditing = false
        this.render()
      }
    })

    this.dom.innerHTML = ''
    this.dom.appendChild(textarea)
    textarea.focus()
  }

  private async showRenderedMath(syntax: string) {
    // 检查缓存
    const cached = mathCache.get(syntax)
    if (cached) {
      this.dom.innerHTML = `<img src="data:image/png;base64,${cached}" />`
      return
    }

    // 调用后端编译
    try {
      const imageData = await this.compileMath(syntax)

      // 缓存结果
      mathCache.set(syntax, imageData)

      this.dom.innerHTML = `<img src="data:image/png;base64,${imageData}" />`
    } catch (error) {
      this.dom.innerHTML = `<span class="math-error">渲染失败</span>`
    }
  }

  private async compileMath(syntax: string): Promise<string> {
    // 调用后端编译单个数学公式
    const result = await invoke<any>('typst_compile_math', {
      syntax,
      displayMode: this.isBlock
    })
    return result.image // base64 PNG
  }

  private updateSyntax(newSyntax: string) {
    if (typeof this.getPos === 'function') {
      const pos = this.getPos()
      const tr = this.view.state.tr.setNodeMarkup(pos, null, {
        ...this.node.attrs,
        typstSyntax: newSyntax,
        imageData: null // 清除缓存
      })
      this.view.dispatch(tr)
    }
  }

  selectNode() {
    this.dom.classList.add('ProseMirror-selectednode')
    this.isEditing = true
    this.render()
  }

  deselectNode() {
    this.dom.classList.remove('ProseMirror-selectednode')
    this.isEditing = false
    this.render()
  }

  ignoreMutation() {
    return true
  }

  destroy() {
    // 清理资源
  }
}
```

#### MathCache.ts 缓存实现

```typescript
interface MathCacheEntry {
  imageData: string
  timestamp: number
}

export class MathCache {
  private cache = new Map<string, MathCacheEntry>()
  private maxAge = 1000 * 60 * 60 // 1 小时
  private maxSize = 100 // 最多缓存 100 个公式

  get(syntax: string): string | null {
    const entry = this.cache.get(syntax)
    if (!entry) return null

    // 检查是否过期
    if (Date.now() - entry.timestamp > this.maxAge) {
      this.cache.delete(syntax)
      return null
    }

    return entry.imageData
  }

  set(syntax: string, imageData: string) {
    // 如果缓存满了，删除最旧的
    if (this.cache.size >= this.maxSize) {
      const oldest = Array.from(this.cache.entries())
        .sort((a, b) => a[1].timestamp - b[1].timestamp)[0]
      this.cache.delete(oldest[0])
    }

    this.cache.set(syntax, {
      imageData,
      timestamp: Date.now()
    })
  }

  clear() {
    this.cache.clear()
  }
}

export const mathCache = new MathCache()
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

**目标**：实现混合预览模式（WYSIWYG + 分栏预览可切换）。

#### 预览策略（参考 spec/editor.md）

根据 [spec/editor.md](spec/editor.md) 的讨论，有三种预览策略：

1. **直接渲染**：在编辑器内用 HTML+CSS 模拟 Typst 样式
   - 优点：易实现，响应快
   - 缺点：与最终 PDF 有差异

2. **集成 Typst 编译器**：使用后端编译生成 PDF/SVG
   - 优点：最准确
   - 缺点：需要性能优化

3. **混合模式**（选择）：编辑器简化样式 + 实时 PDF 预览
   - 优点：平衡准确性和性能
   - 支持 WYSIWYG 和分栏预览切换

#### 预览模式

```typescript
type PreviewMode = 'wysiwyg' | 'split' | 'preview'

// wysiwyg: 纯编辑模式，简化的 HTML 渲染
// split: 左右分栏，左侧编辑，右侧 PDF 预览
// preview: 纯预览模式，只显示编译结果
```

#### 需要创建的文件

1. **[src/components/preview/PreviewPanel.vue](src/components/preview/PreviewPanel.vue)** - 预览面板组件
2. **[src/components/preview/PreviewController.ts](src/components/preview/PreviewController.ts)** - 预览控制器
3. **[src/components/preview/WysiwygRenderer.ts](src/components/preview/WysiwygRenderer.ts)** - WYSIWYG 样式渲染器

#### 需要修改的文件

1. **[src/pages/typst/TypstEditor.vue](src/pages/typst/TypstEditor.vue)** - 添加预览面板和模式切换
2. **[src/store/store.ts](src/store/store.ts)** - 添加预览模式状态

#### 实现要点

**1. 状态管理（store.ts）：**

```typescript
const previewMode = ref<PreviewMode>('wysiwyg')
const isPreviewVisible = computed(() => previewMode.value !== 'wysiwyg')
const compilationResult = ref<any>(null) // 编译结果

const setPreviewMode = (mode: PreviewMode) => {
  previewMode.value = mode
  if (mode !== 'wysiwyg') {
    // 切换到预览模式时触发编译
    compileDocument()
  }
}
```

**2. PreviewPanel.vue 实现：**

```vue
<template>
  <div class="preview-panel" :class="{ visible: isVisible, split: mode === 'split' }">
    <div class="preview-header">
      <h3>预览</h3>
      <div class="preview-controls">
        <button @click="refresh" :disabled="isCompiling">
          {{ isCompiling ? '编译中...' : '刷新' }}
        </button>
        <button @click="exportPDF">导出 PDF</button>
        <button @click="zoomIn">放大</button>
        <button @click="zoomOut">缩小</button>
        <button @click="resetZoom">重置</button>
      </div>
    </div>

    <div class="preview-content" ref="contentRef">
      <!-- 编译错误显示 -->
      <div v-if="error" class="compilation-error">
        <h4>编译错误</h4>
        <pre>{{ error.message }}</pre>
      </div>

      <!-- 无内容 -->
      <div v-else-if="pages.length === 0" class="no-content">
        <p>暂无内容</p>
      </div>

      <!-- 页面渲染 -->
      <div v-else class="pages-container" :style="{ transform: `scale(${scale})` }">
        <div
          v-for="page in pages"
          :key="page.num"
          class="page"
          :style="{
            width: page.width + 'px',
            height: page.height + 'px'
          }"
          @click="handlePageClick(page, $event)"
        >
          <img
            :src="`data:image/png;base64,${page.image}`"
            :alt="`Page ${page.num}`"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { debounce } from '../../shared/util'

interface Page {
  num: number
  width: number
  height: number
  image: string
}

interface Props {
  mode: 'wysiwyg' | 'split' | 'preview'
  content: string
}

const props = defineProps<Props>()

const pages = ref<Page[]>([])
const isCompiling = ref(false)
const error = ref<any>(null)
const scale = ref(1)

const isVisible = computed(() => props.mode !== 'wysiwyg')
const contentRef = ref<HTMLElement>()

// 编译文档
const compileDocument = async () => {
  isCompiling.value = true
  error.value = null

  try {
    const result = await invoke<any>('typst_compile_doc', {
      path: getCurrentFilePath(),
      content: props.content
    })

    const [pageData, diagnostics] = result

    // 更新页面数据
    pages.value = pageData.map((page: any) => ({
      num: page.num,
      width: page.width,
      height: page.height,
      image: page.image // base64 PNG
    }))

    // 处理诊断信息
    if (diagnostics.length > 0) {
      // TODO: 显示编译错误和警告
    }
  } catch (err) {
    error.value = err
    pages.value = []
  } finally {
    isCompiling.value = false
  }
}

// 防抖编译
const debouncedCompile = debounce(compileDocument, 1000)

// 刷新预览
const refresh = () => {
  compileDocument()
}

// 缩放控制
const zoomIn = () => { scale.value = Math.min(scale.value + 0.1, 3) }
const zoomOut = () => { scale.value = Math.max(scale.value - 0.1, 0.3) }
const resetZoom = () => { scale.value = 1 }

// 导出 PDF
const exportPDF = async () => {
  try {
    const outputPath = await invoke<string>('export_pdf', {
      path: getCurrentFilePath(),
      output: getCurrentFilePath().replace('.typ', '.pdf')
    })
    console.log('PDF 导出成功:', outputPath)
  } catch (err) {
    console.error('PDF 导出失败:', err)
  }
}

// 点击页面定位（TODO: 实现双向映射）
const handlePageClick = (page: Page, event: MouseEvent) => {
  console.log('点击页面', page.num, '位置', event.offsetX, event.offsetY)
  // 未来可以通过后端返回的位置信息定位到源码
}

// 监听内容变化
watch(() => props.content, () => {
  if (isVisible.value) {
    debouncedCompile()
  }
})

// 监听模式变化
watch(() => props.mode, (newMode) => {
  if (newMode !== 'wysiwyg') {
    compileDocument()
  }
})

defineExpose({
  refresh,
  compile: compileDocument
})
</script>

<style scoped>
.preview-panel {
  position: fixed;
  right: 0;
  top: 0;
  bottom: 0;
  background: #f5f5f5;
  border-left: 1px solid #ddd;
  overflow-y: auto;
  transform: translateX(100%);
  transition: transform 0.3s ease;
  z-index: 100;
}

.preview-panel.visible {
  transform: translateX(0);
}

.preview-panel.split {
  width: 50%;
}

.preview-header {
  position: sticky;
  top: 0;
  background: white;
  padding: 12px 16px;
  border-bottom: 1px solid #ddd;
  display: flex;
  justify-content: space-between;
  align-items: center;
  z-index: 10;
}

.preview-controls {
  display: flex;
  gap: 8px;
}

.pages-container {
  padding: 20px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
  transform-origin: top center;
  transition: transform 0.2s ease;
}

.page {
  background: white;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  cursor: pointer;
}

.page img {
  display: block;
  width: 100%;
  height: auto;
}

.compilation-error {
  background: #ffebee;
  border: 1px solid #f44336;
  border-radius: 4px;
  padding: 16px;
  margin: 20px;
}

.no-content {
  text-align: center;
  color: #999;
  padding: 40px;
}
</style>
```

**3. WYSIWYG 样式渲染器：**

```typescript
// WysiwygRenderer.ts
// 为编辑器内的内容提供 Typst 样式

export const typstStyles = `
  /* 标题样式 */
  .typst-heading {
    font-weight: 600;
    margin-top: 1em;
    margin-bottom: 0.5em;
  }

  .typst-h1 { font-size: 2em; }
  .typst-h2 { font-size: 1.5em; }
  .typst-h3 { font-size: 1.25em; }

  /* 数学公式样式 */
  .typst-math-inline {
    display: inline-block;
    vertical-align: middle;
    margin: 0 4px;
    cursor: pointer;
  }

  .typst-math-block {
    display: block;
    text-align: center;
    margin: 1em 0;
    cursor: pointer;
  }

  .typst-math-inline:hover,
  .typst-math-block:hover {
    background-color: rgba(0, 0, 0, 0.05);
    border-radius: 4px;
  }

  /* 代码块样式 */
  .typst-code-block {
    background: #f5f5f5;
    border: 1px solid #ddd;
    border-radius: 4px;
    padding: 12px;
    font-family: 'Fira Code', monospace;
    white-space: pre-wrap;
  }

  /* 列表样式 */
  .typst-bullet-list {
    list-style-type: disc;
    padding-left: 2em;
  }

  .typst-ordered-list {
    list-style-type: decimal;
    padding-left: 2em;
  }
`
```

**4. 后端已支持：**

- `typst_compile_doc` 已实现 PNG 渲染
- 返回 `PageData[]` 包含 base64 编码的图片
- 只需在前端显示这些图片
- 未来可考虑添加位置信息（点击预览定位源码）

#### 测试验证

- [ ] 切换到分栏模式显示预览
- [ ] 编辑内容时预览自动更新（防抖）
- [ ] 手动刷新预览
- [ ] 导出 PDF 功能
- [ ] 编译错误正确显示

---

### Phase 5: 智能编辑功能（3-4天）

**目标**：实现自动补全、语法诊断、错误提示等。

#### 技术方案（使用 `typst-ide` crate）

**`typst-ide` 提供的功能：**
- 自动补全（autocomplete）
- 语法诊断（diagnostics）
- 代码跳转（go to definition）
- 悬停信息（hover information）

**后端实现（Rust）：**

```rust
use typst_ide::{autocomplete, complete, Completion};
use typst_syntax::{FileId, Source, VirtualPath};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TypstCompletion {
    pub label: String,
    pub apply: String,
    pub detail: Option<String>,
    pub kind: String,  // "function", "variable", "constant", etc.
}

#[tauri::command]
pub async fn typst_autocomplete(
    source: String,
    cursor_position: usize,
) -> Result<Vec<TypstCompletion>, String> {
    let id = FileId::new(None, VirtualPath::new("main.typ".to_string()));
    let source = Source::new(id, &source);

    // 获取世界（包含所有依赖和标准库）
    let world = create_world(); // 需要实现

    // 计算自动补全
    let completions = autocomplete(&world, &source, cursor_position)
        .map_err(|e| e.to_string())?;

    // 转换为前端格式
    let result: Vec<TypstCompletion> = completions.into_iter()
        .map(|comp| TypstCompletion {
            label: comp.label.to_string(),
            apply: comp.apply.map(|s| s.to_string()).unwrap_or_default(),
            detail: comp.detail.map(|s| s.to_string()),
            kind: format!("{:?}", comp.kind),
        })
        .collect();

    Ok(result)
}

#[derive(Serialize, Deserialize)]
pub struct TypstDiagnostic {
    pub message: String,
    pub severity: String,  // "error" | "warning"
    pub range: [usize; 2],  // [start, end]
    pub hints: Vec<String>,
}

#[tauri::command]
pub async fn typst_diagnostics(source: String) -> Result<Vec<TypstDiagnostic>, String> {
    let id = FileId::new(None, VirtualPath::new("main.typ".to_string()));
    let source = Source::new(id, &source);

    // 创建世界
    let world = create_world();

    // 编译并获取诊断信息
    let _result = typst::compile(&world, &source)
        .map_err(|e| e.to_string())?;

    // 获取诊断信息（从编译结果中）
    // ...

    Ok(diagnostics)
}
```

**前端实现（Vue）：**

```vue
<!-- TypstAutocomplete.vue -->
<template>
  <div v-if="isVisible" class="autocomplete-popup" :style="position">
    <div
      v-for="(item, index) in completions"
      :key="index"
      class="autocomplete-item"
      :class="{ active: index === selectedIndex }"
      @click="selectItem(item)"
      @mouseenter="selectedIndex = index"
    >
      <div class="item-label">{{ item.label }}</div>
      <div v-if="item.detail" class="item-detail">{{ item.detail }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface CompletionItem {
  label: string
  apply: string
  detail?: string
  kind: string
}

const isVisible = ref(false)
const completions = ref<CompletionItem[]>([])
const selectedIndex = ref(0)

// 监听编辑器输入
watch(() => props.content, async (newContent, oldContent) => {
  // 获取光标位置
  const cursorPos = getCursorPosition()

  // 触发自动补全
  const result = await invoke<CompletionItem[]>('typst_autocomplete', {
    source: newContent,
    cursorPosition: cursorPos
  })

  completions.value = result
  isVisible.value = result.length > 0
})

const selectItem = (item: CompletionItem) => {
  // 应用补全
  applyCompletion(item.apply)
  isVisible.value = false
}
</script>
```

**实时诊断：**

```typescript
// 监听文档变化，实时获取诊断信息
const updateDiagnostics = debounce(async (source: string) => {
  const diagnostics = await invoke<TypstDiagnostic[]>('typst_diagnostics', {
    source
  })

  // 在编辑器中标记错误和警告
  showDiagnostics(diagnostics)
}, 500)
```

**优势：**
- ✅ 使用官方 `typst-ide` 实现，补全准确
- ✅ 后端统一处理，前端只负责显示
- ✅ 支持所有 Typst 标准库和自定义函数

#### 需要创建的文件

1. **[src/components/autocomplete/TypstAutocomplete.vue](src/components/autocomplete/TypstAutocomplete.vue)** - 自动补全 UI
2. **[src/components/diagnostics/DiagnosticsPanel.ts](src/components/diagnostics/DiagnosticsPanel.ts)** - 诊断信息面板

#### 需要修改的文件

1. **[src-tauri/src/cmds/typst.rs](src-tauri/src/cmds/typst.rs)** - 实现 `typst_autocomplete` 和 `typst_diagnostics`

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

## 技术挑战与风险（参考 spec/editor.md）

根据 [spec/editor.md](spec/editor.md) 的分析，本项目面临以下技术挑战：

### 1. Typst 可编程性的表示

**挑战**：Typst 具有强大的可编程性（如 `#let` 定义变量、函数调用、条件判断），如何在 ProseMirror 模型中表示这些动态内容？

**解决方案**：
- 将编程代码保留为"原始代码节点"（Raw Code Node）
- 不尝试执行或求值这些代码
- 直接序列化为 Typst 源码
- 在预览时由后端 Typst 编译器处理

**示例**：
```typst
#let x = 5
# if x > 3 [This is shown]
```

ProseMirror 表示：
```json
{
  "type": "paragraph",
  "content": [
    {
      "type": "text",
      "text": "#let x = 5\n#if x > 3 [This is shown]"
    }
  ]
}
```

### 2. 性能问题

**挑战**：每次编辑都重新编译整个文档可能较慢，特别是大文档和大量数学公式。

**解决方案**：
- **防抖（Debounce）**：编辑后延迟 1 秒再编译
- **缓存机制**：
  - 数学公式缓存（相同公式不重复编译）
  - 文档 AST 缓存（增量编译）
- **增量编译**：只重新编译变化的部分（长期目标）
- **虚拟滚动**：大文档使用虚拟滚动
- **Web Worker**：将编译任务移到 Worker 线程（前端解析）

### 3. 双向映射的复杂性

**挑战**：需要将 ProseMirror 的事务与源码变更同步，确保撤销/重做等操作同时作用于模型和源码。

**解决方案**：
- **单一数据源**：ProseMirror 文档是唯一的数据源
- **实时序列化**：保存时序列化为 Typst 源码
- **撤销/重做**：使用 ProseMirror 内置的历史管理
- **避免双向绑定**：不维护源码和模型的实时同步

### 4. 解析准确性

**挑战**：正则表达式解析可能无法处理复杂嵌套和边缘情况。

**解决方案**：
- **Phase 1**：正则表达式解析（快速实现，覆盖 80% 场景）
- **Phase 2+**：逐步增强解析器
- **长期方案**：
  - 方案 A：使用后端 Typst 编译器返回 AST
  - 方案 B：集成 typst.ts WASM 解析器
  - 方案 C：实现完整的 JavaScript 解析器

### 5. Markdown 兼容性

**挑战**：新扩展可能影响现有 Markdown 功能。

**解决方案**：
- 通过文件扩展名选择不同解析器
- `.md` 文件使用 Markdown 扩展
- `.typ` 文件使用 Typst 扩展
- 保留独立的 Markdown 编辑模式

## 风险缓解措施

### 技术风险

| 风险 | 影响 | 概率 | 缓解措施 |
|------|------|------|---------|
| **Typst 数学公式渲染性能** | 高 | 中 | 缓存机制、防抖、考虑 SVG 格式 |
| **双向转换准确性** | 高 | 中 | 分阶段实施，优先处理常见场景 |
| **大文档性能** | 高 | 中 | 虚拟滚动、增量渲染、Web Worker |
| **可编程性表示** | 中 | 低 | 保留原始代码，不尝试求值 |
| **解析器边缘情况** | 中 | 高 | 充分测试、错误提示、降级到文本 |

### 兼容性风险

| 风险 | 影响 | 概率 | 缓解措施 |
|------|------|------|---------|
| **Markdown 模式破坏** | 高 | 低 | 文件扩展名隔离、独立模式 |
| **状态管理复杂度** | 高 | 中 | 清晰的状态管理、使用 Pinia store |
| **浏览器兼容性** | 中 | 低 | 现代浏览器支持、Polyfill |

### 开发风险

| 风险 | 影响 | 概率 | 缓解措施 |
|------|------|------|---------|
| **开发周期过长** | 中 | 中 | 分阶段实施、每个阶段可独立交付 |
| **技术选型错误** | 高 | 低 | 技术验证、原型测试、灵活调整 |
| **团队技能不足** | 中 | 低 | 文档完善、代码示例、知识分享 |

---

## 预估时间

| 阶段 | 时间 | 依赖 | 关键里程碑 |
|------|------|------|-----------|
| Phase 1: 核心扩展系统 | 3-5天 | 无 | 基础 Typst 语法可编辑、保存 |
| Phase 2: 数学公式系统 | 5-7天 | Phase 1 | 数学公式可渲染、编辑 |
| Phase 3: 增强语法支持 | 4-6天 | Phase 1 | 列表、表格、代码块支持 |
| Phase 4: 实时预览 | 4-5天 | Phase 1, Phase 2 | 分栏预览、PDF 导出 |
| Phase 5: 智能编辑 | 3-4天 | Phase 1 | 自动补全、语法高亮 |
| **总计** | **19-27天** | | **约 3-4 周** |

**建议**：按顺序实施，每个阶段完成后进行测试验证，确保质量后再进入下一阶段。

## 总结

### 核心价值

本计划充分利用 **Tauri + Rust 后端** 和 **完整的 Typst crates 生态系统**，将 Typster 从基础 Markdown 编辑器改造为完整的 Typst WYSIWYG 编辑器：

1. **精确的 AST 映射**：ProseMirror Schema 精确映射 Typst 文档结构
2. **前后端分工**：前端专注编辑器交互，后端处理所有 Typst 特定逻辑
3. **原生数学公式**：使用 `typst-render` + `typst-svg` 渲染 Typst 数学语法
4. **混合预览模式**：WYSIWYG 编辑 + 实时 SVG/PDF 预览
5. **智能编辑功能**：使用 `typst-ide` 实现自动补全、诊断、跳转等

### 技术亮点

**充分利用 Rust 后端：**
- ✅ **typst-syntax**：官方解析器，准确解析 Typst 源码
- ✅ **typst-ide**：官方 IDE 功能，自动补全、诊断
- ✅ **typst-render**：官方渲染引擎，高质量输出
- ✅ **typst-svg**：SVG 格式，可缩放，文件小
- ✅ **typst-pdf**：官方 PDF 导出，完美兼容

**前端简化：**
- ✅ ProseMirror 管理文档模型
- ✅ Tiptap 提供扩展架构
- ✅ Vue 3 处理 UI 交互
- ✅ IPC 与后端高效通信

**性能优化：**
- ✅ SVG 格式（可缩放，不损失清晰度）
- ✅ 缓存机制（避免重复渲染）
- ✅ 防抖编译（减少不必要的后端调用）
- ✅ 增量渲染（只渲染变化部分）

### 下一步行动

1. **Phase 1 启动**：创建扩展系统基础设施
2. **后端命令开发**：实现 `typst_parse_source`, `typst_compile_math` 等 IPC 命令
3. **前端集成**：将后端返回的数据集成到 Tiptap 编辑器
4. **技术验证**：验证 SVG 渲染、自动补全等核心功能
5. **用户测试**：尽早收集用户反馈，迭代优化
