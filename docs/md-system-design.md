# Typster Markdown 编辑器技术架构设计

## 项目概述

**目标**：一比一复刻 Typora，打造极致的 Markdown WYSIWYG 编辑体验

**技术栈**：
- **前端**：Vue 3 + TypeScript + Tiptap 3.13.0 + PrimeVue
- **后端**：Tauri 2.0 + Rust（预留 Typst 未来扩展）
- **图表渲染**：Mermaid.js（Phase 3）
- **代码高亮**：Shiki（Phase 2）
- **数学公式**：暂不实施（留待未来版本）

---

## 核心设计理念

### 0. 开发优先级

**实施顺序**（从高到低）

0. **P0**: 编辑器基础功能（编辑区布局，状态栏，侧边栏，设置页面，自动更新app）← **新增，最优先**
1. **P0**: Markdown 语法完善（表格、脚注、YAML）
2. **P0**: 代码高亮和表格增强（Shiki）
3. **P1**: Mermaid 图表支持
4. **P1**: 高级功能（Focus Mode、大纲、搜索等）
5. **P1**: 导出和主题系统
6. **暂不实施**: 数学公式支持（KaTeX）

**注**：数学公式功能留待未来版本考虑

### 0.1 编辑器基础功能架构（Phase 0 - 新增）

**目标**：构建完整的 Markdown 编辑器基础框架，包括 UI 布局、设置系统、自动保存等核心功能。

#### 0.1.1 整体布局架构

```
┌─────────────────────────────────────────────────────────────┐
│                      TitleBar (32px)                        │
│  [☰ 文件树]                                    [设置⚙️] [关闭×] │
├─────────────┬───────────────────────────────────────────────┤
│             │                                               │
│  Sidebar    │            Editor Area                        │
│  (250px)    │            (flexible)                         │
│  [文件/TOC] │            [Tiptap 编辑区]                     │
│             │                                               │
│             │                                               │
├─────────────┴───────────────────────────────────────────────┤
│                      StatusBar (24px)                       │
│  文件路径 | 保存状态 | 字数 | 行：列 | 主题                │
└─────────────────────────────────────────────────────────────┘
```

#### 0.1.2 现有组件状态

| 组件 | 文件路径 | 状态 | 说明 |
|------|----------|------|------|
| 主布局 | `src/pages/home/Home.vue` | ✅ | Grid 布局容器 |
| 标题栏 | `src/pages/home/TitleBar.vue` | ✅ | 顶部工具栏 |
| 状态栏 | `src/pages/home/StatusBar.vue` | ✅ | 底部状态信息 |
| 侧边栏 | `src/pages/home/Sidebar.vue` | ✅ | 文件树 + TOC |
| 编辑器 | `src/pages/typst/TypstEditor.vue` | ✅ | Tiptap 编辑器 |
| 设置页面 | - | ❌ | **待创建** |
| 自动更新 | - | ❌ | **待创建** |

#### 0.1.3 设置页面设计

**文件结构**：
```
src/
├── pages/settings/
│   ├── Settings.vue           # 设置主页面（容器）
│   ├── EditorSettings.vue     # 编辑器设置
│   ├── AppearanceSettings.vue # 外观设置
│   └── AutosaveSettings.vue   # 自动保存设置
├── store/settings-store.ts    # 设置状态管理
└── composables/
    └── useSettings.ts         # 设置逻辑 Hook
```

**设置项设计**：
| 类别 | 设置项 | 类型 | 默认值 |
|------|--------|------|--------|
| **编辑器** | 字体大小 | number | 16 |
| | 字体家族 | string | 系统默认 |
| | 行高 | number | 1.6 |
| | 自动配对括号 | boolean | true |
| **外观** | 主题模式 | 'light'/'dark'/'auto' | 'auto' |
| | 主题预设 | string | 'github-light' |
| | 编辑器宽度 | number | 800 |
| **自动保存** | 启用自动保存 | boolean | true |
| | 保存间隔 (秒) | number | 60 |

#### 0.1.4 自动更新机制

**Tauri Updater 配置**：
```json
// src-tauri/tauri.conf.json
{
  "plugins": {
    "updater": {
      "active": true,
      "dialog": true,
      "endpoints": ["https://github.com/your-org/typster/releases/latest/download/latest.json"],
      "pubkey": "your-public-key"
    }
  }
}
```

**更新检查逻辑**：
```typescript
// src/composables/useAutoUpdate.ts
import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/api/process'

export function useAutoUpdate() {
  const checkForUpdates = async () => {
    const update = await check()
    if (update?.available) {
      await update.downloadAndInstall()
      await relaunch()
    }
  }
  return { checkForUpdates }
}
```

### 1. 完全 WYSIWYG 体验

**设计原则**：
- 无源码模式/预览模式切换
- 所见即所得，实时渲染
- 最小化编辑干扰

**实现方式**：
```
用户输入 → ProseMirror 输入规则 → 即时转换 → 渲染节点
                                    ↓
                            用户看不到原始 Markdown 语法
```

### 2. 扩展性优先

**为未来 Typst 支持预留接口**：

```typescript
// 架构分层
Editor Layer (Tiptap)
    ↓
Format Abstraction Layer (抽象层)
    ├── MarkdownFormat (当前实现)
    └── TypstFormat (未来扩展)
    ↓
Renderer Layer
    ├── MarkdownRenderer
    └── TypstRenderer (未来)
```

