use crate::cell::SudokuCell;
use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SudokuBoard {
    cells: [[SudokuCell; 9]; 9],
    pub verbose: bool,
}

enum BoxOutline {
    Top,
    Middle,
    Bottom,
    Divider,
}

impl Display for BoxOutline {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            BoxOutline::Top => write!(f, "┌───────┬───────┬───────┐"),
            BoxOutline::Middle => write!(f, "├───────┼───────┼───────┤"),
            BoxOutline::Bottom => write!(f, "└───────┴───────┴───────┘"),
            BoxOutline::Divider => write!(f, "│"),
        }
    }
}

impl Display for SudokuBoard {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "{}", BoxOutline::Top)?;
        for (r, row) in self.cells.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if c % 3 == 0 {
                    write!(f, "{} ", BoxOutline::Divider)?;
                }
                write!(f, "{} ", cell)?;
            }
            writeln!(f, "{}", BoxOutline::Divider)?;
            if r == 2 || r == 5 {
                writeln!(f, "{}", BoxOutline::Middle)?;
            }
        }
        writeln!(f, "{}", BoxOutline::Bottom)?;
        Ok(())
    }
}

impl From<&str> for SudokuBoard {
    fn from(input: &str) -> Self {
        let mut cells: [[SudokuCell; 9]; 9] = [[SudokuCell::default(); 9]; 9];
        for (r, row) in input.lines().take(9).enumerate() {
            for (c, cell) in row.chars().take(9).enumerate() {
                if let Some(value) = cell.to_digit(10) {
                    cells[r][c] = SudokuCell::from(value as u8);
                }
            }
        }
        SudokuBoard {
            cells,
            verbose: false,
        }
    }
}

impl SudokuBoard {
    pub fn solve(&mut self) -> Result<usize, (usize, &'static str)> {
        let mut changed = true;
        let mut count = 0;
        while changed {
            changed = false;
            for r in 0..9 {
                for c in 0..9 {
                    if self.cells[r][c] == SudokuCell::Empty {
                        let possibilities = self.possible_values(r, c);
                        count += 1;
                        if possibilities.len() == 1 {
                            match *possibilities.iter().next().unwrap() {
                                SudokuCell::Empty => unreachable!(),
                                value => self.cells[r][c] = value,
                            }
                            changed = true;
                            if self.verbose {
                                println!("Updating ({}, {}) to {}", r, c, self.cells[r][c]);
                                println!("{}", self)
                            }
                        }
                    }
                }
            }
        }
        match self
            .cells
            .iter()
            .flatten()
            .any(|cell| *cell == SudokuCell::Empty)
        {
            true => Err((count, "Could not solve the board")),
            false => Ok(count),
        }
    }
    fn possible_values(&self, r: usize, c: usize) -> HashSet<SudokuCell> {
        let all_possibilities: HashSet<SudokuCell> = (1..=9).map(SudokuCell::from).collect();
        // Remove values in the same row, column, and 3x3 square
        let possibilities = remove_rcs_possibilities(self, r, c, all_possibilities.clone());
        // Consider the remaining possibilities, check if they are also possible after remove_rcs_possibilities for all other cells in the
        // row, column and 3x3 square. If a possibility candidate is not possible for any other cell in its row, its column, or in its 3x3
        // square, then it is the only possibility for the cell, remove all other possibilities. Do not also do this check when assessing
        // possibilities for other cells.
        let mut other_possibilities = HashSet::<SudokuCell>::new();
        // Compute the naive possibilities for the other cells in the same row and column and accumulate the possibilities as the set union.
        for i in 0..9 {
            if i != c {
                let cell_possibilities: HashSet<SudokuCell> =
                    remove_rcs_possibilities(self, r, i, all_possibilities.clone());
                other_possibilities = other_possibilities.union(&cell_possibilities).collect();
            }
            if i != r {
                let cell_possibilities: HashSet<SudokuCell> =
                    remove_rcs_possibilities(self, i, c, all_possibilities.clone());
                other_possibilities = other_possibilities.union(&cell_possibilities).collect();
            }
        }
        // Compute the naive possibilities for the other cells in the 3x3 square and accumulate the possibilities as the set union.
        let r_start = r / 3 * 3;
        let c_start = c / 3 * 3;
        for sr in r_start..r_start + 3 {
            for sc in c_start..c_start + 3 {
                if sr != r && sc != c {
                    let cell_possibilities: HashSet<SudokuCell> =
                        remove_rcs_possibilities(self, sr, sc, all_possibilities.clone());
                    other_possibilities = other_possibilities.union(&cell_possibilities).collect();
                }
            }
        }
        /* The difference is the set of possibilities that are not possible for any other cell in the row, column, or 3x3 square.
        This *should* either be an empty set, meaning that anything that is possible for the cell is also possible for another cell
        OR
        a set with a single element, meaning that this is the only possibility for the cell.
         */
        match possibilities.difference(&other_possibilities).count() {
            0 => possibilities,
            1 => possibilities.difference(&other_possibilities).collect(),
            _ => panic!(
                "Logic error: more than one exclusive possibility for ({}, {})",
                r, c
            ),
        }
    }
}

