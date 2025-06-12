/*REFACTOR 4.0*/
<template>
    <div class="s_UserPageContainer">
        <div class="s_UserPageTopPanel">
            <UserDash :username="username" />
        </div>

        <div class="s_UserPageInfo">
            <p class="s_PreventSelect">
                Devices
            </p>

            <div class="s_UserPageInfoLine"></div>
        </div>

        <div class="s_UserPageContent">
            <DeviceCard v-for="(id) in cards_id" :key="id" :p_card_id="id" :p_card_status="card_statuses.get(id) ?? ConnectionStatus.Connecting" @CardCreated="AddEmptyCard" @CardRemoved="RemoveCard"/>
        </div>
    </div>
</template>

<script setup lang="ts">
    import UserDash from './UserDash.vue'
    import DeviceCard from './deviceCard/DeviceCard.vue'
    import { useRouter } from 'vue-router'
    import { onBeforeUnmount, onMounted, ref } from 'vue'
    import axios from 'axios'
    import { ConnectionStatus } from './common/Enums'

    let cards_id = ref<number[]>([])
    let card_statuses = ref(new Map<number, ConnectionStatus>)

    let username = "empty"
    let state_udpate_call: number

    const router = useRouter()
    const fetch_device_state_interval = 5000

    onMounted(() => {
        let token = localStorage.getItem('token')

        if (token !== null) {
            //Get user info
            axios.post('http://localhost:9000/get_user_page_info', {
                    token: token
            })
            //Save user info and create cards
            .then(function (response) {
                username = response.data["username"]
                let cards = response.data["card_ids"]

                for (let ids of cards) {  
                    let id = parseInt(ids)
                    cards_id.value.push(id)
                    card_statuses.value.set(id, ConnectionStatus.Connecting)
                }
            })
            .catch(function (error) {
                console.log("Cath:",  error)
                let status = error["response"].status
                //Create a enunm for codes
                if (status == 401)
                {
                    ClearStorage()
                    NavigateToLogIn()
                }
            });
        }
        else {
            ClearStorage()
            NavigateToLogIn()
        }

        state_udpate_call = window.setInterval(fetchDeviceState, fetch_device_state_interval)
    });

    onBeforeUnmount(() => {
        clearInterval(state_udpate_call)
    })

    function fetchDeviceState() {
        let token = localStorage.getItem('token')

        if (token !== null) {
            axios.post('http://localhost:9000/get_card_states', {
                    token: token
            })
            .then(function (response) {
                let data = response.data
                for ( const card_status of data) {
                    card_statuses.value.set(card_status.card_id, card_status.status)
                }
            })
        }
    }

    function ClearStorage() {
        localStorage.clear()
    }

    function NavigateToLogIn() {
        router.replace('/')
    }

    function AddEmptyCard() {
        let token = localStorage.getItem('token');

        axios.post('http://localhost:9000/create_card',{
            token: token
        }).then(function(response){
            let next_id = response.data["card_id"]
            cards_id.value.push(next_id)
        }).catch(function(error) {
            console.log("User page - Add empty card error: ", error)
        })
    }

    function RemoveCard(id: number) {
        const index = cards_id.value.indexOf(id)
        
        if (index !== -1) {
            cards_id.value.splice(index, 1)
        }
    }
</script>

<style scoped>
    .s_UserPageContainer {
        width: 1200px;

        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .s_UserPageTopPanel {
        position: fixed;
        height:80px;
        width: 100%;
        
        z-index: 10;
    }

    .s_UserPageInfo {
        margin-top: 80px;
        width: 95%;
    }

    .s_UserPageInfo > p {
        margin: 0;
        padding:  0;
    
        color: #EAEAEA;
        font-size: 24px;

        margin-bottom: 10px;
    }

    .s_UserPageInfoLine {
        background-color: #EAEAEA;
        height: 2px;
        width: 100%;

        margin-bottom: 40px;
    }

    .s_UserPageContent {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: center;
        max-width: 75%;
    }
    @media only screen and (max-width: 1200px) {
        .s_UserPageContainer {
            width: 800px;
        }
    }

    @media only screen and (max-width: 800px) {
        .s_UserPageContainer {
            width: 400px;
        }
    }

    @media only screen and (max-width: 400px) {
        .s_UserPageContainer {
            width: 300px;
        }
    }
</style>
