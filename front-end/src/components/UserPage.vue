//Rework Done
<template>
    <div class="userPageContainer">
        <div class="userPageTopPanel">
            <UserDash :username="username" />
        </div>

        <div class="userPageInfo">
            <p class="preventSelect" @click="clearStorage">
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
    import { useRouter } from 'vue-router';
    import { onMounted, ref } from 'vue';
    import axios from 'axios';

    let nextCardId = 1;
    let cardsId = ref<number[]>([]);
    let username = "empty"

    const router = useRouter();

    onMounted(() => {
        let token = localStorage.getItem('token');

        if (token !== null) {
            //Get user info
            axios.post('http://localhost:9000/get_user_page_info', {
                    token: token
            })
            //Save user info and create cards
            .then(function (response) {
                username = response.data["username"];
                let cards = response.data["card_ids"];
                
                console.log("username: ", username, " cards: ", cards);

                for (let ids in cards) {                    
                    cardsId.value.push(parseInt(ids));
                }
            })
            .then(function() {
                if (cardsId.value.length == 0) {
                    axios.post('http://localhost:9000/get_next_card_id', {
                        token : token
                    })
                    .then(function (response) {
                        console.log("Next card id: ", response.data);
                    })
		            .catch(function (error) {
                        console.log("Cath:",  error);
                    });
                }
            })
            .catch(function (error) {
                console.log("Cath:",  error);
                let status = error["response"].status;
                //Create a enunm for codes
                if (status == 401)
                {
                    clearStorage();
                    navigateToLogIn();
                }
            });
        }
        else {
            clearStorage();
            navigateToLogIn();
        }
    });

    function clearStorage() {
        localStorage.clear();
    }

    function navigateToLogIn() {
        router.replace('/');
    }

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
