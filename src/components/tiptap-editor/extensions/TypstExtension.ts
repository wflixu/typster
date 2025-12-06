import { InputRule } from 'prosemirror-inputrules'
import { Extension } from '@tiptap/core'
import { Heading } from '@tiptap/extension-heading'
import { Bold } from '@tiptap/extension-bold'
import { Italic } from '@tiptap/extension-italic'

// Typst input rules for markdown-like shortcuts
const typstInputRules = [
  // Heading shortcuts: # ## ### #### ##### ######
  new InputRule(/^(#{1,6})\s$/, (state, match, start, end) => {
    const level = match[1].length
    return state.tr.replaceWith(
      start,
      end,
      state.schema.nodes.heading.create({ level })
    )
  }),

  // Bold text: **text** or *text*
  new InputRule(/\*\*([^*]+)\*\*$/, (state, match, start, end) => {
    return state.tr.replaceWith(
      start,
      end,
      state.schema.nodes.text.create(match[1])
    ).addMark(start, end, state.schema.marks.bold.create())
  }),

  // Italic text: _text_ or *text*
  new InputRule(/_([^_]+)_$/, (state, match, start, end) => {
    return state.tr.replaceWith(
      start,
      end,
      state.schema.nodes.text.create(match[1])
    ).addMark(start, end, state.schema.marks.italic.create())
  }),

  // Inline code: `code`
  new InputRule(/`([^`]+)`$/, (state, match, start, end) => {
    return state.tr.replaceWith(
      start,
      end,
      state.schema.nodes.code.create({ text: match[1] })
    )
  }),

  // Math inline: $formula$
  // This is a placeholder for future math support
  new InputRule(/\$([^$\n]+)\$/, (state, match, start, end) => {
    // For now, just treat as text with math notation
    return state.tr.replaceWith(
      start,
      end,
      state.schema.nodes.text.create(`$${match[1]}$`)
    )
  }),

  // Display math: $$formula$$
  // This is a placeholder for future math support
  new InputRule(/\$\$([^$\n]+)\$\$/, (state, match, start, end) => {
    // For now, just treat as text with display math notation
    return state.tr.replaceWith(
      start,
      end,
      state.schema.nodes.text.create(`$$${match[1]}$$`)
    )
  }),

  // Horizontal rule: ---
  new InputRule(/^---$/, (state, match, start, end) => {
    return state.tr.replaceWith(
      start,
      end,
      state.schema.nodes.horizontal_rule.create()
    )
  }),

  // Blockquote: > text
  new InputRule(/^>\s$/, (state, match, start, end) => {
    return state.tr.replaceWith(
      start,
      end,
      state.schema.nodes.blockquote.create({}, state.schema.nodes.paragraph.create())
    )
  }),

  // Unordered list: - or *
  new InputRule(/^[-*]\s$/, (state, match, start, end) => {
    return state.tr.replaceWith(
      start,
      end,
      state.schema.nodes.bullet_list.create({}, [
        state.schema.nodes.list_item.create({}, state.schema.nodes.paragraph.create())
      ])
    )
  }),

  // Ordered list: 1. 2. 3.
  new InputRule(/^\d+\.\s$/, (state, match, start, end) => {
    return state.tr.replaceWith(
      start,
      end,
      state.schema.nodes.ordered_list.create({}, [
        state.schema.nodes.list_item.create({}, state.schema.nodes.paragraph.create())
      ])
    )
  }),
]

// 简化的Markdown扩展 - 专注于基础语法
export const SimpleMarkdownExtension = Extension.create({
  name: 'simple-markdown',

  addInputRules() {
    return typstInputRules
  },

  addKeyboardShortcuts() {
    return {
      // Ctrl/Cmd + B for bold
      'Mod-b': () => this.editor.commands.toggleBold(),
      // Ctrl/Cmd + I for italic
      'Mod-i': () => this.editor.commands.toggleItalic(),
      // Ctrl/Cmd + K for code
      'Mod-k': () => this.editor.commands.toggleCode(),
    }
  },
})