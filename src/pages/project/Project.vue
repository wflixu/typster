<template>
    <div class="page">
        <h1> Projects</h1>
        <div class="content">
            <div v-if="isEmpty">
                <a-empty description="没有项目"/>
            </div>
            <ul class="list">
                <li class="list-item" v-for="project in list" :key="project.title" @click="onSelect(project)">
                    <h2>{{ project.title }}</h2>
                    <p> {{ project.path }}</p>
                    <div class="actions">
                        <a-button @click.stop="onDeleteProject(project)" size="small" shape="circle"
                            :icon="h(DeleteOutlined)" />
                    </div>
                </li>

            </ul>
        </div>

        <div class="footer">
            <a-button @click="addProject" block size="large" type="primary"> Add Project </a-button>
        </div>
        <AddProject v-model:open="show" @finish="onFinish" />
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref, h, computed } from 'vue';
import { DeleteOutlined } from '@ant-design/icons-vue'
import type { IProject } from './interface';
import { useSystemStoreHook } from '../../store/store';
import AddProject from './AddProject.vue'
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api';

const systemStore = useSystemStoreHook();
const router = useRouter()
const show = ref(false);

const list = computed(() => {
    return systemStore.projects;
});

const isEmpty = computed(() =>{
   return list.value?.length < 1;
});

const onSelect = async (pr: IProject) =>{

    systemStore.selectProject(pr);
    systemStore.setLoading(true);
    await invoke('load_project_from_path', {path: pr.path});
    systemStore.setLoading(false);
    router.push('/home')
}

const addProject = async () => {
    show.value = true;
}

const onDeleteProject = (pr: IProject) => {
    systemStore.deleteProject(pr);
}

const onFinish = () => {
    console.log('finish add project')
}

onMounted(() => {

});

</script>

<style scoped>
.page {
    padding: 40px 0;
    height: 100vh;
    position: relative;

    h1 {
        padding-left: 60px;
    }

    .content {
        min-height: 350px;
        overflow-y: auto;
        padding: 0 60px;
        max-height: calc(100vh - 200px);
    }

    .list {
        .list-item {
            cursor: pointer;
            border: 1px solid #ddd;
            border-radius: 4px;
            position: relative;
            padding: 16px;
            margin-top: 40px;

            .actions {
                position: absolute;
                right: 16px;
                bottom: 8px;
            }
        }
    }

    .footer {
        position: absolute;
        bottom: 40px;
        left: 60px;
        right: 60px;
        display: flex;
        justify-content: center;
    }
}
</style>