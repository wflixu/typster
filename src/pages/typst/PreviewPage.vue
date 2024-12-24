<template>
    <div class="preview-page" ref="el" :style="styles">
        <img :src="pageUrl" alt="" srcset="" />
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import defaultUrl from './../../assets/rendering.svg'

import type { TypstRenderResponse } from './interface';

const props = defineProps({
    num: Number,
    hash: String,
    width: Number,
    height: Number,
    scale: { type: Number, default: 1 },
})

const el = ref();
const size = reactive({
    width: 0,
    height: 0
})

const pageUrl = ref(defaultUrl)
const nonce = 0;
const renderPage = async () => {
    try {
        const res: TypstRenderResponse = await invoke<TypstRenderResponse>("typst_render",
            { page: props.num, scale: window.devicePixelRatio * props.scale, nonce: nonce + 1 });
        pageUrl.value = "data:image/png;base64," + res.image;
        console.log(res);
        size.width = res.width;
        size.height = res.height;

    } catch (error) {
        console.log(error)
    }

}

let intersectionObserver = new IntersectionObserver((entries) => {
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
    return () => {
        intersectionObserver.disconnect();
    }
})

const styles = computed(() => {
    if (size.width) {
        return {
            width: size.width.toFixed(1) + 'px' ,
            height: size.height.toFixed(12) + 'px'
        }
    } else {
        return {
            width: props.width ? props.width.toFixed(1) + 'px' : '',
            height: props.height ? props.height.toFixed(1) + 'px' : ''
        }
    }

});

watch(() => props.hash, async (newVal, oldVal) => {
    console.warn(newVal, oldVal);
})
watch(() => props.scale, () => {
    renderPage();
})

</script>

<style scoped>
.preview-page {
    margin: 8px;
    box-shadow: 0 0 5px #aaa;
    flex-shrink: 0;
    img {
        width: 100%;
    }
}
</style>