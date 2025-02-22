<template>
  <div class="sidebar">
    <SidebarToggle v-if="systemStore.showSidebar" class="toggle" />
    <div class="title">
      <span>
        Files
      </span>
      <Button icon="pi pi-plus" aria-label="Save" size="small" @click="onCreateFile" />

    </div>
    <ContextMenu ref="menuRef" :model="items" />
    <Tree class="dir" v-model:selectionKeys="selectedKeys" selectionMode="single" :value="treeData"
      @nodeSelect="onSelect">
      <template #default="{ node }">
        <div @contextmenu="onRightClick($event, node)">
          <span>{{ node.label }}</span>
        </div>
      </template>
    </Tree>

    <div class="footer">
      <Select :value="systemStore.editingProject?.path" :options="projects" optionLabel="title" option-value="path" 
        class="w-full md:w-56" @change="onSelectProject" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue';
import Button from 'primevue/button';

// @ts-ignore
import { readDir, FileEntry, writeTextFile, remove, rename } from '@tauri-apps/plugin-fs';
import { useSystemStoreHook } from '../../store/store';
import SidebarToggle from './SidebarToggle.vue';
// @ts-ignore
import { save } from '@tauri-apps/plugin-dialog';
import { join } from '@tauri-apps/api/path';
import { TreeNode } from 'primevue/treenode';


const systemStore = useSystemStoreHook();

const expandedKeys = ref<string[]>([]);
const selectedKeys = ref<string[]>([]);
const treeData: TreeNode[] = reactive([]);

const projects = computed(() => {
  return systemStore.projects;
})

const menuRef = ref();
const items = ref([
  {
    label: 'Delect', icon: 'pi pi-trash', command: (e) => {
      console.log(e)
    }
  },
  { label: 'Rename', icon: 'pi pi-file-edit' }
]);
const onRightClick = (event, node) => {
  console.log(node, event, menuRef.value)
  menuRef.value?.show();
}

const projectItems = projects.value.map(item => {
  return {
    name: item.title,
    code: item.path
  }
})

const initFiles = async () => {
  if (treeData.length > 0) {
    treeData.splice(0, treeData.length);
  }

  const curProject = systemStore.editingProject
  if (!curProject) {
    return
  }

  const root = {
    label: curProject.path.split('/').pop(),
    data: curProject.path.split('/').pop(),
    key: curProject.path,
    selectable: false,
    icon: 'pi pi-folder',
    children: []
  } as TreeNode;


  // Reads the `$APPDATA/users` directory recursively
  const entries = await readDir(root.key as string);

  async function processEntries(entries: FileEntry[], parent: DataNode) {
    for (const entry of entries) {
      if (entry.name?.endsWith('.DS_Store')) {
        continue;
      }
      const node = {
        label: entry.name,
        icon: 'pi pi-file',
        key: await join(parent.key as string, entry.name), children: [], selectable: true
      } as DataNode;
      if (entry.isDirectory) {
        node.selectable = false;
        node.leaf = false
        node.icon = 'pi pi-folder'
        await processEntries(await readDir(node.key as string), node)
      }
      parent.children?.push(node)
    }
  }
  await processEntries(entries, root)
  treeData.push(root)

  if (systemStore.editingFilePath) {
    selectedKeys.value.push(systemStore.editingFilePath)
    expandedKeys.value.push(curProject.path)
  }
}

const onContextMenuClick = async (treeKey: string, menuKey: string | number) => {
  console.log(`treeKey: ${treeKey}, menuKey: ${menuKey}`);

  if (menuKey == 'delete' && treeKey) {
    if (treeKey.endsWith('main.typ')) {
      alert('The main.typ file cannot be deleted')
      return
    }
    await remove(treeKey);
  }
  if (menuKey == 'rename' && treeKey) {
    const filePath = await save({
      title: "Rename file",
      filters: [{
        name: 'untitled',
        extensions: ['typ', 'bib', 'yml']
      }],
      defaultPath: treeKey
    });
    if (filePath) {
      await rename(treeKey, filePath);
    }
  }
  await initFiles();
};

const onSelect = (node: TreeNode) => {

  systemStore.setEditingFilePath(node.key as string)
}
const onCreateFile = async () => {
  const filePath = await save({
    title: "新建文件",
    filters: [{
      name: 'untitled',
      extensions: ['typ', 'bib', 'yml', 'yaml']
    }],
    defaultPath: systemStore.editingProject?.path

  });
  console.warn(filePath)
  if (filePath) {
    await writeTextFile(filePath, ' ');
    await initFiles();
  }
}

const onSelectProject = ({ key }: any) => {
  
  let selectedProject = systemStore.projects.find(item => item.path == key)
  if (selectedProject) {
    systemStore.selectProject(selectedProject)
    window.location.reload();
  }
}

onMounted(() => {
  initFiles().then(() => {
    // console.log(JSON.stringify(treeData))
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