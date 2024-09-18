//Rework Done
<template>
    <div>
        <div class="useHeader">
            <p class="preventSelect">
                {{ cardName }}
            </p>
            <div class="useOptions cursorPointer preventSelect" @click="$emit('cardOptionsClicked')">...</div>
        </div>
        <div class="useContent">
             <SchockerControll></SchockerControll>
        </div>
    </div>
</template>

<script setup lang="ts">
    import { DeviceType } from '../common/Enums';
    import SchockerControll from './deviceControllers/SchockerControll.vue'
    import { defineProps, onMounted } from 'vue'

    const props = defineProps<{
        cardName: string,
        deviceType: DeviceType,
        deviceProperties: Object
    }>();

    onMounted(() => {
        console.log("UserState: " + props.cardName + " deviceType: " + props.deviceType + " properties: " + props.deviceProperties);
    })

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