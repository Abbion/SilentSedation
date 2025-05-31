//Rework 2.0
<template>
    <div class="s_ControllIcon">
        <div class="s_IconCircleFrame">
            <ThunderBoltIcon class="s_SchockerThunderBoltIcon"></ThunderBoltIcon>
        </div>
    </div>
    <div class="s_PowerControll">
        <p class="s_ControllTitle s_PreventSelect">
            Power
        </p>
        <div class="s_PowerControllSelector">
            <div class="s_PowerControllSelectorChangePowerButton" :class="{ s_SchockerControllButtonClicked : decrease_power_clicked }" @mousedown="DecreasePowerDown" @mouseup="DecreasePowerUp" @mouseleave="ResetClickDown">
                <MinusIcon class="s_PowerControllSelectorIconStyle"/>
            </div>
            <div class="s_PowerControllSelectorPowerValue s_PreventSelect">
                {{ power_level }}
            </div>
            <div class="s_PowerControllSelectorChangePowerButton" :class="{ s_SchockerControllButtonClicked : increase_power_clicked }" @mousedown="IncreasePowerDown" @mouseup="IncreasePowerUp" @mouseleave="ResetClickDown">
                <AddIcon class="s_PowerControllSelectorIconStyle"/>
            </div>
        </div>
    </div>
    <div class="s_ActionControll">
        <p class="s_ControllTitle s_PreventSelect">
            Action
        </p>
        <div class="s_ActionControllButtons">
            <div class="s_ActionControllZapButton" :class="{ s_SchockerControllButtonClicked : zap_button_clicked }" @mousedown="ZapButtonDown" @mouseleave="ResetClickDown">
                <ThunderBoltIcon class="s_SchockerThunderBoltIcon"></ThunderBoltIcon>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
    import ThunderBoltIcon from '../../icons/ThunderBoltIcon.vue';
    import AddIcon from '../../icons/AddIcon.vue';
    import MinusIcon from '../../icons/MinusIcon.vue';
    import { ref, defineProps, onMounted } from 'vue'
    import { DeviceType, DeviceTypeToString } from '@/components/common/Enums';

    const ZAP_RESET_TIME = 2000;

    let power_level = ref<number>(5);
    let decrease_power_clicked = ref(false);
    let increase_power_clicked = ref(false);
    let mouse_down_on_decrease_button = false;
    let mouse_down_on_increase_button = false;

    let zap_button_clicked = ref(false);

    const props = defineProps<{
        p_properties: Record<string, any>;
    }>();

    const emit = defineEmits(['PowerLevelChanged', 'ActionPressed']);


    onMounted(()=> {
        console.log("props: " + props.p_properties + " - " + DeviceTypeToString(DeviceType.ShockCaller));
        
        let properties = props.p_properties[DeviceTypeToString(DeviceType.ShockCaller)];
        
        power_level.value = properties.power;
    });

    function DecreasePowerDown() {
        decrease_power_clicked.value = true;
        mouse_down_on_decrease_button = true;
    }

    function DecreasePowerUp() {
        if (!mouse_down_on_decrease_button)
            return;

        power_level.value -= 1;

        if (power_level.value < 1)
            power_level.value = 1;

        emit('PowerLevelChanged', power_level.value);

        decrease_power_clicked.value = false;
        mouse_down_on_decrease_button = false;
    }

    function IncreasePowerDown() {
        increase_power_clicked.value = true;
        mouse_down_on_increase_button = true;
    }

    function IncreasePowerUp() {
        if (!mouse_down_on_increase_button)
            return;

        power_level.value += 1;

        if (power_level.value > 10)
            power_level.value = 10;

        emit('PowerLevelChanged', power_level.value);

        increase_power_clicked.value = false;
        mouse_down_on_increase_button = false;
    }

    function ResetClickDown() {
        increase_power_clicked.value = false;
        decrease_power_clicked.value = false;
        mouse_down_on_decrease_button = false;
        mouse_down_on_increase_button = false;
    }

    function ZapButtonDown() {
        if (zap_button_clicked.value == true)
            return;

        zap_button_clicked.value = true;
        setTimeout(ResetZapButton, ZAP_RESET_TIME);
        emit('ActionPressed');
    }

    function ResetZapButton() {
        zap_button_clicked.value = false;
    }
</script>

<style scoped>
    .s_ControllIcon {
        height: 65px;
        width: 100%;
    }

    .s_IconCircleFrame {
        height: 70%;
        aspect-ratio: 1/1;

        margin-left: auto;
        margin-right: auto;

        border-radius: 100%;
        border-style: solid;
        border-color: var(--color-main-light);
        border-width: 2px;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    .s_SchockerThunderBoltIcon {
        fill: var(--color-main-light);
    }

    .s_PowerControll {
        height: 95px;
        width: 100%;
    }

    .s_ControllTitle {
        color: var(--color-main-light);
        height: 27px;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;

        font-size: 18px;
    }

    .s_PowerControllSelector {
        height: 60px;

        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
    }

    .s_PowerControllSelectorChangePowerButton {
        height: 40px;
        width: 40px;

        border-radius: 100%;
        border-style: solid;
        border-color: var(--color-main-light);
        border-width: 2px;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }
    
    .s_SchockerControllButtonClicked {
        opacity: 0.65;
    }

    .s_PowerControllSelectorChangePowerButton:hover {
        background-color: var(--color-main-dark--highlight);
        cursor: pointer;
    }

    .s_PowerControllSelectorIconStyle {
        fill: var(--color-main-light);

        height: 40%;
        width: 40%;
    }

    .s_PowerControllSelectorPowerValue {
        color: var(--color-main-light);

        height: 50px;
        width: 60px;

        margin-left: 3px;
        margin-right: 3px;

        padding-bottom: 7px;

        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;

        font-size: 36px;
    }

    .s_ActionControll {
        height: 100px;
        width: 100%;
    }

    .s_ActionControllButtons {
        height: calc(100% - 25px);

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    .s_ActionControllZapButton {
        background-color: transparent;

        height: 50px;
        width: 40px;

        border-style: solid;
        border-color: var(--color-main-light);
        border-radius: 10px;
        border-width: 2px;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    .s_ActionControllZapButton:hover {
        background-color: var(--color-main-dark--highlight);
        cursor: pointer;
    }
    
    .s_ActionControllZapButton > .s_SchockerThunderBoltIcon {
        height: 60%;
        width: 60%;
    }
</style>