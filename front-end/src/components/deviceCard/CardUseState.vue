//Rework 2.0
<template>
    <div>
        <div class="s_UseHeader">
            <p class="s_PreventSelect">
                {{ p_card_data.name }}
            </p>
            <div class="s_UseOptions s_CursorPointer s_PreventSelect" @click="$emit('CardOptionsClicked')">...</div>
        </div>
        <div class="s_UseContent">
             <SchockerControll v-if="p_card_data.device_type == DeviceType.ShockCaller" :p_properties="p_card_data.device_properties"></SchockerControll>
        </div>
    </div>
</template>

<script setup lang="ts">
    import { DeviceType } from '../common/Enums';
    import type { CardData } from '../common/Interfaces';
    import SchockerControll from './deviceControllers/SchockerControll.vue'
    import { defineProps } from 'vue'

    defineProps<{
        p_card_data : CardData
    }>();

</script>

<style scoped>
    .s_UseHeader {
        color: var(--color-main-light);

        height: 12%;

        margin-bottom: 10px;
        
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: end;
    }
    
    .s_UseOptions {
        position: absolute;
        transform: translate(72px, -12px);
        font-size: 20px;
    }

    .s_UseContent {
        height: 88%;
        width: 100%;
    }
</style>