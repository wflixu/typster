<template>
  <div class="sidebar">
    <SidebarToggle v-if="systemStore.showSidebar" class="toggle"/>
    <div class="title">
      <span>
        Files
      </span>
      <a-button :icon="h(PlusOutlined)"></a-button>
    </div>
    <a-directory-tree class="dir" :blockNode="true" :height="300" v-model:expandedKeys="expandedKeys"
      v-model:selectedKeys="selectedKeys" :tree-data="treeData" @select="onSelect"></a-directory-tree>

    <div class="footer">
      footer
    </div>
  </div>
</template>

<script setup lang="ts">
import type { TreeProps } from 'ant-design-vue';
import { ref, h, reactive, onMounted } from 'vue';
import { PlusOutlined } from '@ant-design/icons-vue'
import { readDir, BaseDirectory, FileEntry } from '@tauri-apps/api/fs';
import { useSystemStoreHook } from '../../store/store';
import { DataNode } from 'ant-design-vue/es/tree';
import SidebarToggle from './SidebarToggle.vue';

const systemStore = useSystemStoreHook();

const expandedKeys = ref<string[]>([]);
const selectedKeys = ref<string[]>([]);
const treeData: TreeProps['treeData'] = reactive([]);


const initFiles = async () => {
  console.log(systemStore.editingProject)
  const curProject = systemStore.editingProject
  if (!curProject) {
    return
  }

  const root = {
    title: curProject.path.split('/').pop(),
    key: curProject.path,
    children: []
  }
  

  // Reads the `$APPDATA/users` directory recursively
  const entries = await readDir(root.key, { recursive: true });

  function processEntries(entries: FileEntry[], parent: DataNode) {
    for (const entry of entries) {
      const node = { title: entry.path.split('/').pop(), key: entry.path, children: [] }
      console.log(`Entry: ${entry.path}`);
      if (entry.children) {
        processEntries(entry.children, node)
      }
      parent.children?.push(node)
    }
  }
  processEntries(entries, root)
  treeData.push(root)

  if (systemStore.editingFilePath) {
    selectedKeys.value.push(systemStore.editingFilePath)
    expandedKeys.value.push(curProject.path)
  }
}



const onSelect = (selectedKeys: string[]) => {
  console.log(selectedKeys)
  systemStore.setEditingFilePath(selectedKeys[0])
}

onMounted(() => {
  initFiles().then(res => {
    console.log(JSON.stringify(treeData))
  });
})


</script>

<style scoped>
.sidebar {
  width: 300px;
  padding-top: 36px;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
  border-right: 1px solid #ddd;
  position: relative;
  .toggle {
    position: absolute;
    right: 8px;
    top: 0;
  }
  .title {
    height: 60px;
    padding: 20px 16px 0 16px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid #ddd;
    margin-bottom: 16px
  }

  & :deep(.dir) {
    flex: 1;
  }

  .footer {
    height: 42px;
    border-top: 1px solid #ddd;
    margin-top: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
}
</style>