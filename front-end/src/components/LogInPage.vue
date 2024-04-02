//Rework Done
<template>
    <div class="loginContainer">
        <div class="loginPageTitle preventSelect">
        Silent sedation
        </div>

        <LoginInput inputTitle="Master name"/>
        <LoginInput inputTitle="Password" :hideInput="true"/>

        <LoginActions/>
    </div>
</template>

<script setup lang="ts">
    import LoginInput from './LoginInput.vue'
    import LoginActions from './LoginActions.vue'
</script>

<style>
    .loginContainer {
        width: 600px;

        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);

        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .loginPageTitle {
        color: #EAEAEA;

        width: 100%;

        margin-bottom: 50px;

        text-align: center;
        font-size: 82px;

        font-family: 'Bahnschrift Light';
    }

    @media screen and (max-width: 600px) {
        .loginContainer {
            width: auto;
        }
    }
</style>