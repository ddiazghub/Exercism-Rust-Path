use int_enum::IntEnum;
use enum_iterator::{self, Sequence};

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {

    if let Ok(color) = ResistorColor::from_int(value) {
        format!("{:?}", color)
    } else {
        String::from("value out of range")
    }
}

pub fn colors() -> Vec<ResistorColor> {
    enum_iterator::all::<ResistorColor>().collect::<Vec<ResistorColor>>()
}
