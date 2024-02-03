<template>
    <div class="dashContainer" :class="{dashContainerClose: !dashState, dashContainerOpen: dashState}">
        <div class="userAccount" @click="toggleMore">
            <div class="userIconBox">
                <userIcon class="userIcon" />
            </div>
            <div v-show="!dashState" class="more preventSelect">
                ...
            </div>
            <div v-show="dashState" class="userName preventSelect">
                Wiktor
            </div>
        </div>
        <div v-show="dashState" class="logOut preventSelect" @click="logOut">
            <p>
                log out
            </p>
        </div>
    </div>


</template>

<script setup lang="ts">
    import UserIcon from './icons/IconUser.vue'
    import { ref } from 'vue'

    var dashState = ref(false)

    function toggleMore() {
        dashState .value= !dashState.value;
    }

    function logOut() {
        console.log("LogOut");
    }

</script>

<style scoped>
.dashContainer {    
    border-color: #EAEAEA;
    border-width: 2px;
    border-style: solid;

    border-radius: 15px;

    display: flex;
    flex-direction: column;
    align-items: space;
}

.userAccount {
    width: 100%;

    display: flex;
    flex-direction: row;
    align-items: center;

}

.logOut {
    width: 100%;
    height: 28px;

    border-radius: 0 0 12px 12px;
    margin-top: 6px;

    background-color: #ecaaaa;
    z-index: -1;
}

.logOut > p {
    margin-left: 10px;
    
    font-size: 20px;
}

.dashContainer:hover{
    cursor: pointer;
}

.dashContainerClose {
    width: 60px;
    max-height: 30px;
}

.dashContainerOpen {
    width: 100px;
    height: 55px;
}

.userIconBox{
    width: 20px;
    height: 20px;
    background-color: #EAEAEA;
    border-radius: 100%;

    margin-left: 5px;
    margin-right: 7px;

    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
}

.userIcon{
    width: 80%;
    height: 80%;

    fill: #00040f;
    margin-top: 4px;
}

.more {
    font-size: 20px;
    color: #EAEAEA;

    transform: translate(0px, -2px);
}

.userName {
    font-size: 20px;
    color: #EAEAEA;

    transform: translate(0px, 2px);
}

</style>