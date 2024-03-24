<template>
    <div class="controllIcon">
        <div class="iconCircleFrame">
            <ThunderBoltIcon class="schockerThunderBoltIcon"></ThunderBoltIcon>
        </div>
    </div>
    <div class="powerControll">
        <p class="controllTitle preventSelect">
            Power
        </p>
        <div class="powerControllSelector">
            <div class="powerControllSelectorChangePowerButton" :class="{ schockerControllButtonClicked : decreasePowerClicked }" @mousedown="DecreasePowerDown" @mouseup="DecreasePowerUp" @mouseleave="ResetClickDown">
                <MinusIcon class="powerControllSelectorIconStyle"/>
            </div>
            <div class="powerControllSelectorPowerValue preventSelect">
                {{ powerLevel }}
            </div>
            <div class="powerControllSelectorChangePowerButton" :class="{ schockerControllButtonClicked : increasePowerClicked }" @mousedown="IncreasePowerDown" @mouseup="IncreasePowerUp" @mouseleave="ResetClickDown">
                <AddIcon class="powerControllSelectorIconStyle"/>
            </div>
        </div>
    </div>
    <div class="actionControll">
        <p class="controllTitle preventSelect">
            Action
        </p>
        <div class="actionControllButtons">
            <div class="actionControllZapButton" :class="{ schockerControllButtonClicked : zapButtonClicked }" @mousedown="ZapButtonDown" @mouseup="ZapButtonUp" @mouseleave="ResetClickDown">
                <ThunderBoltIcon class="schockerThunderBoltIcon"></ThunderBoltIcon>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
    import ThunderBoltIcon from '../icons/ThunderBoltIcon.vue';
    import AddIcon from '../icons/IconAdd.vue';
    import MinusIcon from '../icons/IconMinus.vue';
    import { ref } from 'vue'

    var powerLevel = ref(5);
    var decreasePowerClicked = ref(false);
    var increasePowerClicked = ref(false);
    var mouseDownOnDecreaseButton = false;
    var mouseDownOnIncreaseButton = false;

    var zapButtonClicked = ref(false);
    var mouseDonwOnZapButton = false;

    function DecreasePowerDown() {
        decreasePowerClicked.value = true;
        mouseDownOnDecreaseButton = true;
    }

    function DecreasePowerUp() {
        if (!mouseDownOnDecreaseButton)
            return;

        powerLevel.value -= 1;

        if (powerLevel.value < 1)
            powerLevel.value = 1;

        decreasePowerClicked.value = false;
        mouseDownOnDecreaseButton = false;
    }

    function IncreasePowerDown() {
        increasePowerClicked.value = true;
        mouseDownOnIncreaseButton = true;
    }

    function IncreasePowerUp() {
        if (!mouseDownOnIncreaseButton)
            return;

        powerLevel.value += 1;

        if (powerLevel.value > 10)
            powerLevel.value = 10;

        increasePowerClicked.value = false;
        mouseDownOnIncreaseButton = false;
    }

    function ResetClickDown() {
        increasePowerClicked.value = false;
        decreasePowerClicked.value = false;
        mouseDownOnDecreaseButton = false;
        mouseDownOnIncreaseButton = false;

        zapButtonClicked.value = false;
        mouseDonwOnZapButton = false;
    }

    function ZapButtonDown() {
        zapButtonClicked.value = true;
        mouseDonwOnZapButton = true;
    }

    function ZapButtonUp() {
        if (!mouseDonwOnZapButton)
        return;

        zapButtonClicked.value = false;
        mouseDonwOnZapButton = false;
    }
</script>

<style scoped>
    .controllIcon{
        width: 100%;
        height: 65px;
    }

    .iconCircleFrame {
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

    .schockerThunderBoltIcon {
        fill: var(--color-main-light);
        
    }

    .powerControll{
        width: 100%;
        height: 95px;
    }

    .controllTitle {
        height: 27px;
        color: var(--color-main-light);

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;

        font-size: 18px;
    }

    .powerControllSelector {
        height: 60px;

        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
    }

    .powerControllSelectorChangePowerButton {
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
    
    .schockerControllButtonClicked {
        opacity: 0.65;
    }

    .powerControllSelectorChangePowerButton:hover {
        cursor: pointer;
        background-color: var(--color-main-dark--highlight);
    }

    .powerControllSelectorIconStyle {
        fill: var(--color-main-light);
        width: 40%;
        height: 40%;
    }

    .powerControllSelectorPowerValue {
        color: var(--color-main-light);
        height: 50px;
        width: 50px;

        margin-left: 3px;
        margin-right: 3px;

        padding-bottom: 7px;

        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;

        font-size: 36px;
    }

    .actionControll {
        width: 100%;
        height: 100px;
    }

    .actionControllButtons {
        height: calc(100% - 25px);

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    .actionControllZapButton {
        width: 40px;
        height: 50px;
        background-color: transparent;

        border-style: solid;
        border-color: var(--color-main-light);
        border-radius: 10px;
        border-width: 2px;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    .actionControllZapButton:hover {
        cursor: pointer;
        background-color: var(--color-main-dark--highlight);
    }
    
    .actionControllZapButton > .schockerThunderBoltIcon {
        height: 60%;
        width: 60%;
    }
</style>