fn remove_rcs_possibilities(
    board: &SudokuBoard,
    r: usize,
    c: usize,
    mut possibilities: HashSet<SudokuCell>,
) -> HashSet<SudokuCell> {
    for i in 0..9 {
        possibilities.remove(&board.cells[r][i]);
        possibilities.remove(&board.cells[i][c]);
    }
    let r_start = r / 3 * 3;
    let c_start = c / 3 * 3;
    for r in r_start..r_start + 3 {
        for c in c_start..c_start + 3 {
            possibilities.remove(&board.cells[r][c]);
        }
    }
    possibilities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let input = "123456789\n\
                     456789123\n\
                     789123456\n\
                     234567891\n\
                     567891234\n\
                     891234567\n\
                     345678912\n\
                     678912345\n\
                     912345678";
        let board = SudokuBoard::from(input);
        assert_eq!(board.cells[0][0], SudokuCell::One);
        assert_eq!(board.cells[0][1], SudokuCell::Two);
        assert_eq!(board.cells[0][2], SudokuCell::Three);
        assert_eq!(board.cells[8][8], SudokuCell::Eight);
    }

    #[test]
    fn test_from_bad_input() {
        let input = "qwerty\n\n\n\tkj";
        let board = SudokuBoard::from(input);
        assert_eq!(board.cells[0][0], SudokuCell::Empty);
    }

    #[test]
    fn test_display() {
        let input = "123456789\n\
                     456789123\n\
                     789123456\n\
                     234567891\n\
                     567891234\n\
                     891234567\n\
                     345678912\n\
                     678912345\n\
                     912345678";
        let board = SudokuBoard::from(input);
        let expected = "┌───────┬───────┬───────┐\n\
            │ 1 2 3 │ 4 5 6 │ 7 8 9 │\n\
            │ 4 5 6 │ 7 8 9 │ 1 2 3 │\n\
            │ 7 8 9 │ 1 2 3 │ 4 5 6 │\n\
            ├───────┼───────┼───────┤\n\
            │ 2 3 4 │ 5 6 7 │ 8 9 1 │\n\
            │ 5 6 7 │ 8 9 1 │ 2 3 4 │\n\
            │ 8 9 1 │ 2 3 4 │ 5 6 7 │\n\
            ├───────┼───────┼───────┤\n\
            │ 3 4 5 │ 6 7 8 │ 9 1 2 │\n\
            │ 6 7 8 │ 9 1 2 │ 3 4 5 │\n\
            │ 9 1 2 │ 3 4 5 │ 6 7 8 │\n\
            └───────┴───────┴───────┘\n";
        assert_eq!(format!("{}", board), expected);
    }

    #[test]
    fn test_exclusive_possible_values() {
        let input = "_________\n\
                           ______1__\n\
                           ___1_____\n\
                           ________1\n\
                           _____1___\n\
                           __1______\n\
                           _______1_\n\
                           ____1____\n\
                           _1_______";
        let board = SudokuBoard::from(input);
        let possibilities = board.possible_values(0, 0);
        let expected: HashSet<SudokuCell> = vec![
            SudokuCell::One,
        ]
        .into_iter()
        .collect();
        assert_eq!(possibilities, expected);
    }
    #[test]
    fn test_rcs_possible_values() {
        let input = "__3______\n\
                           4_5___1__\n\
                           7________\n\
                           _________\n\
                           _________\n\
                           _________\n\
                           _2_______\n\
                           _________\n\
                           9_______8";
        let board = SudokuBoard::from(input);
        let possibilities = board.possible_values(0, 0);
        let expected: HashSet<SudokuCell> = vec![
            SudokuCell::One,
            SudokuCell::Two,
            SudokuCell::Six,
            SudokuCell::Eight,
        ]
        .into_iter()
        .collect();
        assert_eq!(possibilities, expected);
    }
}
