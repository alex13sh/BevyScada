use super::{
    Enabled,
};

use bevy::prelude::*;
use std::collections::BTreeMap;

#[derive(Bundle)]
pub struct Register {
    // address
    enabled: Enabled,
    // raw value as u32 or ByteArray
    value: RegisterValue,
    config: RegisterConfig,
}

#[derive(Component, Debug, PartialEq, Deref)]
pub struct RegisterAddress(u16);

#[derive(Component, Debug, PartialEq, Deref)]
pub struct RegisterValue(u32);

#[derive(Bundle, Debug)]
pub struct RegisterConfig {
    io: RegisterIO,
    size: RegisterSize,
}

#[derive(Component, Debug, PartialEq)]
pub enum RegisterIO {
    ReadOnly,
    ReadWrite,
}

/// Количество байт в регистре
#[derive(Component, Debug, PartialEq)]
pub enum RegisterSize {
    TwoBytes,
    FourBytes,
}

impl TryFrom<u8> for RegisterSize {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(RegisterSize::TwoBytes),
            4 => Ok(RegisterSize::FourBytes),
            _ => Err("Unsupported value for RegisterSize"),
        }
    }
}

/// Источник данных из регистров для Tag
/// Ну или в один entity хранить Register и Tag
pub struct RegisterSource {
    // endtity id of register
}

/// Карта регистров для каждого устройства
pub struct RegistersMap {
    registers: BTreeMap<RegisterAddress, RegisterConfig>,
}
