<template>
    <div class="cardContainer centerContentVertical"
         :class="[ state == CardState.Add ? ['cardContainerDashed', 'cursorPointer', 'highlightElement'] : 'cardContainerSolid' ]"
          ref="cardContainerHTML"
          @click="CardContainerClick">
    
    <CardAddState v-if="state == CardState.Add"></CardAddState>

    <CardEditState v-if="state == CardState.Edit" 
                  :cardDataProp="cardData"
                  @cardEditAddButtonClicked="HandleEditStateData" 
                  @cardEditCancelButtonClicked="HandleEditStateCancel"></CardEditState>

    <CardUseState v-if="state == CardState.Use"
                  :p_cardData="cardData"
                  @cardOptionsClicked="GoToOptionState"></CardUseState>

    <CardOptionState v-if="state == CardState.Options"
                    :cardName="cardData.name"
                    @cardOptionsEditButtonClicked="GoToEditState" 
                    @cardOptionsDeleteButtonClicked="HandleDeleteCard"
                    @cardOptionsReturnButtonClicked="GoToUseState"></CardOptionState>

    </div>
</template>

<script setup lang="ts">
    import CardAddState from './CardAddState.vue'
    import CardEditState from './CardEditState.vue'
    import CardOptionState from './CardOptionState.vue'
    import CardUseState from './CardUseState.vue'

    import { ref, defineEmits, onMounted, useTransitionState } from 'vue'

    import type { CardData } from '../common/Interfaces'
    import { DeviceType, StringToDeviceType } from '../common/Enums';

    import axios from 'axios';

    enum CardState {
        Add,
        Edit,
        Options,
        Use
    }

    const emit = defineEmits(['cardCreated', 'cardRemoved']);
    const props = defineProps<{
        cardId: number
    }>();

    let state = ref(CardState.Add);
    let cardContainerHTML = ref<HTMLDivElement>();
    let cardData : CardData = { id: -1, name: "", deviceType: DeviceType.None, deviceProperties: {}, code: [] };
    let cancelFromEdit = false;
    
    onMounted(()=>{
        cardData.id = props.cardId;

        let token = localStorage.getItem('token');
        if (token !== null) {
            axios.post('http://localhost:9000/get_card', {
                    token: token,
                    card_id: cardData.id
            })
            .then(function (response) {
                console.log(response.data);

                let card_id = response.data["card_id"];
                let device_name = response.data["device_name"];


                if (device_name.length < 1)
                {

                }
                else
                {   
                    let device_type_obj = response.data["device_type"];
                    let device_code = response.data["code"];
                    let device_type_array = Object.keys(device_type_obj);

                    if (device_type_array.length != 1)
                        throw new Error("Device type has returned: " + device_type_array.length + " entries!");
                
                    let device_type = device_type_array[0];

                    cardData.name = device_name;
                    cardData.deviceType = StringToDeviceType(device_type);
                    cardData.deviceProperties = device_type_obj;
                    cardData.code = device_code;

                    GoToUseState();
                }
                
            }).catch( function(error) {
                console.log("Device card ", cardData.id, " error: ", error);
                
            })
        }
    });

    function CardContainerClick() {
        if (cancelFromEdit) {
            cancelFromEdit = false;
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
        var firstEdit = cardData.name === "" ? true : false;

        cardData = data;        
        GoToUseState();

        if (firstEdit) {
            console.log("Create card post: ", cardData);
            
            let token = localStorage.getItem('token');

            if (token === null) {
                console.log("Device card - Handle edit state data error: token is null");
                return;
            }

            axios.post('http://localhost:9000/update_card',{
                token: token,
                card_data : cardData
            }).then(function() {
                console.log("Here");
                
                //console.log("Add card response: ", respose);
                //emit('cardCreated');
            }).catch(function(error){
                console.log("Device card - Handle edit state data error: ", error);
                
            })

        }
    }
    
    function HandleEditStateCancel() {        
        if (cardData.name.length < 1) {
            cancelFromEdit = true;
            GoToAddState();      
        }
        else {
            GoToUseState();
        }
    }

    function HandleDeleteCard() {
        cardData.name = "";
        cardData.deviceType = DeviceType.None;
        cardData.code = [];

        cancelFromEdit = true;
        emit('cardRemoved', cardData.id);
    }
</script>

<style>
    .cardContainer {
        width: 200px;
        height: 320px;
        
        border-color: var(--color-main-light);
        border-radius: 20px;
        border-width: 2px;

        margin-left: 18px;
        margin-right: 18px;
        margin-bottom: 36px;
    }

    .highlightElement:hover {
        background-color: var(--color-main-dark--highlight);
    }

    .cardContainerDashed {
        border-style: dashed;
    }

    .cardContainerSolid {
        border-style: solid;
    }
</style>