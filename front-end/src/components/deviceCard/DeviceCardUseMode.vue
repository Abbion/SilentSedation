<template>
    <div class="useHeader">
        <p class="preventSelect">
            Title
        </p>
        <div class="useOptions cursorPointer preventSelect" @click="optionsClicked">...</div>
    </div>
    <div class="useContent">
        <SchockerControll></SchockerControll>
    </div>
</template>

<script setup lang="ts">
    import SchockerControll from './SchockerControll.vue'
    import { defineEmits } from 'vue';

    const emit = defineEmits(['optionsWidgetClicked']);

    function optionsClicked() {
        emit('optionsWidgetClicked');
    }
</script>

<style scoped>
    .useHeader {
        height: 12%;
        
        color: var(--color-main-light);

        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: end;

        margin-bottom: 10px;
    }
    
    .useOptions {
        position: absolute;
        transform: translate(72px, -12px);
        font-size: 20px;
    }

    .useContent {
        width: 100%;
        height: 88%;
    }
</style>