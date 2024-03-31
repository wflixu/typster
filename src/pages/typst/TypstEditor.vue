<template>
    <div class="typster" :class="layoutcls">
        <div class="actions" :class="{ 'expand': systemStore.showSidebar }">
            <div class="left">
                <SidebarToggle v-if="!systemStore.showSidebar" class="toggle" />
                <!-- <a-button size="small" >
                    <template #icon>
                        <a-tooltip title="保存">
                            <SaveOutlined />
                        </a-tooltip>
                    </template>
</a-button> -->
                <a-button size="small" @click="exportPdf">
                    <template #icon>
                        <a-tooltip title="导出PDF">
                            <ExportOutlined />
                        </a-tooltip>
                    </template>
                </a-button>
                <!-- <a-button @click="onTest">test</a-button> -->
            </div>
            <div class="middle">
                <a-radio-group v-model:value="mode" button-style="solid" size="small">
                    <a-radio-button value="all">
                        <OneToOneOutlined />
                    </a-radio-button>
                    <a-radio-button value="edit">
                        <EditOutlined />
                    </a-radio-button>
                    <a-radio-button value="preview">
                        <ReadOutlined />
                    </a-radio-button>
                </a-radio-group>
            </div>
            <div class="right">
                <!-- <template v-if="mode == 'preview'">
                    <a-radio-group v-model:value="adjust" button-style="solid" size="small">
                        <a-radio-button value="full">
                            <OneToOneOutlined />
                        </a-radio-button>
                        <a-radio-button value="width">
                            <EditOutlined />
                        </a-radio-button>
                        <a-radio-button value="height">
                            <ReadOutlined />
                        </a-radio-button>
                    </a-radio-group>
                </template> -->
            </div>
        </div>
        <div class="content">
            <div class="source bbox" v-show="mode != 'preview'">
                <MonacoEditor :path="systemStore.editingFilePath" :root="systemStore.editingProject?.path" @change="onChange" @compiled="onCompile">
                </MonacoEditor>
            </div>

            <div class="result" v-show="mode != 'edit'">
                <PreviewPage v-for="page in pages" :key="page.hash" v-bind="page" />
            </div>
        </div>

    </div>
</template>

<script setup lang="ts">
import { onMounted, ref,  computed } from 'vue';
import { readTextFile } from '@tauri-apps/api/fs';
import { invoke } from "@tauri-apps/api";
import { EditOutlined, ReadOutlined, OneToOneOutlined, ExportOutlined } from '@ant-design/icons-vue'
import type { IAdjust, IMode, TypstPage } from './interface';
import { useSystemStoreHook } from '../../store/store';
import SidebarToggle from '../home/SidebarToggle.vue';
import MonacoEditor from './../../components/MonacoEditor.vue'
import PreviewPage from "./PreviewPage.vue"
import { save } from '@tauri-apps/api/dialog';


const systemStore = useSystemStoreHook();
const mode = ref<IMode>(systemStore.mode);
const pages = ref<TypstPage[]>([])


const layoutcls = computed(() => {
    if (mode.value == 'edit') {
        return 'single-left'
    } else if (mode.value == 'preview') {
        return 'single-right'
    } else {
        return ''
    }
})

const exportPdf = async () => {
    const filePath = await save({
        filters: [{
            name: 'export_pdf',
            extensions: ['pdf']
        }]
    });
    const res = await invoke('export_pdf', { path: filePath })
}


const compile_main_file = async () => {
    const mainpath = systemStore.editingProject?.path + '/main.typ';
    try {
        const content = await readTextFile(mainpath);
        const res = await invoke<TypstPage[]>("typst_compile_doc", { path: '/main.typ', content });
        if (Array.isArray(res)) {
            pages.value = res;
        }
    } catch (error) {
        pages.value = [];
    }
}

const onCompile = (data: TypstPage[]) => {
    console.log('onCompile: data', data)
    if (Array.isArray(data)) {
        pages.value = data;
    }
}


const onChange = (text: string) => {

}

onMounted(async () => {
    await compile_main_file();
})



</script>

<style scoped>
.typster {
    --action-bar: 36px;
    height: 100%;
    overflow: hidden;

    .actions {
        height: var(--action-bar);
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 0 32px;
        padding-left: 72px;
        gap: 8px;
        border-bottom: 1px solid #ddd;

        &.expand {
            padding-left: 32px;
        }

        .left,
        .right,
        .middle {
            display: inline-flex;
            gap: 8px;
            align-items: center;
        }
    }

    .content {
        display: flex;
        height: calc(100% - var(--action-bar));

        .source {
            flex: 1;
            overflow: auto;
            height: 100%;
        }

        .result {
            flex: 1;
            background-color: #f1f1f1;
            display: flex;
            flex-direction: column;
            align-items: center;
            overflow: auto;
            height: 100%;
        }
    }
}
</style>