### 3. 性能优化

**策略**：
- 虚拟滚动（大文档支持）
- 增量渲染
- 智能缓存
- Web Worker 处理复杂渲染

---

## 整体架构

### 系统分层架构

```
┌─────────────────────────────────────────────────────────────┐
│                     用户界面层 (UI Layer)                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │ 编辑器主界面  │  │ 侧边栏       │  │ 状态栏/标题栏     │  │
│  └──────────────┘  └──────────────┘  └──────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                   编辑器核心层 (Editor Core)                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │ Tiptap 核心  │  │ 自定义扩展    │  │ NodeView 系统    │  │
│  └──────────────┘  └──────────────┘  └──────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                 格式抽象层 (Format Abstraction)               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │ Markdown 解析 │  │ 序列化器      │  │ 格式转换器       │  │
│  └──────────────┘  └──────────────┘  └──────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                   渲染服务层 (Renderer Layer)                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │ Mermaid 渲染 │  │ 代码高亮渲染 │  │ (未来: Typst渲染) │  │
│  └──────────────┘  └──────────────┘  └──────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                   后端服务层 (Backend Services)               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │ 文件系统操作 │  │ PDF 导出     │  │ 配置管理         │  │
│  └──────────────┘  └──────────────┘  └──────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

---

## 核心子系统设计

### 1. Tiptap 扩展系统

#### 扩展架构

```typescript
// 核心扩展配置
const extensions = [
  // === 基础扩展 ===
  StarterKit,                    // Tiptap 基础功能包
  Markdown,                      // Markdown 序列化支持

  // === Markdown 语法扩展 ===
  MarkdownShortcuts,             // Markdown 快捷输入
  TableExtension,                // 表格支持
  TaskListExtension,             // 任务列表
  FootnoteExtension,             // 脚注支持
  YAMLFrontMatter,              // YAML 前言

  // === 图表扩展 ===
  MermaidExtension,              // Mermaid 图表 (Phase 3)

  // === 注: 数学公式支持暂不实施 ===
  // MathExtension,              // 未来版本: 数学公式（KaTeX）

  // === 编辑器功能扩展 ===
  TableOfContents,               // 目录生成
  CodeBlockHighlight,            // 代码块高亮
  AutoLink,                      // 自动链接
  ImageUpload,                   // 图片上传

  // === 高级功能扩展 ===
  FocusMode,                     // 专注模式
  TypewriterMode,                // 打字机模式
  WordCount,                     // 字数统计
  SearchAndReplace,              // 搜索替换

  // === Typst 预留扩展 ===
  // TypstSyntax,                // 未来 Typst 语法支持
  // TypstRenderer,              // 未来 Typst 渲染
]
```

#### 自定义扩展设计模式

```typescript
// 扩展基类设计
interface BaseExtensionConfig {
  name: string
  priority: number
  dependencies?: string[]
}

abstract class TypstExtension extends Extension {
  // 为未来 Typst 支持预留的方法
  abstract toMarkdown(): string
  abstract toTypst?(): string  // 未来实现
  abstract parseMarkdown(content: string): Node
  abstract parseTypst?(content: string): Node  // 未来实现
}
```

### 2. Mermaid 图表系统

#### 组件架构

```
MermaidExtension
    ├── MermaidNode (ProseMirror 节点)
    │   ├── attributes: code (图表代码)
    │   └── parseHTML: ```mermaid ... ```
    │
    ├── MermaidNodeView (NodeView)
    │   ├── 编辑模式：显示代码编辑器
    │   ├── 渲染模式：显示 SVG 图表
    │   └── 错误处理：显示错误信息
    │
    └── MermaidRenderer.vue (渲染组件)
        ├── mermaid.render() API
        ├── 主题适配
        └── 响应式缩放
```

#### 支持的 Mermaid 图表类型

```typescript
enum MermaidDiagramType {
  Flowchart = 'flowchart',           // 流程图
  Sequence = 'sequenceDiagram',       // 时序图
  Class = 'classDiagram',            // 类图
  State = 'stateDiagram',            // 状态图
  Entity = 'entityRelationshipDiagram', // 实体关系图
  Gantt = 'gantt',                   // 甘特图
  Pie = 'pie',                       // 饼图
  Mindmap = 'mindmap',               // 思维导图
  ER = 'erDiagram',                  // ER 图
  Journey = 'journey',               // 用户旅程图
}
```

#### Mermaid 集成方式

**依赖安装**：
```bash
pnpm add mermaid
```

**核心实现**：
```typescript
import mermaid from 'mermaid'

// 初始化 Mermaid
mermaid.initialize({
  startOnLoad: false,
  theme: 'default',
  securityLevel: 'loose',
})

// 渲染图表
const renderMermaid = async (code: string, id: string) => {
  const { svg } = await mermaid.render(id, code)
  return svg
}
```

### 3. 代码块高亮系统

#### 技术选型：Shiki

**选择理由**：
- 使用 TextMate 语法（与 VS Code 一致）
- 更准确的高亮
- 更好的性能
- 支持更多语言

#### 架构设计

```typescript
// Shiki 集成
import { getHighlighter } from 'shiki'

const highlighter = await getHighlighter({
  themes: ['github-light', 'github-dark'],
  langs: [
    'javascript', 'typescript', 'python', 'rust', 'go',
    'java', 'cpp', 'c', 'bash', 'json', 'yaml', 'markdown'
  ],
})

