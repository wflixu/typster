<template>
    <div class="sidebar-toggle " :class="{ 'expand': expand }">
        <a-button type="link"  @click="back" :icon="h(AppstoreOutlined)">
        </a-button>
        <a-button type="link"  @click="onToggle" :icon="toggleIcon">
        </a-button>
    </div>
</template>

<script setup lang="ts">
import { DoubleLeftOutlined, DoubleRightOutlined, AppstoreOutlined } from '@ant-design/icons-vue'
import { useRouter } from 'vue-router';
import { computed, h } from 'vue'

const expand = defineModel('expand', { type: Boolean, default: true })
const router = useRouter();
const back = () => {
    router.push('/project')
}

const toggleIcon = computed(() => {
    if (expand.value) {
        return h(DoubleLeftOutlined)
    } else {
        return h(DoubleRightOutlined)
    }
})

const onToggle = () => {
    expand.value = !expand.value;
}

</script>

<style scoped>
.sidebar-toggle {
    position: absolute;
    left: 80px;
    display: inline-flex;
    justify-content: center;
    height: 40px;
    align-items: center;
}

.sidebar-toggle.expand {
    left: 200px;
}
</style>