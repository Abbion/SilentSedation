<template>
    <div class="loginActions">
      <div class="namedCheckBox">
        <div class="checkbox" @click="SwitchCheck">
          <CheckIcon class="checkIcon" v-show="isChecked" />
        </div>
        <div class="checkBoxText preventSelect">
          Remember me
        </div>
      </div>
      <button>
        log in
      </button>
    </div>
</template>

<script setup lang="ts">
  import CheckIcon from './icons/IconCheck.vue'

  import { ref } from 'vue'
  let isChecked = ref(true);

  function SwitchCheck() {
      isChecked.value = !isChecked.value;
      console.log(isChecked);
      
  }

</script>

<style scoped>
   .loginActions {
    width: 56%;

    margin-top: 10px;

    display: flex;
    flex-direction: row;
    justify-content: space-between;
  }

  .namedCheckBox {
    display: flex;
    flex-direction: row;
    justify-content: center;
  }

  .checkbox {
    height: 16px;
      width: 16px;

      margin-right: 5px;
      margin-left: 10px;

      margin: 0;
      padding: 0;
      
      border-color: #EAEAEA;
      border-width: 1px;
      border-style: solid;
      border-radius: 5px;

      display: flex;
      align-items: center;
      justify-content: center;
  }

  .checkbox:hover {
      cursor: pointer;
  }

  .checkIcon {
    height: 70%;
      width: 70%;
      
      fill: #EAEAEA;
  }

  .checkBoxText {
    color: #EAEAEA;
  }

  button {
    background-color: #EAEAEA;
      color: #00040D;

      height: 40px;
      width: 90px;

      border: none;
      border-radius: 10px;
  }

  button:hover {
      cursor: pointer;
  }

  @media screen and (max-width: 600px) {
    .loginActions {
      width: 100%;
    }
  }
</style>