///! Устройства овен
use super::Name;

pub enum OwenDeviceType {
    // Аналоговый медленный
    // Аналоговый быстрый
    // Цифровой
}

pub struct IoPin {
    direct: IoPinDirect, // Input / Output
    number: IoPinNumber, // Number pin
    name: Name,
}

pub enum IoPinDirect {
    Input,
    Output,
}

pub struct IoPinNumber(u8);

/// Регистр содержащие главное значение
pub struct ValueRegister {

}

/// Регистры конфигурации
pub struct ConfigRegisters {

}

/// Устройства с пинами
struct DevicePins {

}

impl DevicePins {
    // Использование или привящка (bind) пина.
    // Создание набор регистров для пина.
    // Это возможно должно быть trait или универсальной структурой.
    // Этот метод должен работать для каждого устройства по разному, так как карта ргистров разная.
    pub fn use_pin(pin: u8) {

    }
}
