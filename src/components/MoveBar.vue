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
    height: 100%;
    width: 100%;
    cursor: move;
    background-color: aqua;
}
</style>