<template>
  <!-- 
  <div class="loginContainer">
    
    <div class="title preventSelect">
      Silent sedation
    </div>

    <LoginInput inputTitle="Master name"/>
    <LoginInput inputTitle="Password" :hideInput="true"/>

    <LoginActions/>
  </div>
  -->

  <div class="mainPageContainer">
    <div class="topPanel">
      <UserDash />
    </div>
    <div class="pageInfo">
      <p class="preventSelect">
        Devices
      </p>
      <div class="pageInfoLine"></div>

    </div>
    <div class="content">
      <DeviceCard v-for="(id) in cardsId" :key="id" :cardId="id" @cardCreated="AddNewCard" @cardRemoved="RemoveCard"/>
    </div>
    
  </div>
  
</template>

<script setup lang="ts">
  import LoginInput from './components/LoginInput.vue'
  import LoginActions from './components/LoginActions.vue'

  import UserDash from './components/UserDash.vue'
  import DeviceCard from './components/deviceCard/DeviceCard.vue'
  import { ref } from 'vue';

  let nextCardId = 1;
  let cardsId = ref<number[]>([0]);

  function AddNewCard() {
    cardsId.value.push(nextCardId);
    nextCardId++;
  }

  function RemoveCard(id: number) {
    const index = cardsId.value.indexOf(id);

    console.log("id: ", id, " index: ", index);
    

    if (index !== -1) {
      cardsId.value.splice(index, 1);
    }
  }

</script>

<style scoped>

.mainPageContainer {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.topPanel {
  height:80px;
  width: 100%;
  
  padding-top: 40px;
  padding-left: 70px;
}

.pageInfo{
  width: 75%;
}

.pageInfo > p {
  margin: 0;
  padding:  0;
  
  color: #EAEAEA;
  font-size: 24px;

  margin-bottom: 10px;
}

.pageInfoLine {
  background-color: #EAEAEA;
  width: 100%;
  height: 2px;

  margin-bottom: 40px;
}

.content {
  display: flex;
  flex-direction: row;
  justify-content: center;
  width: 75%;
}

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

.title {
  margin-bottom: 50px;
  width: 100%;

  text-align: center;
  font-size: 82px;

  color: #EAEAEA;
  font-family: 'Bahnschrift Light';
}

@media screen and (max-width: 600px) {
  .loginContainer {
    width: auto;
  }
}
</style>