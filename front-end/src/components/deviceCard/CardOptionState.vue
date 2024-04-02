//Rework Done
<template>
    <div class="centerContentVertical">
        <p class="preventSelect">
            {{ cardName }}
        </p>
        <div class="roundButton highlightElement cursorPointer" @click="$emit('cardOptionsEditButtonClicked')">
            <EditIcon class="editIcon"/>
        </div>
        <div class="roundButton highlightElement cursorPointer" @click="$emit('cardOptionsDeleteButtonClicked')">
            <TrashIcon class="trashIcon"/>
        </div>
        <div class="roundButton highlightElement cursorPointer" @click="$emit('cardOptionsReturnButtonClicked')">
            <GoBackIcon class="goBackIcon" />
        </div>
    </div>
</template>

<script setup lang="ts">
    import EditIcon from '../icons/EditIcon.vue'
    import GoBackIcon from '../icons/GoBackIcon.vue'
    import TrashIcon from '../icons/TrashIcon.vue'

    import { defineProps } from 'vue'

    defineProps<{
        cardName: string
    }>();
</script>

<style scoped>
    p {
        color: var(--color-main-light);
    }

    .roundButton {
        width: 60px;
        height: 60px;

        margin-top: 10px;

        border-color: var(--color-main-light);
        border-radius: 100%;
        border-style: solid;
        border-width: 2px;

        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .editIcon {
        stroke: var(--color-main-light);

        height: 45%;
        width: 45%;
    }

    .trashIcon {
        height: 50%;
        width: 50%;
    }

    .goBackIcon {
        fill: var(--color-main-light);

        height: 50%;
        width: 50%;

        margin-left: -2px;
    }
</style>