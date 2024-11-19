<template>
    <div class="s_LoginActions">
        <div class="s_NamedCheckBox">
            <div class="s_Checkbox" @click="SwitchCheck">
                <CheckIcon class="s_CheckIcon" v-show="is_checked" />
            </div>

            <div class="s_CheckBoxText s_PreventSelect">
                Remember me
            </div>
        </div>

        <div class="s_LogInButton s_CursorPointer" @click="$emit('LogInButtonClicked')">
            log in
        </div>
    </div>
</template>

<script setup lang="ts">
    import CheckIcon from './icons/CheckIcon.vue'
    import { ref } from 'vue'

    //Needs to be used when defineModel is used
    defineEmits(['LogInButtonClicked']);

    let is_checked = ref(false);
    let remember_me = defineModel<boolean>();

    function SwitchCheck() {
        is_checked.value = !is_checked.value;  
        remember_me.value = is_checked.value;
    }
</script>

<style scoped>
    .s_LoginActions {
        margin-top: 10px;

        display: flex;
        flex-direction: row;
        align-items: start;
        justify-content: space-between;
    }

    .s_NamedCheckBox {
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
    }

    .s_Checkbox {
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

    .s_Checkbox:hover {
        cursor: pointer;
    }

    .s_CheckIcon {
        height: 70%;
        width: 70%;
      
        fill: #EAEAEA;
    }

    .s_CheckBoxText {
        color: #EAEAEA;
    }

    .s_LogInButton {
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