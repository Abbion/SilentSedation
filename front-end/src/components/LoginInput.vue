/*REFACTOR 4.0*/
<template>
    <div class="s_EntryContainer">
        <p class="s_PreventSelect">
            {{ props.p_input_title }}
        </p>
        <input :type="inputType" v-model="value">
    </div>
</template>

<script setup lang="ts">
    interface Props {
        p_input_title: string,
        p_hide_input?: boolean
    };

    const props = withDefaults(defineProps<Props>(), {
        p_hide_input: false
    });

    let value = defineModel<String>()
    const inputType = props.p_hide_input ? 'password' : 'text'
</script>

<style scoped>
    p {
        color: #EAEAEA;

        margin: 0;
        padding: 0;

        margin-bottom: 0.5em;
        margin-left: 0.2em;
    }

    input {
        background-color: transparent;

        height: 2.5em;
        width: 24em;

        margin: 0;
        padding: 0;

        margin-bottom: 1em;

        padding-left: 0.5em;
        padding-right: 0.5em;

        border-style: solid;
        border-radius: 10px;
        border-color: #eaeaeaa2;
        border-width: 1px;

        color: #EAEAEA;
    }

    input:focus{
        outline: none;
        border-color: #EAEAEA;
    }
</style>