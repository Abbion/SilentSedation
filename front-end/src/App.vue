<template>
  <div class="loginContainer">
    <div class="title">
      Silent sedation
    </div>

    <LoginInput inputTitle="Master name"/>
    <LoginInput inputTitle="Password" :hideInput="true"/>

    <LoginActions/>
  </div>
</template>

<script setup lang="ts">
import LoginInput from './components/LoginInput.vue'
import LoginActions from './components/LoginActions.vue'
import HelloWorld from './components/HelloWorld.vue'
import TheWelcome from './components/TheWelcome.vue'
</script>

<style scoped>
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

.title{
  margin-bottom: 50px;
  width: 100%;

  text-align: center;
  font-size: 82px;
}
</style>
