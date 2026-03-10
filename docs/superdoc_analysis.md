# SuperDoc 技术架构分析与 Typst 编辑器借鉴

## 一、项目概述

**SuperDoc** 是一个基于 JavaScript 的 DOCX 文件浏览器编辑器，直接操作 OOXML 格式，使用 ProseMirror 作为编辑引擎核心。其核心理念是"真正的 DOCX，而非富文本"——建立在 OOXML 之上，支持真实的分页、节分隔、页眉页脚等 Word 特性。

### 核心特性

- **真正的 OOXML 编辑**：非 contenteditable 包装器 + 导出，而是直接操作 DOCX 的 OOXML 结构
- **ProseMirror 驱动**：使用 ProseMirror 管理文档状态和编辑命令
- **自定义渲染管线**：ProseMirror DOM 隐藏，视觉输出由 DomPainter 完成
- **实时协作**：基于 Yjs 的 CRDT 实现
- **框架无关**：支持 React、Vue、Angular、Svelte、原生 JS

## 二、整体架构

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           Frontend (Vue/React)                          │
├─────────────────────────────────────────────────────────────────────────┤
│  SuperDoc.vue / SuperDocEditor.tsx                                      │
│  ├── PresentationEditor (包装隐藏的 ProseMirror Editor)                 │
│  ├── Toolbar                                                            │
│  └── UI Components                                                      │
└───────────────────────────────┬─────────────────────────────────────────┘
                                │
┌───────────────────────────────▼─────────────────────────────────────────┐
│                        Core Pipeline                                     │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   ┌──────────────┐    ┌──────────────┐    ┌──────────────┐              │
│   │  ProseMirror │    │  pm-adapter  │    │ layout-engine│              │
│   │    Editor    │───▶│              │───▶│              │              │
│   │   (hidden)   │    │  PM → Block  │    │ Pagination   │              │
│   └──────────────┘    └──────────────┘    └──────┬───────┘              │
│                                                   │                      │
│                                          ┌────────▼───────┐              │
│                                          │   DomPainter   │              │
│                                          │ (Visual Output)│              │
│                                          └────────────────┘              │
└─────────────────────────────────────────────────────────────────────────┘
                                │
