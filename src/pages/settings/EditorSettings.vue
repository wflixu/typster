<template>
  <div class="editor-settings">
    <h2>编辑器设置</h2>

    <div class="settings-group">
      <h3>字体</h3>

      <div class="setting-item">
        <label for="font-size">字体大小</label>
        <div class="setting-control">
          <InputNumber
            id="font-size"
            v-model="settings.fontSize"
            :min="12"
            :max="24"
            :step="1"
            suffix=" px"
            @update:modelValue="updateSetting"
          />
        </div>
      </div>

      <div class="setting-item">
        <label for="font-family">字体家族</label>
        <div class="setting-control">
          <InputText
            id="font-family"
            v-model="settings.fontFamily"
            placeholder="system-ui, -apple-system, sans-serif"
            @blur="updateSetting"
          />
        </div>
        <small>使用系统默认字体，或指定自定义字体</small>
      </div>

      <div class="setting-item">
        <label for="line-height">行高</label>
        <div class="setting-control">
          <InputNumber
            id="line-height"
            v-model="settings.lineHeight"
            :min="1.0"
            :max="2.5"
            :step="0.1"
            @update:modelValue="updateSetting"
          />
        </div>
      </div>
    </div>

    <div class="settings-group">
      <h3>编辑行为</h3>

      <div class="setting-item">
        <label for="auto-pair">自动配对括号</label>
        <div class="setting-control">
          <InputSwitch
            id="auto-pair"
            v-model="settings.autoPairBrackets"
            @update:modelValue="updateSetting"
          />
        </div>
        <small>自动补全括号、引号等符号</small>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, onMounted } from 'vue'
import InputNumber from 'primevue/inputnumber'
import InputText from 'primevue/inputtext'
import InputSwitch from 'primevue/inputswitch'

const settings = reactive({
  fontSize: 16,
  fontFamily: 'system-ui',
  lineHeight: 1.6,
  autoPairBrackets: true,
})

const loadSettings = () => {
  const saved = localStorage.getItem('typster_editor_settings')
  if (saved) {
    try {
      const parsed = JSON.parse(saved)
      Object.assign(settings, parsed)
    } catch (e) {
      console.error('Failed to parse editor settings:', e)
    }
  }
}

const updateSetting = () => {
  localStorage.setItem('typster_editor_settings', JSON.stringify(settings))
  // 发送设置更新事件
  window.dispatchEvent(new CustomEvent('editor-settings-update', {
    detail: settings
  }))
}

onMounted(() => {
  loadSettings()
})
</script>

<style scoped>
.editor-settings {
  max-width: 600px;
}

.editor-settings h2 {
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
</style>
