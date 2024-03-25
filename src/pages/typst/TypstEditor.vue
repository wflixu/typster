<template>
    <div class="typster">
        <div class="source">
            <div>
                <button @click="onTest">test</button>
                <button @click="onCompile">compile</button>
            </div>
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
import { onMounted, ref, watch } from 'vue';
import { readTextFile } from '@tauri-apps/api/fs';
import { invoke } from "@tauri-apps/api";

import defaultUrl from './../../assets/vue.svg'
import { TypstCompileEvent, TypstRenderResponse } from './interface';
import { appWindow } from '@tauri-apps/api/window';

const source = ref("")
const mainpath = "/Users/lixu/play/doc/main.typ"
const imgUrl = ref(defaultUrl);

const readDefaultContent = async () => {
    const contents = await readTextFile(mainpath);
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




</script>

<style scoped>
.typster {
    height: 100%;
    background-color: aqua;
    display: flex;
    gap: 0 16px;

    .source {
        flex: 1;
        background-color: coral;
        padding: 16px;
    }

    .result {
        flex: 1;
        background-color: white;

        img {
            width: 100%;
        }
    }
}
</style>