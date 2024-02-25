<template>
    <p class="preventSelect">
        device name
    </p>
    <div class="roundButton highlight cursorPointer" @click="editClicked">
        <EditIcon class="editIcon"/>
    </div>
    <div class="roundButton delete cursorPointer" @click="deleteClicked">
        <DeleteIcon class="deleteIcon"/>
    </div>
    <div class="roundButton highlight cursorPointer" @click="backClicked">
        <GoBackIcon class="goBackIcon" />
    </div>
</template>

<script setup lang="ts">
    import EditIcon from '../icons/IconEdit.vue'
    import GoBackIcon from '../icons/GoBackIcon.vue'
    import DeleteIcon from '../icons/DeleteIcon.vue'
    import { defineEmits } from 'vue'
    
    const emit = defineEmits(['editButtonClicked', 'deleteButtonClicked', 'backButtonClicked']);

    function editClicked() {
        emit('editButtonClicked');
    }

    function deleteClicked() {
        emit('deleteButtonClicked');
    }

    function backClicked() {
        emit('backButtonClicked');
    }
</script>

<style scoped>
    
</style>