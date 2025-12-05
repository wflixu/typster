# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Typster is a modern cross-platform Typst editor built with Tauri 2.0 (Rust backend) and Vue 3 (frontend), featuring a Typst-like WYSIWYG editing experience. The application provides seamless "what you see is what you get" document authoring with real-time mathematical formula rendering, similar to Typora, but specifically designed for Typst markup language.

## Architecture

### 1. **Overall Architecture**

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │   Backend       │    │   Configuration │
│   (Vue 3)        │    │   (Rust)        │    │   System        │
├─────────────────┤    ├─────────────────┤    ├─────────────────┤
│ • WYSIWYG Editor│    │ • Typst Service │    │ • TOML-based    │
│ • Tiptap Core   │    │ • IPC Commands   │    │ • Global/Project│
│ • Math Rendering│    │ • File System    │    │ • Runtime       │
│ • Formula Nodes │    │ • Compiler       │    │ • Validation    │
│ • State Mgmt    │    │ • Package Mgmt  │    │ • Repair        │
└─────────────────┘    └─────────────────┘    └─────────────────┘
        │                       │                       │
        └───────────────────────┼───────────────────────┘
                                │
                    ┌─────────────────┐
                    │   IPC Bridge    │
                    │ (Tauri API)     │
                    └─────────────────┘
```

### 2. **Technology Stack**

**Frontend (Vue 3 + TypeScript)**
- Vue 3 with Composition API
- TypeScript for type safety
- PrimeVue UI library
- Tiptap for WYSIWYG editing (primary editor)
- Custom math formula nodes
- KaTeX/MathJax for math rendering
- Pinia for state management
- Vue Router for navigation

**Backend (Rust + Tauri 2.0)**
- Rust 2021 edition
- Tauri 2.0 for cross-platform native apps
- Complete Typst 0.14.0 ecosystem
- TOML configuration
- Tokio for async runtime
- Serde for serialization

**Key Dependencies**
- **Frontend**: `@tiptap/vue-3`, `@tiptap/pm`, `katex`, `primevue`, `pinia`
- **Backend**: `typst = "0.14.0"` (complete ecosystem), `tauri = "2.9.3"`, `tokio`, `serde`, `toml`

## Development Commands

### **Development Environment Setup**

```bash
# Install frontend dependencies
pnpm install

# Run development server (Tauri + Vite)
pnpm start    # or `pnpm tauri dev`
# Runs on http://localhost:1420

