<template>
  <div class="sidebar">
    <div class="move">
      <MoveBar />
    </div>
    <div class="content">
      <div class="title">
        <template v-if="isToc">
          <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
            width="24" height="24" fill="none" @click="onToggle" class="cursor-pointer">
            <rect id="toc" width="24" height="24" x="0.000000" y="0.000000" fill="rgba(120, 118, 118, 1)"
              fill-opacity="0" />
            <path id="矢量 1"
              d="M18.984 13.9713L18.984 11.0272L21 11.0272L21 13.9713L18.984 13.9713ZM18.984 5L21 5L21 8.01435L18.984 8.01435L18.984 5ZM18.984 20L18.984 16.9856L21 16.9856L21 20L18.984 20ZM3 20L3 16.9856L17.016 16.9856L17.016 20L3 20ZM3 13.9713L3 11.0272L17.016 11.0272L17.016 13.9713L3 13.9713ZM3 8.01435L3 5L17.016 5L17.016 8.01435L3 8.01435Z"
              fill="rgba(120, 118, 118, 1)" fill-rule="nonzero" />
          </svg>
          <span>
            Table of Contents
          </span>
        </template>

        <template v-else>
          <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
            width="24" height="24" fill="none" class="cursor-pointer" @click="onToggle">
            <rect id="file-tree" width="24" height="24" x="0.000000" y="0.000000" fill="rgba(120, 118, 118, 1)"
              fill-opacity="0" />
            <path id="矢量 2"
              d="M3 3L9 3L9 7L3 7L3 3ZM15 10L21 10L21 14L15 14L15 10M15 17L21 17L21 21L15 21L15 17ZM13 13L7 13L7 18L13 18L13 20L5 20L5 9L7 9L7 11L13 11L13 13L13 13Z"
              fill="rgba(120, 118, 118, 1)" fill-rule="nonzero" />
          </svg>
          <span>
            Files
          </span>
        </template>
        <span></span>
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
    </div>
    <div class="footer">
      <Button icon="pi pi-plus" aria-label="Save" size="small" @click="onCreateFile" />
      <Select :value="systemStore.editingProject?.path" size="small" :options="projects" optionLabel="title"
        option-value="path" class="w-full md:w-56" @change="onSelectProject" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue';
import { readDir, writeTextFile, remove, rename } from '@tauri-apps/plugin-fs';
import { save } from '@tauri-apps/plugin-dialog';
import { join } from '@tauri-apps/api/path';
import type { DirEntry } from '@tauri-apps/plugin-fs';
import type { TreeNode } from 'primevue/treenode';
import Button from 'primevue/button';
import MoveBar from '../../components/MoveBar.vue';
import { useSystemStoreHook } from '../../store/store';
import { SidebarType } from '../../shared/interface';

// 定义文件系统条目的类型，用于处理 Tauri fs API 返回的数据
type FileSystemEntry = {
  name: string;
  path: string;
  isDirectory: boolean;
  children?: FileSystemEntry[];
}


const sidebarType = ref<SidebarType>('file');

const isToc = computed(() => {
  return sidebarType.value === 'toc';
})
const onToggle = () => {
  if (sidebarType.value === 'file') {
    sidebarType.value = 'toc';
  } else {
    sidebarType.value = 'file';
  }
}


const systemStore = useSystemStoreHook();

