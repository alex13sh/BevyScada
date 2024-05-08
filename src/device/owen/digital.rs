// Какие особенности цифровых пинов?
// Вроде всё как обычно: Номер пина, имя и Input/Output
use super::{
    IoPin, IoPinDirect, IoPinNumber,
    Name,
};

// Это должно конвертироваться в список регистров со значениями по умолчанию.
// И это должно конвертироваться в Тэги, которые связанные с регистрами.
pub fn input(pin: u8, name: &str) -> IoPin {
    IoPin {
        direct: IoPinDirect::Input,
        number: IoPinNumber(pin),
        name: Name(name.to_string()),
    }
}
