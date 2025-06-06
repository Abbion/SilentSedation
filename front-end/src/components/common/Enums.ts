export enum DeviceType {
    Empty = 0,
    ShockCaller = 1
}

export function StringToDeviceType(device_str : String) : DeviceType {
    switch(String(device_str.toLowerCase())) {
        case "shockcaller":
            return DeviceType.ShockCaller;
    }

    return DeviceType.Empty;
}

export function DeviceTypeToString(device_type : DeviceType) : string {
    switch(device_type) {
        case DeviceType.ShockCaller:
            return "ShockCaller"
    }

    return "Empty";
}

export enum DeviceActionType {
    None = 0,
    Zap = 1
}

export enum ConnectionStatus {
    Connecting,
    Online
}

