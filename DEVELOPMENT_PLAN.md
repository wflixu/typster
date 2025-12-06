# Typst Editor 开发计划

> 基于PRD文档和当前Phase 1完成状态制定的详细开发计划

## 项目概述

Typst Editor 是一个现代化的编辑器，使用 Tauri 2.0 + Vue 3 + TypeScript 构建，提供 Typora 风格的 WYSIWYG 编辑体验。

**开发策略**:
- **第一阶段**: 专注实现完美的Markdown编辑器（仿Typora）
- **第二阶段**: 扩展支持Typst文件编辑功能
- **架构设计**: 前期为后期Typst支持做准备

### 技术栈
- **前端**: Vue 3 + TypeScript + Tiptap + PrimeVue + KaTeX
- **后端**: Rust + Tauri 2.0 (第二阶段集成Typst 0.14.0)
- **架构**: 纯 WYSIWYG 编辑器（无预览模式切换）

---

## 🎯 总体开发策略

### 第一阶段: 完美Markdown编辑器 (Phase 1-4)
**目标**: 创建一个功能完整的Typora克隆编辑器
- 专注Markdown文件格式的完美支持
- 实现数学公式的KaTeX渲染
- 优化编辑体验和性能
- **为第二阶段做好架构准备**

### 第二阶段: Typst编辑器扩展 (Phase 5-7)
**目标**: 扩展支持Typst文件格式
- 集成Typst编译和渲染
- 保持Markdown功能完整
- 提供统一的多格式编辑体验

---

## ✅ Phase 1: 基础WYSIWYG编辑器 (已完成)

### 目标
创建仿照Typora风格的基础WYSIWYG界面，专注Markdown文件编辑。

### 已完成任务

#### ✅ 1.1 基础界面布局
- [x] Typora风格标题栏设计
- [x] 窗口控制按钮（最小化、最大化、关闭）
- [x] 编辑区域布局（居中设计）
- [x] 状态栏设计（字数、字符数、光标位置）

#### ✅ 1.2 Tiptap编辑器集成
- [x] 安装和配置Tiptap核心依赖
- [x] 创建`TypstEditor.vue`主编辑器组件
- [x] 实现基础的富文本编辑功能
- [x] 配置编辑器属性（样式、行为）

#### ✅ 1.3 Markdown语法支持
- [x] 创建`SimpleMarkdownExtension`扩展
- [x] 实现输入规则（# ## **text** *text*）
- [x] 添加快捷键支持（Ctrl+B/I/K）
- [x] 支持多级标题（H1-H6）
- [x] 为后期Typst语法扩展预留架构接口

#### ✅ 1.4 基础功能验证
- [x] 解决依赖和编译问题
- [x] 确保编辑器可以正常输入文本
- [x] 状态栏统计功能正常工作
- [x] Tauri应用正常运行（http://localhost:1420）

#### 📁 关键文件
- `src/pages/typst/TypstEditor.vue` - 主编辑器界面
- `src/components/tiptap-editor/extensions/TypstExtension.ts` - Markdown扩展
- `src/pages/home/Sidebar.vue` - 简化的侧边栏

#### 🎯 Phase 1成果
- ✅ 创建了完整的Typora风格WYSIWYG编辑器界面
- ✅ 实现了基础文本编辑和Markdown语法支持
- ✅ 应用可正常运行，用户可以输入和编辑文本
- ✅ 为后续Typst扩展奠定了坚实基础
- ✅ 架构设计支持后续功能扩展

---

## 📋 Phase 2: Markdown数学公式渲染系统

### 目标
**专注Markdown的数学公式渲染**，使用KaTeX实现LaTeX数学公式的实时渲染，为后期Typst数学公式渲染做技术准备。

### 计划任务

#### 🔲 2.1 数学公式节点设计 (Markdown专用)
- [ ] 创建自定义ProseMirror数学节点
- [ ] 实现Inline Math (`$...$`) 和 Display Math (`$$...$$`) 两种模式
- [ ] 设计节点的HTML结构（支持KaTeX渲染）
- [ ] 定义节点的编辑和渲染状态切换逻辑
- [ ] **为后期Typst数学公式节点预留接口**

