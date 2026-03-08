import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

const SETTINGS_KEY = 'typster_settings_v1'

export interface EditorSettings {
  fontSize: number
  fontFamily: string
  lineHeight: number
  autoPairBrackets: boolean
}

export interface AppearanceSettings {
  themeMode: 'light' | 'dark' | 'auto'
  themePreset: string
  editorWidth: number
}

export interface AutosaveSettings {
  enabled: boolean
  interval: number
}

export interface Settings {
  editor: EditorSettings
  appearance: AppearanceSettings
  autosave: AutosaveSettings
}

const defaultEditorSettings: EditorSettings = {
  fontSize: 16,
  fontFamily: 'system-ui',
  lineHeight: 1.6,
  autoPairBrackets: true,
}

const defaultAppearanceSettings: AppearanceSettings = {
  themeMode: 'auto',
  themePreset: 'github-light',
  editorWidth: 800,
}

const defaultAutosaveSettings: AutosaveSettings = {
  enabled: true,
  interval: 60,
}

const loadSettings = (): Partial<Settings> => {
  const saved = localStorage.getItem(SETTINGS_KEY)
  if (saved) {
    try {
      return JSON.parse(saved)
    } catch (e) {
      console.error('Failed to parse settings:', e)
    }
  }
  return {}
}

export const useSettingsStore = defineStore('settings', () => {
  const loaded = loadSettings()

  const editor = ref<EditorSettings>({
    ...defaultEditorSettings,
    ...loaded.editor,
  })

  const appearance = ref<AppearanceSettings>({
    ...defaultAppearanceSettings,
    ...loaded.appearance,
  })

  const autosave = ref<AutosaveSettings>({
    ...defaultAutosaveSettings,
    ...loaded.autosave,
  })

  // 持久化设置
  watch([editor, appearance, autosave], () => {
    localStorage.setItem(SETTINGS_KEY, JSON.stringify({
      editor: editor.value,
      appearance: appearance.value,
      autosave: autosave.value,
    }))
  }, { deep: true })

  // 更新编辑器设置
  const updateEditorSettings = (newSettings: Partial<EditorSettings>) => {
    editor.value = { ...editor.value, ...newSettings }
  }

  // 更新外观设置
  const updateAppearanceSettings = (newSettings: Partial<AppearanceSettings>) => {
    appearance.value = { ...appearance.value, ...newSettings }
  }

  // 更新自动保存设置
  const updateAutosaveSettings = (newSettings: Partial<AutosaveSettings>) => {
    autosave.value = { ...autosave.value, ...newSettings }
  }

  // 重置所有设置
  const resetAllSettings = () => {
    editor.value = { ...defaultEditorSettings }
    appearance.value = { ...defaultAppearanceSettings }
    autosave.value = { ...defaultAutosaveSettings }
  }

  return {
    editor,
    appearance,
    autosave,
    updateEditorSettings,
    updateAppearanceSettings,
    updateAutosaveSettings,
    resetAllSettings,
  }
})
