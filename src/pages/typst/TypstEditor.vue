<template>
    <div class="typster">
        <div class="actions">
            <button @click="onTest">test</button>
            <button @click="onCompile">compile</button>
        </div>
        <div class="source">
            <pre>
                {{ source }}
            </pre>
        </div>
        <div class="result">
            <img :src=imgUrl alt="">
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref, unref, watch } from 'vue';
import { readTextFile } from '@tauri-apps/api/fs';
import { invoke } from "@tauri-apps/api";

import defaultUrl from './../../assets/vue.svg'
import { TypstCompileEvent, TypstRenderResponse } from './interface';
import { appWindow } from '@tauri-apps/api/window';
import { useSystemStoreHook } from '../../store/store';


const systemStore = useSystemStoreHook();
const source = ref("")
const mainpath = "/Users/lixu/play/doc/main.typ"
const imgUrl = ref(defaultUrl);

const readDefaultContent = async () => {
    let filePath = unref(systemStore.editingFilePath)
    if(!filePath) {
      return;
    }
    const contents = await readTextFile(filePath);
    source.value = contents;
    console.log(contents)
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
    await renderPage()
}

const onCompile = async () =>{
   const mainpath = systemStore.editingProject?.path + '/main.typ';
   console.log(mainpath)
   const res = await invoke<TypstRenderResponse>("typst_compile", { path:mainpath, content: source.value });
   console.log(res)
}

onMounted(() => {
    readDefaultContent()

    return appWindow.listen<TypstCompileEvent>("typst_compile", ({ event, payload }) => {
      const { document, diagnostics } = payload;
       console.log(document,diagnostics)
    });
})

watch(() => source.value, (newVal) => {
    console.log('watch --- source :', newVal)

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
        gap: 16px;
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
</style>