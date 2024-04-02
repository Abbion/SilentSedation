//Rework Done
<template>
    <div class="dashContainer" :class="[ dashState ? 'dashContainerOpen' : 'dashContainerClose']" @mouseenter="cursorInsideDash = true" @mouseleave="cursorInsideDash = false">
        <div id="userInfo" class="userAccount cursorPointer" @click="ToggleMore">
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
        <div class="line" v-show="dashState"></div>
        <div id="logOut" class="logOut highlightElement cursorPointer preventSelect" v-show="dashState" @click="LogOut">
            <LogOutIcon class="logOutIcon"/>
            <p id="logOutText">
                log out
            </p>
        </div>
    </div>
</template>

<script setup lang="ts">
    import UserIcon from './icons/IconUser.vue'
    import LogOutIcon from './icons/LogOutIcon.vue'
    import { ref } from 'vue'

    let dashState = ref(false)
    let cursorInsideDash = false;

    function ToggleMore() {
        dashState .value= !dashState.value;
    }

    function LogOut() {
        console.log("LogOut");
    }

    document.onmousedown = function(e) {        
        if (!cursorInsideDash) {
            dashState .value = false;
        }
    }
</script>

<style scoped>
    .dashContainer {
        margin-left: 35px;
        margin-top: 30px;

        border-color: var(--color-main-light);
        border-width: 2px;
        border-style: solid;
        border-radius: 15px;

        display: flex;
        flex-direction: column;
        align-items: space;

        font-size: 18px;
    }

    .dashContainerClose {
        height: 28px;
        width: 60px;
    }

    .dashContainerOpen {
        height: 57px;
        width: 130px;
    }

    .userAccount {
        height: 28px;
        width: 100%;

        display: flex;
        flex-direction: row;
        align-items: center;
    }

    .userIconBox {
        background-color: var(--color-main-light);

        height: 20px;
        width: 20px;

        margin-left: 5px;
        margin-right: 7px;

        border-radius: 100%;

        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;

        z-index: -1;
    }

    .userIcon {
        fill: var(--color-main-dark);

        height: 80%;
        width: 80%;

        margin-top: 4px;

        z-index: -1;
    }

    .more {
        color: var(--color-main-light);

        transform: translate(0px, -2px);

        font-size: 20px;
        z-index: -1;
    }

    .userName {
        color: var(--color-main-light);

        margin-right: 10px;
        z-index: -1;
    }

    .line {
        background-color: var(--color-main-light);
        opacity: 50%;

        height: 2px;
        width: 100%;

        margin-left: auto;
        margin-right: auto;
    }

    .logOut {
        height: 28px;
        width: 100%;

        border-radius: 0 0 12.5px 12.5px;

        display: flex;
        flex-direction: row;
        align-items: center;
    }

    .logOut > p {
        color: var(--color-main-light);
        margin-left: 5px;
    }

    .logOutIcon {
        stroke: var(--color-main-light);
        margin-left: 3px;
    }
</style>