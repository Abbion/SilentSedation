//Rework Done
<template>
    <div class="mainCardEditContainer">
        <div ref="editiOptionsDivElement" class="editOptions">
            <div class="cardNameInput">
                <input v-model="cardName" type="text" placeholder="device name"/>
                <div class="cardNameInputLine" :class="{'errorMarkerBg' : cardNameError}"></div>
            </div>

            <div class="deviceTypeSelector">
                <p class="preventSelect">
                    device type
                </p>
                <select ref="deviceTypeElement" class="deviceTypeSelectorInput">
                    <option v-for="(option, index) in deviceOptions" :key="index" :value="option.value">
                        {{ option.label }}
                    </option>
                </select>
            </div>

            <div>
                <p class="preventSelect">
                    device code
                </p>
                <div class="deviceCodeInput">
                    <div class="deviceCodeInputField" :class="{ errorOutline : deviceCodeError }" v-for="(_, index) in 6">
                        <input class="deviceCodeFieldInput" ref="deviceCodeElements" :key="index" type="text" maxlength="1"
                            @keydown="CodeDigitEntered($event as KeyboardEvent, index)"
                            @input="HandleCodeInput($event as InputEvent, index)"/>
                    </div>
                </div>
            </div>

            <ul class="errorMessages">
                <li v-for="(errorMessage, index) in errorMessages" :key="index" class="errorMessage">
                    {{ errorMessage }}
                </li>
            </ul>
        </div>

        <div class="actions">
            <div class="actionBase actionClose highlightElement cursorPointer" @click="Cancel">
                <DeleteIcon class="actionIconCancel" />
            </div>
            <div class="actionBase actionApply highlightElement cursorPointer" @click="Add">
                <CheckIcon class="actionIconAccept" />
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
    import CheckIcon from '../icons/CheckIcon.vue';
    import DeleteIcon from '../icons/DeleteIcon.vue'

    import type { CardData } from '../common/Interfaces'
    import { DeviceTypeUtils } from '../common/EnumUtils';

    import { ref, defineEmits, onUpdated } from 'vue'

    const props = defineProps<{
        cardDataProp : CardData
    }>();

    //Emiters
    const emit = defineEmits(['cardEditAddButtonClicked', 'cardEditCancelButtonClicked']);

    //Variables
    const deviceOptions = DeviceTypeUtils.CreateLabelsAndValues();

    let cardNameError = ref(false);
    let deviceCodeError = ref(false);
    let errorMessages = ref<string[]>([]);

    const cardName = ref<string>("");
    const deviceTypeElement = ref<HTMLSelectElement>();
    const deviceCodeElements = ref<HTMLInputElement[]>([]);
    const editiOptionsDivElement = ref<HTMLDivElement>();

    let wasShown = false;

    //Events
    onUpdated(() => {
        if (editiOptionsDivElement.value) {
            const hasScrollBar = editiOptionsDivElement.value.offsetHeight < editiOptionsDivElement.value.scrollHeight;
            
            if (hasScrollBar) {
                editiOptionsDivElement.value.style.paddingRight = "calc(8% - 5px)";
            }
            else {
                editiOptionsDivElement.value.style.paddingRight = "8%";
            }
        }

        if (!wasShown) {
            wasShown = true;
            if (props.cardDataProp.name != "") {
                cardName.value = props.cardDataProp.name;

                if (deviceTypeElement.value && props.cardDataProp.deviceType > 0) {
                    deviceTypeElement.value.selectedIndex = props.cardDataProp.deviceType - 1;
                }

                for (let i = 0; i < props.cardDataProp.code.length; i++) {
                    if (deviceCodeElements.value) {
                        deviceCodeElements.value[i].value = props.cardDataProp.code[i].toString();
                    }
                }
            }
        }
    });

    //Functions
    //[TODO: Move this to a seperate .vue file]
    function CodeDigitEntered(event : KeyboardEvent, index : number) {   
        const controlKeys = ["Backspace", "ArrowLeft", "ArrowRight"];
        const isNumber = /^[0-9]$/.test(event.key);
        
        //Block input for other kays than numvers and control keys
        if (!isNumber && !controlKeys.includes(event.key)) {
            event.preventDefault();
            return;
        }

        //Handle controlKeys
        if (isNumber) {
            return;
        }

        const inputIsEmpty = deviceCodeElements.value[index].value === '';

        if (inputIsEmpty)
        {
            //Backspace
            if (event.key === controlKeys[0]) {
                PreviousCodeInput(index);
            }
            //LeftArrow
            else if (event.key === controlKeys[1]) {
                PreviousCodeInput(index);
                event.preventDefault();
            }
            //RightArrow
            else if (event.key === controlKeys[2]) {
                NextCodeInput(index);
                event.preventDefault();
            }
        }
        else
        {
            const isCursorAtFront = deviceCodeElements.value[index].selectionStart === 0;    

            //LeftArrow
            if (isCursorAtFront && event.key === controlKeys[1]) {
                PreviousCodeInput(index);
                event.preventDefault();
            }
            //RightArrow
            else if (!isCursorAtFront && event.key === controlKeys[2]) {
                NextCodeInput(index);
                event.preventDefault();
            }
        }
    }

    function HandleCodeInput(event : InputEvent, index : number) {
        if (event.data == null) {
            return;
        }

        NextCodeInput(index);
    }

    function PreviousCodeInput(index : number) {
        if (index > 0) {
            deviceCodeElements.value[index - 1].focus();
        }
    }

    function NextCodeInput(index : number) {
        if (index < deviceCodeElements.value.length - 1) {
            deviceCodeElements.value[index + 1].focus();
        }
    }

    function ClearFields() {
        cardName.value = "";
        cardNameError.value = false;

        if (deviceTypeElement.value !== undefined) {
            deviceTypeElement.value.value = deviceOptions[0].value;
        }

        for (let i = 0; i < deviceCodeElements.value.length; i++) {
            deviceCodeElements.value[i].value = '';
        }

        deviceCodeError.value = false;        
        
        errorMessages.value = [];
    }

    function Cancel() {
        wasShown = false;
        ClearFields();

        emit('cardEditCancelButtonClicked');
    }

    function Add() {
        let isDataInputedCorrectely = CheckIfDataIsInputed();

        let index = 0;
        if (deviceTypeElement.value) {
            index = deviceTypeElement.value.selectedIndex;
        }

        if (isDataInputedCorrectely) {
            wasShown = false;

            const retunrCardData : CardData = { 
                id: props.cardDataProp.id,
                name: cardName.value.toString(),
                deviceType: DeviceTypeUtils.IndexToDeviceType(index),
                code: deviceCodeElements.value.filter(element => element).map(element => { return parseInt(element.value)}),
             };

             ClearFields();

            emit('cardEditAddButtonClicked', retunrCardData);
        }
    }

    function CheckIfDataIsInputed() {
        errorMessages.value = [];

        let isDataInputedCorrectely = CheckCardName();
        isDataInputedCorrectely = CheckDeviceType() && isDataInputedCorrectely;
        isDataInputedCorrectely = CheckDeviceCode() && isDataInputedCorrectely;

        return isDataInputedCorrectely;
    }

    function CheckCardName() {
        if (cardName.value !== undefined) {
            const cardNameString = cardName.value;
            let isNameInputCorrect = true;

            if (cardNameString.length < 6 || cardNameString.length > 24) {
                isNameInputCorrect = false;
                errorMessages.value.push("Device name should contain [6 - 24] characters!");
            }
            
            if (isNameInputCorrect) {
                //TODO Check if name is in database
                //errorMessages.value.push("This device name already exists in your devices!");
            }

            if (isNameInputCorrect) {
                cardNameError.value = false;
                return true;
            }
            else {
                cardNameError.value = true;
                return false;
            }
        } 
        else {
            cardNameError.value = true;
            errorMessages.value.push("Device name is undefined. Internal Error!");

            return false;
        }
    }

    function CheckDeviceType() {
        if (deviceTypeElement.value !== undefined) {
            const deviceTypeValues: string[] = deviceOptions.map(option => option.value);
            
            if (!(deviceTypeValues.includes(deviceTypeElement.value.value))) {
                errorMessages.value.push("Device type does not exist!");
                return false;
            }
        } 
        else {
            errorMessages.value.push("Device type is undefined. Internal Error!");
            return false;
        }
        return true;
    }

    function CheckDeviceCode() {
        if (deviceCodeElements.value.length > 0) {
            let isCodeCorrect = true;

            for (let i = 0; i < deviceCodeElements.value.length; i++) {
                if (deviceCodeElements.value[i] != undefined) {
                    const digit = deviceCodeElements.value[i].value;
                    
                    if (digit.length != 1 || isNaN(parseInt(digit))) {                        
                        isCodeCorrect = false;
                        errorMessages.value.push("Device code has missing values!");
                        break;
                    }
                }
                else {
                    isCodeCorrect = false;
                    errorMessages.value.push("Device code input is undefined. Internal Error!");
                    break;
                }
            }

            if (isCodeCorrect) {
                deviceCodeError.value = false;
                return true;
            }
            else {
                deviceCodeError.value = true;
                return false;
            }
        }
        else {
            errorMessages.value.push("Device code is undefined. Internal Error!");
            return false;
        }
    }

