<template>
    <div class="preview-page">
        <img :src="pageUrl" alt="" srcset="">
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import defaultUrl from './../../assets/vue.svg'

import type { TypstCompileEvent, TypstRenderResponse } from './interface';

const props = defineProps({
    page: Number,
    hash: String,
})


const pageUrl = ref(defaultUrl)
const nonce = 1;
const renderPage = async () => {
    try {
        const res: TypstRenderResponse = await invoke<TypstRenderResponse>("typst_render", { page: props.page, scale: window.devicePixelRatio, nonce });
        pageUrl.value = "data:image/png;base64," + res.image;
        console.log(res)
    } catch (error) {
        console.log(error)
    }

}


onMounted(() => {
    renderPage().catch(err => {
        console.warn(err)
    })
})


</script>

<style scoped>
.preview-page {
  margin: 8px;
  box-shadow: 0 0 5px #aaa;
  img {
     width: 100%;
  }
}
</style>