#### 🔲 2.2 KaTeX渲染集成
- [ ] 集成KaTeX库进行数学公式渲染
- [ ] 实现前端LaTeX到数学公式的转换
- [ ] 优化KaTeX渲染性能（缓存机制）
- [ ] 处理LaTeX语法错误和异常情况
- [ ] **为后期Typst渲染集成预留扩展点**

#### 🔲 2.3 点击编辑系统
- [ ] 实现点击已渲染公式进入编辑模式
- [ ] 创建LaTeX输入编辑器组件
- [ ] 实现编辑状态下的实时LaTeX预览
- [ ] 支持LaTeX语法高亮和自动补全

#### 🔲 2.4 模糊渲染功能
- [ ] 实现编辑器失去焦点时自动渲染
- [ ] 优化渲染时机和用户体验
- [ ] 添加渲染状态指示
- [ ] 支持数学公式的大小和样式调整

#### 📁 预期文件
- `src/components/math-editor/MathNode.ts` - 数学公式ProseMirror节点
- `src/components/math-editor/MathRenderer.vue` - KaTeX数学公式渲染组件
- `src/components/math-editor/LatexEditor.vue` - LaTeX输入编辑器
- `src/components/math-editor/MathExtension.ts` - 数学编辑器扩展

#### ⏱️ 预估时间: 5-7天

---

## 📋 Phase 3: Markdown语法增强系统

### 目标
**完善Markdown语法的支持**，实现完整的Typora克隆体验，为后期Typst语法支持建立架构基础。

### 计划任务

#### 🔲 3.1 完整Markdown语法支持
- [ ] 实现完整的GFM (GitHub Flavored Markdown) 语法
- [ ] 添加表格语法支持和可视化编辑
- [ ] 实现任务列表、删除线、脚注等扩展语法
- [ ] 支持代码块语法高亮 (prism.js/highlight.js)
- [ ] **为后期Typst表格语法预留转换接口**

#### 🔲 3.2 Markdown语法高亮系统
- [ ] 创建Markdown语法定义和解析
- [ ] 实现实时语法高亮
- [ ] 支持嵌套语法的正确高亮
- [ ] 添加暗色主题语法高亮支持
- [ ] **为Typst语法高亮预留架构扩展点**

#### 🔲 3.3 智能补全系统 (Markdown专用)
- [ ] 创建Markdown语法补全数据库
- [ ] 实现代码补全UI组件
- [ ] 支持Emoji、链接、图片等快捷插入
- [ ] 添加Markdown语法模板和片段

#### 🔲 3.4 错误检测和提示
- [ ] 实现Markdown链接有效性检查
- [ ] 添加表格语法格式错误检测
- [ ] 创建语法提示和修复建议
- [ ] 实现文档结构完整性检查

#### 📁 预期文件
- `src/components/syntax-highlight/MarkdownHighlight.ts` - Markdown语法高亮扩展
- `src/components/autocomplete/MarkdownAutocomplete.vue` - Markdown自动补全组件
- `src/components/table/TableEditor.vue` - 表格可视化编辑器
- `src/components/code-highlight/CodeBlock.ts` - 代码块语法高亮

#### ⏱️ 预估时间: 4-6天

---

## 📋 Phase 4: Markdown编辑器性能优化

### 目标
**优化Markdown编辑器的性能**，确保大型Markdown文档的流畅编辑体验，为后期Typst文档优化积累经验。

### 计划任务

#### 🔲 4.1 虚拟滚动优化
- [ ] 实现文档虚拟滚动
- [ ] 优化长Markdown文档的渲染性能
- [ ] 实现智能视口管理
- [ ] 减少DOM节点数量
- [ ] **为后期Typst虚拟滚动预留架构**

#### 🔲 4.2 增量渲染系统
- [ ] 实现文档变更的增量检测
- [ ] 优化数学公式的增量渲染
- [ ] 减少不必要的重新计算
- [ ] 实现Markdown渲染缓存机制
- [ ] **为后期Typst增量编译建立基础**

#### 🔲 4.3 内存优化
- [ ] 优化编辑器的内存使用
- [ ] 实现历史记录的智能管理
- [ ] 减少内存泄漏
- [ ] 优化大型Markdown文档的内存占用

