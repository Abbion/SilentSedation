import { DeviceType } from './Enums'

export interface CardData {
    id: number;
    name: string;
    deviceType: DeviceType;
    deviceProperties: Object;
    code: number[];
}

export interface Option {
    label: string;
    value: string;
}
