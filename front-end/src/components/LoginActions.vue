<template>
    <div class="loginActions">
        <div class="namedCheckBox">
            <div class="checkbox" @click="SwitchCheck">
                <CheckIcon class="checkIcon" v-show="isChecked" />
            </div>

            <div class="checkBoxText preventSelect">
                Remember me
            </div>
        </div>

        <div class="logInButton cursorPointer" @click="$emit('logInButtonClicked')">
            log in
        </div>
    </div>
</template>

<script setup lang="ts">
    import CheckIcon from './icons/CheckIcon.vue'

    import { ref } from 'vue'

    //Needs to be used when defineModel is used
    defineEmits(['logInButtonClicked']);

    let isChecked = ref(false);
    let rememberMe = defineModel<boolean>();

    function SwitchCheck() {
        isChecked.value = !isChecked.value;  
        rememberMe.value = isChecked.value;
    }
</script>

<style scoped>
    .loginActions {
        margin-top: 10px;

        display: flex;
        flex-direction: row;
        align-items: start;
        justify-content: space-between;
    }

    .namedCheckBox {
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
    }

    .checkbox {
        height: 16px;
        width: 16px;

        margin-right: 5px;
        margin-top: 2px;
      
        border-color: #EAEAEA;
        border-width: 1px;
        border-style: solid;
        border-radius: 5px;

        display: flex;
        align-items: center;
        justify-content: center;
    }

    .checkbox:hover {
        cursor: pointer;
    }

    .checkIcon {
        height: 70%;
        width: 70%;
      
        fill: #EAEAEA;
    }

    .checkBoxText {
        color: #EAEAEA;
    }

    .logInButton {
        background-color: #EAEAEA;
        color: #00040D;

        height: 35px;
        width: 90px;

        border: none;
        border-radius: 10px;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    @media screen and (max-width: 600px) {
        .loginActions {
            width: 100%;
        }
    }
</style>