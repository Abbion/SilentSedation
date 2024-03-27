//Rework Done
<template>
    <div>
        <div class="useHeader">
            <p class="preventSelect">
                Title
            </p>
            <div class="useOptions cursorPointer preventSelect" @click="$emit('cardOptionsClicked')">...</div>
        </div>
        <div class="useContent">
            <SchockerControll></SchockerControll>
        </div>
    </div>
</template>

<script setup lang="ts">
    import SchockerControll from './deviceControllers/SchockerControll.vue'
</script>

<style scoped>
    .useHeader {
        color: var(--color-main-light);

        height: 12%;

        margin-bottom: 10px;
        
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: end;
    }
    
    .useOptions {
        position: absolute;
        transform: translate(72px, -12px);
        font-size: 20px;
    }

    .useContent {
        height: 88%;
        width: 100%;
    }
</style>