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
      <template v-if="isToc">
        <div class="toc" ref="tocContainer">
          <ToC :items="tocList" />
          <!-- <a :href="'#' + item.id" @click.prevent="() => onClickTocItem(item)" class="toc-item" v-for="item in tocList"
            :key="item.id" :class="{
              'is-active': item.isActive && !item.isScrolledOver,
              'is-scrolled-over': item.isScrolledOver,
            }" :style="{ '--level': item.level }"
            :data-item-index="item.itemIndex"
            >
            {{ item.textContent }}
          </a> -->
        </div>
      </template>
      <template v-else>
        <ContextMenu ref="menuRef" :model="items" />
        <Tree class="dir" v-model:selectionKeys="selectedKeys" v-model:expandedKeys="expandedKeys"
          selectionMode="single" :value="treeData" @nodeSelect="onSelect">
          <template #default="{ node }">
            <div @contextmenu="onRightClick($event, node)">
              <span>{{ node.label }}</span>
            </div>
          </template>
        </Tree>
      </template>
    </div>
    <div class="footer" v-if="!isToc">
      <Button icon="pi pi-plus" aria-label="Save" size="small" @click="onCreateFile" />
      <Select v-model="selectedProjectPath" size="small" :options="projects" optionLabel="title" option-value="path"
        class="w-full md:w-56" @update:modelValue="onSelectProject" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch, nextTick } from 'vue';
import { readDir, writeTextFile, remove, rename } from '@tauri-apps/plugin-fs';
import { save } from '@tauri-apps/plugin-dialog';
import { join } from '@tauri-apps/api/path';
import type { DirEntry } from '@tauri-apps/plugin-fs';
import type { TreeNode } from 'primevue/treenode';
import Button from 'primevue/button';
import ContextMenu from 'primevue/contextmenu';
import Select from 'primevue/select';
import MoveBar from '../../components/MoveBar.vue';
import { useSystemStoreHook } from '../../store/store';
import { EventBus } from '../../shared/EventBus';
import { validateTypstFilePath, hasPathTraversalPattern, sanitizePath } from '../../utils/path-security';
import { TreeSelectionKeys } from 'primevue/tree';
import ToC from './ToC.vue';

// 定义文件系统条目的类型，用于处理 Tauri fs API 返回的数据
type FileSystemEntry = {
  name: string;
  path: string;
  isDirectory: boolean;
  children?: FileSystemEntry[];
}

const systemStore = useSystemStoreHook();

// 项目选择
const selectedProjectPath = ref(systemStore.editingProject?.path || '');

// 监听当前项目变化，同步更新 selectedProjectPath
watch(() => systemStore.editingProject?.path, (newPath) => {
  if (newPath) {
    selectedProjectPath.value = newPath;
  }
});

// 使用全局状态
const isToc = computed(() => {
  return systemStore.sidebarType === 'toc';
})

const onToggle = () => {
  if (systemStore.sidebarType === 'file') {
    systemStore.setSidebarType('toc');
  } else {
    systemStore.setSidebarType('file');
  }
}

const expandedKeys = ref<Record<string, boolean>>({});
const selectedKeys: TreeSelectionKeys = ref({});
const treeData = ref([]);

const projects = computed(() => {
  return systemStore.projects;
})

const menuRef = ref();
const selectedNode = ref<TreeNode | null>(null);

// TOC 容器引用
const tocContainer = ref<HTMLElement | null>(null);

const tocList = computed(() => {
  console.info(systemStore.toc)
  return systemStore.toc;
})
const onClickTocItem = (data: Record<string, any>) => {
  EventBus.emit('select-toc-item', data);
}

// 滚动 TOC 到当前 active 项
const scrollToActiveTocItem = () => {
  if (!tocContainer.value) return;

  const activeItem = tocContainer.value.querySelector('.toc-item.is-active') as HTMLElement;
  if (!activeItem) return;

  const container = tocContainer.value;
  const containerHeight = container.clientHeight;
  const itemTop = activeItem.offsetTop;
  const itemHeight = activeItem.offsetHeight;

  // 将 active 项滚动到容器中间位置
  const scrollTop = itemTop - containerHeight / 2 + itemHeight / 2;
  container.scrollTo({
    top: scrollTop,
    behavior: 'smooth'
  });
};

