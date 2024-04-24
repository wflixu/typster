<template>
    <div class="dialognostics" :class="[diagnostic?.severity]">
        <template v-if="diagnostic">
            <div class="message">
                <div> 错误： {{ diagnostic.message }} </div>
                <div>
                    位置： 第 {{ diagnostic.pos[0] }} 行 第 {{ diagnostic.pos[1] }}列
                </div>
            </div>
        </template>

    </div>
</template>

<script setup lang="ts">
import { TypstSourceDiagnostic } from './interface';

interface Props {
    diagnostic: TypstSourceDiagnostic | null
}

defineProps<Props>();

</script>

<style scoped>
.dialognostics {
    display: none;
    position: absolute;
    font-size: 24px;
    line-height: 2;
    color: whitesmoke;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background-color: rgba(100, 100, 100, 0.6);
    top: 0;
    right: 0;

}

.dialognostics.error {
    bottom: 0;
    left: 0;
    display: flex;
    z-index: 200;

}

.dialognostics.warning {
    display: flex;
    background-color: rgba(100, 100, 100, 0.4);
    border-radius: 4px;
    font-size: 16px;
    line-height: 1.2;
    align-items: flex-start;
    z-index: 200;
    padding: 8px
}
</style>