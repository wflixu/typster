<template>
    <div class="preview-page" ref="el" :style="styles">
        <img :src="pageUrl" alt="" srcset="">
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api';
import defaultUrl from './../../assets/rendering.svg'

import type { TypstCompileEvent, TypstRenderResponse } from './interface';

const props = defineProps({
    num: Number,
    hash: String,
    width: Number,
    height: Number,
})
const el = ref();

const pageUrl = ref(defaultUrl)
const nonce = 1;
const renderPage = async () => {
    try {
        const res: TypstRenderResponse = await invoke<TypstRenderResponse>("typst_render", { page: props.num, scale: window.devicePixelRatio, nonce: nonce+1 });
        pageUrl.value = "data:image/png;base64," + res.image;
        console.log(res)
    } catch (error) {
        console.log(error)
    }

}

let intersectionObserver = new IntersectionObserver((entries) => {
    console.log('entries:', entries);
    for (const entry of entries) {
        if (entry.isIntersecting) {
            renderPage().catch(err => {
                console.warn(err)
            })
        }
    }
}, {
    threshold: [0.1, 1]
});


onMounted(() => {
    renderPage().catch(err => {
        console.warn(err)
    })
    if (el.value) {
        intersectionObserver.observe(el.value);
    }
    console.log(props)
    return () => {
        intersectionObserver.disconnect();
    }
})

const styles = computed(() => {
    return {
        width: props.width ? props.width.toFixed(1) + 'px' : '',
        height: props.height ? props.height.toFixed(1) + 'px' : ''
    }
});

watch(() => props.hash, async (newVal,oldVal) => {
  console.warn(newVal, oldVal);
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