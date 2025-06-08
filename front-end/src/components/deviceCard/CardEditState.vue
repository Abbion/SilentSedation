/*REFACTOR 4.0*/
<template>
    <div class="s_MainCardEditContainer">
        <div ref="editi_options_div_element" class="s_EditOptions">
            <div class="s_CardNameInput">
                <input v-model="card_name" type="text" placeholder="device name"/>
                <div class="s_CardNameInputLine" :class="{'s_ErrorMarkerBg' : card_name_error}"></div>
            </div>

            <div class="s_DeviceTypeSelector">
                <p class="s_PreventSelect">
                    device type
                </p>
                <select ref="device_type_element" class="s_DeviceTypeSelectorInput">
                    <option v-for="(option, index) in device_options" :key="index" :value="option.value">
                        {{ option.label }}
                    </option>
                </select>
            </div>

            <div>
                <p class="s_PreventSelect">
                    device code
                </p>
                <div class="s_DeviceCodeInput">
                    <div class="s_DeviceCodeInputField" :class="{ 's_ErrorOutline' : device_code_error }" v-for="(_, index) in 6">
                        <input class="s_DeviceCodeFieldInput" ref="device_code_elements" :key="index" type="text" maxlength="1"
                            @keydown="CodeDigitEntered($event as KeyboardEvent, index)"
                            @input="HandleCodeInput($event as InputEvent, index)"/>
                    </div>
                </div>
            </div>

            <ul class="s_ErrorMessages">
                <li v-for="(errorMessage, index) in error_messages" :key="index" class="s_ErrorMessage">
                    {{ errorMessage }}
                </li>
            </ul>
        </div>

        <div class="s_Actions">
            <div class="s_ActionBase s_ActionClose s_HighlightElement s_CursorPointer" @click="Cancel">
                <DeleteIcon class="s_ActionIconCancel" />
            </div>
            <div class="s_ActionBase s_ActionApply s_HighlightElement s_CursorPointer" @click="Add">
                <CheckIcon class="s_ActionIconAccept" />
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
    import CheckIcon from '../icons/CheckIcon.vue'
    import DeleteIcon from '../icons/DeleteIcon.vue'

    import type { CardData, CardDataWithCode } from '../common/Interfaces'
    import { DeviceTypeUtils } from '../common/EnumUtils'

    import { ref, defineEmits, onUpdated, onMounted  } from 'vue'

    const props = defineProps<{
        p_card_data : CardData
    }>();

    defineExpose({
        HandleServerResponse,
        ClearFields
    });

    //Emiters
    const emit = defineEmits(['CardEditAddButtonClicked', 'CardEditCancelButtonClicked'])

    //Variables
    const device_options = DeviceTypeUtils.CreateLabelsAndValues()

    let card_name_error = ref(false)
    let device_code_error = ref(false)
    let error_messages = ref<string[]>([])

    const card_name = ref<string>("");
    const device_type_element = ref<HTMLSelectElement>()
    const device_code_elements = ref<HTMLInputElement[]>([])
    const edit_options_div_element = ref<HTMLDivElement>()

    onMounted(() => {
        if (props.p_card_data.name != "") {
            card_name.value = props.p_card_data.name

            if (device_type_element.value && props.p_card_data.device_type > 0) {
                device_type_element.value.selectedIndex = props.p_card_data.device_type - 1
            }
        }
    });

    onUpdated(() => {
        if (edit_options_div_element.value) {
            const has_scroll_bar = edit_options_div_element.value.offsetHeight < edit_options_div_element.value.scrollHeight
            
            if (has_scroll_bar)
                edit_options_div_element.value.style.paddingRight = "calc(8% - 5px)"
            else
                edit_options_div_element.value.style.paddingRight = "8%"
        }
    });

    //Functions
    //[TODO: Move this to a seperate .vue file]
    function CodeDigitEntered(event : KeyboardEvent, index : number) {   
        const control_keys = ["Backspace", "ArrowLeft", "ArrowRight"]
        const is_number = /^[0-9]$/.test(event.key)
        
        //Block input for other kays than numvers and control keys
        if (!is_number && !control_keys.includes(event.key)) {
            event.preventDefault()
            return
        }

        //Handle controlKeys
        if (is_number) {
            return
        }

        const input_is_empty = device_code_elements.value[index].value === ''
        device_code_error.value = false

        if (input_is_empty)
        {
            //Backspace
            if (event.key === control_keys[0])
                PreviousCodeInput(index)

            //LeftArrow
            else if (event.key === control_keys[1]) {
                PreviousCodeInput(index)
                event.preventDefault()
            }
            //RightArrow
            else if (event.key === control_keys[2]) {
                NextCodeInput(index)
                event.preventDefault()
            }
        }
        else
        {
            const is_cursor_at_front = device_code_elements.value[index].selectionStart === 0;   

            //LeftArrow
            if (is_cursor_at_front && event.key === control_keys[1]) {
                PreviousCodeInput(index)
                event.preventDefault()
            }
            //RightArrow
            else if (!is_cursor_at_front && event.key === control_keys[2]) {
                NextCodeInput(index)
                event.preventDefault()
            }
        }
    }

    function HandleCodeInput(event : InputEvent, index : number) {
        if (event.data == null) {
            return
        }

        NextCodeInput(index)
    }

    function PreviousCodeInput(index : number) {
        if (index > 0) {
            device_code_elements.value[index - 1].focus()
        }
    }

    function NextCodeInput(index : number) {
        if (index < device_code_elements.value.length - 1) {
            device_code_elements.value[index + 1].focus()
        }
    }

    function HandleServerResponse(message : string) {
        error_messages.value.push(message)
        device_code_error.value = true
    }

    function ClearFields() {
        card_name.value = ""
        card_name_error.value = false

        if (device_type_element.value !== undefined) {
            device_type_element.value.value = device_options[0].value
        }

        for (let i = 0; i < device_code_elements.value.length; i++) {
            device_code_elements.value[i].value = ''
        }

        device_code_error.value = false
        error_messages.value = []
    }

    function Cancel() {
        ClearFields()
        emit('CardEditCancelButtonClicked')
    }

    function Add() {
        let is_data_inputed_correctely = CheckIfDataIsInputed()

        let index = 0
        if (device_type_element.value) {
            index = device_type_element.value.selectedIndex + 1
        }

        if (is_data_inputed_correctely) {
            const cardData : CardData = { 
                id: props.p_card_data.id,
                name: card_name.value.toString(),
                device_type: DeviceTypeUtils.IndexToDeviceType(index),
                device_properties: {},
             };


            let code = device_code_elements.value.filter(element => element).map(element => { return element.value}).join('')
            
            let return_data : CardDataWithCode = {
                card_data : cardData,
                code : code
            };

            emit('CardEditAddButtonClicked', return_data)
        }
    }

    function CheckIfDataIsInputed() {
        error_messages.value = []

        let is_data_inputed_correctely = CheckCardName()
        is_data_inputed_correctely = CheckDeviceType() && is_data_inputed_correctely
        is_data_inputed_correctely = CheckDeviceCode() && is_data_inputed_correctely

        return is_data_inputed_correctely
    }

    function CheckCardName() {
        if (card_name.value !== undefined) {
            const card_name_string = card_name.value
            let is_name_input_correct = true

            if (card_name_string.length < 6 || card_name_string.length > 24) {
                is_name_input_correct = false
                error_messages.value.push("Device name should contain [6 - 24] characters!")
            }
            
            if (is_name_input_correct) {
                card_name_error.value = false
                return true
            }
            else {
                card_name_error.value = true
                return false
            }
        } 
        else {
            card_name_error.value = true
            error_messages.value.push("Device name is undefined. Internal Error!")

            return false
        }
    }

    function CheckDeviceType() {
        if (device_type_element.value !== undefined) {
            const deviceTypeValues: string[] = device_options.map(option => option.value)
            
            if (!(deviceTypeValues.includes(device_type_element.value.value))) {
                error_messages.value.push("Device type does not exist!")
                return false
            }
        } 
        else {
            error_messages.value.push("Device type is undefined. Internal Error!")
            return false
        }
        return true
    }

    function CheckDeviceCode() {
        if (device_code_elements.value.length > 0) {
            let isCodeCorrect = true

            for (let i = 0; i < device_code_elements.value.length; i++) {
                if (device_code_elements.value[i] != undefined) {
                    const digit = device_code_elements.value[i].value
                    
                    if (digit.length != 1 || isNaN(parseInt(digit))) {                        
                        isCodeCorrect = false
                        error_messages.value.push("Device code has missing values!")
                        break;
                    }
                }
                else {
                    isCodeCorrect = false
                    error_messages.value.push("Device code input is undefined. Internal Error!")
                    break
                }
            }

            if (isCodeCorrect) {
                device_code_error.value = false
                return true
            }
            else {
                device_code_error.value = true
                return false
            }
        }
        else {
            error_messages.value.push("Device code is undefined. Internal Error!")
            return false
        }
    }

