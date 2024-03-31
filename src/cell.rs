use crate::cellvalue::CellValue;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub value: CellValue,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cellvalue::CellValue;

    #[test]
    fn test_display() {
        let cell = Cell {
            value: CellValue::One,
        };
        assert_eq!(format!("{}", cell), "1");
    }
}