#### 🔲 4.4 响应性能优化
- [ ] 优化用户输入的响应延迟
- [ ] 实现防抖和节流机制
- [ ] 优化键盘事件处理
- [ ] 减少UI阻塞时间

#### 📁 预期文件
- `src/components/virtual-scroll/VirtualScroll.ts` - 虚拟滚动扩展
- `src/utils/incremental-render.ts` - 增量渲染工具
- `src/utils/markdown-cache.ts` - Markdown缓存管理
- `src/utils/performance.ts` - 性能监控工具

#### ⏱️ 预估时间: 3-5天

---

## 🚀 第二阶段: Typst编辑器扩展

**注**: 以下Phase 5-7为第二阶段，在完成第一阶段(完美Markdown编辑器)后开始

---

## 📋 Phase 5: Typst集成和文档管理

### 目标
**扩展支持Typst文件格式**，集成Typst编译和渲染功能，保持Markdown功能的完整性。

### 计划任务

#### 🔲 5.1 Typst文件格式支持
- [ ] 集成Typst 0.14.0编译器
- [ ] 实现Typst文件的读写
- [ ] 支持Typst语法检测和验证
- [ ] 建立Markdown到Typst的转换机制

#### 🔲 5.2 Typst渲染系统
- [ ] 实现Typst文档的实时渲染
- [ ] 支持Typst特有的数学公式渲染
- [ ] 添加Typst图形和表格渲染
- [ ] 优化渲染性能和缓存

#### 🔲 5.3 多格式文档管理
- [ ] 实现Markdown和Typst的统一管理
- [ ] 支持文档格式转换
- [ ] 添加文档类型自动识别
- [ ] 实现统一的项目管理

#### 🔲 5.4 导出功能增强
- [ ] 实现Typst文档PDF导出
- [ ] 支持多种格式的统一导出
- [ ] 添加Typst特有的导出选项
- [ ] 支持批量导出和项目导出

#### 📁 预期文件
- `src-tauri/src/typst_service/compiler.rs` - Typst编译服务
- `src-tauri/src/typst_service/renderer.rs` - Typst渲染服务
- `src/components/typst-editor/TypstRenderer.vue` - Typst渲染组件
- `src/components/file-manager/FileManager.vue` - 多格式文件管理器

#### ⏱️ 预估时间: 6-8天

---

## 📋 Phase 6: Typst语法和功能扩展

### 目标
**完善Typst语法的支持**，实现Typst特有的编辑功能，提供完整的Typst编辑体验。

### 计划任务

#### 🔲 6.1 Typst语法完整支持
- [ ] 实现Typst特有标记语法 (= == 标题)
- [ ] 添加Typst函数和标记支持
- [ ] 实现Typst特有的表格语法
- [ ] 支持Typst特殊符号和表达式

#### 🔲 6.2 Typst智能补全
- [ ] 创建Typst函数补全数据库
- [ ] 实现Typst上下文感知补全
- [ ] 支持Typst函数参数提示
- [ ] 添加Typst模板和片段

#### 🔲 6.3 混合编辑体验
- [ ] 实现Markdown和Typst的混合编辑
- [ ] 支持文档内的语法自动切换
- [ ] 优化不同格式的编辑体验
- [ ] 实现统一的快捷键和操作

#### 🔲 6.4 Typst高级功能
- [ ] 实现Typst参考文献管理
- [ ] 支持Typst图表和图形编辑
- [ ] 添加Typst样式和主题支持
- [ ] 实现Typst文档模板系统

#### 📁 预期文件
- `src/components/typst-syntax/TypstExtension.ts` - Typst语法扩展
- `src/components/typst-complete/TypstAutocomplete.vue` - Typst自动补全
- `src/components/hybrid-editor/HybridEditor.vue` - 混合格式编辑器
- `src/data/typst-functions.json` - Typst函数数据库

#### ⏱️ 预估时间: 5-7天

---

## 📋 Phase 7: 统一编辑器体验和高级功能

### 目标
**完善统一的编辑器体验**，实现高级功能，提供媲美Typora的用户体验。

### 计划任务

#### 🔲 7.1 统一用户界面
- [ ] 实现Markdown和Typst的统一界面
- [ ] 支持格式自适应的UI组件
- [ ] 优化不同格式的视觉一致性
- [ ] 实现无缝的格式切换体验