┌───────────────────────────────▼─────────────────────────────────────────┐
│                        Data Layer                                        │
├─────────────────────────────────────────────────────────────────────────┤
│  SuperConverter (OOXML ↔ ProseMirror JSON)                              │
│  ├── Import: DOCX XML → ProseMirror Document                            │
│  └── Export: ProseMirror Document → DOCX XML                            │
│                                                                          │
│  Style Engine (OOXML 样式解析)                                           │
│  └── 样式级联：defaults → styles → numbering → direct formatting        │
└─────────────────────────────────────────────────────────────────────────┘
```

### 关键洞察：ProseMirror 不用于视觉渲染

```
PM Doc (hidden) → pm-adapter → FlowBlock[] → layout-engine → Layout[] → DomPainter → DOM
```

- `PresentationEditor` 包装一个**隐藏的** ProseMirror `Editor` 实例用于文档状态和编辑命令
- ProseMirror 的 contenteditable DOM **从不展示给用户**
- **DomPainter** 拥有所有视觉渲染逻辑
- 样式解析的属性通过 `style-engine` → `pm-adapter` → `DomPainter` 流向

## 三、核心模块分析

### 3.1 Super Editor (ProseMirror 编辑器)

`packages/super-editor/` 是核心编辑引擎，基于 ProseMirror 构建。

#### 目录结构

```
packages/super-editor/src/
├── core/
│   ├── Editor.ts              # 主编辑器类，管理生命周期
│   ├── Extension.ts           # 扩展基类
│   ├── Mark.ts                # Mark 扩展基类
│   ├── Node.ts                # Node 扩展基类
│   ├── OxmlNode.ts            # OOXML 特化的 Node 扩展
│   ├── CommandService.js      # 命令执行服务
│   ├── ExtensionService.js    # 扩展管理服务
│   ├── InputRule.js           # 输入规则
│   └── super-converter/       # DOCX 导入/导出
│       ├── SuperConverter.js  # 主转换器
│       ├── exporter.js        # 导出逻辑
│       ├── v2/                # V2 导入器
│       └── v3/                # V3 转换器（Node Translator 模式）
├── extensions/                # 60+ 内置扩展
│   ├── bold/                  # Mark 扩展示例
│   ├── paragraph/             # Node 扩展示例
│   ├── table/                 # 表格扩展
│   ├── track-changes/         # 修订追踪
│   └── ...
└── schema/                    # 文档 Schema 定义
```

#### 扩展系统设计模式

SuperDoc 使用类似 TipTap 的扩展系统，采用流式构建器模式：

```javascript
// Mark 扩展示例 (bold.js)
export const Bold = Mark.create({
  name: 'bold',

  addOptions() {
    return { htmlAttributes: {} };
  },

  addAttributes() {
    return {
      value: {
        default: null,
        renderDOM: (attrs) => {
          if (attrs.value === '0') return { style: 'font-weight: normal' };
          return {};
        },
      },
    };
  },

  parseDOM() {
    return [
      { tag: 'strong' },
      { tag: 'b', getAttrs: (node) => node.style.fontWeight != 'normal' && null },
    ];
  },

  renderDOM({ htmlAttributes }) {
    return ['strong', htmlAttributes, 0];
  },

  addCommands() {
    return {
      setBold: () => /* ... */,
      unsetBold: () => /* ... */,
      toggleBold: () => /* ... */,
    };
  },

  addShortcuts() {
    return {
      'Mod-b': () => this.editor.commands.toggleBold(),
    };
  },
});
```

```javascript
// Node 扩展示例 (paragraph.js) - 使用 OxmlNode
export const Paragraph = OxmlNode.create({
  name: 'paragraph',
  oXmlName: 'w:p',           // OOXML 元素名
  priority: 1000,
  group: 'block',
  content: 'inline*',

  addOptions() {
    return {
      headingLevels: [1, 2, 3, 4, 5, 6],
      htmlAttributes: {},
    };
  },

  addAttributes() {
    return {
      paraId: { rendered: false },
      textId: { rendered: false },
      paragraphProperties: { /* ... */ },
      // ... 更多 OOXML 属性
    };
  },

  addCommands() {
    return {
      splitBlock: () => /* ... */,
      // ...
    };
  },

  addPmPlugins() {
    return [
      createNumberingPlugin(),
      createLeadingCaretPlugin(),
      // ...
    ];
  },
});
```

#### OxmlNode 特化

`OxmlNode` 扩展了 `Node`，添加 OOXML 特定属性：

```typescript
export interface OxmlNodeConfig extends NodeConfig {
  oXmlName: string;           // OOXML 元素名，如 'w:p'
  childToAttributes?: string[]; // 子元素到属性的映射
}
```

### 3.2 Layout Engine (布局引擎)

`packages/layout-engine/` 包含完整的布局和渲染管线。

#### 子包结构

| 包 | 职责 | 入口文件 |
|---|---|---|
| `contracts/` | 共享类型定义 (FlowBlock, Layout 等) | `src/index.ts` |
| `pm-adapter/` | ProseMirror → FlowBlocks 转换 | `src/internal.ts` |
| `layout-engine/` | 分页算法 | `src/index.ts` |
| `layout-bridge/` | 管道编排 | `src/layout-pipeline.ts` |
| `painters/dom/` | DOM 渲染 | `src/renderer.ts` |
| `style-engine/` | OOXML 样式解析 | `src/index.ts` |
| `geometry-utils/` | 布局数学工具 | `src/index.ts` |

#### PM Adapter (ProseMirror → FlowBlock)

`pm-adapter` 负责将 ProseMirror 文档转换为布局引擎可处理的 `FlowBlock[]`：

```typescript
// 核心转换函数
export function toFlowBlocks(pmDoc: PMNode, options?: AdapterOptions): FlowBlocksResult {
  // 1. 解析段落节点
  // 2. 根据 mark 边界分割文本内容为样式化 runs
  // 3. 为布局追踪生成确定性 BlockId
  // 4. 规范化空白和处理空段落
}

// 节点处理器分发
export const nodeHandlers: Record<string, NodeHandler> = {
  paragraph: handleParagraphNode,
  table: handleTableNode,
  image: handleImageNode,
  vectorShape: handleVectorShapeNode,
  chart: handleChartNode,
  // ...
};
```

#### DomPainter (渲染器)

DomPainter 是"哑"渲染器——接收预计算的 `Layout` 并渲染，不包含布局逻辑：

```typescript
// renderer.ts - 核心渲染逻辑
class DomPainter {
  // 页面虚拟化 - 只渲染可见页面
  private pageIndexToState: Map<number, PageState>;

