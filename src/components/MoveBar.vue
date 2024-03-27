<template>
    <div class="move-bar" @mousedown="mousedownHandler" @mouseup="mouseupHandler" @mousemove="mousemoveHandler"
        @mouseleave="mouseleaveHandler"></div>
</template>

<script setup lang="ts">
import { useWinMove } from "./../shared/move-hook"
const { mousedownHandler,
    mouseupHandler,
    mousemoveHandler,
    mouseleaveHandler, } = useWinMove()



</script>

<style scoped>
.move-bar {
    position: fixed;
    top: 0;
    left: 0;
    width: 200px;
    height: 36px;
    z-index: 10;
}
</style>