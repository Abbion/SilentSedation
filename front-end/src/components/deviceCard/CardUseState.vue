//Rework 2.0
<template>
    <div>
        <div class="s_UseHeader">
            <div class="s_ToolTip s_ConnectionStatus" :class="[ connection_status != ConnectionStatus.Connecting ? 's_ShiftStatus' : '' ]">
                <div v-if="connection_status == ConnectionStatus.Connecting" class="s_Loader"/>
                <div v-if="connection_status == ConnectionStatus.Online" class="s_BasicStatus s_Online"/>
                <div v-if="connection_status == ConnectionStatus.Offline" class="s_Offline"/>
                <div v-if="connection_status == ConnectionStatus.Failed" class="s_BasicStatus s_Failed"/>
                <div class="s_ToolTipText s_CustomToolTipText">
                    {{ connection_status_message  }}
                </div>
            </div>
            <div class="s_CenterContentVertical s_PreventSelect">
                    {{ p_card_data.name }}
            </div>
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
    import { defineProps, ref } from 'vue'

    enum ConnectionStatus {
        Connecting,
        Online,
        Offline,
        Failed
    }

    const props = defineProps<{
        p_card_data : CardData
    }>();

    const emit = defineEmits(['PropertiesUpdated', 'CardOptionsClicked']);

    let connection_status = ref(ConnectionStatus.Connecting);
    let connection_status_message = ref("Connecting...")

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
    }

    .s_CustomToolTipText {
        width: 140px;
    }
    
    .s_ConnectionStatus {
        position: absolute;
        transform: translate(-70px, -2px);
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

    .s_ShiftStatus {
        margin-right: 20px;
    }

    .s_BasicStatus {
        width: 10px;
        height: 10px;

        border-radius: 10px;
    }

    .s_Online {
        background-color: rgb(174, 228, 178);
    }

    .s_Offline {
        width: 8px;
        height: 8px;

        border-color: var(--color-main-light);

        border-width: 2px;
        border-style: solid;
        border-radius: 10px;
    }

    .s_Failed {
        background-color: rgb(231, 130, 130);
    }

    /* source: https://css-loaders.com/wavy/ */
    /* HTML: <div class="loader"></div> */
    .s_Loader {
        display: inline-grid;
        padding: 5px;
        filter: blur(0.5px) contrast(12);
    }

    .s_Loader:before, .s_Loader:after {
        content: "";
        grid-area: 1/1;
        height: 10px;
        aspect-ratio: 2;
        --c: #0000 65%,#fff 66% 98%,#fff0 101%;
        background:
            radial-gradient(35% 146% at 50% 159%,var(--c)) 0 0,
            radial-gradient(35% 146% at 50% -59%,var(--c)) 100% 100%;
        background-size: calc(200%/3) 50%;
        background-repeat: repeat-x;
        clip-path: inset(0 100% 0 0);
        animation: l16 2.0s infinite linear;
    }

    .s_Loader:after {
        scale: -1 1;
        animation-delay: -.75s;
    }

    @keyframes l16 {
        50% {clip-path: inset(0)}
        to {clip-path: inset(0 0 0 100%)}
    }
</style>