const expandedKeys = ref<string[]>([]);
const selectedKeys = ref<string[]>([]);
const treeData: any[] = reactive([]);

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
  try {
    // 清空现有数据
    if (treeData.length > 0) {
      treeData.splice(0, treeData.length);
    }

    const curProject = systemStore.editingProject
    if (!curProject || !curProject.path) {
      console.warn('No current project or path available');
      return
    }

    const projectPath = curProject.path;

    const root = {
      label: projectPath.split('/').pop() || 'Untitled Project',
      data: projectPath,
      key: projectPath,
      selectable: false,
      icon: 'pi pi-folder',
      children: []
    } as TreeNode;

    try {
      // 读取目录内容
      const entries = await readDir(projectPath);

      type DataNode = TreeNode & {
        leaf?: boolean;
        isDirectory?: boolean;
      };

      // 递归处理目录条目
      async function processEntries(entries: DirEntry[], parent: DataNode): Promise<void> {
        for (const entry of entries) {
          try {
            // 将 DirEntry 转换为我们可用的格式
            const fileEntry = entry as unknown as FileSystemEntry;

            // 跳过隐藏文件和系统文件
            if (fileEntry.name?.startsWith('.') || fileEntry.name?.endsWith('.DS_Store')) {
              continue;
            }

            const fullPath = await join(parent.key as string, fileEntry.name);

            const node: DataNode = {
              label: fileEntry.name,
              icon: fileEntry.isDirectory ? 'pi pi-folder' : getFileIcon(fileEntry.name),
              key: fullPath,
              children: [],
              selectable: !fileEntry.isDirectory,
              isDirectory: fileEntry.isDirectory,
              leaf: !fileEntry.isDirectory
            };

            // 如果是目录，递归处理子内容
            if (fileEntry.isDirectory) {
              node.selectable = false;
              node.leaf = false;
              try {
                const subEntries = await readDir(fullPath);
                await processEntries(subEntries, node);
              } catch (subError) {
                console.warn(`Failed to read subdirectory ${fullPath}:`, subError);
                // 即使子目录读取失败，仍保留目录节点
              }
            }

            parent.children?.push(node);
          } catch (error) {
            const fileEntry = entry as unknown as FileSystemEntry;
            console.warn(`Failed to process entry ${fileEntry?.name || 'unknown'}:`, error);
            // 继续处理其他条目，不中断整个过程
          }
        }
      }

      await processEntries(entries, root);
      treeData.push(root);

      // 恢复选中状态
      if (systemStore.editingFilePath) {
        selectedKeys.value = [systemStore.editingFilePath];

        // 展开到文件的路径
        const pathParts = systemStore.editingFilePath.split('/');
        for (let i = 1; i < pathParts.length - 1; i++) {
          const parentPath = pathParts.slice(0, i + 1).join('/');
          expandedKeys.value.push(parentPath);
        }
        expandedKeys.value.push(projectPath);
      }

    } catch (readError) {
      console.error(`Failed to read directory ${projectPath}:`, readError);
      // 即使目录读取失败，仍显示根节点
      treeData.push(root);
    }

  } catch (error) {
    console.error('Error in initFiles:', error);
  }
}

// 根据文件扩展名返回对应图标
const getFileIcon = (fileName: string): string => {
  const ext = fileName.split('.').pop()?.toLowerCase();
  switch (ext) {
    case 'typ':
      return 'pi pi-file';
    case 'bib':
      return 'pi pi-book';
    case 'yml':
    case 'yaml':
      return 'pi pi-cog';
    case 'pdf':
      return 'pi pi-file-pdf';
    case 'png':
    case 'jpg':
    case 'jpeg':
    case 'gif':
      return 'pi pi-image';
    default:
      return 'pi pi-file';
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
  console.info(node)
  systemStore.setEditingFilePath(node.key as string)
}
const onCreateFile = async () => {
  const filePath = await save({
    title: "新建文件",
    filters: [{
      name: 'untitled',
      extensions: ['typ', 'bib', 'yml', 'yaml', 'md']
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
  console.info('------')
  initFiles().then(() => {
    // console.log(JSON.stringify(treeData))
  });
})


</script>

<style scoped>
.sidebar {
  width: 300px;
  height: 100%;
  display: grid;
  grid-template-columns: 1fr;
  grid-template-rows: 32px 1fr 36px;
  border-right: 1px solid #ddd;
  position: relative;

  .move {
    height: 32px;
    padding-left: 80px;
  }

  .title {
    height: 40px;
    padding: 8px 16px 0 16px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid #ddd;
  }

  & :deep(.dir) {
    height: calc(100% - 40px);
    padding: 8px 0;
    overflow-y: auto;
  }

  .footer {
    height: 36px;
    padding: 0 16px;
    gap: 8px;
    border-top: 1px solid #ddd;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
}
</style>