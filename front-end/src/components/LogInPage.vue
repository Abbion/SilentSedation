//Rework Done
<template>
    <div class="loginContainer">
        <div class="loginPageTitle preventSelect" @click="getUserTest">
        Silent sedation
        </div>

        <form>
            <LoginInput inputTitle="Master name" v-model="username"/>
            <LoginInput inputTitle="Password" :hideInput="true" v-model="password"/>

            <LoginActions v-model="remember" @logInButtonClicked="logData"/>
        </form>
    </div>
</template>

<script setup lang="ts">
    import LoginInput from './LoginInput.vue'
    import LoginActions from './LoginActions.vue'

    import axios from 'axios';

    let username : string = "";
    let password : string = "";
    let remember : boolean = false;

    function logData() {
        console.log(username, " p: ", password, " rm: ", remember);

        axios.post('http://localhost:90/login', {
            username: username,
            password: password
        })
        .then(function (response) {
            console.log(response);
        })
        .catch(function (error) {
            console.log("Cath:",  error);
        });

    }

    async function getUserTest() {
        try {
            const response = await axios.get('http://localhost:90/test');
            console.log(response.data);
        } catch (error) {
            console.error("Error", error);
        }
    }
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