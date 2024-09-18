use serde::Serialize;

#[derive(Serialize, Debug)]
pub  struct ShockCallerData{
    pub power : u8
}

#[derive(Serialize, Debug)]
pub enum DeviceType{
    SHOCK_CALLER(ShockCallerData)
}
