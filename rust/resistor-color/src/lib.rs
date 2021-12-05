use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;
use std::fmt;

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, IntoEnumIterator)]
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
    White = 9,
}
impl fmt::Display for Breakfast 
impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResistorColor::Black => write!(f, "Black"),
            ResistorColor::Brown => write!(f, "Brown"),
            ResistorColor::Red => write!(f, "Red"),
            ResistorColor::Orange => write!(f, "Orange"),
            ResistorColor::Yellow => write!(f, "Yellow"),
            ResistorColor::Green => write!(f, "Green"),
            ResistorColor::Blue => write!(f, "Blue"),
            ResistorColor::Violet => write!(f, "Violet"),
            ResistorColor::Grey => write!(f, "Grey"),
            ResistorColor::White => write!(f, "White"),
        }
    }
}

pub fn color_to_value(color: ResistorColor) -> usize {
    match color {
        ResistorColor::Black => ResistorColor::Black.int_value(),
        ResistorColor::Brown => ResistorColor::Brown.int_value(),
        ResistorColor::Red => ResistorColor::Red.int_value(),
        ResistorColor::Orange => ResistorColor::Orange.int_value(),
        ResistorColor::Yellow => ResistorColor::Yellow.int_value(),
        ResistorColor::Green => ResistorColor::Green.int_value(),
        ResistorColor::Blue => ResistorColor::Blue.int_value(),
        ResistorColor::Violet => ResistorColor::Violet.int_value(),
        ResistorColor::Grey => ResistorColor::Grey.int_value(),
        ResistorColor::White => ResistorColor::White.int_value(),
    }
}

pub fn value_to_color_string(value: usize) -> String {
    match value {
        0 => ResistorColor::Black.to_string(),
        1 => ResistorColor::Brown.to_string(),
        2 => ResistorColor::Red.to_string(),
        3 => ResistorColor::Orange.to_string(),
        4 => ResistorColor::Yellow.to_string(),
        5 => ResistorColor::Green.to_string(),
        6 => ResistorColor::Blue.to_string(),
        7 => ResistorColor::Violet.to_string(),
        8 => ResistorColor::Grey.to_string(),
        9 => ResistorColor::White.to_string(),
        _ => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut colors = Vec::new();
    for color in ResistorColor::into_enum_iter() {
        colors.push(color)
    }
    colors
}
