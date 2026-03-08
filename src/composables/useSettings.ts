import { ref, watch, onMounted } from 'vue'
import { useSettingsStore } from '../store/settings-store'

export interface SettingsEvents {
  'editor-settings-update': Partial<EditorSettings>
  'appearance-settings-update': Partial<AppearanceSettings>
  'autosave-settings-update': Partial<AutosaveSettings>
}

export function useSettings() {
  const settingsStore = useSettingsStore()

  // 应用编辑器设置到编辑器 DOM
  const applyEditorSettings = () => {
    const editor = document.querySelector('.tiptap-editor')
    if (!editor) return

    const { fontSize, fontFamily, lineHeight } = settingsStore.editor

    // 应用编辑器样式
    editor.setAttribute('style', `
      font-size: ${fontSize}px;
      font-family: ${fontFamily};
      line-height: ${lineHeight};
    `)
  }

  // 应用外观设置
  const applyAppearanceSettings = () => {
    const { themeMode, themePreset, editorWidth } = settingsStore.appearance

    // 设置编辑器宽度
    const editor = document.querySelector('.tiptap-editor')
    if (editor) {
      (editor as HTMLElement).style.setProperty('--editor-width', `${editorWidth}px`)
    }

    // 应用主题
    applyTheme(themeMode, themePreset)
  }

  // 应用主题
  const applyTheme = (mode: string, preset: string) => {
    const root = document.documentElement

    // 设置主题模式
    if (mode === 'dark') {
      root.classList.add('dark-theme')
    } else if (mode === 'light') {
      root.classList.remove('dark-theme')
    } else {
      // 跟随系统
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      root.classList.toggle('dark-theme', prefersDark)
    }

    // TODO: 在创建主题 CSS 文件后应用主题预设
    console.log('Applied theme:', { mode, preset })
  }

  // 初始化设置
  const initSettings = () => {
    applyEditorSettings()
    applyAppearanceSettings()
  }

  // 监听设置变化
  watch(() => settingsStore.editor, () => {
    applyEditorSettings()
  }, { deep: true })

  watch(() => settingsStore.appearance, () => {
    applyAppearanceSettings()
  }, { deep: true })

  // 系统主题变化监听
  onMounted(() => {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
    mediaQuery.addEventListener('change', () => {
      if (settingsStore.appearance.themeMode === 'auto') {
        applyAppearanceSettings()
      }
    })

    // 初始化设置
    initSettings()
  })

  return {
    settingsStore,
    applyEditorSettings,
    applyAppearanceSettings,
    initSettings,
  }
}
