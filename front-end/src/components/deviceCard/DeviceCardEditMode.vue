<template>
    <div class="editOptions">
        <div class="deviceNameInput">
            <input v-model="deviceName" type="text" placeholder="device name"/>
            <div ref="deviceNameLineElement" class="deviceNameInputLine"></div>
        </div>

        <div class="deviceTypeSelector">
            <p class="preventSelect">
                device type
            </p>
            <select ref="deviceTypeElement" class="deviceTypeSelectorInput">
                <option v-for="(option, index) in options" :key="index" :value="option.value">
                    {{ option.label }}
                </option>
            </select>
        </div>

        <div>
            <p class="preventSelect">
                device code
            </p>
            <div class="deviceCodeInput">
                <div class="deviceCodeInputField" v-for="(_, index) in 6">
                    <input class="deviceCodeFieldInput" ref="deviceCodeElements" :key="index" type="text" maxlength="1"
                        @keydown="codeDigitEntered($event as KeyboardEvent, index)"
                        @input="handleCodeInput($event as InputEvent, index)"/>
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
        <div class="actionBase actionClose cursorPointer" @click="cancel">
            <DeleteIcon class="actionIcon" />
        </div>
        <div class="actionBase actionApply cursorPointer" @click="add">
            <CheckIcon class="actionIcon" />
        </div>
    </div>
</template>

<script setup lang="ts">
    import CheckIcon from '../icons/IconCheck.vue';
    import DeleteIcon from '../icons/DeleteIcon.vue'
    import { ref, defineEmits, onMounted } from 'vue'

    interface Option {
        label: string;
        value: string;
    }

    const options: Option[] = [
        { label: 'electric shocker', value: 'e-shock' },
    ];
    var errorMessages = ref<string[]>();

    const emit = defineEmits(['addButtonClicked', 'cancelButtonClicked']);

    const deviceName = ref<String>("");
    const deviceTypeElement = ref<HTMLSelectElement>();
    const deviceCodeElements = ref<HTMLInputElement[]>([]);

    const deviceNameLineElement = ref<HTMLDivElement>();

    function codeDigitEntered(event : KeyboardEvent, index : number) {   
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
                previousCodeInput(index);
            }
            //LeftArrow
            else if (event.key === controlKeys[1]) {
                previousCodeInput(index);
                event.preventDefault();
            }
            //RightArrow
            else if (event.key === controlKeys[2]) {
                nextCodeInput(index);
                event.preventDefault();
            }
        }
        else
        {
            const isCursorAtFront = deviceCodeElements.value[index].selectionStart === 0;    

            //LeftArrow
            if (isCursorAtFront && event.key === controlKeys[1]) {
                previousCodeInput(index);
                event.preventDefault();
            }
            //RightArrow
            else if (!isCursorAtFront && event.key === controlKeys[2]) {
                nextCodeInput(index);
                event.preventDefault();
            }
        }
    }

    function handleCodeInput(event : InputEvent, index : number) {
        if (event.data == null) {
            return;
        }

        nextCodeInput(index);
    }

    function previousCodeInput(index : number) {
        if (index > 0) {
            deviceCodeElements.value[index - 1].focus();
        }
    }

    function nextCodeInput(index : number) {
        if (index < deviceCodeElements.value.length - 1) {
            deviceCodeElements.value[index + 1].focus();
        }
    }

    function clearFields() {
        for (let i = 0; i < deviceCodeElements.value.length; i++) {
            deviceCodeElements.value[i].value = '';
        }

        deviceName.value = "";

        if (deviceTypeElement.value !== undefined) {
            deviceTypeElement.value.value = options[0].value;
        }
    }

    function cancel() {
        clearFields();

        emit('cancelButtonClicked');
    }

    function add() {
        var isDataInputedCorrectely = checkIfDataIsInputed();
        console.log(errorMessages);
        

       // emit('addButtonClicked');
    }

    function checkIfDataIsInputed() {
        var isDataInputedCorrectely = true;
        errorMessages.value = [];

        //=========================================================================================
        if (deviceName.value !== undefined) {
            const deviceNameString = deviceName.value;
            var isNameInputCorrect = true;

            if (deviceNameString.length < 6 || deviceNameString.length > 24) {
                
                isNameInputCorrect = false;
                isDataInputedCorrectely = false;

                errorMessages.value.push("Device name should contain [6 - 24] characters!");
            }
            
            if (isNameInputCorrect) {
                //TODO Check if name is in database
                errorMessages.value.push("This device name already exists in your devices!");
            }

            if (deviceNameLineElement.value !== undefined) {
                if (isNameInputCorrect) {
                    deviceNameLineElement.value.classList.remove("errorMarkerBg");
                }
                else {
                    deviceNameLineElement.value.classList.add("errorMarkerBg");
                }
            }
        } 
        else {
            isDataInputedCorrectely = false;

            if (deviceNameLineElement.value !== undefined) {
                deviceNameLineElement.value.classList.add("errorMarkerBg");
            }

            errorMessages.value.push("Device name is undefined. Internal Error!");
        }

        //=========================================================================================
        if (deviceTypeElement.value !== undefined) {
            const deviceTypeValues: string[] = options.map(option => option.value);
            
            if (!(deviceTypeValues.includes(deviceTypeElement.value.value))) {
                isDataInputedCorrectely = false;
                deviceTypeElement.value.classList.add("errorOutline");
                
                errorMessages.value.push("Device type does not exist!");
            }
            else {
                deviceTypeElement.value.classList.remove("errorOutline");
            }
        } 
        else {
            isDataInputedCorrectely = false;
            errorMessages.value.push("Device type is undefined. Internal Error!");
        }

        //=========================================================================================
        if (deviceCodeElements.value.length > 0) {
            var isCodeCorrect = true;

            for (var i = 0; i < deviceCodeElements.value.length; i++) {
                if (deviceCodeElements.value[i] != undefined) {
                    const digit = deviceCodeElements.value[i].value;
                    
                    if (digit.length != 1 || isNaN(parseInt(digit))) {                        
                        isCodeCorrect = false;
                        isDataInputedCorrectely = false;

                        errorMessages.value.push("Device code has missing values!");
                        break;
                    }
                }
                else {
                    isCodeCorrect = false;
                    isDataInputedCorrectely = false;

                    errorMessages.value.push("Device code input is undefined. Internal Error!");
                    break;
                }
            }

            for (var i = 0; i < deviceCodeElements.value.length; i++) {
                const parentDiv = deviceCodeElements.value[i].parentElement;

                if (parentDiv === null)
                    continue;

                if (isCodeCorrect) {
                    parentDiv.classList.remove("errorOutline");
                }
                else {       
                    parentDiv.classList.add("errorOutline");
                }
            }
        }
        else {
            isDataInputedCorrectely = false;
            errorMessages.value.push("Device code is undefined. Internal Error!");
        }

        return isDataInputedCorrectely;
    }
