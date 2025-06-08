/*REFACTOR 4.0*/
<template>
    <div class="s_DashContainer" :class="[ dash_state ? 's_DashContainerOpen' : 's_DashContainerClose']" @mouseenter="cursor_inside_dash = true" @mouseleave="cursor_inside_dash = false">
        <div id="s_UserInfo" class="s_UserAccount s_CursorPointer" @click="ToggleMore">
            <div class="s_UserIconBox">
                <userIcon class="s_UserIcon" />
            </div>
            <div v-show="!dash_state" class="s_More s_PreventSelect">
                ...
            </div>
            <div v-show="dash_state" class="s_UserName s_PreventSelect">
                {{ username }}
            </div>
        </div>
        <div class="s_Line" v-show="dash_state"></div>
        <div class="s_LogOut s_HighlightElement s_CursorPointer s_PreventSelect" v-show="dash_state" @click="LogOut">
            <LogOutIcon class="s_LogOutIcon"/>
            <p>
                log out
            </p>
        </div>
    </div>
</template>

<script setup lang="ts">
    import UserIcon from './icons/IconUser.vue'
    import LogOutIcon from './icons/LogOutIcon.vue'
    import { ref } from 'vue'
    import { useRouter } from 'vue-router';

    const router = useRouter()
    let dash_state = ref(false)
    let cursor_inside_dash : boolean = false

    defineProps<{
        username : string
    }>();

    function ToggleMore() {
        dash_state .value= !dash_state.value
    }

    function LogOut() {
        router.replace('/')
        localStorage.clear()
    }

    document.onmousedown = function(_) {
        if (!cursor_inside_dash) {
            dash_state .value = false
        }
    }
</script>

<style scoped>
    .s_DashContainer {
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

    .s_DashContainerClose {
        height: 28px;
        width: 60px;
    }

    .s_DashContainerOpen {
        height: 57px;
        width: 130px;
    }

    .s_UserAccount {
        height: 28px;
        width: 100%;

        display: flex;
        flex-direction: row;
        align-items: center;
    }

    .s_UserIconBox {
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

    .s_UserIcon {
        fill: var(--color-main-dark);

        height: 80%;
        width: 80%;

        margin-top: 4px;

        z-index: -1;
    }

    .s_More {
        color: var(--color-main-light);

        transform: translate(0px, -2px);

        font-size: 20px;
        z-index: -1;
    }

    .s_UserName {
        color: var(--color-main-light);

        margin-right: 10px;
        z-index: -1;
    }

    .s_Line {
        background-color: var(--color-main-light);
        opacity: 50%;

        height: 2px;
        width: 100%;

        margin-left: auto;
        margin-right: auto;
    }

    .s_LogOut {
        height: 28px;
        width: 100%;

        border-radius: 0 0 12.5px 12.5px;

        display: flex;
        flex-direction: row;
        align-items: center;
    }

    .s_LogOut > p {
        color: var(--color-main-light);
        margin-left: 5px;
    }

    .s_LogOutIcon {
        stroke: var(--color-main-light);
        margin-left: 3px;
    }
</style>