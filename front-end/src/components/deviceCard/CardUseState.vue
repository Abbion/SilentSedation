//Rework Done
<template>
    <div>
        <div class="useHeader">
            <p class="preventSelect">
                {{ p_cardData.name }}
            </p>
            <div class="useOptions cursorPointer preventSelect" @click="$emit('cardOptionsClicked')">...</div>
        </div>
        <div class="useContent">
             <SchockerControll v-if="p_cardData.deviceType == DeviceType.ShockCaller" :properties="p_cardData.deviceProperties"></SchockerControll>
        </div>
    </div>
</template>

<script setup lang="ts">
    import { DeviceType } from '../common/Enums';
    import type { CardData } from '../common/Interfaces';
    import SchockerControll from './deviceControllers/SchockerControll.vue'
    import { defineProps } from 'vue'

    defineProps<{
        p_cardData : CardData
    }>();

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