///! Устройство, которое генерирует IO Tags
use bevy::prelude::*;

mod owen;
mod register;

#[derive(Bundle)]
pub struct Device {
    // register as children
    // transport: tcp modbus, mqtt
}

#[derive(Component, Debug, PartialEq)]
pub struct DeviceID(pub String);

pub struct Transport {

}

/// Для наименования устройств и регистров
#[derive(Component, Debug, PartialEq)]
pub struct Name(String);

// pub struct Description(String);

/// Включние/выключение устройства, регстр или пин.
#[derive(Component, Debug, PartialEq)]
pub struct Enabled(bool);
