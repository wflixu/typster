<template>
    <Dialog v-model:visible="open" modal header="Add Project" :style="{ width: '40vw' }">
        <template #footer>
            <Button class="w-full" key="submit"  :loading="loading" @click="handleOk">添加项目</Button>
        </template>
        <p>
            添加一个文件夹作为一个新项目
        </p>
        <div class="mb-4">
            <h3>Name</h3>
            <InputText type="text" v-model="project.title"  placeholder="project name"/>
        </div>
        <div class="mb-4">
            <h3>Project Directory</h3>
            <Button @click="onSelectDir">{{ project.path ? project.path : '选择文件夹' }}</Button>
        </div>
    </Dialog>

</template>

<script setup lang="ts">
import { reactive, ref } from 'vue';
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import { appDataDir } from '@tauri-apps/api/path';
import { useToast } from 'primevue/usetoast';
import { useSystemStoreHook } from '../../store/store';
import type { IProject } from '../../shared/interface';

const toast = useToast();

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
        toast.add({
            severity: 'warn',
            summary: '警告',
            detail: '请填写完整',
            life: 3000
        });
        loading.value = false;
    }
};

</script>

<style scoped></style>
