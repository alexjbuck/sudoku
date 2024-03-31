use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};
use std::ops::Deref;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellValue {
    #[default]
    Empty,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<u8> for CellValue {
    fn from(value: u8) -> Self {
        match value {
            1 => CellValue::One,
            2 => CellValue::Two,
            3 => CellValue::Three,
            4 => CellValue::Four,
            5 => CellValue::Five,
            6 => CellValue::Six,
            7 => CellValue::Seven,
            8 => CellValue::Eight,
            9 => CellValue::Nine,
            _ => CellValue::Empty,
        }
    }
}

impl Display for CellValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CellValue::Empty => "_",
                CellValue::One => "1",
                CellValue::Two => "2",
                CellValue::Three => "3",
                CellValue::Four => "4",
                CellValue::Five => "5",
                CellValue::Six => "6",
                CellValue::Seven => "7",
                CellValue::Eight => "8",
                CellValue::Nine => "9",
            }
        )
    }
}

impl<'a> FromIterator<&'a CellValue> for HashSet<CellValue> {
    fn from_iter<I: IntoIterator<Item = &'a CellValue>>(iter: I) -> Self {
        let mut set = HashSet::new();
        for value in iter {
            if *value != CellValue::Empty {
                set.insert(*value);
            }
        }
        set
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub value: CellValue,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
