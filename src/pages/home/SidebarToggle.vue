<template>
    <div class="sidebar-toggle ">
        <a-button size="small" type="link" @click="back" :icon="h(AppstoreOutlined)">
        </a-button>
        <a-button size="small" type="link" @click="onToggle">
            <template #icon>
                <DoubleLeftOutlined v-if="systemStore.showSidebar" key="left" />
                <DoubleRightOutlined v-else key="right" />
            </template>
        </a-button>
    </div>
</template>

<script setup lang="ts">
import { DoubleLeftOutlined, DoubleRightOutlined, AppstoreOutlined } from '@ant-design/icons-vue'
import { useRouter } from 'vue-router';
import { h } from 'vue';
import { useSystemStoreHook } from '../../store/store';

const systemStore = useSystemStoreHook();

const router = useRouter();
const back = () => {
    router.push('/project')
}

const onToggle = () => {
   systemStore.toggleShowSidebar()
}

</script>

<style scoped>
.sidebar-toggle {
    display: inline-flex;
    justify-content: center;
    height: 36px;
    align-items: center;
    z-index: 100;
}

</style>