  // 块查找映射 - 用于变更检测
  private blockIdToEntry: Map<BlockId, BlockEntry>;

  // 渲染入口
  render(layout: Layout[]): void {
    // 1. 比较新旧布局版本
    // 2. 只更新变更的页面
    // 3. 渲染定位后的 fragments
  }
}
```

#### Feature Modules 模式

渲染逻辑按 OOXML 特性拆分为独立模块：

```
painters/dom/src/features/
├── feature-registry.ts      # OOXML 元素 → 特性模块查找表
├── paragraph-borders/       # 段落边框和底纹
└── ...
```

```typescript
// feature-registry.ts
export const FEATURE_REGISTRY = {
  'w:pBdr': { feature: 'paragraph-borders', module: './paragraph-borders' },
  'w:shd': { feature: 'paragraph-borders', module: './paragraph-borders' },
  // ...
};
```

### 3.3 SuperConverter (DOCX 转换器)

`packages/super-editor/src/core/super-converter/` 负责 DOCX 的导入导出。

#### V3 Node Translator 模式

V3 版本引入了声明式的 Node Translator 模式：

```javascript
// Node Translator 定义
export class NodeTranslator {
  xmlName;          // OOXML 元素名
  sdNodeOrKeyName;  // SuperDoc 节点名
  encodeFn;         // OOXML → SuperDoc
  decodeFn;         // SuperDoc → OOXML
  attributes;       // 属性映射配置
}

// 属性配置
const attrConfig = {
  xmlName: 'w:val',
  sdName: 'value',
  encode: (val) => val,  // 可选的编码函数
  decode: (val) => val,  // 可选的解码函数
};
```

#### Handlers 结构

```
v3/handlers/
├── w/                    # Word 命名空间处理器 (195 个)
│   ├── b/                # <w:b> 粗体
│   ├── i/                # <w:i> 斜体
│   ├── p/                # <w:p> 段落
│   ├── tbl/              # <w:tbl> 表格
│   └── ...
├── sd/                   # SuperDoc 特定元素
│   ├── pageReference/
│   ├── tableOfContents/
│   └── ...
└── index.js              # 所有 translator 注册
```

### 3.4 Style Engine (样式引擎)

`packages/layout-engine/style-engine/` 是 OOXML 样式级联的唯一真相来源。

#### 级联顺序

```
defaults → styles → numbering → direct formatting
```

#### 关键 API

```typescript
// 解析运行属性
resolveRunProperties(context, properties, styleId);

// 解析段落属性
resolveParagraphProperties(context, properties, styleId);

// 解析表格单元格属性
resolveTableCellProperties(context, properties, styleId);

// 属性合并
combineProperties(lowPriority, highPriority);
```

#### 重要原则：Converter 只解析，Style Engine 只解析

- **Converter** (`super-converter/`) 只解析和存储 XML 中明确的内容（内联属性、样式引用）
- **不应该**在导入时解析样式级联、条件格式或继承属性
- **Style Engine** 是样式级联逻辑的唯一来源

**原因**：如果在导入时解析样式，会将它们烘焙为内联属性。导出时，这些会作为直接格式写入而非样式引用，丢失原始文档意图。

## 四、可借鉴的设计模式

### 4.1 架构层面

#### 1. 分离编辑模型与渲染视图

**SuperDoc 的做法**：
- ProseMirror 作为"隐藏"的编辑模型，管理文档状态和编辑命令
- DomPainter 作为独立的渲染层，负责视觉输出
- 通过 `pm-adapter` 桥接两者

**对 Typst 的借鉴**：
```
Typst 源码/AST → Typst Adapter → FlowBlock[] → Layout Engine → DomPainter → DOM
                   ↑
              隐藏的编辑状态
