//Rework 2.0
<template>
    <div class="s_CardContainer s_CenterContentVertical"
         :class="[ state == CardState.Add ? ['s_CardContainerDashed', 's_CursorPointer', 's_HighlightElement'] : 's_CardContainerSolid' ]"
          ref="card_container_html"
          @click="CardContainerClick">
    
    <CardAddState v-if="state == CardState.Add"></CardAddState>

    <CardEditState v-if="state == CardState.Edit" 
                  :p_card_data="card_data"
                  @CardEditAddButtonClicked="HandleEditStateData" 
                  @CardEditCancelButtonClicked="HandleEditStateCancel"></CardEditState>

    <CardUseState v-if="state == CardState.Use"
                  :p_card_data="card_data"
                  @CardOptionsClicked="GoToOptionState"></CardUseState>

    <CardOptionState v-if="state == CardState.Options"
                    :p_card_name="card_data.name"
                    @CardOptionsEditButtonClicked="GoToEditState" 
                    @CardOptionsDeleteButtonClicked="HandleDeleteCard"
                    @CardOptionsReturnButtonClicked="GoToUseState"></CardOptionState>

    </div>
</template>

<script setup lang="ts">
    import CardAddState from './CardAddState.vue'
    import CardEditState from './CardEditState.vue'
    import CardOptionState from './CardOptionState.vue'
    import CardUseState from './CardUseState.vue'

    import { ref, defineEmits, onMounted } from 'vue'

    import type { CardData } from '../common/Interfaces'
    import { DeviceType, StringToDeviceType } from '../common/Enums';

    import axios from 'axios';

    enum CardState {
        Add,
        Edit,
        Options,
        Use
    }

    const emit = defineEmits(['CardCreated', 'CardRemoved']);
    const props = defineProps<{
        p_card_id: number
    }>();

    let state = ref(CardState.Add);
    let card_container_html = ref<HTMLDivElement>();
    let card_data : CardData = { id: -1, name: "", device_type: DeviceType.Empty, device_properties: {}, code: [] };
    let cancel_from_edit = false;
    
    onMounted(()=>{
        card_data.id = props.p_card_id;
        GetCard();
    });

    function GetCard() {
        let token = localStorage.getItem('token');

        if (token !== null) {
            axios.post('http://localhost:9000/get_card', {
                    token: token,
                    card_id: card_data.id
            })
            .then(function (response) {
                //Check if the id is correct. Compare to p_card_id
                let card_id = response.data["card_id"];
                
                if (card_id != card_data.id)
                    throw new Error("Device id mismatch! Device id: " + card_id + " expected: " + card_data.id)
                
                let device_type_obj = response.data["device_type"];
                let device_type_array = Object.keys(device_type_obj);

                if (device_type_array.length != 1)
                    throw new Error("Device type has returned: " + device_type_array.length + " entries!");

                let device_type = StringToDeviceType(device_type_array[0]);

                if (device_type === DeviceType.Empty)
                    return;

                let device_name = response.data["device_name"];
                
                if (device_name.length < 1)
                    throw new Error("Incomplete device name!");
                   
                let device_code = response.data["code"];

                card_data.name = device_name;
                card_data.device_type = device_type;
                card_data.device_properties = device_type_obj;
                card_data.code = device_code;

                GoToUseState();
                
            }).catch( function(error) {
                console.log("Device card ", card_data.id, " error: ", error);
            })
        }
    }

    function CardContainerClick() {
        if (cancel_from_edit) {
            cancel_from_edit = false;
            return;
        }

        if (state.value == CardState.Add) {
            GoToEditState();
        }
    }

    function GoToAddState() {
        state.value = CardState.Add;
    }

    function GoToEditState() {        
        state.value = CardState.Edit;
    }

    function GoToOptionState() {
        state.value = CardState.Options;
    }

    function GoToUseState() {
        state.value = CardState.Use;
    }

    function DeleteCard() {
        state.value = CardState.Add;
    }

    function HandleEditStateData(data : CardData) {
        var firstEdit = card_data.name === "" ? true : false;

        if (firstEdit) {
            console.log("Create card post: ", data);
            
            let token = localStorage.getItem('token');

            if (token === null) {
                console.log("Device card - Handle edit state data error: token is null");
                return;
            }

            axios.post('http://localhost:9000/update_card',{
                token: token,
                card_data : data
            }).then(function() {
                GetCard();
                emit('CardCreated')
            }).catch(function(error){
                console.log("Device card - Handle edit state data error: ", error);
            })
        }
    }
    
    function HandleEditStateCancel() {        
        if (card_data.name.length < 1) {
            cancel_from_edit = true;
            GoToAddState();      
        }
        else {
            GoToUseState();
        }
    }

    function HandleDeleteCard() {
        card_data.name = "";
        card_data.device_type = DeviceType.Empty;
        card_data.code = [];

        cancel_from_edit = true;
        emit('CardRemoved', card_data.id);
    }
</script>

<style>
    .s_CardContainer {
        width: 200px;
        height: 320px;
        
        border-color: var(--color-main-light);
        border-radius: 20px;
        border-width: 2px;

        margin-left: 18px;
        margin-right: 18px;
        margin-bottom: 36px;
    }

    .s_HighlightElement:hover {
        background-color: var(--color-main-dark--highlight);
    }

    .s_CardContainerDashed {
        border-style: dashed;
    }

    .s_CardContainerSolid {
        border-style: solid;
    }
</style>