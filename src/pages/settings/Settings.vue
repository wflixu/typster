<template>
  <div class="settings-page">
    <div class="settings-header">
      <h1>设置</h1>
      <button class="icon-button" @click="closeSettings" aria-label="关闭">
        <i class="pi pi-times"></i>
      </button>
    </div>

    <div class="settings-content">
      <TabMenu :model="tabs" v-model:activeIndex="activeTab" />

      <div class="settings-panel">
        <EditorSettings v-if="activeTab === 0" />
        <AppearanceSettings v-if="activeTab === 1" />
        <AutosaveSettings v-if="activeTab === 2" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import TabMenu from 'primevue/tabmenu'
import EditorSettings from './EditorSettings.vue'
import AppearanceSettings from './AppearanceSettings.vue'
import AutosaveSettings from './AutosaveSettings.vue'

const router = useRouter()
const activeTab = ref(0)

const tabs = [
  { label: '编辑器', icon: 'pi pi-file-edit' },
  { label: '外观', icon: 'pi pi-palette' },
  { label: '自动保存', icon: 'pi pi-save' },
]

const closeSettings = () => {
  router.back()
}
</script>

<style scoped>
.settings-page {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--color-bg-primary, #ffffff);
}

.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-md, 16px) var(--spacing-lg, 24px);
  border-bottom: 1px solid var(--color-border-light, #e1e4e8);
}

.settings-header h1 {
  font-size: var(--font-size-large, 18px);
  font-weight: 600;
  color: var(--color-fg-primary, #24292f);
  margin: 0;
}

.icon-button {
  width: 32px;
  height: 32px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-sm, 3px);
  cursor: pointer;
  transition: background 0.15s ease;
  background: transparent;
  border: none;
  color: var(--color-fg-secondary, #57606a);
}

.icon-button:hover {
  background: var(--color-bg-tertiary, #f0f2f5);
}

.settings-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.settings-panel {
  flex: 1;
  padding: var(--spacing-lg, 24px);
  overflow-y: auto;
}
</style>