// 代码块扩展
class CodeBlockExtension extends Extension {
  addNodeView() {
    return ({ node }) => {
      const code = node.textContent
      const lang = node.attrs.lang
      const html = highlighter.codeToHtml(code, {
        lang: lang || 'text',
        theme: 'github-light'
      })
      return new CodeBlockNodeView(html)
    }
  }
}
```

### 4. 输入规则系统

#### Typora 风格输入规则

```typescript
const inputRules = [
  // === 标题 ===
  textblockTypeInputRule(/^# $/, heading, { level: 1 }),
  textblockTypeInputRule(/^## $/, heading, { level: 2 }),
  textblockTypeInputRule(/^### $/, heading, { level: 3 }),
  textblockTypeInputRule(/^#### $/, heading, { level: 4 }),
  textblockTypeInputRule(/^##### $/, heading, { level: 5 }),
  textblockTypeInputRule(/^###### $$/, heading, { level: 6 }),

  // === 文本格式 ===
  wrappingInputRule(/^\*\*([^*]+)\*\*$/, bold),
  wrappingInputRule(/^\*([^*]+)\*$/, italic),
  wrappingInputRule(/^~~([^~]+)~~$/, strike),

  // === 代码 ===
  textblockTypeInputRule(/^```$/, codeBlock),

  // === 引用 ===
  wrappingInputRule(/^>\s$/, blockquote),

  // === 列表 ===
  wrappingInputRule(/^\d+\.\s$/, orderedList),
  wrappingInputRule(/^[\-\*]\s$/, bulletList),
  wrappingInputRule(/^\[ \]$/, taskList),    // 未完成
  wrappingInputRule(/^\[x\]$/, taskList),    // 已完成

  // === 水平线 ===
  textblockTypeInputRule(/^---$/, horizontalRule),
  textblockTypeInputRule(/^\*\*\*$/, horizontalRule),

  // === Mermaid 图表 ===
  textblockTypeInputRule(/^```mermaid$/, mermaidBlock),

  // === 数学公式（低优先级） ===
  inputRule(/\$([^$]+)\$/, mathInline),
  inputRule(/\$\$([^$]+)\$\$/, mathDisplay),
]
```

### 5. 表格系统

#### 表格编辑器架构

```
TableExtension
    ├── TableNode (表格节点)
    ├── TableRowNode (行节点)
    ├── TableHeaderNode (表头节点)
    └── TableCellNode (单元格节点)
        └── TableCellNodeView
            ├── 渲染模式：显示表格
            ├── 编辑模式：可编辑单元格
            └── 快捷操作：插入行/列，删除等
```

#### Typora 风格表格交互

```typescript
// 表格快捷操作
interface TableActions {
  insertRowAbove: () => void
  insertRowBelow: () => void
  insertColumnLeft: () => void
  insertColumnRight: () => void
  deleteRow: () => void
  deleteColumn: () => void
  toggleHeader: () => void
  alignLeft: () => void
  alignCenter: () => void
  alignRight: () => void
}
```

### 6. 大纲和目录系统

#### TOC 自动生成

```typescript
// 目录扩展示例
class TableOfContentsExtension extends Extension {
  constructor() {
    super({
      name: 'tableOfContents',
      anchorTypes: ['heading'],
      onUpdate: (content) => {
        const headings = extractHeadings(content)
        const toc = buildTOC(headings)
        EventBus.emit('toc-update', toc)
      },
    })
  }
}

// TOC 数据结构
interface TOCItem {
  id: string
  level: number
  text: string
  children: TOCItem[]
}
```

#### 大纲面板

```vue
<!-- OutlinePanel.vue -->
<template>
  <div class="outline-panel">
    <Tree :value="tocTree" @node-select="scrollToHeading" />
  </div>
</template>

<script setup lang="ts">
const scrollToHeading = (id: string) => {
  const element = document.querySelector(`[data-id="${id}"]`)
  element?.scrollIntoView({ behavior: 'smooth' })
}
</script>
```

### 7. 主题系统（CSS Design Tokens）

#### 主题架构

使用 CSS 原生变量实现主题系统，无需 JavaScript 运行时切换。

**CSS Design Tokens 定义**：

```css
/* src/styles/tokens/base.css */
:root {
  /* === 基础色彩 === */
  --color-bg-primary: #ffffff;
  --color-bg-secondary: #f6f8fa;
  --color-bg-tertiary: #f0f2f5;

  --color-fg-primary: #24292f;
  --color-fg-secondary: #57606a;
  --color-fg-tertiary: #8b949e;

  --color-accent-primary: #0969da;
  --color-accent-secondary: #1f6feb;
  --color-accent-hover: #0756b3;

  --color-border: #d0d7de;
  --color-border-light: #e1e4e8;

  --color-success: #1a7f37;
  --color-warning: #9a6700;
  --color-error: #cf222e;
  --color-info: #0969da;

  /* === 字体 === */
  --font-family-base: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  --font-family-code: "Monaco", "Menlo", "Consolas", monospace;

  --font-size-base: 16px;
  --font-size-small: 14px;
  --font-size-large: 18px;

  --line-height-base: 1.6;
  --line-height-tight: 1.4;
  --line-height-loose: 1.8;

  /* === 间距 === */
  --spacing-xs: 4px;
  --spacing-sm: 8px;
  --spacing-md: 16px;
  --spacing-lg: 24px;
  --spacing-xl: 32px;

  /* === 圆角 === */
  --radius-sm: 3px;
  --radius-md: 6px;
  --radius-lg: 8px;

  /* === 阴影 === */
  --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.05);
  --shadow-md: 0 4px 6px rgba(0, 0, 0, 0.07);
  --shadow-lg: 0 10px 15px rgba(0, 0, 0, 0.1);

  /* === Z-Index === */
  --z-dropdown: 1000;
  --z-modal: 1100;
  --z-tooltip: 1200;

  /* === 编辑器特定 === */
  --editor-width: 800px;
  --editor-max-width: 90%;
  --editor-padding: 20px;

  /* === 代码块 === */
  --code-bg: #f6f8fa;
  --code-border: #d0d7de;

  /* === 表格 === */
  --table-border: #d0d7de;
  --table-header-bg: #f6f8fa;
  --table-hover-bg: #f9f9f9;

  /* === Mermaid 图表 === */
  --mermaid-bg: #f8f8f8;
  --mermaid-border: #e0e0e0;
}
```

**主题预设**：

```css
/* src/styles/tokens/github-light.css */
:root {
  --color-bg-primary: #ffffff;
  --color-bg-secondary: #f6f8fa;
  --color-fg-primary: #24292f;
  --color-fg-secondary: #57606a;
  --color-accent-primary: #0969da;
}

/* src/styles/tokens/github-dark.css */
:root {
  --color-bg-primary: #0d1117;
  --color-bg-secondary: #161b22;
  --color-fg-primary: #c9d1d9;
  --color-fg-secondary: #8b949e;
  --color-accent-primary: #58a6ff;
  --color-border: #30363d;
}

/* src/styles/tokens/nord.css */
:root {
  --color-bg-primary: #eceff4;
  --color-bg-secondary: #e5e9f0;
  --color-fg-primary: #2e3440;
  --color-fg-secondary: #4c566a;
  --color-accent-primary: #5e81ac;
}

/* src/styles/tokens/dracula.css */
:root {
  --color-bg-primary: #282a36;
  --color-bg-secondary: #21222c;
  --color-fg-primary: #f8f8f2;
  --color-fg-secondary: #6272a4;
  --color-accent-primary: #bd93f9;
}
```

**主题切换实现**：

```typescript
// src/composables/useTheme.ts
export function useTheme() {
  const themes = [
    { name: 'GitHub Light', file: 'github-light.css', type: 'light' },
    { name: 'GitHub Dark', file: 'github-dark.css', type: 'dark' },
    { name: 'Nord', file: 'nord.css', type: 'dark' },
    { name: 'Dracula', file: 'dracula.css', type: 'dark' },
  ]

  const currentTheme = ref(themes[0])

  const applyTheme = (theme: typeof themes[0]) => {
    // 移除旧主题
    document.querySelectorAll('link[data-theme]').forEach(el => el.remove())

    // 添加新主题
    const link = document.createElement('link')
    link.rel = 'stylesheet'
    link.href = `/src/styles/tokens/${theme.file}`
    link.setAttribute('data-theme', theme.name)
    document.head.appendChild(link)

    // 保存到 localStorage
    localStorage.setItem('theme', theme.name)
    currentTheme.value = theme
  }

  // 初始化
  onMounted(() => {
    const savedTheme = localStorage.getItem('theme')
    if (savedTheme) {
      const theme = themes.find(t => t.name === savedTheme)
      if (theme) applyTheme(theme)
    } else {
      // 检测系统主题
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      const defaultTheme = prefersDark ? themes[1] : themes[0]
      applyTheme(defaultTheme)
    }
  })

  return { currentTheme, themes, applyTheme }
}
```

**组件中使用 Design Tokens**：

```vue
<template>
  <div class="editor-container">
    <h1>标题</h1>
    <p class="text-secondary">副标题</p>
  </div>
</template>

<style scoped>
.editor-container {
  background-color: var(--color-bg-primary);
  color: var(--color-fg-primary);
  padding: var(--spacing-md);
  border-radius: var(--radius-md);
}

.text-secondary {
  color: var(--color-fg-secondary);
  font-size: var(--font-size-small);
}
</style>
```

### 8. 导出系统

#### 支持的导出格式

```typescript
enum ExportFormat {
  Markdown = 'md',
  PDF = 'pdf',
}

// 导出服务
class ExportService {
  async export(content: string, format: ExportFormat): Promise<void> {
    switch (format) {
      case ExportFormat.Markdown:
        return this.exportToMarkdown(content)
      case ExportFormat.PDF:
        return this.exportToPDF(content)
    }
  }

  // 导出为 Markdown（直接保存当前内容）
  private async exportToMarkdown(content: string): Promise<void> {
    // 获取 Markdown 格式内容
    const markdown = this.editor.storage.markdown.getMarkdown()

    // 调用 Tauri 文件系统 API 保存
    const filePath = await save({
      defaultPath: 'document.md',
      filters: [{ name: 'Markdown', extensions: ['md'] }]
    })

    await writeTextFile(filePath, markdown)
  }

  // 导出为 PDF（调用后端服务）
  private async exportToPDF(content: string): Promise<void> {
    // 获取 HTML 内容
    const html = this.editor.getHTML()

    // 调用 Tauri 后端 PDF 导出命令
    await invoke('export_pdf', {
      html: html,
      path: this.getFilePath(),
    })
  }
}
```

---

## UI 库风格一致性：PrimeVue × Typora

### 挑战

**PrimeVue** 有自己的默认样式，可能与 **Typora** 的极简风格冲突：
- PrimeVue：组件化、功能丰富、视觉突出
- Typora：极简、无干扰、内容优先

### 解决方案

#### 策略 1：最小化 PrimeVue 使用

**只在必要时使用 PrimeVue 组件**：

```typescript
// ✅ 使用原生 HTML/CSS 的场景
- 编辑器主体：原生 div + contenteditable
- 工具栏：原生 button
- 状态栏：原生 div
- 大纲面板：原生 ul/li
- 标题栏：原生 header

// ⚠️ 使用 PrimeVue 的场景
- 文件选择器：Tree 组件（复杂交互）
- 对话框：Dialog（模态管理）
- 下拉选择：Select（表单交互）
- 上下文菜单：ContextMenu（右键菜单）
- 进度提示：ProgressSpinner（加载状态）
```

**示例：最小化使用**
```vue
<template>
  <!-- 使用原生元素，只用 PrimeVue 做复杂交互 -->
  <div class="editor-container">
    <header class="editor-header">
      <h1 class="document-title">{{ title }}</h1>
      <div class="header-actions">
        <button class="icon-button" @click="openSettings" aria-label="设置">
          <SettingsIcon />
        </button>
      </div>
    </header>

    <!-- 主编辑器：完全自定义样式 -->
    <editor-content :editor="editor" class="tiptap-editor" />

    <!-- 只在必要时使用 PrimeVue -->
    <Sidebar v-model:visible="showSidebar" class="custom-sidebar">
      <Tree :value="fileTree" @node-select="handleFileSelect" />
    </Sidebar>
  </div>
</template>
```

#### 策略 2：覆盖 PrimeVue 默认样式

**使用 CSS Design Tokens 覆盖 PrimeVue 样式**：

```css
/* src/styles/primevue-overrides.css */

/* === PrimeVue 全局样式覆盖 === */
.p-component {
  font-family: var(--font-family-base);
  font-size: var(--font-size-base);
  color: var(--color-fg-primary);
}

/* === Button 覆盖 === */
.p-button {
  background: transparent;
  border: none;
  color: var(--color-fg-secondary);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-sm);
  transition: all 0.15s ease;
}

.p-button:hover {
  background: var(--color-bg-secondary);
  color: var(--color-fg-primary);
}

.p-button:focus {
  box-shadow: none;
  outline: 1px solid var(--color-accent-primary);
  outline-offset: 2px;
}

/* 图标按钮（Typora 风格） */
.icon-button {
  width: 32px;
  height: 32px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: background 0.15s ease;
}

.icon-button:hover {
  background: var(--color-bg-tertiary);
}

/* === Dialog 覆盖 === */
.p-dialog {
  background: var(--color-bg-primary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
}

.p-dialog-header {
  border-bottom: 1px solid var(--color-border-light);
  padding: var(--spacing-md) var(--spacing-lg);
}

.p-dialog-title {
  font-size: var(--font-size-base);
  font-weight: 500;
  color: var(--color-fg-primary);
}

.p-dialog-content {
  padding: var(--spacing-lg);
}

.p-dialog-footer {
  border-top: 1px solid var(--color-border-light);
  padding: var(--spacing-md) var(--spacing-lg);
}

/* === Select 覆盖 === */
.p-select {
  background: var(--color-bg-primary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
}

.p-select:hover {
  border-color: var(--color-accent-primary);
}

.p-select-option {
  padding: var(--spacing-sm) var(--spacing-md);
  font-size: var(--font-size-small);
}

.p-select-option:hover {
  background: var(--color-bg-secondary);
}

/* === Tree 覆盖 === */
.p-tree {
  background: transparent;
  border: none;
}

.p-tree-node-content {
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: background 0.1s ease;
}

.p-tree-node-content:hover {
  background: var(--color-bg-secondary);
}

.p-tree-node-content.selected {
  background: var(--color-accent-primary);
  color: white;
}

.p-tree-node-icon {
  color: var(--color-fg-tertiary);
}

/* === ContextMenu 覆盖 === */
.p-contextmenu {
  background: var(--color-bg-primary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  padding: var(--spacing-xs);
  min-width: 180px;
}

.p-contextmenu-item {
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-sm);
  font-size: var(--font-size-small);
  cursor: pointer;
}

.p-contextmenu-item:hover {
  background: var(--color-bg-secondary);
}

.p-contextmenu-item-link {
  color: var(--color-fg-primary);
  text-decoration: none;
}

/* === ProgressSpinner 覆盖 === */
.p-progress-spinner {
  width: 24px;
  height: 24px;
}

.p-progress-spinner-circle {
  stroke: var(--color-accent-primary);
  stroke-width: 2;
}
```

#### 策略 3：PrimeVue 主题配置

**使用 PrimeVue 的 `usePreset` 主题系统**：

```typescript
// src/main.ts
import { createApp } from 'vue'
import PrimeVue from 'primevue/config'
import Aura from '@primevue/themes/aura'
import { definePreset } from '@primevue/themes'

// 创建 Typora 风格的预设
const TyporaPreset = definePreset(Aura, {
  primitive: {
    borderRadius: {
      none: '0',
      small: '3px',
      medium: '6px',
      large: '8px',
    },
    transitionDuration: {
      '0': '0.15s',
    },
  },
  semantic: {
    primary: {
      50: '{blue.50}',
      100: '{blue.100}',
      // ... 使用 Design Tokens
      500: 'var(--color-accent-primary)',
      600: 'var(--color-accent-hover)',
      700: 'var(--color-accent-primary)',
    },
    surface: {
      0: '{slate.0}',
      50: '{slate.50}',
      100: '{slate.100}',
      // ...
      500: 'var(--color-bg-secondary)',
    },
  },
})

const app = createApp(App)

app.use(PrimeVue, {
  theme: {
    preset: TyporaPreset,
    options: {
      prefix: 'p',
      darkModeSelector: '.dark-theme',
      cssLayer: false,
    },
  },
})
```

#### 策略 4：组件样式隔离

**为 PrimeVue 组件创建独立的样式层**：

```css
/* src/styles/components.css */

/* === 编辑器区域：完全自定义，无 PrimeVue 样式 === */
.editor-area {
  /* Typora 风格：极简、无干扰 */
  background: var(--color-bg-primary);
  color: var(--color-fg-primary);
  font-family: var(--font-family-base);
  font-size: var(--font-size-base);
  line-height: var(--line-height-base);
  padding: var(--spacing-lg);
  max-width: var(--editor-width);
  margin: 0 auto;
}

/* 编辑器内元素：完全控制 */
.editor-area h1,
.editor-area h2,
.editor-area h3,
.editor-area h4,
.editor-area h5,
.editor-area h6 {
  color: var(--color-fg-primary);
  font-weight: 600;
  margin-top: var(--spacing-lg);
  margin-bottom: var(--spacing-md);
}

.editor-area p {
  margin-bottom: var(--spacing-md);
}

.editor-area code {
  background: var(--code-bg);
  padding: 2px 6px;
  border-radius: var(--radius-sm);
  font-family: var(--font-family-code);
  font-size: 0.9em;
}

/* === UI 区域：应用 PrimeVue 覆盖样式 === */
.ui-panel {
  /* 工具栏、侧边栏等 UI */
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border-light);
}

/* 确保编辑器内容不受 PrimeVue 影响 */
.ProseMirror {
  outline: none;
}

.ProseMirror p {
  margin: var(--spacing-md) 0;
}
```

#### 策略 5：PrimeVue 组件使用指南

**建立组件使用规范**：

```typescript
// docs/PRIMEVUE_USAGE_GUIDE.md

# PrimeVue 使用指南

## 原则

1. **优先使用原生元素**：能用原生 HTML/CSS 实现的，不使用 PrimeVue
2. **极简样式**：覆盖 PrimeVue 默认样式，匹配 Typora 风格
3. **功能优先**：只使用 PrimeVue 的交互功能，样式完全自定义

## 推荐使用的 PrimeVue 组件

| 组件 | 使用场景 | 自定义程度 |
|------|----------|-----------|
| Tree | 文件浏览器 | 高度自定义 |
| Dialog | 模态对话框 | 中度自定义 |
| Select | 下拉选择 | 中度自定义 |
| ContextMenu | 右键菜单 | 高度自定义 |
| ProgressSpinner | 加载状态 | 轻度自定义 |

## 不推荐使用的 PrimeVue 组件

| 组件 | 不使用原因 | 替代方案 |
|------|------------|----------|
| Button | 样式过于突出 | 原生 button + CSS |
| Input | 边框样式不符 | 原生 input + CSS |
| DataTable | 视觉过重 | 原生 table + CSS |
| Card | 边框阴影过多 | 原生 div + CSS |

## 组件示例

### ✅ 推荐：文件树（使用 PrimeVue Tree）
```vue
<template>
  <div class="file-browser">
    <Tree
      :value="files"
      @node-select="openFile"
      class="typora-tree"
    />
  </div>
</template>

<style scoped>
.typo-tree :deep(.p-tree) {
  background: transparent;
  border: none;
}

.typo-tree :deep(.p-tree-node-content) {
  padding: 4px 8px;
  border-radius: 3px;
}

.typo-tree :deep(.p-tree-node-content:hover) {
  background: var(--color-bg-secondary);
}
</style>
```

### ❌ 避免：按钮（使用原生）
```vue
<template>
  <!-- 不推荐 -->
  <Button label="保存" />

  <!-- 推荐 -->
  <button class="typora-button" @click="save">
    保存
  </button>
</template>

<style scoped>
.typora-button {
  padding: 6px 12px;
  background: transparent;
  border: none;
  color: var(--color-fg-secondary);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: background 0.15s ease;
}

.typora-button:hover {
  background: var(--color-bg-secondary);
  color: var(--color-fg-primary);
}
</style>
```
```

#### 策略 6：全局样式重置

**重置 PrimeVue 可能影响的样式**：

```css
/* src/styles/reset.css */

/* === 重置 PrimeVue 影响的样式 === */

/* 确保编辑器内容不受影响 */
.ProseMirror,
.ProseMirror * {
  box-sizing: border-box;
}

/* 重置可能的按钮样式继承 */
.ProseMirror button {
  background: none;
  border: none;
  padding: 0;
  font: inherit;
  color: inherit;
}

/* 重置可能的输入框样式继承 */
.ProseMirror input,
.ProseMirror textarea {
  font: inherit;
  color: inherit;
  background: transparent;
  border: none;
  outline: none;
}

/* 确保链接样式符合 Typora 风格 */
.ProseMirror a {
  color: var(--color-accent-primary);
  text-decoration: none;
}

.ProseMirror a:hover {
  text-decoration: underline;
}
```

### 实施步骤

#### 1. 创建样式覆盖文件

**文件结构**：
```
src/styles/
├── primevue-overrides.css      # PrimeVue 样式覆盖
├── reset.css                   # 全局样式重置
├── components.css              # 组件样式（编辑器等）
└── tokens/                     # Design Tokens
    ├── base.css
    ├── github-light.css
    └── ...
```

#### 2. 在 main.ts 中导入样式

```typescript
// src/main.ts
import './styles/reset.css'              // 全局重置
import './styles/tokens/base.css'       // Design Tokens
import './styles/tokens/github-light.css' // 默认主题
import './styles/primevue-overrides.css' // PrimeVue 覆盖
import './styles/components.css'         // 组件样式
```

#### 3. 配置 PrimeVue 主题

```typescript
// src/main.ts
import PrimeVue from 'primevue/config'

app.use(PrimeVue, {
  theme: {
    options: {
      cssLayer: false,  // 禁用 CSS 层，便于覆盖
      darkModeSelector: '.dark-theme',
    },
  },
})
```

### 效果对比

**Before（PrimeVue 默认）**：
- 按钮有明显的背景色和边框
- 对话框有厚重的阴影
- 组件视觉突出，与编辑内容冲突

**After（Typora 风格）**：
- 按钮极简，hover 时才显示背景
- 对话框轻量，融入整体设计
- 组件低调，不干扰编辑体验

---

## 性能优化策略

### 1. 虚拟滚动

```typescript
// 虚拟滚动扩展
class VirtualScrollExtension extends Extension {
  addProseMirrorPlugins() {
    return [
      new Plugin({
        props: {
          decorations: (state) => {
            const doc = state.doc
            const viewport = this.getViewport()
            return DecorationSet.create(doc, this.createViewportDecos(doc, viewport))
          },
        },
      }),
    ]
  }

  private getViewport(): { from: number; to: number } {
    // 只渲染可见区域 ± 缓冲区
    const scrollTop = window.scrollY
    const windowHeight = window.innerHeight
    return {
      from: this.posAtHeight(scrollTop - 1000),
      to: this.posAtHeight(scrollTop + windowHeight + 1000),
    }
  }
}
```

### 2. 增量渲染

```typescript
// 图表增量渲染缓存
class MermaidRenderCache {
  private cache = new Map<string, string>()

  async render(code: string, id: string): Promise<string> {
    const hash = this.hash(code)

    if (this.cache.has(hash)) {
      return this.cache.get(hash)!
    }

    const svg = await mermaid.render(id, code)
    this.cache.set(hash, svg)
    return svg
  }
}
```

### 3. Web Worker 渲染

```typescript
// 代码高亮 Web Worker
const highlightWorker = new Worker('/workers/highlight.ts')

highlightWorker.postMessage({
  type: 'highlight',
  code: blockContent,
  lang: blockLang,
})

highlightWorker.onmessage = (e) => {
  const highlightedHTML = e.data.result
  // 更新 UI
}
```

---

## 文件组织结构

```
src/
├── components/
│   ├── editor/
│   │   ├── EditorCore.vue           # 主编辑器组件
│   │   ├── EditorToolbar.vue        # 工具栏
│   │   └── EditorStatusBar.vue      # 状态栏
│   │
│   ├── outline/
│   │   ├── OutlinePanel.vue         # 大纲面板
│   │   └── TOCGenerator.ts          # TOC 生成器
│   │
│   ├── theme/
│   │   └── ThemeSelector.vue        # 主题选择器
│   │
│   └── export/
│       ├── ExportDialog.vue         # 导出对话框
│       └── ExportService.ts         # 导出服务（仅 md + pdf）
│
├── extensions/                      # Tiptap 扩展
│   ├── markdown/
│   │   ├── MarkdownShortcuts.ts      # Markdown 快捷键
│   │   ├── TableExtension.ts         # 表格扩展
│   │   ├── TaskListExtension.ts      # 任务列表
│   │   ├── FootnoteExtension.ts      # 脚注
│   │   └── YAMLFrontMatter.ts        # YAML 前言
│   │   
│   ├── diagrams/
│   │   ├── MermaidExtension.ts       # Mermaid 扩展
│   │   ├── MermaidNode.ts            # Mermaid 节点
│   │   ├── MermaidNodeView.ts        # Mermaid NodeView
│   │   └── MermaidRenderer.vue       # Mermaid 渲染器
│   │   
│   │   // 注: 数学公式 (math/) 目录暂不创建，留待未来版本
│   │   
│   ├── code/
│   │   ├── CodeBlockExtension.ts     # 代码块扩展
│   │   ├── CodeBlockNodeView.ts      # 代码块 NodeView
│   │   └── Highlighter.ts            # Shiki 集成
│   │
│   └── features/
│       ├── FocusMode.ts              # 专注模式
│       ├── TypewriterMode.ts         # 打字机模式
│       ├── WordCount.ts              # 字数统计
│       ├── SearchAndReplace.ts       # 搜索替换
│       └── AutoLink.ts               # 自动链接
│
├── styles/
│   ├── tokens/                      # CSS Design Tokens
│   │   ├── base.css                # 基础变量定义
│   │   ├── github-light.css        # GitHub Light 主题
│   │   ├── github-dark.css         # GitHub Dark 主题
│   │   ├── nord.css                # Nord 主题
│   │   └── dracula.css             # Dracula 主题
│   │
│   ├── themes.css                   # 主题入口文件
│   ├── editor.css                   # 编辑器基础样式（使用 tokens）
│   ├── table.css                    # 表格样式
│   ├── code-block.css               # 代码块样式
│   └── mermaid.css                  # Mermaid 图表样式
│
├── utils/
│   ├── markdown-serializer.ts       # Markdown 序列化器
│   ├── markdown-parser.ts           # Markdown 解析器
│   ├── format-converter.ts          # 格式转换器（为 Typst 预留）
│   └── performance.ts               # 性能工具
│
├── composables/
│   └── useTheme.ts                  # 主题切换 composable
│
└── store/
    └── editor-store.ts              # 编辑器状态管理

├── pages/                           # 页面组件
│   ├── home/
│   │   ├── Home.vue                  # 主页布局（编辑器容器）
│   │   └── Sidebar.vue               # 侧边栏（文件树/大纲切换）
│   │
│   ├── settings/
│   │   ├── Settings.vue              # 设置主页面
│   │   ├── EditorSettings.vue        # 编辑器设置
│   │   ├── AppearanceSettings.vue    # 外观/主题设置
│   │   └── KeymapSettings.vue        # 快捷键设置
│   │
│   └── export/
│       └── ExportPage.vue            # 导出页面（可选）
```

---

## 技术依赖清单

### 核心依赖

```json
{
  "dependencies": {
    "@tiptap/core": "^3.13.0",
    "@tiptap/vue-3": "^3.13.0",
    "@tiptap/starter-kit": "^3.13.0",
    "@tiptap/pm": "^3.13.0",
    "@tiptap/markdown": "^3.13.0",
    "@tiptap/extension-table": "^3.13.0",
    "prosemirror-inputrules": "^1.5.1",

    "mermaid": "^11.0.0",
    "shiki": "^1.0.0",

    "vue": "^3.5.25",
    "pinia": "^3.0.4",
    "primevue": "^4.5.1",
    "@tauri-apps/api": "^2.9.1"
  }
}
```

**注**: KaTeX 依赖暂不添加，数学公式功能留待未来版本

---

## 与现有代码的集成

### 重用现有模块

1. **状态管理**：[src/store/store.ts](src/store/store.ts) - 扩展现有 store
2. **文件系统**：[src-tauri/src/cmds/fs.rs](src-tauri/src/cmds/fs.rs) - 复用现有文件操作
3. **UI 组件**：PrimeVue 组件库 - 继续使用
4. **路由**：[src/router.ts](src/router.ts) - 保持现有路由结构

### 需要修改的文件

1. **[src/pages/typst/TypstEditor.vue](src/pages/typst/TypstEditor.vue)** - 主编辑器，需要添加新扩展
2. **[src/pages/home/Home.vue](src/pages/home/Home.vue)** - 主布局，可能需要调整
3. **[src/App.vue](src/App.vue)** - 应用入口，可能需要主题支持

---

## 未来 Typst 扩展预留接口

### 格式抽象层

```typescript
// 格式接口定义
interface DocumentFormat {
  name: string
  extensions: string[]
  serialize(state: EditorState): string
  parse(content: string): Node
  render?(content: string): Promise<string>
}

// Markdown 格式实现
class MarkdownFormat implements DocumentFormat {
  name = 'markdown'
  extensions = ['.md', '.markdown']
  serialize = (state) => state.doc.toJSON()
  parse = (content) => MarkdownParser.parse(content)
}

// Typst 格式实现（未来）
class TypstFormat implements DocumentFormat {
  name = 'typst'
  extensions = ['.typ']
  serialize = (state) => TypstSerializer.serialize(state)
  parse = (content) => TypstParser.parse(content)
  render = async (content) => {
    // 调用后端 Typst 编译服务
    return invoke('typst_compile', { content })
  }
}
```

### 扩展点

```typescript
// 编辑器配置预留扩展点
const editorConfig = {
  // 当前：Markdown 扩展
  format: new MarkdownFormat(),
  extensions: markdownExtensions,

  // 未来：可以切换到 Typst
  // format: new TypstFormat(),
  // extensions: typstExtensions,
}
```

---

## 安全性考虑

### 1. 内容安全

- **XSS 防护**：ProseMirror 自动转义 HTML
- **Mermaid 安全**：配置 `securityLevel: 'loose'` 但需内容过滤
- **文件路径验证**：复用现有 [src/utils/path-security.ts](src/utils/path-security.ts)

### 2. 性能监控

```typescript
// 性能监控装饰器
function measurePerformance(target: any, propertyKey: string, descriptor: PropertyDescriptor) {
  const originalMethod = descriptor.value
  descriptor.value = async function (...args: any[]) {
    const start = performance.now()
    const result = await originalMethod.apply(this, args)
    const duration = performance.now() - start
    console.log(`${propertyKey} took ${duration}ms`)
    return result
  }
}
```

---

## 总结

本架构设计提供了：

1. **完整的 Typora 功能复刻**：所有核心功能都有明确的实现路径
2. **Mermaid 图表支持**：完整的图表渲染系统
3. **扩展性**：为未来 Typst 支持预留了清晰的接口
4. **性能优化**：虚拟滚动、增量渲染、Web Worker
5. **可维护性**：清晰的模块划分和文件组织

---

*文档版本: v1.0*
*最后更新: 2025-03-08*