# Build for production
pnpm pack     # or `pnpm tauri build`
# Creates platform-specific binaries
```

### **Available Scripts**

```json
{
  "dev": "vite",                    # Frontend dev server
  "build": "vue-tsc --noEmit && vite build",
  "preview": "vite preview",       # Preview production build
  "tauri": "tauri",                # Tauri CLI wrapper
  "td": "tauri dev",                # Tauri development mode
  "tb": "tauri build",              # Tauri production build
  "start": "pnpm tauri dev",       # Start full development
  "pack": "pnpm tauri build"        # Production packaging
}
```

### **Development Workflow**

1. **Frontend Development**: `pnpm dev` - Hot reload Vue components
2. **Full Development**: `pnpm start` - Tauri app with devtools
3. **Production Build**: `pnpm pack` - Platform-specific binaries

## Critical Technical Patterns

### 1. **IPC Command System**

The application uses Tauri's command system for frontend-backend communication:

```rust
// src-tauri/src/lib.rs - Command Registration
.invoke_handler(tauri::generate_handler![
    cmds::doc::load_doc_from_path,
    cmds::fs::fs_list_dir,
    cmds::fs::fs_read_file_text,
    cmds::typst::typst_compile_doc,
    cmds::typst::typst_render,
    cmds::typst::typst_autocomplete,
    cmds::typst::export_pdf,
])
```

**Key Commands:**
- `typst_compile_doc` - Compile Typst documents
- `fs_read_file_text` - File system operations
- `export_pdf` - PDF export functionality

### 2. **WYSIWYG Editor Architecture**

The core is a unified Typst-like editor using Tiptap as the foundation:

```
TypstEditor.vue (Main Editor)
├── TiptapEditor.vue
│   ├── Rich text editing foundation
│   ├── Typst-specific extensions
│   ├── Math formula nodes
│   ├── Markdown-like shortcuts
│   └── Real-time rendering
├── MathNode.ts (Custom ProseMirror Node)
│   ├── Click-to-edit behavior
│   ├── LaTeX math rendering
│   ├── Inline and display modes
│   └── Blur-to-render functionality
└── TypstExtensions.ts
    ├── Typst markup support
    ├── Heading shortcuts (# ##)
    ├── Bold/italic shortcuts
    └── Auto-completion
```

**Math Formula Editing Flow:**
```typescript
// Click-to-edit, blur-to-render pattern
const handleMathNodeClick = (mathNode: MathNode) => {
  // Switch to edit mode
  mathNode.setEditable(true)
  // Focus the math input
  mathNode.focusEditor()
}

const handleMathNodeBlur = (mathNode: MathNode) => {
  // Render back to visual mode
  mathNode.setEditable(false)
  // Convert LaTeX to rendered math
  renderMathFormula(mathNode)
}
```

### 3. **Typst Compilation Pipeline**

The backend implements a complete Typst 0.14.0 compilation pipeline:

```rust
// src-tauri/src/typst_service/compiler.rs
pub fn compile(timer: &mut Timer, args: &CompileArgs) -> StrResult<()> {
    let mut config = CompileConfig::new(args)?;
    let mut world = SystemWorld::new(&args.input, &world_args, &process_args)?;

    timer.record(&mut world, |world| compile_once(world, &mut config))?
}
```

**Compilation Flow:**
1. Create `SystemWorld` with file system access
2. Parse and compile Typst source
3. Export to various formats (PDF, PNG, SVG)
4. Handle diagnostics and errors
5. Cache rendered output for performance

### 4. **Configuration System**

Three-tier configuration architecture:

```rust
// src-tauri/src/config/mod.rs
pub enum ConfigSource {
    Global,   // ~/.config/typster/config.toml
    Project,  // {project}/typst.toml
    User,     // User preferences
}
```

**Configuration Hierarchy:**
1. **Global Config**: System-wide settings, fonts, paths
2. **Project Config**: Per-project overrides
3. **Runtime Config**: Merged and resolved configuration

### 5. **State Management with Pinia**

Centralized state management:

```typescript
// src/store/store.ts
const useSystemStoreHook = defineStore("system", () => {
  const editingFilePath = ref("")
  const projects = reactive<IProject[]>([])
  const editingProject = ref<IProject | null>(null)
  const mode = ref<IMode>("all")

  // Persistence via localStorage
  const setEditingFilePath = (val: string) => {
    editingFilePath.value = val
    window.localStorage.setItem(EDITING_FILE, val)
  }
})
```

## File Structure Highlights

### **Frontend Structure**

```
src/
├── components/
│   ├── TypstEditor.vue       # Main WYSIWYG editor interface
│   ├── tiptap-editor/        # Tiptap-based editing core
│   │   ├── TiptapEditor.vue  # Rich text editor foundation
│   │   └── extensions/       # Typst-specific extensions
│   │       ├── MathNode.ts   # Custom math formula node
│   │       ├── TypstExtensions.ts
│   │       └── Shortcuts.ts  # Markdown-like shortcuts
│   └── math-renderer/        # Math formula rendering
│       ├── MathRenderer.vue  # KaTeX integration
│       └── formula-parser.ts # LaTeX to Typst conversion
├── pages/
│   ├── home/
│   │   ├── Home.vue          # Main layout
│   │   └── Sidebar.vue       # Project sidebar
│   ├── project/
│   │   ├── Project.vue       # Project management
│   │   └── AddProject.vue    # Project creation
│   └── typst/
│       ├── interface.ts      # Type definitions
│       └── editor-shortcuts.ts
├── store/
│   └── store.ts             # Pinia state management
└── shared/
    ├── util.ts              # Utility functions
    ├── typst-converter.ts   # HTML/Typst conversion
    └── math-utils.ts        # Math formula utilities
```

### **Backend Structure**

```
src-tauri/src/
├── cmds/                   # IPC command handlers
│   ├── mod.rs              # Command module exports
│   ├── typst.rs            # Typst compilation commands
│   ├── fs.rs               # File system operations
│   └── doc.rs              # Document handling
├── typst_service/          # Typst 0.14.0 integration
│   ├── mod.rs              # Service module exports
│   ├── compiler.rs         # Compilation engine
│   ├── world.rs            # File system abstraction
│   ├── config.rs           # Configuration management
│   ├── args.rs             # Command arguments
│   └── ...                 # Typst ecosystem modules
├── config/                 # Configuration system
│   ├── mod.rs              # Configuration architecture
│   ├── global.rs           # Global config manager
│   ├── project.rs          # Project config manager
│   └── ...
├── lib.rs                  # Main application entry
└── main.rs                 # Tauri application startup
```

### **Key Interfaces and Types**

```typescript
// Frontend type definitions
export interface TypstCompileResult {
  pages: TypstPage[]
  diagnostics: TypstSourceDiagnostic[]
}

export interface TypstPage {
  hash: string
  width: number
  height: number
  num: number
}

export type IMode = "all" | "edit" | "preview"
```

## Advanced Features

### 1. **WYSIWYG Editing Experience**

- **Typst-like Interface**: Clean, distraction-free editing like Typora
- **Real-time Math Rendering**: Formulas render as you type with blur-to-render
- **Markdown Shortcuts**: Type `# ` for headings, `*text*` for bold, etc.
- **Inline Math**: `$formula$` renders inline math
- **Display Math**: `$$formula$$` renders display math

### 2. **Math Formula System**

- **Click-to-Edit**: Click any rendered formula to edit LaTeX source
- **Blur-to-Render**: Formulas automatically render when you click away
- **LaTeX Support**: Full LaTeX math syntax compatibility
- **KaTeX Integration**: Fast math formula rendering
- **Typst Math**: Backend compilation for perfect Typst math output

### 3. **Smart Document Editing**

- **Auto-completion**: Typst markup and function suggestions
- **Syntax Highlighting**: Visual feedback for Typst syntax
- **Error Detection**: Real-time syntax and semantic error checking
- **Auto-save**: Automatic saving with configurable intervals
- **Document Outline**: Navigation through document structure

### 4. **Export and Integration**

- **PDF Export**: Native Typst compilation to PDF
- **Cross-platform**: Native performance on Windows, macOS, Linux
- **File Management**: Project-based file organization
- **Recent Documents**: Quick access to recent files

## Development Guidelines

### **Frontend Development**

1. **Vue 3 Composition API**: Use `<script setup>` syntax
2. **TypeScript**: Strict type checking enabled
3. **Component Organization**: Feature-based structure
4. **State Management**: Pinia stores for global state

### **Backend Development**

1. **Rust Patterns**: Async/await with Tokio
2. **Error Handling**: Result types with proper error propagation
3. **Configuration**: TOML-based with validation
4. **Performance**: Parallel compilation and caching

### **IPC Communication**

1. **Command Registration**: All commands registered in `lib.rs`
2. **Type Safety**: Shared types between frontend and backend
3. **Async Operations**: All commands are async
4. **Error Handling**: Proper error propagation to frontend

## WYSIWYG Editor Development

### **Key Components**

1. **TypstEditor.vue**: Main unified editor interface (replaces hybrid approach)
2. **TiptapEditor.vue**: Core WYSIWYG editing engine with Typst extensions
3. **MathNode.ts**: Custom ProseMirror node for math formulas
4. **MathRenderer.vue**: KaTeX-based math formula rendering component

### **Math Formula Editing Pattern**

```typescript
// Click-to-edit, blur-to-render implementation
class MathNodeView {
  constructor(node, view, getPos) {
    this.node = node
    this.view = view
    this.getPos = getPos
    this.isEditing = false

    this.dom = this.createDOM()
    this.contentDOM = null // Math nodes don't have editable content by default
  }

  selectNode() {
    this.dom.classList.add('ProseMirror-selectednode')
    if (!this.isEditing) {
      this.enterEditMode()
    }
  }

  deselectNode() {
    this.dom.classList.remove('ProseMirror-selectednode')
    if (this.isEditing) {
      this.exitEditMode()
    }
  }

  enterEditMode() {
    this.isEditing = true
    this.showLatexEditor()
  }

  exitEditMode() {
    this.isEditing = false
    this.renderMathFormula()
  }
}
```

### **Typst Markup Integration**

The system uses ProseMirror plugins to handle Typst-specific markup:

```typescript
// Typst markup input rules
const typstInputRules = [
  // Heading shortcuts
  textInputRule({ regex: /^(#{1,6})\s$/, getMatch },
    (state, match, start, end) => {
      const level = match[1].length
      return state.tr.replaceWith(
        start,
        end,
        state.schema.nodes.heading.create({ level })
      )
    }),

  // Math inline shortcuts
  textInputRule({ regex: /\$([^$\n]+)\$/, getMatch },
    (state, match, start, end) => {
      return state.tr.replaceWith(
        start,
        end,
        state.schema.nodes.math_inline.create({ latex: match[1] })
      )
    })
]
```

This architecture provides a seamless, Typst-like WYSIWYG editing experience with real-time math rendering, eliminating the need for mode switching while maintaining the power of Typst's compilation system for perfect output generation.