#### 🔲 7.2 高级编辑功能
- [ ] 实现文档大纲导航 (支持两种格式)
- [ ] 添加协作编辑功能
- [ ] 支持版本控制和历史管理
- [ ] 实现插件系统架构

#### 🔲 7.3 高级导出和分享
- [ ] 支持多格式的统一导出
- [ ] 实现文档分享和协作
- [ ] 添加演示文稿模式
- [ ] 支持云端同步和备份

#### 🔲 7.4 性能优化和稳定性
- [ ] 优化混合文档的性能
- [ ] 实现智能缓存机制
- [ ] 优化内存使用和响应速度
- [ ] 确保跨平台的稳定性

#### 📁 预期文件
- `src/components/unified-editor/UnifiedEditor.vue` - 统一编辑器
- `src/components/collaboration/CollaborationManager.vue` - 协作管理器
- `src/plugins/PluginSystem.ts` - 插件系统
- `src/utils/performance-optimizer.ts` - 性能优化器

#### ⏱️ 预估时间: 6-8天

---

## 📅 总体时间规划

### 第一阶段: 完美Markdown编辑器
| Phase | 内容 | 预估时间 | 状态 |
|-------|------|----------|------|
| Phase 1 | 基础WYSIWYG编辑器 | - | ✅ 已完成 |
| Phase 2 | Markdown数学公式渲染 | 5-7天 | 🔄 下一步 |
| Phase 3 | Markdown语法增强 | 4-6天 | ⏳ 计划中 |
| Phase 4 | Markdown性能优化 | 3-5天 | ⏳ 计划中 |

**第一阶段总计**: 约 12-18 天 (2-3 周)

### 第二阶段: Typst编辑器扩展
| Phase | 内容 | 预估时间 | 状态 |
|-------|------|----------|------|
| Phase 5 | Typst集成和文档管理 | 6-8天 | ⏳ 待定 |
| Phase 6 | Typst语法和功能扩展 | 5-7天 | ⏳ 待定 |
| Phase 7 | 统一编辑器体验 | 6-8天 | ⏳ 待定 |

**第二阶段总计**: 约 17-23 天 (2-3 周)

**项目总计**: 约 29-41 天 (1.5-2.5 个月)

---

## 🎯 优先级建议

### 🚀 第一阶段 - 高优先级 (立即执行)
1. **Phase 2**: Markdown数学公式渲染系统 - 完善Markdown编辑体验
2. **Phase 3**: Markdown语法增强 - 实现完整的Typora克隆
3. **Phase 4**: Markdown性能优化 - 确保基础功能稳定流畅

### 🔄 第二阶段 - 中优先级 (第一阶段完成后)
4. **Phase 5**: Typst集成 - 扩展支持Typst格式
5. **Phase 6**: Typst语法完善 - 完整Typst编辑体验

### ⭐ 长期规划 - 低优先级
6. **Phase 7**: 统一编辑器体验 - 高级功能和扩展

---

## 📝 当前下一步行动

### 🎯 立即开始 Phase 2: Markdown数学公式渲染系统

基于Phase 1的完成情况，专注Markdown编辑器的完善：

1. **创建数学公式ProseMirror节点** (KaTeX渲染)
2. **实现LaTeX数学公式的点击编辑**
3. **优化数学公式的渲染性能**
4. **完善数学公式的用户体验**

**目标**: 打造一个功能完整的Typora克隆编辑器，为后期Typst支持做好技术准备。

---

### 📋 架构设计要点

#### 第一阶段 (Markdown专用)
- **专注**: 完美支持Markdown标准
- **渲染**: 使用KaTeX处理数学公式
- **架构**: 为后期Typst扩展预留接口
- **基础**: 建立性能优化的技术基础

#### 第二阶段 (Typst扩展)
- **集成**: 添加Typst编译和渲染
- **兼容**: 保持Markdown功能完整
- **统一**: 提供一致的用户体验
- **扩展**: 支持多格式的混合编辑

---

*开发计划版本: v2.0*
*最后更新: 2025-12-05*
*当前状态: Phase 1 完成，准备开始 Phase 2 (Markdown数学公式渲染)*