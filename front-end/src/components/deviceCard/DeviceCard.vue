//Rework 2.0
<template>
    <div class="s_CardContainer s_CenterContentVertical"
         :class="[ state == CardState.Add ? ['s_CardContainerDashed', 's_CursorPointer', 's_HighlightElement'] : 's_CardContainerSolid' ]"
          ref="card_container_html"
          @click="CardContainerClick">
    
    <CardAddState v-if="state == CardState.Add"></CardAddState>

    <CardEditState v-if="state == CardState.Edit" 
                  :p_card_data="card_data"
                  ref="edit_state_comp_ref"
                  @CardEditAddButtonClicked="CreateCard"
                  @CardEditCancelButtonClicked="HandleEditStateCancel"></CardEditState>

    <CardUseState v-if="state == CardState.Use"
                  :p_card_data="card_data"
                  @CardOptionsClicked="GoToOptionState"
                  @PropertiesUpdated="UpdateCardProperties"></CardUseState>

    <CardOptionState v-if="state == CardState.Options"
                    :p_card_name="card_data.name"
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

    let edit_state_comp_ref = ref<typeof CardEditState>();
    let state = ref(CardState.Add);
    let card_container_html = ref<HTMLDivElement>();
    let card_data : CardData = { id: -1, name: "", device_type: DeviceType.Empty, device_properties: {}, code: "" };
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
            .then(function(response) {
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
                   
                card_data.name = device_name;
                card_data.device_type = device_type;
                card_data.device_properties = device_type_obj;

                GoToUseState();
            })
            .catch(function(error) {
                console.log("Device card ", card_data.id, " error: ", error);
            });
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

    function CreateCard(data : CardData) {
        ConnectCardToDevice(data)
        .then(function(update_return) {
            // Check if the device was found and assigned to card
            if (update_return && edit_state_comp_ref.value) {
                const data = update_return.data;
                if (data.success == false) {
                    edit_state_comp_ref.value.HandleServerResponse(data.message);
                    throw new Error;
                }

                edit_state_comp_ref.value.ClearFields();
            }
        }).then(function() {
            UpdateCard(data)
        }).then(function() {
            GetCard();
            emit('CardCreated');
        }).catch(function(error) {
            console.log("Device card - Handle create card error: ", error);
        });
    }

    async function ConnectCardToDevice(data : CardData) {
        let token = localStorage.getItem('token');

        if (token === null) {
            console.log("Device card - Handle edit state data error: token is null");
            return;
        }

        return axios.post('http://localhost:9000/connect_card_to_device', {
            token : token,
            id : data.id,
            device_type : data.device_type,
            code : data.code
        });
    }

    async function UpdateCard(data : CardData) {
        let token = localStorage.getItem('token');

        if (token === null) {
            console.log("Device card - Handle edit state data error: token is null");
            return;
        }

        axios.post('http://localhost:9000/update_card',{
            token: token,
            card_data : data
        });
    }

    function UpdateCardProperties(properties : Object) {
        card_data.device_properties = properties;
        UpdateCard(card_data)
        .catch(function(error) {
            console.log("Device card - Handle update card properties error: ", error);
        });
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
        let token = localStorage.getItem('token');

        if (token !== null) {
            axios.post('http://localhost:9000/delete_card', {
                    token: token,
                    card_id: card_data.id
            })
            .then(function(response) {
                console.log("delete resp: ", response.data);

                card_data.name = "";
        
                card_data.device_type = DeviceType.Empty;
                card_data.code = "";

                cancel_from_edit = true;
                
                emit('CardRemoved', card_data.id);
            })
            .catch(function(error) {
                console.log("Device card ", card_data.id, " error: ", error);
            });
        }
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