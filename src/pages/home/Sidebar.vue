<template>
  <div class="sidebar">
    <SidebarToggle v-if="systemStore.showSidebar" class="toggle" />
    <div class="title">
      <span>
        Files
      </span>
      <a-button :icon="h(PlusOutlined)" @click="onCreateFile"></a-button>
    </div>
    <a-directory-tree class="dir" :blockNode="true" v-model:expandedKeys="expandedKeys"
      v-model:selectedKeys="selectedKeys" :tree-data="treeData" @select="onSelect">
      <template #title="{ key: treeKey, title }">
        <a-dropdown :trigger="['contextmenu']">
          <span>{{ title }}</span>
          <template #overlay>
            <a-menu @click="({ key: menuKey }: any) => onContextMenuClick(treeKey, menuKey)">
              <a-menu-item key="delete">删除</a-menu-item>
            </a-menu>
          </template>
        </a-dropdown>
      </template>
    </a-directory-tree>

    <div class="footer">
      <a-dropdown trigger="click">
        <div class="hover">
          <FolderOpenOutlined />
          <span class="ml-2">
            {{ systemStore.editingProject?.path }}
          </span>
        </div>

        <template #overlay>
          <a-menu @select="onSelectProject" selectable>
            <a-menu-item v-for="pro in projects" :key="pro.path">
              <span>{{ pro.title }}</span> : <span>{{ pro.path }}</span>
            </a-menu-item>
            
          </a-menu>
        </template>
      </a-dropdown>

    </div>
  </div>
</template>

<script setup lang="ts">
import type { TreeProps } from 'ant-design-vue';
import { ref, h, reactive, onMounted, computed } from 'vue';
import { PlusOutlined, FolderOpenOutlined } from '@ant-design/icons-vue'
import { readDir, BaseDirectory, FileEntry, writeTextFile, removeFile } from '@tauri-apps/api/fs';
import { useSystemStoreHook } from '../../store/store';
import { DataNode } from 'ant-design-vue/es/tree';
import SidebarToggle from './SidebarToggle.vue';
import { save } from '@tauri-apps/api/dialog';
import { router } from '../../router';

const systemStore = useSystemStoreHook();

const expandedKeys = ref<string[]>([]);
const selectedKeys = ref<string[]>([]);
const treeData: TreeProps['treeData'] = reactive([]);

const projects = computed(() =>{
  return systemStore.projects;
})

const initFiles = async () => {
  if (treeData.length > 0) {
    treeData.splice(0, treeData.length);
  }
  console.log(systemStore.editingProject)
  const curProject = systemStore.editingProject
  if (!curProject) {
    return
  }

  const root = {
    title: curProject.path.split('/').pop(),
    key: curProject.path,
    selectable: false,
    children: []
  }


  // Reads the `$APPDATA/users` directory recursively
  const entries = await readDir(root.key, { recursive: true });

  function processEntries(entries: FileEntry[], parent: DataNode) {
    for (const entry of entries) {
      if (entry.name?.endsWith('.DS_Store')) {
        continue;
      }
      const node = { title: entry.path.split('/').pop(), key: entry.path, children: [], selectable: true };
      if (entry.children) {
        node.selectable = false;
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

const onContextMenuClick = async (treeKey: string, menuKey: string | number) => {
  console.log(`treeKey: ${treeKey}, menuKey: ${menuKey}`);
  if (menuKey == 'delete' && treeKey) {
    await removeFile(treeKey);
    await initFiles();
  }
};

const onSelect = (selectedKeys: string[]) => {
  console.log(selectedKeys)
  systemStore.setEditingFilePath(selectedKeys[0])
}
const onCreateFile = async () => {
  const filePath = await save({
    title: "新建文件",
    filters: [{
      name: 'untitled',
      extensions: ['typ']
    }],
    defaultPath: systemStore.editingProject?.path

  });
  console.log(filePath)
  if (filePath) {
    await writeTextFile({ path: filePath, contents: ' ' });
    await initFiles();
  }
}

const onSelectProject = ({key}:any) =>{

  let selectedProject = systemStore.projects.find(item => item.path == key)
  if(selectedProject) {
    systemStore.selectProject(selectedProject)
    systemStore.setEditingFilePath(selectedProject.path + '/main.typ');
    window.location.reload();
  }
}

onMounted(() => {
  initFiles().then(() => {
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