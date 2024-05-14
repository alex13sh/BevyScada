///! Устройство, которое генерирует IO Tags
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use derivative::Derivative;
// mod owen;
mod register;

#[derive(Bundle)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Derivative)]
#[derivative(PartialEq)]
pub struct Device {
    name: Name,
    #[serde(default)]
    enable: Enabled,
    // transport: tcp modbus, mqtt
    config: DeviceConfig,
    // register as children
    #[serde(skip)]
    #[derivative(PartialEq="ignore")]
    registers: Registers,
}

#[derive(Component, Debug, PartialEq)]
pub struct DeviceID(pub String);

pub struct Transport {

}

/// Для наименования устройств и регистров
#[derive(Component, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Name(String);

// pub struct Description(String);

/// Включние/выключение устройства, регстр или пин.
#[derive(Component, Debug, PartialEq, Default)]
#[derive(Serialize, Deserialize)]
pub struct Enabled(bool);

#[derive(Component, Debug, Default, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct DeviceConfig {
    registers_map: register::RegistersMap,
}

#[derive(Component, Debug, Default)]
pub struct Registers {
    registers: Vec<Entity>,

}


#[test]
fn test_device_conf() {
    let js = serde_json::json!({
        "name": "Analog Device",
        "config": {
            "registers_map": {
                "1": {
                    "io": "ReadOnly",
                    "size": "FourBytes"
                },
                "3": {
                    "io": "ReadWrite",
                    "size": 2
                }
            }
        },
    });
    let test_device: Device = serde_json::from_value(js).unwrap();
    let regs = register::tests::get_test_regs();
    let device = Device {
        name: Name("Analog Device".into()),
        enable: Enabled::default(),
        config: DeviceConfig {
            registers_map: regs,
        },
        registers: Default::default(),
    };

    assert_eq!(device, test_device);
}
