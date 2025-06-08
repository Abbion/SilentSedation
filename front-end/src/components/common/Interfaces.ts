/*REFACTOR 4.0*/
import { DeviceType } from './Enums'

export interface CardData {
    id: number,
    name: string,
    device_type: DeviceType,
    device_properties: Object
}

export interface CardDataWithCode {
    card_data : CardData,
    code : string
}

export interface Option {
    label: string,
    value: string
}