```

好处：
- 编辑逻辑与渲染逻辑解耦
- 可以针对 Typst 特性优化渲染（如数学公式、表格）
- 支持"所见即所得"的同时保持 Typst 源码语义

#### 2. 声明式扩展系统

**SuperDoc 的做法**：
```javascript
OxmlNode.create({
  name: 'paragraph',
  oXmlName: 'w:p',
  addAttributes() { /* ... */ },
  addCommands() { /* ... */ },
  addShortcuts() { /* ... */ },
  addPmPlugins() { /* ... */ },
});
```

**对 Typst 的借鉴**：
```javascript
TypstNode.create({
  name: 'equation',
  typstName: '$...$',      // Typst 语法标识
  addAttributes() {
    return {
      mode: { default: 'inline' },  // inline | block
      numbered: { default: false },
    };
  },
  addCommands() {
    return {
      toggleEquation: () => /* ... */,
    };
  },
});
```

#### 3. 双向转换器模式 (Node Translator)

**SuperDoc 的做法**：
```javascript
const translator = {
  xmlName: 'w:b',
  sdNodeOrKeyName: 'bold',
  encode: (params) => ({ type: 'bold', attrs: { value: params.value } }),
  decode: (node) => ({ name: 'w:b', attributes: { 'w:val': node.attrs.value } }),
};
```

**对 Typst 的借鉴**：
```javascript
const typstTranslators = {
  // 粗体: *text*
  bold: {
    typstSyntax: '*...*',
    encode: (text) => `*${text}*`,
    decode: (source) => parseBold(source),
  },
  // 数学公式: $...$
  equation: {
    typstSyntax: '$...$',
    encode: (latex) => `$${latex}$`,
    decode: (source) => parseEquation(source),
  },
  // 标题: = Heading
  heading: {
    typstSyntax: '= ...',
    encode: (level, text) => `${'='.repeat(level)} ${text}`,
    decode: (source) => parseHeading(source),
  },
};
```

### 4.2 数据层面

#### 1. 样式解析边界清晰

**SuperDoc 的做法**：
- Importer 只存储原始 OOXML 属性
- Style Engine 负责所有样式级联逻辑
- 渲染时调用 Style Engine 获取计算后的属性

**对 Typst 的借鉴**：
```
Typst Importer:
  - 只解析源码中的显式内容
  - 保留样式引用（如 #set text(...)）

Typst Style Engine:
  - 处理 Typst 的样式规则
  - #set 指令解析
  - #show 规则应用
  - 默认值填充

渲染时:
  - 调用 Style Engine 获取最终样式
```

#### 2. 增量布局更新

**SuperDoc 的做法**：
- `blockIdToEntry` 映射用于变更检测
- 只更新变更的页面
- 页面虚拟化减少 DOM 节点

**对 Typst 的借鉴**：
```typescript
interface TypstLayoutState {
  blockIdToEntry: Map<BlockId, BlockEntry>;
  layoutVersion: number;

  // 当内容变更时
  updateContent(change: Transaction): void {
    // 1. 计算受影响的 blocks
    // 2. 只重新布局这些 blocks
    // 3. 增量更新 DOM
  }
}
```

### 4.3 具体功能层面

#### 1. 数学公式处理

**SuperDoc 的做法**（虽然没有原生数学支持，但架构可借鉴）：
- 自定义 Node 类型
- NodeView 处理点击编辑/模糊渲染
- 属性存储 LaTeX 源码

**对 Typst 的借鉴**：
```javascript
export const Equation = TypstNode.create({
  name: 'equation',
  group: 'block',
  atom: true,
  draggable: true,

  addAttributes() {
    return {
      source: { default: '' },      // Typst 数学源码
      mode: { default: 'inline' },  // inline | block
      numbered: { default: false },
    };
  },

  addNodeView() {
    return ({ node, view, getPos }) => {
      // 1. 创建容器
      // 2. 渲染数学公式（使用 Typst 编译或 KaTeX）
      // 3. 点击进入编辑模式
      // 4. 模糊时渲染公式
    };
  },
});
```

#### 2. 表格处理

**SuperDoc 的做法**：
- `prosemirror-tables` 作为基础
- 自定义 OOXML 属性映射
- 复杂表格特性支持（合并单元格、嵌套表格）

**对 Typst 的借鉴**：
- Typst 表格语法更简洁
- 可参考 ProseMirror Tables 的交互模式
- 需要处理 Typst 特有的 `#table()` 函数语法

#### 3. 实时协作

**SuperDoc 的做法**：
- Yjs + y-prosemirror
- CRDT 保证最终一致性
- 协作光标显示