</script>

<style scoped>
    .editOptions{
        height: 85%;
        max-width: 80%;
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
        color: var(--color-main-light);
    }

    .deviceNameInput > input:focus {
        outline: none;
    }

    .deviceNameInputLine {
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
        width: 100%;
        height: 25px;

        background-color: transparent;
        color: var(--color-main-light);
        
        border-color: var(--color-main-light);
        border-style: solid;
        border-radius: 15px;
        padding-left: 5px;
    }

    .deviceCodeInput {
        margin-top: 10px;
        width: 100%;
        height: 25px;

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
        width: 100%;
        background-color: transparent;
        border: none;
        color: var(--color-main-light);

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
        margin-top: -2px;

        aspect-ratio: 1/1;

        display: flex;
        justify-content: center;
        align-items: center;

        border-color: var(--color-main-light);
        border-width: 2px;
        border-style: solid;
    }

    .actionClose {
        border-radius: 0 20px 0 20px;
        margin-left: -1.8px;
        background-color: var(--color-red);
    }

    .actionClose:hover {
        background-color:  var(--color-red-highlight);
    }

    .actionApply {
        border-radius: 20px 0 20px 0;
        margin-right: -1.8px;
        background-color: var(--color-green);
    }

    .actionApply:hover{
        background-color: var(--color-green-highlight);
    }

    .actionIcon {
        fill: var(--color-main-dark);
        height: 40%;
        width: 40%;
    }

    .errorMarkerBg {
        background-color: var(--color-error);
    }

    .errorOutline {
        border-color: var(--color-error);
    }

    .errorMessages {
        padding-top: 15px;
        margin-left: 15px;
    }
    .errorMessage {
        color: var(--color-error);
        font-size: 22px;
    }
</style>