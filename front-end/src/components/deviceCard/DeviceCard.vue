<template>
    <div class="cardContainer cardCenter" ref="cardContainerHTML">
        <div v-show="state == CardState.Add" class="addState highlight cardCenter cursorPointer" @click="goToEditState">
            <p class="preventSelect">
                new device
            </p>
            <div class="roundButton cursorPointer">
                <AddIcon class="addIcon"/>
            </div>
        </div>

        <div v-show="state == CardState.Use" class="useState">
            <UseMode @optionsWidgetClicked="goToOptionState"></UseMode>
        </div>

        <div v-show="state == CardState.Edit" class="editState">
            <EditMode @addButtonClicked="handleNewData" @cancelButtonClicked="goToAddState"></EditMode>
        </div>

        <div v-show="state == CardState.Options" class="editState cardCenter">
           <OptionMode @editButtonClicked="goToEditState" @deleteButtonClicked="goToAddState" @backButtonClicked="goToUseState"></OptionMode> 
        </div>
    </div>
</template>

<script setup lang="ts">
    import EditMode from './DeviceCardEditMode.vue'
    import OptionMode from './DeviceCardOptionMode.vue'
    import UseMode from './DeviceCardUseMode.vue'

    import AddIcon from '../icons/IconAdd.vue';
    import { ref } from 'vue'

    import type { CardData } from '../common/Interfaces'
    import { DeviceType } from '../common/Enums';

    enum CardState {
        Add,
        Edit,
        Options,
        Use
    }

    var state = ref(CardState.Add);
    var cardContainerHTML = ref<HTMLDivElement>();
    var cardData: CardData = { name: "", deviceType: DeviceType.None, code: [] }

    function goToEditState() {        
        state.value = CardState.Edit;

        if (cardContainerHTML.value) {
            cardContainerHTML.value.style.borderStyle = "solid";
        }
    }

    function goToAddState() {
        state.value = CardState.Add;

        if (cardContainerHTML.value) {
            cardContainerHTML.value.style.borderStyle = "dashed";   
        }
    }

    function goToOptionState() {
        state.value = CardState.Options;
    }

    function goToUseState() {
        state.value = CardState.Use;
    }

    function DeleteCard() {
        state.value = CardState.Add;

        if (cardContainerHTML.value) {
            cardContainerHTML.value.style.borderStyle = "dashed";   
        }
    }

    function handleNewData(data : CardData) {
        cardData = data;
        goToUseState();
    }

    
</script>

<style>
.cardContainer {
    width: 200px;
    height: 320px;
    
    border-color: var(--color-main-light);
    border-radius: 20px;
    border-width: 2px;
    border-style: dashed;

    margin-left: 20px;
    margin-right: 20px;
}

.cardCenter {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}

.roundButton {
    width: 60px;
    height: 60px;

    margin-top: 10px;
    margin-bottom: 10px;

    border-color: var(--color-main-light);
    border-radius: 100%;
    border-style: solid;
    border-width: 2px;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}

.addState {
    height: 100%;
    width: 100%;
    border-radius: 18px;
}

.addState > p {
    color: var(--color-main-light);
    
    font-size: 25px;
}

.useState {
    
    height: 100%;
    width: 100%;
}

.editState > p {
    color: var(--color-main-light);
    
    font-size: 18px;
    margin-bottom: 10px;
}

.addIcon{
    fill: var(--color-main-light);
    width: 30%;
    height: 30%;
}

.editIcon {
    fill: var(--color-main-light);
    width: 50%;
    height: 50%;
}

.deleteIcon {
    fill: var(--color-main-light);
}

.goBackIcon {
    fill: var(--color-main-light);
}

.editState {
    color: var(--color-main-light);
    
    height: 100%;
    width: 100%;

    margin-top: 15px;

    display: flex;
    flex-direction: column;
    justify-content: space-between;
}

.highlight:hover {
    background-color: var(--color-main-dark--highlight);
}

.delete {
    background-color: var(--color-red);
}

.delete:hover {
    background-color: var(--color-red-highlight);
}

</style>