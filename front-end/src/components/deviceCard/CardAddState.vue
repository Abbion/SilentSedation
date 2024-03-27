//Rework Done
<template>
    <div class="centerContentVertical">
        <p class="preventSelect">
            new device
        </p>
        <div class="roundButton">
            <AddIcon class="addIcon"/>
        </div>
    </div>
</template>

<script setup lang="ts">
    import AddIcon from '../icons/AddIcon.vue';
</script>

<style scoped>
    p {
        color: var(--color-main-light);
        font-size: 25px;
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

    .addIcon{
        fill: var(--color-main-light);
        width: 30%;
        height: 30%;
    }
</style>