use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntEnum, PartialOrd, Ord, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    if value > 9 {
        return String::from("value out of range");
    }
    format!("{:?}", ResistorColor::from_int(value).unwrap())
}

pub fn colors() -> Vec<ResistorColor> {
    let mut result = all::<ResistorColor>().collect::<Vec<_>>();
    result.sort();
    result
}
