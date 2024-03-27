<template>
    <div class="typster" :class="layoutcls">
        <div class="actions" :class="{ 'expand': systemStore.showSidebar }">
            <div class="left">
                <SidebarToggle v-if="!systemStore.showSidebar" class="toggle" />
                <a-button size="small" :icon="h(SaveOutlined)" @click="saveSource"></a-button>
                <a-button size="small" :icon="h(ExportOutlined)" @click="exportPdf"></a-button>
                <a-button @click="onTest">test</a-button>
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

            </div>

        </div>
        <div class="content">
            <div class="source bbox" v-if="mode != 'preview'">
                <MonacoEditor :mode="mode" v-model="source"></MonacoEditor>
            </div>

            <div class="result" v-show="mode != 'edit'">
                <template v-for="page in pages" :key="page.hash">
                    <PreviewPage v-bind="page" />
                </template>
            </div>
        </div>

    </div>
</template>

<script setup lang="ts">
import { onMounted, ref, unref, watch, h, computed } from 'vue';
import { readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import { invoke } from "@tauri-apps/api";
import { EditOutlined, ReadOutlined, SaveOutlined, OneToOneOutlined, ExportOutlined } from '@ant-design/icons-vue'
import defaultUrl from './../../assets/vue.svg'
import type { IMode, TypstCompileEvent, TypstPage, TypstRenderResponse } from './interface';
import { useSystemStoreHook } from '../../store/store';
import SidebarToggle from '../home/SidebarToggle.vue';
import MonacoEditor from './../../components/MonacoEditor.vue'
import PreviewPage from "./PreviewPage.vue"
import { save } from '@tauri-apps/api/dialog';
const systemStore = useSystemStoreHook();
const source = ref("")

const mode = ref<IMode>(systemStore.mode);


const imgUrl = ref(defaultUrl);
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

const readDefaultContent = async () => {
    let filePath = unref(systemStore.editingFilePath)
    if (!filePath) {
        return;
    }
    const contents = await readTextFile(filePath);
    source.value = contents;

}
const renderPage = async () => {
    try {
        const res: TypstRenderResponse = await invoke<TypstRenderResponse>("typst_render", { page: 1, scale: window.devicePixelRatio, nonce: 1 });
        imgUrl.value = "data:image/png;base64," + res.image;
        console.log(res)
    } catch (error) {
        console.log(error)
    }

}

const onTest = async () => {
    const mainpath = systemStore.editingProject?.path + '/main.typ';
    const res = await invoke<TypstPage[]>('typst_compile_doc',{ path: mainpath, content: source.value })
    console.warn(res)
}

const onCompile = async () => {
    const mainpath = systemStore.editingProject?.path + '/main.typ';
    console.log(mainpath)
    try {
        const res = await invoke<TypstPage>("typst_compile_doc", { path: mainpath, content: source.value });
        if(Array.isArray(res)) {
            pages.value = res;
        }
    } catch (error) {
        pages.value = [];
    }

}

const saveSource = async () => {
    await writeTextFile({ path: systemStore.editingFilePath, contents: source.value });
}

onMounted(() => {
    readDefaultContent().then(_ => {
        return onCompile()
    }).then(() => {
        return renderPage()
    }).catch(err => {
        console.log(err)
    })

   
})

watch(() => source.value, (newVal) => {
    console.log('sdddd',source.value)
    onCompile().then(()=>{
        return renderPage()
    }).catch(err => {
        console.log(err)
    })
})

watch(() => systemStore.editingFilePath, () => {
    readDefaultContent();
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