<template>
  <div class="sidebar">
    <div class="title">
      <span>
        Files
      </span>
      <a-button :icon="h(PlusOutlined)"></a-button>
    </div>
    <a-directory-tree class="dir" :blockNode="true" :height="300" v-model:expandedKeys="expandedKeys"
      v-model:selectedKeys="selectedKeys"  :tree-data="treeData" @select="onSelect"></a-directory-tree>

    <div class="footer">
      footer
    </div>
  </div>
</template>

<script setup lang="ts">
import type { TreeProps } from 'ant-design-vue';
import { ref, h, reactive } from 'vue';
import { PlusOutlined } from '@ant-design/icons-vue'
import { readDir, BaseDirectory, FileEntry } from '@tauri-apps/api/fs';
import { useSystemStoreHook } from '../../store/store';
import { DataNode } from 'ant-design-vue/es/tree';

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
 console.log('-----')
  const root = {
    title: curProject.path.split('/').pop() ,
    key:curProject.path,
    children: []
  }

  // Reads the `$APPDATA/users` directory recursively
  const entries = await readDir( root.key, {  recursive: true });

  function processEntries(entries: FileEntry[], parent: DataNode) {
    for (const entry of entries) {
      const node = {title: entry.path.split('/').pop(), key:entry.path,  children: []}
      console.log(`Entry: ${entry.path}`);
      if (entry.children) {
        processEntries(entry.children, node)
      }
      parent.children?.push(node)
    }
  }
  processEntries(entries, root)
  treeData.push(root)
}

initFiles().then(res=>{
  console.log(JSON.stringify(treeData))
});

const onSelect = (selectedKeys: string[]) =>{
   console.log(selectedKeys)
   systemStore.setEditingFilePath(selectedKeys[0])
}


</script>

<style scoped>
.sidebar {
  width: 300px;
  padding-top: 36px;
  display: flex;
  flex-direction: column;

  .title {
    height: 60px;
    padding: 20px 16px 0 16px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid #ddd;
    margin-bottom: 16px
  }

  .dir {
    flex: 1;
    height: calc(100% - 200px);
  }

  .footer {
    height: 60px;
    border-top: 1px solid #ddd;
    margin-top: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
}
</style>