// 监听 tocList 变化，自动滚动到 active 项
watch(tocList, () => {
  nextTick(() => {
    scrollToActiveTocItem();
  });
}, { deep: true });
// 动态菜单项
const items = computed(() => {
  if (!selectedNode.value) return [];

  const node = selectedNode.value;
  const baseItems = [
    {
      label: '重命名',
      icon: 'pi pi-file-edit',
      command: () => handleRename(node)
    }
  ];

  // 只有文件才显示删除选项，且不能删除 main.typ
  if (!node.isDirectory && node.label !== 'main.typ') {
    baseItems.push({
      label: '删除',
      icon: 'pi pi-trash',
      command: () => handleDelete(node)
    });
  }

  return baseItems;
});

const onRightClick = (event: MouseEvent, node: TreeNode) => {
  event.preventDefault();
  selectedNode.value = node;
  menuRef.value?.show(event);
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
    treeData.value = [];
    selectedKeys.value = {};
    // 同时清空展开状态
    expandedKeys.value = {};

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
      treeData.value.push(root);

      // 等待 DOM 更新后再设置展开状态
      await nextTick();

      // 恢复选中状态和展开状态
      if (systemStore.editingFilePath) {
        selectedKeys.value[systemStore.editingFilePath] = true;

        // 展开到文件的路径
        // 从项目路径开始，逐层展开到文件的父目录
        let currentPath = projectPath;
        expandedKeys.value[currentPath] = true; // 确保根目录展开

        // 获取相对于项目路径的路径部分
        const relativePath = systemStore.editingFilePath.substring(projectPath.length + 1);
        const pathParts = relativePath.split('/');

        // 逐层构建并添加父目录路径（排除最后的文件名）
        for (let i = 0; i < pathParts.length - 1; i++) {
          currentPath = await join(currentPath, pathParts[i]);
          expandedKeys.value[currentPath] = true;
        }

        console.log('Expanded keys:', expandedKeys.value);
        console.log('Selected file:', systemStore.editingFilePath);
        console.log('Project path:', projectPath);
      }

    } catch (readError) {
      console.error(`Failed to read directory ${projectPath}:`, readError);
      // 即使目录读取失败，仍显示根节点
      treeData.value.push(root);
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

// 处理重命名
const handleRename = async (node: TreeNode) => {
  if (!node?.key) return;

  try {
    const filePath = await save({
      title: "重命名文件",
      filters: [{
        name: '文件',
        extensions: ['typ', 'bib', 'yml', 'yaml', 'md']
      }],
      defaultPath: node.key as string
    });

    if (filePath && filePath !== node.key) {
      // 安全验证：检查新路径是否安全
      if (hasPathTraversalPattern(filePath)) {
        console.error('安全警告: 检测到路径穿越攻击尝试', filePath);
        return;
      }

      const projectPath = systemStore.editingProject?.path;
      if (projectPath && !validateTypstFilePath(filePath, projectPath)) {
        console.error('安全警告: 文件路径验证失败', filePath);
        return;
      }

      await rename(node.key as string, filePath);

      // 如果重命名的文件是当前编辑的文件，更新编辑路径
      if (systemStore.editingFilePath === node.key) {
        systemStore.setEditingFilePath(filePath);
      }

      await initFiles();
    }
  } catch (error) {
    console.error('重命名失败:', error);
  }
};

// 处理删除
const handleDelete = async (node: TreeNode) => {
  if (!node?.key || node.isDirectory) return;

  // 确认删除
  const confirmDelete = confirm(`确定要删除文件 "${node.label}" 吗？`);
  if (!confirmDelete) return;

  try {
    // 安全验证：检查路径是否安全
    const filePath = node.key as string;
    if (hasPathTraversalPattern(filePath)) {
      console.error('安全警告: 检测到路径穿越攻击尝试', filePath);
      return;
    }

    const projectPath = systemStore.editingProject?.path;
    if (projectPath && !validateTypstFilePath(filePath, projectPath)) {
      console.error('安全警告: 文件路径验证失败', filePath);
      return;
    }

    await remove(filePath);

    // 如果删除的是当前编辑的文件，清空编辑器
    if (systemStore.editingFilePath === node.key) {
      systemStore.setEditingFilePath('');
    }

    await initFiles();
  } catch (error) {
    console.error('删除失败:', error);
  }
};

const onSelect = (node: TreeNode) => {
  console.info(node)
  systemStore.setEditingFilePath(node.key as string)
}

const onCreateFile = async () => {
  const projectPath = systemStore.editingProject?.path;
  if (!projectPath) {
    console.error('错误: 未选择项目');
    return;
  }

  const filePath = await save({
    title: "新建文件",
    filters: [{
      name: 'untitled',
      extensions: ['typ', 'bib', 'yml', 'yaml', 'md']
    }],
    defaultPath: projectPath

  });

  if (filePath) {
    // 安全验证：检查新文件路径
    try {
      const sanitizedPath = sanitizePath(filePath);

      if (hasPathTraversalPattern(filePath)) {
        console.error('安全警告: 检测到路径穿越攻击尝试', filePath);
        return;
      }

      if (!validateTypstFilePath(filePath, projectPath)) {
        console.error('安全警告: 文件路径必须在项目范围内', filePath);
        return;
      }

      await writeTextFile(sanitizedPath, ' ');
      await initFiles();
    } catch (error) {
      console.error('创建文件失败:', error);
    }
  }
}

const onSelectProject = (path: string) => {
  let selectedProject = systemStore.projects.find(item => item.path === path)
  if (selectedProject) {
    systemStore.selectProject(selectedProject)
    window.location.reload();
  }
}

// 监听 treeData 变化，在数据加载完成后展开文件夹
watch(treeData, async () => {
  if (treeData.value.length > 0 && systemStore.editingFilePath) {
    // 等待 DOM 更新
    await nextTick();

    // 再次等待确保 Tree 组件完全渲染
    await nextTick();

    // 重新设置展开状态
    const projectPath = systemStore.editingProject?.path;
    if (!projectPath) return;

    expandedKeys.value = {};
    let currentPath = projectPath;
    expandedKeys.value[currentPath] = true; // 确保根目录展开

    const relativePath = systemStore.editingFilePath.substring(projectPath.length + 1);
    const pathParts = relativePath.split('/');

    for (let i = 0; i < pathParts.length - 1; i++) {
      currentPath = await join(currentPath, pathParts[i]);
      expandedKeys.value[currentPath] = true;
    }

    console.log('From watcher - Expanded keys:', expandedKeys.value);
  }
}, { deep: true });

onMounted(() => {
  initFiles().then(() => {
    // 初始加载完成后，如果有正在编辑的文件，确保它被选中和展开
    if (systemStore.editingFilePath) {
      selectedKeys.value[systemStore.editingFilePath] = true;
    }
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
  overflow: hidden;

  .move {
    grid-row: 1;
    height: 32px;
    padding-left: 80px;
  }

  .title {
    padding: 0 16px;
    height: 40px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid #ddd;
  }

  .content {
    grid-row: 2;
    min-height: 0;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .footer {
    grid-row: 3;
    height: 36px;
    padding: 0 16px;
    gap: 8px;
    border-top: 1px solid #ddd;
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-shrink: 0;
  }

  & :deep(.dir) {
    flex: 1;
    overflow-y: auto;
    padding: 8px 0;
    min-height: 0;
    height: calc(100% - 40px);
  }

  /* toc */
  .toc {
    flex: 1;
    overflow-y: auto;
    padding: var(--spacing-md);
    min-height: 0;

    .toc-item {
      height: 36px;
      display: flex;
      align-items: center;
      padding: 0 var(--spacing-md);
      margin: 2px 0;
      color: var(--color-fg-secondary);
      text-decoration: none;
      cursor: pointer;
      padding-left: calc(14px * (var(--level) - 1));
      border-radius: var(--radius-sm);
      transition: all 0.15s ease;
      font-size: 16px;
      line-height: 1.5;
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }

    .toc-item:hover {
      color: var(--color-accent-primary);
      background: var(--color-bg-secondary);
    }

    .toc-item.is-active {
      background: var(--color-accent-primary);
      color: white;
      font-weight: 500;
      box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
    }

    .toc-item.is-active:hover {
      background: var(--color-accent-hover);
    }

    .toc-item.is-scrolled-over {
      font-weight: 500;
      color: var(--color-fg-primary);
      opacity: 0.7;
    }
  }
}
</style>