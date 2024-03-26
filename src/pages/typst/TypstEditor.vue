<template>
    <div class="typster" :class="layoutcls">
        <div class="actions">
            <button @click="onTest">test</button>
            <button @click="onCompile">compile</button>
            <a-button :icon="h(SaveOutlined)" @click="saveSource"></a-button>
            <a-button :icon="h(EditOutlined)" @click="systemStore.toggleEditView()"></a-button>
            <a-button :icon="h(ReadOutlined)" @click="systemStore.togglePreview()"></a-button>
        </div>
        <div class="source" v-show="systemStore.editView">
            <MonacoEditor v-model="source"></MonacoEditor>
        </div>
        <div class="result" v-show="systemStore.preview">
            <img :src=imgUrl alt="">
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref, unref, watch,h, computed } from 'vue';
import { readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import { invoke } from "@tauri-apps/api";
import {EditOutlined, ReadOutlined, SaveOutlined} from '@ant-design/icons-vue'
import defaultUrl from './../../assets/vue.svg'
import { TypstCompileEvent, TypstRenderResponse } from './interface';
import { appWindow } from '@tauri-apps/api/window';
import { useSystemStoreHook } from '../../store/store';

import MonacoEditor from './../../components/MonacoEditor.vue'

const systemStore = useSystemStoreHook();
const source = ref("")

const imgUrl = ref(defaultUrl);

const layoutcls = computed(() =>{
   
    if(systemStore.editView && (!systemStore.preview) ) {
        return 'single-left'
    } else if ((!systemStore.editView) && systemStore.preview ) {
        return 'single-right'
    } else {
        return ''
    }
})

const readDefaultContent = async () => {
    let filePath = unref(systemStore.editingFilePath)
    if(!filePath) {
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
    console.log(await navigator.clipboard.readText())
    // await renderPage()
}

const onCompile = async () =>{
   const mainpath = systemStore.editingProject?.path + '/main.typ';
   console.log(mainpath)
   const res = await invoke<TypstRenderResponse>("typst_compile", { path:mainpath, content: source.value });
   console.log('res:', res);
}

const saveSource = async () =>{
    await writeTextFile({ path: systemStore.editingFilePath, contents: source.value });
}

onMounted(() => {
    readDefaultContent().then(_ =>{
      return onCompile()
    }).then(() =>{
        return renderPage()
    }).catch(err =>{
        console.log(err)
    })
    


    return appWindow.listen<TypstCompileEvent>("typst_compile", ({ event, payload }) => {
        const { document, diagnostics } = payload;
       console.log(document,diagnostics)
    });
})

watch(() => source.value, (newVal) => {
    renderPage()
})

watch(() => systemStore.editingFilePath, ()=>{
    readDefaultContent();
})




</script>

<style scoped>
.typster {
    height: 100%;
    background-color: aqua;
    display: grid;
    grid-template-areas: 
            "a a"
            "b c";
   grid-template-rows: 36px 1fr; /* 3.各区域 宽高设置 */
   grid-template-columns: 1fr 1fr;
    .actions {
        grid-area: a;
        display: flex;
        align-items: center;
        justify-content: flex-end;
        padding: 0 32px;
        gap: 8px;
    }
    .source {
        grid-area: b;
        background-color: coral;
        padding: 16px;
    }

    .result {
        grid-area: c;
        background-color: white;

        img {
            width: 100%;
        }
    }
}
.typster.single-left {
    grid-template-areas: 
            "a a"
            "b b";
    .source {
       grid-area: b;
    }
}
.typster.single-right {
    grid-template-areas: 
            "a a"
            "b b";
    .result {
       grid-area: b;
    }
}
</style>