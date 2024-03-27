import { DeviceType } from './Enums'

export interface CardData {
    id: number;
    name: string;
    deviceType: DeviceType;
    code: number[];
}

export interface Option {
    label: string;
    value: string;
}
