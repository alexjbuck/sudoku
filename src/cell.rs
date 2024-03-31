use core::fmt;
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SudokuCell {
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

impl From<u8> for SudokuCell {
    fn from(value: u8) -> Self {
        match value {
            1 => SudokuCell::One,
            2 => SudokuCell::Two,
            3 => SudokuCell::Three,
            4 => SudokuCell::Four,
            5 => SudokuCell::Five,
            6 => SudokuCell::Six,
            7 => SudokuCell::Seven,
            8 => SudokuCell::Eight,
            9 => SudokuCell::Nine,
            _ => SudokuCell::Empty,
        }
    }
}

impl Display for SudokuCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SudokuCell::Empty => "_",
                SudokuCell::One => "1",
                SudokuCell::Two => "2",
                SudokuCell::Three => "3",
                SudokuCell::Four => "4",
                SudokuCell::Five => "5",
                SudokuCell::Six => "6",
                SudokuCell::Seven => "7",
                SudokuCell::Eight => "8",
                SudokuCell::Nine => "9",
            }
        )
    }
}

impl<'a> FromIterator<&'a SudokuCell> for HashSet<SudokuCell> {
    fn from_iter<I: IntoIterator<Item = &'a SudokuCell>>(iter: I) -> Self {
        let mut set = HashSet::new();
        for value in iter {
            if *value != SudokuCell::Empty {
                set.insert(*value);
            }
        }
        set
    }
}

// Write a test for CellValue

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(SudokuCell::from(0), SudokuCell::Empty);
        assert_eq!(SudokuCell::from(1), SudokuCell::One);
        assert_eq!(SudokuCell::from(2), SudokuCell::Two);
        assert_eq!(SudokuCell::from(3), SudokuCell::Three);
        assert_eq!(SudokuCell::from(4), SudokuCell::Four);
        assert_eq!(SudokuCell::from(5), SudokuCell::Five);
        assert_eq!(SudokuCell::from(6), SudokuCell::Six);
        assert_eq!(SudokuCell::from(7), SudokuCell::Seven);
        assert_eq!(SudokuCell::from(8), SudokuCell::Eight);
        assert_eq!(SudokuCell::from(9), SudokuCell::Nine);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", SudokuCell::Empty), "_");
        assert_eq!(format!("{}", SudokuCell::One), "1");
        assert_eq!(format!("{}", SudokuCell::Two), "2");
        assert_eq!(format!("{}", SudokuCell::Three), "3");
        assert_eq!(format!("{}", SudokuCell::Four), "4");
        assert_eq!(format!("{}", SudokuCell::Five), "5");
        assert_eq!(format!("{}", SudokuCell::Six), "6");
        assert_eq!(format!("{}", SudokuCell::Seven), "7");
        assert_eq!(format!("{}", SudokuCell::Eight), "8");
        assert_eq!(format!("{}", SudokuCell::Nine), "9");
    }

    #[test]
    fn test_from_iter() {
        let values = vec![
            SudokuCell::Empty,
            SudokuCell::One,
            SudokuCell::Two,
            SudokuCell::Empty,
            SudokuCell::Four,
            SudokuCell::Five,
            SudokuCell::Empty,
            SudokuCell::Seven,
            SudokuCell::Eight,
        ];
        let set: HashSet<SudokuCell> = values.iter().collect();
        let expected: HashSet<SudokuCell> = vec![
            SudokuCell::One,
            SudokuCell::Two,
            SudokuCell::Four,
            SudokuCell::Five,
            SudokuCell::Seven,
            SudokuCell::Eight,
        ]
        .into_iter()
        .collect();
        assert_eq!(set, expected);
    }
}
