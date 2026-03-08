<template>
  <div class="autosave-settings">
    <h2>自动保存设置</h2>

    <div class="settings-group">
      <h3>保存选项</h3>

      <div class="setting-item">
        <label for="autosave-enabled">启用自动保存</label>
        <div class="setting-control">
          <InputSwitch
            id="autosave-enabled"
            v-model="settings.enabled"
            @update:modelValue="updateSetting"
          />
        </div>
        <small>编辑器会自动保存您的更改</small>
      </div>

      <div class="setting-item" :class="{ disabled: !settings.enabled }">
        <label for="autosave-interval">保存间隔</label>
        <div class="setting-control">
          <InputNumber
            id="autosave-interval"
            v-model="settings.interval"
            :min="10"
            :max="300"
            :step="10"
            :disabled="!settings.enabled"
            suffix=" 秒"
            @update:modelValue="updateSetting"
          />
        </div>
        <small>自动保存的时间间隔（10-300 秒）</small>
      </div>
    </div>

    <div class="settings-group">
      <h3>保存状态</h3>
      <div class="save-status">
        <div class="status-item">
          <span class="status-label">上次保存:</span>
          <span class="status-value">{{ lastSaveTime || '从未保存' }}</span>
        </div>
        <div class="status-item">
          <span class="status-label">未保存更改:</span>
          <span class="status-value">{{ hasUnsavedChanges ? '是' : '否' }}</span>
        </div>
      </div>
    </div>

    <div class="settings-info">
      <i class="pi pi-info-circle"></i>
      <span>您也可以使用 <kbd>Cmd/Ctrl + S</kbd> 快捷键手动保存文档</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, onMounted, ref, onUnmounted } from 'vue'
import InputSwitch from 'primevue/inputswitch'
import InputNumber from 'primevue/inputnumber'

const settings = reactive({
  enabled: true,
  interval: 60,
})

const lastSaveTime = ref('')
const hasUnsavedChanges = ref(false)

const loadSettings = () => {
  const saved = localStorage.getItem('typster_autosave_settings')
  if (saved) {
    try {
      const parsed = JSON.parse(saved)
      Object.assign(settings, parsed)
    } catch (e) {
      console.error('Failed to parse autosave settings:', e)
    }
  }
}

const updateSetting = () => {
  localStorage.setItem('typster_autosave_settings', JSON.stringify(settings))
  window.dispatchEvent(new CustomEvent('autosave-settings-update', {
    detail: settings
  }))
}

const handleSaveStatus = (event: CustomEvent) => {
  if (event.detail.type === 'saved') {
    lastSaveTime.value = new Date().toLocaleTimeString()
    hasUnsavedChanges.value = false
  }
}

const handleUnsavedChanges = (event: CustomEvent) => {
  hasUnsavedChanges.value = event.detail
}

onMounted(() => {
  loadSettings()
  window.addEventListener('save-status', handleSaveStatus as EventListener)
  window.addEventListener('unsaved-changes', handleUnsavedChanges as EventListener)
})

onUnmounted(() => {
  window.removeEventListener('save-status', handleSaveStatus as EventListener)
  window.removeEventListener('unsaved-changes', handleUnsavedChanges as EventListener)
})
</script>

<style scoped>
.autosave-settings {
  max-width: 600px;
}

.autosave-settings h2 {
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

.setting-item.disabled {
  opacity: 0.5;
  pointer-events: none;
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

.save-status {
  background: var(--color-bg-secondary, #f6f8fa);
  border: 1px solid var(--color-border, #d0d7de);
  border-radius: var(--radius-md, 6px);
  padding: var(--spacing-md, 16px);
}

.status-item {
  display: flex;
  justify-content: space-between;
  padding: var(--spacing-sm, 8px) 0;
}

.status-item:not(:last-child) {
  border-bottom: 1px solid var(--color-border-light, #e1e4e8);
}

.status-label {
  font-size: var(--font-size-small, 14px);
  color: var(--color-fg-secondary, #57606a);
}

.status-value {
  font-size: var(--font-size-small, 14px);
  font-weight: 500;
  color: var(--color-fg-primary, #24292f);
}

.settings-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm, 8px);
  padding: var(--spacing-md, 16px);
  background: var(--color-bg-secondary, #f6f8fa);
  border: 1px solid var(--color-border, #d0d7de);
  border-radius: var(--radius-md, 6px);
  font-size: var(--font-size-small, 14px);
  color: var(--color-fg-secondary, #57606a);
}

.settings-info i {
  color: var(--color-info, #0969da);
}

kbd {
  background: var(--color-bg-tertiary, #f0f2f5);
  border: 1px solid var(--color-border, #d0d7de);
  border-radius: var(--radius-sm, 3px);
  padding: 2px 6px;
  font-family: var(--font-family-code, monospace);
  font-size: 12px;
}
</style>
