//Rework 2.0
<template>
    <div class="s_LoginContainer">
        <div class="s_LoginPageTitle s_PreventSelect">
            Silent sedation
        </div>

        <form>
            <LoginInput p_input_title="Master name" v-model="username"/>
            <LoginInput p_input_title="Password" :p_hide_input="true" v-model="password"/>

            <LoginActions v-model="remember" @LogInButtonClicked="Login"/>
        </form>

        <div class="s_LoginErrorMessages" v-show="error_message">
            {{error_message}}
        </div>
    </div>
</template>

<script setup lang="ts">
    import LoginInput from './LoginInput.vue'
    import LoginActions from './LoginActions.vue'
    import { useRouter } from 'vue-router';

    import axios from 'axios';
    import { onMounted, ref } from 'vue';

    let username : string = "";
    let password : string = "";
    let remember : boolean = false;
    let error_message = ref<string>("");

    const router = useRouter();

    onMounted(() => {
        let token = localStorage.getItem('token');
        if (token !== null) {
            router.replace('/userPage');
        }
    })

    function Login() {
        error_message.value = "";

        if (username.length == 0 && password.length == 0) {
            error_message.value = "username and password fields are empty";
            return;
        }

        axios.post('http://localhost:9000/login', {
            username: username,
            password: password
        })
        .then(function (response) {
            let token = response.data["token"];
            localStorage.setItem('token', token);
            router.replace('/userPage');
        })
        .catch(function (error) {
            console.log("Cath:",  error);
            error_message.value = error.response.data["message"];
            console.log(error_message);
        });
    }
</script>

<style>
    .s_LoginContainer {
        width: 600px;
        height: 420px;

        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, calc(-50% + 35px));

        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .s_LoginPageTitle {
        color: #EAEAEA;

        width: 100%;

        margin-bottom: 50px;

        text-align: center;
        font-size: 82px;

        font-family: 'Bahnschrift Light';
    }

    @media screen and (max-width: 600px) {
        .s_LoginContainer {
            width: auto;
        }
    }

    .s_LoginErrorMessages {
        border-color: var(--color-error);
        color: var(--color-error);

        font-size: 15px;

        margin-top: 1em;
        padding: 5px;

        border-width: 1px;
        border-radius: 3px;
        border-style: solid;
    }
</style>