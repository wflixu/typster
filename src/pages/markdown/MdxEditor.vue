<template>
    <div class="mdx-editor">

        <Sidebar class="sidebar" />

        <div class="header">
            <button @click="addImg">
                add IMg
            </button>
            <button @click="onSave">Save</button>
            <button @click="uploadImg">Upload</button>
            <button @click="onTest">test</button>
        </div>
        <div class="body">
            <template v-if="editing">
                <div class="source">
                    <MonacoEditor v-model="source" />
                </div>
                <div class="output">
                    <div v-html="output" class="markdown-body">

                    </div>
                </div>
            </template>

        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, unref, watch } from 'vue';
import { invoke, path } from '@tauri-apps/api';
import Sidebar from './Sidebar.vue'
import { open } from '@tauri-apps/api/dialog';
import { appDir } from '@tauri-apps/api/path';
import { useSystemStoreHook } from '../../store/store';
import { readTextFile, BaseDirectory, writeTextFile, readBinaryFile } from '@tauri-apps/api/fs';
import MonacoEditor from './MonacoEditor.vue'
import { MdAstNode } from './interface';




const systemStore = useSystemStoreHook();
const source = ref(``)
const output = ref("no output")

const editing = computed(() => {
    return !!systemStore.editingFilePath;
})
const readEditingFile = async () => {

    const contents = await readTextFile(systemStore.editingFilePath);
    source.value = contents
}
onMounted(() => {
    if (editing.value) {
        readEditingFile()
    }
})

const onSave = async () => {
    await writeTextFile({ path: systemStore.editingFilePath, contents: source.value });
}

const uploadImg = async () => {
    let ast = await invoke('md_to_ast', { text: source.value });
    console.log(ast)
    await traverse(ast as MdAstNode)
    console.log(JSON.stringify(ast))
    const md = await invoke('ast_to_md', { json: JSON.stringify(ast) })
    console.log(md)
    
}
const onTest = async () => {
    let ast = await invoke('md_trans_ast', { text: source.value });
    console.log(ast) 
}


const traverse = async (node: MdAstNode) => {
    // if (node.type == 'image') {
    //    await upload(node);
    // }
    if (Array.isArray(node.children)) {
        node.children.forEach(async (item)  => {
           await traverse(item)
        })
    }
}

const upload = async (node: MdAstNode) => {
    if (node.url) {
        let fullPath = await path.join(systemStore.editingFilePath, '../', node.url);
        console.warn(fullPath)
        const imgFile = await readBinaryFile(fullPath)
        const form = new FormData();
        form.append('doc', new Blob([imgFile], { type: "image/png" }), fullPath.split('\/').pop());
        console.log(form.getAll('file'))
        const response = await fetch(systemStore.imgBedUrl, {
            method: 'POST',
            headers: {
                "Authorization": unref(systemStore.imgBedToken),
            },
            body: form,
            mode: 'cors'

        })

        console.log(response)
        const res = await response.json();
        console.log(res);
        if (res.code == 200) {
            const id = res.data[0].id
            const newUrl = `${systemStore.imgBedShowBaseUrl}?id=${id}`
            node.url = newUrl;
        }
    }
}

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
watch(() => systemStore.editingFilePath, async (newVal) => {

    if (newVal) {
        readEditingFile()
    } else {

    }
},)


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
        background-color: rgb(116, 96, 244);
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
            padding: 8px;

            textarea {
                width: 100%;
                height: 90%;
            }
        }

        .output {
            flex: 1;
            padding: 16px;
            background-color: mediumturquoise;
        }
    }
}
</style>