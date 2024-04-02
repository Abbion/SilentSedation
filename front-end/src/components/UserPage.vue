//Rework Done
<template>
    <div class="userPageContainer">
        <div class="userPageTopPanel">
            <UserDash />
        </div>

        <div class="userPageInfo">
            <p class="preventSelect" @click="AddNewCard">
                Devices
            </p>

            <div class="userPageInfoLine"></div>
        </div>

        <div class="userPageContent">
            <DeviceCard v-for="(id) in cardsId" :key="id" :cardId="id" @cardCreated="AddNewCard" @cardRemoved="RemoveCard"/>
        </div>
    </div>
</template>

<script setup lang="ts">
    import UserDash from './UserDash.vue'
    import DeviceCard from './deviceCard/DeviceCard.vue'

    import { ref } from 'vue';

    let nextCardId = 1;
    let cardsId = ref<number[]>([0]);

    function AddNewCard() {
        cardsId.value.push(nextCardId);
        nextCardId++;
    }

    function RemoveCard(id: number) {
        const index = cardsId.value.indexOf(id);
        
        if (index !== -1) {
            cardsId.value.splice(index, 1);
        }
    }
</script>

<style scoped>
    .userPageContainer {
        width: 1280px;

        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .userPageTopPanel {
        position: fixed;
        height:80px;
        width: 100%;
    }

    .userPageInfo {
        margin-top: 80px;
        width: 75%;
    }

    .userPageInfo > p {
        margin: 0;
        padding:  0;
    
        color: #EAEAEA;
        font-size: 24px;

        margin-bottom: 10px;
    }

    .userPageInfoLine {
        background-color: #EAEAEA;
        height: 2px;
        width: 100%;

        margin-bottom: 40px;
    }

    .userPageContent {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: center;
        max-width: 75%;
    }
</style>