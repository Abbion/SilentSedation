<template>
  <div class="entryContainer">
    <p class="preventSelect">
        {{ props.inputTitle }}
    </p>
    <input :type="inputType">
  </div>
</template>

<script setup lang="ts">
  interface Props {
    inputTitle: string
    hideInput?: boolean
  }

  const props = withDefaults(defineProps<Props>(), {
    hideInput: false
  })

  const inputType = props.hideInput ? 'password' : 'text';
</script>

<style scoped>
p {
  margin: 0;
  padding: 0;

  margin-bottom: 0.5em;
  margin-left: 0.5em;

  color: #EAEAEA;
}

input {
  margin: 0;
  padding: 0;

  margin-bottom: 1em;

  padding-left: 0.5em;
  padding-right: 0.5em;

  width: 24em;
  height: 2.5em;
  
  border-radius: 10px;
  background-color: transparent;
  
  border-style: solid;
  border-color: #eaeaeaa2;
  border-width: 1px;

  color: #EAEAEA;
}

input:focus{
  outline: none;
  border-color: #EAEAEA;
}

</style>
