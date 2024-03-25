<template>
    <div>
      <button @click="onAdd">add</button>
       <ul>
        <li>
            {{ systemStore.editingFilePath }}
        </li>
       </ul>
    </div>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/api/dialog';
import { appDir } from '@tauri-apps/api/path';
import { useSystemStoreHook } from '../../store/store';

const systemStore = useSystemStoreHook();
const onAdd =  async () => {
    const selected = await open({
        filters: [{
            name: 'MD',
            extensions: ['md', 'mdx']
        }],
        multiple: false,
        defaultPath: await appDir(),
    });
    console.log(selected)
    if (selected) {
        // user cancelled the selection
        systemStore.setEditingFilePath(selected as string)
    } else {

    }
}


</script>

<style scoped>




</style>