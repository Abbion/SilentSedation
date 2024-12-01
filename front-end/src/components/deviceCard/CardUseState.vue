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
             <SchockerControll v-if="p_card_data.device_type == DeviceType.ShockCaller" :p_properties="p_card_data.device_properties"
                                @PowerLevelChanged="UpdateProperties"></SchockerControll>
        </div>
    </div>
</template>

<script setup lang="ts">
    import { DeviceType } from '../common/Enums';
    import type { CardData } from '../common/Interfaces';
    import SchockerControll from './deviceControllers/SchockerControll.vue'
    import { defineProps } from 'vue'

    const props = defineProps<{
        p_card_data : CardData
    }>();

    const emit = defineEmits(['PropertiesUpdated', 'CardOptionsClicked']);

    function UpdateProperties(power_level : number) {
        switch (props.p_card_data.device_type) {
            case DeviceType.ShockCaller: {
                console.log("send power level " + power_level);
                
                emit('PropertiesUpdated', { "power" : power_level });
                break;
            }
        }
    }

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