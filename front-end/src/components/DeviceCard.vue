<template>
    <div class="cardContainer cardCenter" ref="cardContainerHTML">
        <div v-show="state == CardState.Add" class="addState highlight cardCenter cursorPointer" @click="GoToEditState">
            <p class="preventSelect">
                new device
            </p>
            <div class="roundButton cursorPointer">
                <AddIcon class="addIcon"/>
            </div>
        </div>

        <div v-show="state == CardState.Use" class="useState">
            <div class="useHeader">
                <p class="preventSelect">
                    Title
                </p>
                <div class="useOptions cursorPointer" @click="GoToOptionState">...</div>
            </div>
            <div class="useContent">

            </div>
        </div>

        <div v-show="state == CardState.Edit" class="editState">
            <div class="editOptions">
                <div class="deviceNameInput">
                    <input type="text" placeholder="device name"/>
                    <div class="deviceNameInputLine"></div>
                </div>
                <div class="deviceTypeSelector">
                    <p class="preventSelect">
                        device type
                    </p>
                    <select class="deviceTypeSelectorInput">
                        <option value="electric-shocker">electric shocker</option>
                    </select>
                </div>
                <div class="deviceCode">
                    <p class="preventSelect">
                        device code
                    </p>

                    <div class="deviceCodeInput">
                        <div class="deviceCodeField" v-for="field in 6">
                            <input class="deviceCodeFieldInput" type="text">
                        </div>
                    </div>
                </div>
            </div>
            <div class="editActions">
                <div class="editActionBase editActionsClose cursorPointer" @click="GoToAddState">
                    <DeleteIcon class="editActionIcon" />
                </div>
                <div class="editActionBase editActionsApply cursorPointer" @click="GoToUseState">
                    <CheckIcon class="editActionIcon" />
                </div>
            </div>
        </div>

        <div v-show="state == CardState.Options" class="editState cardCenter">
            <p class="preventSelect">
                device name
            </p>
            <div class="roundButton highlight cursorPointer" @click="GoToEditState">
                <EditIcon class="editIcon"/>
            </div>
            <div class="roundButton delete cursorPointer" @click="DeleteCard">
                <DeleteIcon class="deleteIcon"/>
            </div>
            <div class="roundButton highlight cursorPointer" @click="GoToUseState">
                <GoBackIcon class="goBackIcon" />
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
    import AddIcon from './icons/IconAdd.vue';
    import CheckIcon from './icons/IconCheck.vue';
    import EditIcon from './icons/IconEdit.vue'
    import DeleteIcon from './icons/DeleteIcon.vue'
    import GoBackIcon from './icons/GoBackIcon.vue'
    import { ref } from 'vue'

    const code = [0, 0, 0, 0, 0, 0];

    enum CardState {
        Add,
        Edit,
        Options,
        Use
    }

    var state = ref(CardState.Add);
    var cardContainerHTML = ref<HTMLDivElement | null>(null);

    function GoToEditState() {        
        state.value = CardState.Edit;

        if (cardContainerHTML.value) {
            cardContainerHTML.value.style.borderStyle = "solid";
        }
    }

    function GoToAddState() {
        state.value = CardState.Add;

        if (cardContainerHTML.value) {
            cardContainerHTML.value.style.borderStyle = "dashed";   
        }
    }

    function GoToOptionState() {
        state.value = CardState.Options;
    }

    function GoToUseState() {
        state.value = CardState.Use;
    }

    function DeleteCard() {
        state.value = CardState.Add;

        if (cardContainerHTML.value) {
            cardContainerHTML.value.style.borderStyle = "dashed";   
        }
    }
</script>

<style>
.cardContainer {
    width: 200px;
    height: 320px;
    
    border-color: #EAEAEA;
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

    border-color: #EAEAEA;
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
    color: #EAEAEA;;
    
    font-size: 25px;
}

.useState {
    
    height: 100%;
    width: 100%;
}

.useHeader {
    height: 12%;
    
    color: #EAEAEA;

    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: end;
}

.useOptions {
    position: absolute;
    transform: translate(72px, -12px);
    font-size: 20px;
}

.editState > p {
    color: #EAEAEA;
    
    font-size: 18px;
    margin-bottom: 10px;
}

.addIcon{
    fill: #EAEAEA;
    width: 30%;
    height: 30%;
}

.editIcon {
    fill: #EAEAEA;
    width: 50%;
    height: 50%;
}

.deleteIcon {
    fill: #EAEAEA;
}

.goBackIcon {
    fill: #EAEAEA;
}

.editState {
    color: white;
    
    height: 100%;
    width: 100%;

    margin-top: 15px;
}

.editOptions{


    height: 85%;
    width: 80%;
    margin: auto;
}

.deviceNameInput{
    margin-bottom: 15px;
}

.deviceNameInput > input {
    width: 100%;
    background-color: transparent;
    border-style: none;
    margin-bottom: 3px;
    color: #EAEAEA;
}

.deviceNameInput > input:focus {
    outline: none;
}

.deviceNameInputLine {
    background-color: azure;
    height: 1px;
    width: 100%;
}

.deviceTypeSelector {
    margin-bottom: 15px;
}

.deviceTypeSelector > p {
    margin-bottom: 5px;
}

.deviceTypeSelectorInput {
    width: 100%;
    height: 25px;

    background-color: transparent;
    color: #EAEAEA;
    
    border-color: #EAEAEA;
    border-style: solid;
    border-radius: 15px;
    padding-left: 5px;
}

.editActions {
    height: 15%;

    display: flex;
    flex-direction: row;
    justify-content: space-between;
}

.editActionBase {
    height: 100%;
    margin-top: -2px;

    aspect-ratio: 1/1;

    display: flex;
    justify-content: center;
    align-items: center;

    border-color: #EAEAEA;
    border-width: 2px;
    border-style: solid;
}

.editActionsClose {
    border-radius: 0 20px 0 20px;
    margin-left: -1.8px;
    background-color: #E1B5B5;
}

.editActionsClose:hover {
    background-color: #e79c9c;
}

.editActionsApply {
    border-radius: 20px 0 20px 0;
    margin-right: -1.8px;
    background-color: #9EDDB7;
}

.editActionsAdd:hover{
    background-color: #80c89d;
}

.editActionIcon {
    fill: #00040f;
    height: 40%;
    width: 40%;
}

.highlight:hover {
    background-color: #eaeaea24;
}

.delete {
    background-color: #E1B5B5;
}

.delete:hover {
    background-color: #e79c9c;
}

.deviceCodeInput {
    margin-top: 10px;
    width: 100%;
    height: 25px;

    display: flex;
    flex-direction: row;
    justify-content: space-between;
}

.deviceCodeField {
    height: 100%;
    width: 18px;

    border-radius: 6px;
    border-color: #EAEAEA;
    border-width: 2px;
    border-style: solid;

    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
}

.deviceCodeFieldInput {
    width: 100%;
    background-color: transparent;
    border: none;
    color: #EAEAEA;

    text-align: center;
}

.deviceCodeFieldInput:focus {
    outline: none;
}

</style>