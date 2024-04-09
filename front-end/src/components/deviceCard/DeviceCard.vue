<template>
    <div class="cardContainer centerContentVertical"
         :class="[ state == CardState.Add ? ['cardContainerDashed', 'cursorPointer', 'highlightElement'] : 'cardContainerSolid' ]"
          ref="cardContainerHTML"
          @click="CardContainerClick">
    
    <CardAddState v-show="state == CardState.Add"></CardAddState>

    <CardEditState v-show="state == CardState.Edit" 
                  @cardEditAddButtonClicked="HandleEditStateData" 
                  @cardEditCancelButtonClicked="HandleEditStateCancel"
                 :cardDataProp="cardData"></CardEditState>

    <CardUseState v-show="state == CardState.Use" :cardName="cardData.name" @cardOptionsClicked="GoToOptionState"></CardUseState>

    <CardOptionState v-show="state == CardState.Options"
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

    import { ref, defineEmits, onMounted } from 'vue'

    import type { CardData } from '../common/Interfaces'
    import { DeviceType } from '../common/Enums';

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
    let cardData : CardData = { id: -1, name: "", deviceType: DeviceType.None, code: [] };
    let cancelFromEdit = false;

    onMounted(()=>{
        cardData.id = props.cardId;        
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
            emit('cardCreated');
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
    background-color: --color-main-dark--highlight;
    }

    .cardContainerDashed {
        border-style: dashed;
    }

    .cardContainerSolid {
        border-style: solid;
    }
</style>