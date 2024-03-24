import { DeviceType } from './Enums'

export interface CardData {
    name: string;
    deviceType: DeviceType;
    code: Number[];
}

export interface Option {
    label: string;
    value: string;
}
