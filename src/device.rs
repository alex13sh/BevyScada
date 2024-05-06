///! Устройство, которое генерирует IO Tags
use bevy::prelude::*;

#[derive(Bundle)]
pub struct Device {
    // register as children
    // transport: tcp modbus, mqtt
}

#[derive(Component, Debug, PartialEq)]
pub struct DeviceID(pub String);

pub struct Transport {

}

#[derive(Bundle)]
pub struct Register {
    // address
    // enabled
    // raw value as u32 or ByteArray
    value: RegisterValue,
}

#[derive(Component, Debug, PartialEq, Deref)]
pub struct RegisterValue(u32);

/// Источник данных из регистров для Tag
/// Ну или в один entity хранить Register и Tag
pub struct RegisterSource {
    // endtity id of register
}
