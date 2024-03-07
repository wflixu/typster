<template>
    <div class="mdx-editor">

        <div class="sidebar">
            sidebar
        </div>
        <div class="header">
            <button @click="addImg">
                add IMg
            </button>
        </div>
        <div class="body">
            <div class="source">
                <textarea class="text-field" v-model="source"></textarea>
            </div>
            <div class="output">
                <div v-html="output" class="markdown-body">

                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';
import { appDir } from '@tauri-apps/api/path';

const source = ref(`= Introduction
In this report, we will explore the
various factors that influence _fluid
dynamics_ in glaciers and how they
contribute to the formation and
behavior of these natural structures.
# Hello World

+ The climate
  - Temperature
  - Precipitation
+ The topography
+ The geology
`)
const output = ref("no output")

onMounted(() => {
    if (source.value) {


    }
})

const addImg = async () => {
    // Open a selection dialog for directories
    const selected = await open({
        filters: [{
            name: 'Image',
            extensions: ['png', 'jpeg']
        }],
        multiple: false,
        defaultPath: await appDir(),
    });
    console.log(selected)
    if (Array.isArray(selected)) {

        // user selected multiple directories
    } else if (selected === null) {
        // user cancelled the selection
    } else {
        // user selected a single directory
    }
}

watch(() => source.value, async (newVal) => {

    console.log(newVal)
    invoke('compileToHtml', { text: newVal }).then(res => {
        console.log(res);
        output.value = res as string;
    }).catch(err => {
        console.log(err)
    })

    output.value = newVal
}, {
    immediate: true
})


</script>

<style scoped>
.mdx-editor {
    --sidebar-width: 240px;
    --actionbar-height: 42px;
    height: 100%;
    display: grid;
    grid-template-columns: var(--sidebar-width) 1fr;
    grid-template-rows: var(--actionbar-height) 1fr;

    .sidebar {
        grid-column: 1 / 2;
        grid-row: 1 / 3;
        width: var(--sidebar-width);
        background-color: sandybrown;
    }

    .header {
        grid-column: 2 / 3;
        grid-row: 1 / 2;
        display: flex;
        background-color: blueviolet;
        justify-content: space-between;
    }

    .body {
        display: flex;
        height: 100%;

        .source {
            flex: 1;
            background-color: #ddd;

            textarea {
                width: 100%;
                height: 90%;
            }
        }

        .output {
            flex: 1;
            background-color: mediumturquoise;
        }
    }
}
</style>