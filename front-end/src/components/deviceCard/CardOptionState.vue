/*REFACTOR 4.0*/
<template>
    <div class="s_CenterContentVertical">
        <p class="s_PreventSelect">
            {{ p_card_name }}
        </p>
        <div class="s_RoundButton s_HighlightElement s_CursorPointer" @click="$emit('CardOptionsDeleteButtonClicked')">
            <TrashIcon class="s_TrashIcon"/>
        </div>
        <div class="s_RoundButton s_HighlightElement s_CursorPointer" @click="$emit('CardOptionsReturnButtonClicked')">
            <GoBackIcon class="s_GoBackIcon" />
        </div>
    </div>
</template>

<script setup lang="ts">
    import GoBackIcon from '../icons/GoBackIcon.vue'
    import TrashIcon from '../icons/TrashIcon.vue'

    import { defineProps } from 'vue'

    defineProps<{
        p_card_name: string
    }>();
</script>

<style scoped>
    p {
        color: var(--color-main-light);
    }

    .s_RoundButton {
        width: 60px;
        height: 60px;

        margin-top: 20px;

        border-color: var(--color-main-light);
        border-radius: 100%;
        border-style: solid;
        border-width: 2px;

        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .s_TrashIcon {
        height: 50%;
        width: 50%;
    }

    .s_GoBackIcon {
        fill: var(--color-main-light);

        height: 50%;
        width: 50%;

        margin-left: -2px;
    }
</style>