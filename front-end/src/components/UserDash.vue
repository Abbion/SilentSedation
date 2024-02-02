<template>
    <div class="dashContainer dashContainerOpen" @click="toggleMore">
        <div class="userIconBox">
            <userIcon class="userIcon" />
        </div>
        <div class="more preventSelect">
            ...
        </div>
    </div>


</template>

<script setup lang="ts">
    import UserIcon from './icons/IconUser.vue'

    var dashState = false

    function toggleMore() {
        dashState = !dashState;

        if (dashState) {
            showDash();
        }
        else {
            closeDash();
        }
    }

    function showDash() {
        
    }

    function closeDash() {

    }
</script>

<style scoped>
.dashContainer {    
    border-color: #EAEAEA;
    border-width: 2px;
    border-style: solid;

    border-radius: 20px;

    display: flex;
    flex-direction: row;
    align-items: center;
}

.dashContainer:hover{
    cursor: pointer;
}

.dashContainerClose {
    width: 60px;
    height: 30px;
}

.dashContainerOpen {
    width: 100px;
    height: 100px;
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
    margin-top: 3px;
}

.more {
    font-size: 20px;
    color: #EAEAEA;
    padding-bottom: 3px;
}

</style>