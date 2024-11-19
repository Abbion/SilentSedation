import { DeviceType } from './Enums'

export interface CardData {
    id: number;
    name: string;
    device_type: DeviceType;
    device_properties: Object;
    code: number[];
}

export interface Option {
    label: string;
    value: string;
}
