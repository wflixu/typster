<template>
  <div class="appearance-settings">
    <h2>外观设置</h2>

    <div class="settings-group">
      <h3>主题</h3>

      <div class="setting-item">
        <label for="theme-mode">主题模式</label>
        <div class="setting-control">
          <Select
            id="theme-mode"
            v-model="settings.themeMode"
            :options="themeModeOptions"
            optionLabel="label"
            optionValue="value"
            placeholder="选择主题模式"
            @update:modelValue="updateSetting"
          />
        </div>
        <small>跟随系统或固定使用浅色/深色主题</small>
      </div>

      <div class="setting-item">
        <label for="theme-preset">主题预设</label>
        <div class="setting-control">
          <Select
            id="theme-preset"
            v-model="settings.themePreset"
            :options="themePresets"
            optionLabel="name"
            optionValue="file"
            placeholder="选择主题"
            @update:modelValue="updateTheme"
          />
        </div>
        <small>选择颜色主题</small>
      </div>
    </div>

    <div class="settings-group">
      <h3>编辑器布局</h3>

      <div class="setting-item">
        <label for="editor-width">编辑器宽度</label>
        <div class="setting-control">
          <InputNumber
            id="editor-width"
            v-model="settings.editorWidth"
            :min="600"
            :max="1200"
            :step="50"
            suffix=" px"
            @update:modelValue="updateSetting"
          />
        </div>
        <small>设置编辑器的最大宽度</small>
      </div>
    </div>

    <!-- 主题预览 -->
    <div class="settings-group">
      <h3>主题预览</h3>
      <div class="theme-preview">
        <div class="preview-content">
          <h1>标题样式</h1>
          <p>这是正文文本样式预览。编辑器将使用您选择的主题进行渲染。</p>
          <ul>
            <li>列表项 1</li>
            <li>列表项 2</li>
          </ul>
          <code>inline code</code>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, onMounted } from 'vue'
import Select from 'primevue/select'
import InputNumber from 'primevue/inputnumber'

const themeModeOptions = [
  { label: '跟随系统', value: 'auto' },
  { label: '浅色', value: 'light' },
  { label: '深色', value: 'dark' },
]

const themePresets = [
  { name: 'GitHub Light', file: 'github-light', type: 'light' },
  { name: 'GitHub Dark', file: 'github-dark', type: 'dark' },
  { name: 'Nord', file: 'nord', type: 'dark' },
  { name: 'Dracula', file: 'dracula', type: 'dark' },
]

const settings = reactive({
  themeMode: 'auto',
  themePreset: 'github-light',
  editorWidth: 800,
})

const loadSettings = () => {
  const saved = localStorage.getItem('typster_appearance_settings')
  if (saved) {
    try {
      const parsed = JSON.parse(saved)
      Object.assign(settings, parsed)
    } catch (e) {
      console.error('Failed to parse appearance settings:', e)
    }
  }
}

const updateSetting = () => {
  localStorage.setItem('typster_appearance_settings', JSON.stringify(settings))
  window.dispatchEvent(new CustomEvent('appearance-settings-update', {
    detail: settings
  }))
}

const updateTheme = () => {
  updateSetting()
  // 应用主题
  const theme = themePresets.find(t => t.file === settings.themePreset)
  if (theme) {
    applyTheme(theme)
  }
}

const applyTheme = (theme: typeof themePresets[0]) => {
  // 移除旧主题
  document.querySelectorAll('link[data-theme]').forEach(el => el.remove())

  // 添加新主题（如果主题文件存在）
  // TODO: 在后续步骤中创建主题 CSS 文件后取消注释
  // const link = document.createElement('link')
  // link.rel = 'stylesheet'
  // link.href = `/src/styles/tokens/${theme.file}.css`
  // link.setAttribute('data-theme', theme.name)
  // document.head.appendChild(link)

  console.log('Applied theme:', theme.name)
}

onMounted(() => {
  loadSettings()
})
</script>

<style scoped>
.appearance-settings {
  max-width: 600px;
}

.appearance-settings h2 {
  font-size: var(--font-size-large, 18px);
  font-weight: 600;
  margin-bottom: var(--spacing-lg, 24px);
  color: var(--color-fg-primary, #24292f);
}

.settings-group {
  margin-bottom: var(--spacing-xl, 32px);
}

.settings-group h3 {
  font-size: var(--font-size-base, 16px);
  font-weight: 600;
  margin-bottom: var(--spacing-md, 16px);
  color: var(--color-fg-primary, #24292f);
  border-bottom: 1px solid var(--color-border-light, #e1e4e8);
  padding-bottom: var(--spacing-sm, 8px);
}

.setting-item {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm, 8px);
  margin-bottom: var(--spacing-md, 16px);
}

.setting-item label {
  font-size: var(--font-size-small, 14px);
  font-weight: 500;
  color: var(--color-fg-primary, #24292f);
}

.setting-control {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm, 8px);
}

.setting-item small {
  font-size: 12px;
  color: var(--color-fg-tertiary, #8b949e);
}

.theme-preview {
  border: 1px solid var(--color-border, #d0d7de);
  border-radius: var(--radius-md, 6px);
  padding: var(--spacing-md, 16px);
  background: var(--color-bg-secondary, #f6f8fa);
}

.preview-content h1 {
  font-size: 1.5em;
  margin-bottom: var(--spacing-sm, 8px);
}

.preview-content p {
  margin-bottom: var(--spacing-sm, 8px);
}

.preview-content code {
  background: var(--code-bg, #f6f8fa);
  padding: 2px 6px;
  border-radius: var(--radius-sm, 3px);
  font-family: var(--font-family-code, monospace);
}
</style>