**对 Typst 的借鉴**：
- 可复用 Yjs 生态
- 需要设计 Typst 文档的 CRDT 映射
- 或者直接在 ProseMirror 层协作，导出时转换为 Typst

### 4.4 推荐的实现路径

#### 阶段一：基础架构

1. **定义 Typst Schema**
   ```javascript
   // 基于 ProseMirror 定义 Typst 文档 Schema
   const typstSchema = new Schema({
     nodes: {
       doc: { content: 'block+' },
       paragraph: { group: 'block', content: 'inline*' },
       heading: { group: 'block', content: 'inline*', attrs: { level: { default: 1 } } },
       equation: { group: 'block', atom: true, attrs: { source: {}, mode: {} } },
       // ...
     },
     marks: {
       bold: {},
       italic: {},
       link: { attrs: { href: {} } },
       // ...
     },
   });
   ```

2. **创建 Typst Adapter**
   ```javascript
   // 类似 pm-adapter
   export function toTypstBlocks(pmDoc) {
     // 将 PM Doc 转换为渲染用的 Block 结构
   }
   ```

3. **实现基础转换器**
   ```javascript
   // Typst ↔ PM 双向转换
   export class TypstConverter {
     typstToPm(source: string): PMNode;
     pmToTypst(pmNode: PMNode): string;
   }
   ```

#### 阶段二：编辑器核心

1. **TypstEditor 类**
   ```javascript
   class TypstEditor {
     #pmEditor;        // 隐藏的 PM Editor
     #renderer;        // DomPainter 或自定义渲染器
     #converter;       // TypstConverter

     async loadDocument(path: string) {
       const source = await fs.readFile(path);
       const pmDoc = this.#converter.typstToPm(source);
       this.#pmEditor.setState(EditorState.create({ doc: pmDoc }));
     }

     saveDocument() {
       const source = this.#converter.pmToTypst(this.#pmEditor.state.doc);
       // 保存...
     }
   }
   ```

2. **扩展系统**
   ```javascript
   // 复用 SuperDoc 的扩展模式
   export const TypstBold = Mark.create({
     name: 'bold',
     typstSyntax: '*...*',
     // ...
   });
   ```

#### 阶段三：高级功能

1. **数学公式编辑器**
   - 自定义 NodeView
   - 集成 Typst 数学编译或 KaTeX
   - 点击编辑、模糊渲染

2. **样式系统**
   - 解析 `#set` 指令
   - 支持 `#show` 规则
   - 样式面板 UI

3. **实时预览**
   - 后端 Typst 编译
   - PDF/图片预览同步

## 五、技术栈对比

| 方面 | SuperDoc | Typst 编辑器建议 |
|-----|----------|-----------------|
| 编辑引擎 | ProseMirror | ProseMirror（复用） |
| 渲染引擎 | DomPainter | DomPainter 或自定义 |
| 格式转换 | OOXML ↔ PM | Typst ↔ PM |
| 样式解析 | Style Engine | Typst Style Engine |
| 数学公式 | 无原生支持 | Typst 数学 / KaTeX |
| 协作 | Yjs | Yjs |
| 后端 | 无（纯前端） | Tauri + Typst CLI |

## 六、关键代码参考

### 入口点索引

| 功能 | 位置 |
|-----|------|
| 主编辑器 | `packages/super-editor/src/core/Editor.ts` |
| 扩展基类 | `packages/super-editor/src/core/Mark.ts`, `Node.ts`, `OxmlNode.ts` |
| 扩展注册 | `packages/super-editor/src/extensions/index.js` |
| DOCX 转换 | `packages/super-editor/src/core/super-converter/SuperConverter.js` |
| Node Translator | `packages/super-editor/src/core/super-converter/v3/node-translator/` |
| PM Adapter | `packages/layout-engine/pm-adapter/src/internal.ts` |
| DOM 渲染 | `packages/layout-engine/painters/dom/src/renderer.ts` |
| 样式引擎 | `packages/layout-engine/style-engine/src/index.ts` |
| 类型定义 | `packages/layout-engine/contracts/src/index.ts` |

### 项目链接

- GitHub: https://github.com/superdoc-dev/superdoc
- 文档: https://docs.superdoc.dev
- 官网: https://www.superdoc.dev

---

*文档生成时间：2025-03-10*