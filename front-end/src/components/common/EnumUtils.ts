/*REFACTOR 4.0*/
import type { Option } from './Interfaces'
import { DeviceType } from './Enums'

export const DeviceTypeUtils = {
    //Create a test for it to test if the values are comperable with the enum deviceType
    CreateLabelsAndValues() : Option[] {
        return [
            { label: 'electric shocker', value: 'e-shock' }, //Index 0
        ];
    },

    IndexToDeviceType(index : number) : DeviceType {
        switch (index) {
            case 1:
                return DeviceType.ShockCaller
            default:
                return DeviceType.Empty
        }
    }
}