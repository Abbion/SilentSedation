<template>
    <div class="dashContainer" :class="{dashContainerClose: !dashState, dashContainerOpen: dashState}">
        <div id="userInfo" class="userAccount cursorPointer" @click="toggleMore">
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
        <div id="logOut" v-show="dashState" class="logOut cursorPointer preventSelect" @click="logOut">
            <p id="logOutText">
                log out
            </p>
        </div>
    </div>


</template>

<script setup lang="ts">
    import UserIcon from './icons/IconUser.vue'
    import { ref } from 'vue'

    var dashState = ref(false)
    var dashClicked = false;

    function toggleMore() {
        dashState .value= !dashState.value;
    }

    function logOut() {
        console.log("LogOut");
    }

    document.onmousedown = function(e) {
        const attr = e.target.attributes;
        if (attr) {
            const idAttr = attr['id'];
            if (idAttr){
                const value = idAttr.value;
                //This is pain XD
                if (value == "userInfo" || value == "logOut" || value == "logOutText"){
                    return;
                }
            }
        }
        
        if (dashState.value) {
            dashState .value = false;
        }
    }

</script>

<style scoped>
.dashContainer {    
    border-color: var(--color-main-light);
    border-width: 2px;
    border-style: solid;

    border-radius: 15px;

    display: flex;
    flex-direction: column;
    align-items: space;
}

.userAccount {
    width: 100%;
    height: 28px;

    display: flex;
    flex-direction: row;
    align-items: center;

}

.logOut {
    width: 100%;
    height: 28px;

    background-color: red;
    border-radius: 0 0 12.5px 12.5px;
}

.logOut > p {
    margin-left: 15px;
}

.dashContainerClose {
    width: 60px;
    height: 28px;
}

.dashContainerOpen {
    width: fit-content;
    height: 55px;
}

.userIconBox{
    width: 20px;
    height: 20px;
    background-color: var(--color-main-light);
    border-radius: 100%;

    margin-left: 5px;
    margin-right: 7px;

    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;

    z-index: -1;
}

.userIcon{
    width: 80%;
    height: 80%;

    fill: var(--color-main-dark);
    margin-top: 4px;

    z-index: -1;
}

.more {
    font-size: 20px;
    color: var(--color-main-light);;

    transform: translate(0px, -2px);

    z-index: -1;
}

.userName {
    color: var(--color-main-light);
    margin-right: 10px;

    z-index: -1;
}

</style>