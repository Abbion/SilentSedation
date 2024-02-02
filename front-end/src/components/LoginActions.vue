<template>
    <div class="loginActions">
      <div class="namedCheckBox">
        <div class="checkbox" @click="switchCheck">
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

  function switchCheck() {
      isChecked.value = !isChecked.value;
      console.log(isChecked);
      
  }

</script>

<style scoped>
.loginActions {
  margin-top: 10px;

  width: 56%;

  display: flex;
  flex-direction: row;
  justify-content: space-between;
}

.namedCheckBox{
  display: flex;
  flex-direction: row;
  justify-content: center;
}

.checkbox {
    margin: 0;
    padding: 0;

    margin-right: 5px;
    margin-left: 10px;

    width: 16px;
    height: 16px;
    
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
    width: 70%;
    height: 70%;

    fill: #EAEAEA;
}

.checkBoxText {
  color: #EAEAEA;
}

button {
    width: 90px;
    height: 40px;

    background-color: #EAEAEA;
    color: #00040D;

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