</script>

<style scoped>
    .s_MainCardEditContainer {
        color: var(--color-main-light);
        
        height: 100%;
        width: 100%;
        
        margin-top: 15px;

        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }

    .s_EditOptions {
        height: 80%;
        
        padding-left: 8%;
        padding-right: 8%;

        margin-right: 4%;
        margin-left: 4%;

        overflow-y: auto;
    }

    .s_CardNameInput {
        margin-bottom: 15px;
    }

    .s_CardNameInput > input {
        background-color: transparent;
        color: var(--color-main-light);

        width: 100%;

        margin-bottom: 3px;

        border-style: none;
    }

    .s_CardNameInput > input:focus {
        outline: none;
    }

    .s_CardNameInputLine {
        background-color: var(--color-main-light);

        height: 1px;
        width: 100%;
    }

    .s_DeviceTypeSelector {
        margin-bottom: 15px;
    }

    .s_DeviceTypeSelector > p {
        margin-bottom: 5px;
    }

    .s_DeviceTypeSelectorInput {
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

    .s_DeviceCodeInput {
        height: 25px;
        width: 100%;
        
        margin-top: 5px;

        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    .s_DeviceCodeInputField {
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

    .s_DeviceCodeFieldInput {
        background-color: transparent;
        color: var(--color-main-light);

        width: 100%;
        
        border: none;
        text-align: center;
    }

    .s_DeviceCodeFieldInput:focus {
        outline: none;
    }

    .s_Actions {
        height: 15%;

        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    .s_ActionBase {
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

    .s_ActionClose {
        border-radius: 0 20px 0 0;
        border-left: none;
    }

    .s_ActionApply {
        border-radius: 20px 0 0 0;
        border-right: none;
    }

    .s_ActionIconCancel {
        fill: var(--color-main-light);

        height: 40%;
        width: 40%;
    }

    .s_ActionIconAccept {
        fill: var(--color-main-light);

        height: 48%;
        width: 48%;
    }

    .s_ErrorMarkerBg {
        background-color: var(--color-error);
    }

    .s_ErrorOutline {
        border-color: var(--color-error);
    }

    .s_ErrorMessages {
        max-height: 100px;

        padding-top: 15px;
        margin-left: 18px;
    }
    .s_ErrorMessage {
        color: var(--color-error);
        
        font-size: 14px;
    }
</style>