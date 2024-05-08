use super::{
    Enabled,
};

use bevy::prelude::*;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

#[derive(Bundle)]
pub struct Register {
    // address
    enabled: Enabled,
    // raw value as u32 or ByteArray
    value: RegisterValue,
    config: RegisterConfig,
}

#[derive(Component, Debug, Deref)]
#[derive(PartialEq, Ord, Eq, PartialOrd)]
#[derive(Serialize, Deserialize)]
pub struct RegisterAddress(u16);

#[derive(Component, Debug, PartialEq, Deref)]
pub struct RegisterValue(u32);

#[derive(Bundle, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct RegisterConfig {
    io: RegisterIO,
    #[serde(deserialize_with = "deser::size_str_or_num")]
    size: RegisterSize,
}

#[derive(Component, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum RegisterIO {
    ReadOnly,
    ReadWrite,
}

/// Количество байт в регистре
#[derive(Component, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
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

/// Карта регистров для каждого устройства
#[derive(Serialize, Deserialize, Debug)]
pub struct RegistersMap {
    registers: BTreeMap<RegisterAddress, RegisterConfig>,
}

#[test]
fn test_registers_map_conf() {
    let js = serde_json::json!({
        "1": {
            "io": "ReadOnly",
            "size": "FourBytes"
        },
        "3": {
            "io": "ReadWrite",
            "size": 2
        }
    });
    let test_map: BTreeMap<RegisterAddress, RegisterConfig> = serde_json::from_value(js).unwrap();
    let mut regs = BTreeMap::new();
    regs.insert(RegisterAddress(1), RegisterConfig{
        io: RegisterIO::ReadOnly,
        size: RegisterSize::FourBytes,
    });
    regs.insert(RegisterAddress(3), RegisterConfig{
        io: RegisterIO::ReadWrite,
        size: RegisterSize::TwoBytes,
    });

    assert_eq!(regs, test_map);
}

/// Источник данных из регистров для Tag
/// Ну или в один entity хранить Register и Tag
pub struct RegisterSource {
    // endtity id of register
}

mod deser {
    use super::RegisterSize;
    use serde::{Deserialize, Deserializer};
    pub(super) fn size_str_or_num<'de, D>(deserializer: D) -> Result<RegisterSize, D::Error>
    where
        D: Deserializer<'de>,
    {
        let err = serde::de::Error::custom;
        let my_err = || err("Failed to deserialize u64 from string or number");

        let value = serde_json::value::Value::deserialize(deserializer)?;
        if value.is_number() {
            let size = value.as_u64().ok_or(my_err())? as u8;
            Ok(RegisterSize::try_from(size).map_err(|_e| my_err())?)
        } else if value.is_string() {
            Ok(serde_json::from_value(value).map_err(|_e| my_err())?)
        } else {
            Err(my_err())
        }
    }

}