</script>

<style scoped>
    .mainCardEditContainer {
        color: var(--color-main-light);
        
        height: 100%;
        width: 100%;
        
        margin-top: 15px;

        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }

    .editOptions {
        height: 80%;
        
        padding-left: 8%;
        padding-right: 8%;

        margin-right: 4%;
        margin-left: 4%;

        overflow-y: auto;
    }

    .cardNameInput {
        margin-bottom: 15px;
    }

    .cardNameInput > input {
        background-color: transparent;
        color: var(--color-main-light);

        width: 100%;

        margin-bottom: 3px;

        border-style: none;
    }

    .cardNameInput > input:focus {
        outline: none;
    }

    .cardNameInputLine {
        background-color: var(--color-main-light);

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
        background-color: transparent;
        color: var(--color-main-light);

        height: 25px;
        width: 100%;
        
        border-color: var(--color-main-light);
        border-style: solid;
        border-width: 1px;
        border-radius: 7px;
        padding-left: 5px;
    }

    .deviceCodeInput {
        height: 25px;
        width: 100%;
        
        margin-top: 5px;

        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    .deviceCodeInputField {
        height: 100%;
        width: 18px;

        border-radius: 6px;
        border-color: var(--color-main-light);
        border-width: 2px;
        border-style: solid;

        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
    }

    .deviceCodeFieldInput {
        background-color: transparent;
        color: var(--color-main-light);

        width: 100%;
        
        border: none;
        text-align: center;
    }

    .deviceCodeFieldInput:focus {
        outline: none;
    }

    .actions {
        height: 15%;

        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    .actionBase {
        height: 100%;
        aspect-ratio: 1/1;
        
        margin-top: -2px;

        border-color: var(--color-main-light);
        border-width: 2px;
        border-style: solid;
        border-bottom: none;

        display: flex;
        justify-content: center;
        align-items: center;
    }

    .actionClose {
        border-radius: 0 20px 0 0;
        border-left: none;
    }

    .actionApply {
        border-radius: 20px 0 0 0;
        border-right: none;
    }

    .actionIconCancel {
        fill: var(--color-main-light);

        height: 40%;
        width: 40%;
    }

    .actionIconAccept {
        fill: var(--color-main-light);

        height: 48%;
        width: 48%;
    }

    .errorMarkerBg {
        background-color: var(--color-error);
    }

    .errorOutline {
        border-color: var(--color-error);
    }

    .errorMessages {
        max-height: 100px;

        padding-top: 15px;
        margin-left: 18px;
    }
    .errorMessage {
        color: var(--color-error);
        
        font-size: 14px;
    }
</style>