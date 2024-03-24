import type { Option } from './Interfaces'
import { DeviceType } from './Enums'

export const DeviceTypeUtils = {
    //Create a test for it to test if the values are comperable with the enum deviceType
    CreateLabelsAndValues() : Option[] {
        return [
            { label: 'electric shocker', value: 'e-shock' }, //Index 0
        ];
    },

    IndexToDeviceType(index : Number) : DeviceType {
        switch (index) {
            case 0:
                return DeviceType.ElectricShocker;
            default:
                return DeviceType.None;
        }
    }
}