<template>
    <div class="view-scale">
        <a-button size="small" @click="onChange('-')">
            <template #icon>
                <MinusOutlined />
            </template>
        </a-button>
        <span>
            {{ showText }}
        </span>
        <a-button size="small" @click="onChange('+')">
            <template #icon>
                <PlusOutlined />
            </template>
        </a-button>
    </div>
</template>

<script setup lang="ts">
import { PlusOutlined,MinusOutlined } from '@ant-design/icons-vue'
import { computed } from 'vue';
const model = defineModel({ type: Number, default: 1 })

const onChange = (type: '-'| '+') =>{
  if(type == '-') {
    model.value = Math.max(0.2, model.value - 0.2)
  }else {
    model.value = Math.min(2.0, model.value + 0.2)
  }
}

const showText = computed (() =>{
    return model.value.toFixed(1)
})
</script>

<style scoped>
.view-scale {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    justify-content: center;
}
</style>