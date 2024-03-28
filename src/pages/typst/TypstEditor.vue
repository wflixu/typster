<template>
    <div class="typster" :class="layoutcls">
        <div class="actions" :class="{ 'expand': systemStore.showSidebar }">
            <div class="left">
                <SidebarToggle v-if="!systemStore.showSidebar" class="toggle" />
                <a-button size="small" @click="saveSource">
                    <template #icon>
                        <a-tooltip title="保存">
                            <SaveOutlined />
                        </a-tooltip>
                    </template>
                </a-button>
                <a-button size="small"  @click="exportPdf">
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
            <div class="source bbox" v-if="mode != 'preview'">
                <MonacoEditor :mode="mode" v-model="source"></MonacoEditor>
            </div>

            <div class="result" v-show="mode != 'edit'">
                <PreviewPage v-for="page in pages" :key="page.hash" v-bind="page" />
            </div>
        </div>

    </div>
</template>

<script setup lang="ts">
import { onMounted, ref, unref, watch, h, computed, nextTick } from 'vue';
import { readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import { invoke } from "@tauri-apps/api";
import { EditOutlined, ReadOutlined, SaveOutlined, OneToOneOutlined, ExportOutlined } from '@ant-design/icons-vue'
import type { IAdjust, IMode, TypstPage } from './interface';
import { useSystemStoreHook } from '../../store/store';
import SidebarToggle from '../home/SidebarToggle.vue';
import MonacoEditor from './../../components/MonacoEditor.vue'
import PreviewPage from "./PreviewPage.vue"
import { save } from '@tauri-apps/api/dialog';
import { message } from 'ant-design-vue';

const systemStore = useSystemStoreHook();
const source = ref("")

const mode = ref<IMode>(systemStore.mode);
const adjust = ref<IAdjust>('full');


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
    console.log(systemStore.editingProject)

    const filePath = await save({
        filters: [{
            name: 'export_pdf',
            extensions: ['pdf']
        }]
    });
    const res = await invoke('export_pdf', { path: filePath })
    console.log(res);
}

const readText = async () => {
    let filePath = unref(systemStore.editingFilePath)
    if (!filePath) {
        filePath = systemStore.editingProject?.path + 'main.typ'
    }
    const contents = await readTextFile(filePath);
    systemStore.setEditingFilePath(filePath);
    source.value = contents;
}

const onTest = async () => {
    const mainpath = systemStore.editingProject?.path + '/main.typ';
    const res = await invoke<TypstPage[]>('typst_compile_doc', { path: mainpath, content: source.value })
    console.warn(res)
}

const compile_typst_source = async () => {
    const mainpath = systemStore.editingProject?.path + '/main.typ';
    try {
        const res = await invoke<TypstPage>("typst_compile_doc", { path: mainpath, content: source.value });
        console.warn('compile_doc:', res);
        if (Array.isArray(res)) {
            pages.value = res;
        }
    } catch (error) {
        pages.value = [];
    }

}

const saveSource = async () => {
    try {
        console.log(source.value)
        await compile_typst_source();
        await writeTextFile({ path: systemStore.editingFilePath, contents: source.value });
        message.info("保存成功")
    } catch (error) {
        message.warn(`保存失败 : ${error}`);
    }
}


const init = async () => {
    try {
        await readText();
        await compile_typst_source();
    } catch (error) {
        console.log(error);
    }

}

onMounted(async () => {

    await init();

})

watch(() => source.value, (newVal) => {

})

watch(() => systemStore.editingFilePath, async () => {
    await readText();
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