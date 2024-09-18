export enum DeviceType {
    None = 0,
    ShockCaller = 1
}

export function StringToDeviceType(device_str : String) : DeviceType {
    switch(String(device_str.toLowerCase())) {
        case "shock_caller":
            return DeviceType.ShockCaller;
    }

    return DeviceType.None;
}

export function DeviceTypeToString(device_type : DeviceType) : String {
    switch(device_type) {
        case DeviceType.ShockCaller:
            return String("shock_caller".toUpperCase())
    }

    return "None";
}