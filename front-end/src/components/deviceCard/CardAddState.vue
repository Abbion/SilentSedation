//Rework 2.0
<template>
    <div class="s_CenterContentVertical">
        <p class="s_PreventSelect">
            new device
        </p>
        <div class="s_RoundButton">
            <AddIcon class="s_AddIcon"/>
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

    .s_RoundButton {
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

    .s_AddIcon{
        fill: var(--color-main-light);
        width: 30%;
        height: 30%;
    }
</style>