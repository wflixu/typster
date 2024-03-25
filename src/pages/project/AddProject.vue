<template>
    <a-modal v-model:open="open" title="Add Project" @ok="handleOk">
        <template #footer>
            <a-button class="w-full" key="submit" type="primary" :loading="loading" @click="handleOk">Submit</a-button>
        </template>
        <p>
            添加一个文件夹作为一个新项目
        </p>
        <div class="mb-4">
            <h3>Name</h3>
            <a-input type="text" placeholder="project name" v-model:value="project.title" />
        </div>
        <div class="mb-4">
            <h3>Project Directory</h3>
            <a-button @click="onSelectDir">{{ project.path ? project.path : '选择文件夹' }}</a-button>
        </div>
    </a-modal>
</template>

<script setup lang="ts">
import { reactive, ref, toRaw } from 'vue';
import { open as openDialog } from '@tauri-apps/api/dialog';
import { appDataDir } from '@tauri-apps/api/path';
import { message } from 'ant-design-vue';
import { IProject } from './interface';
import { useSystemStoreHook } from '../../store/store';

const open = defineModel('open', { type: Boolean, default: false })
const emit = defineEmits<{
    (e: 'finish'): void
}>()

const systemStore = useSystemStoreHook();

let project = reactive<IProject>({
    title: '',
    path: ''
})

const loading = ref<boolean>(false);

const onSelectDir = async () => {
    const selected = await openDialog({
        directory: true,
        multiple: false,
        defaultPath: await appDataDir()
    })
    if (selected) {
        project.path = selected as string;
    }
}
const resetProject = () => {
    project = reactive ({
        title:'',
        path: ''
    })
}

const handleOk = () => {
    loading.value = true;

    if (project.title && project.path) {
        systemStore.addProject(project)
        loading.value = false;
        open.value = false;
        emit('finish')
        resetProject()
    } else {
        message.warn("请填写完整")
    }
};

</script>

<style scoped></style>