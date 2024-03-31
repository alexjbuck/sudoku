use crate::cell::{Cell, CellValue};
use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SudokuBoard {
    cells: [[Cell; 9]; 9],
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
        let mut cells: [[Cell; 9]; 9] = [[Cell::default(); 9]; 9];
        for (r, row) in input.lines().take(9).enumerate() {
            for (c, cell) in row.chars().take(9).enumerate() {
                if let Some(value) = cell.to_digit(10) {
                    cells[r][c].value = CellValue::from(value as u8);
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
                    if self.cells[r][c].value == CellValue::Empty {
                        let possibilities = self.possible_values(r, c);
                        count += 1;
                        if possibilities.len() == 1 {
                            match *possibilities.iter().next().unwrap() {
                                CellValue::Empty => unreachable!(),
                                value => self.cells[r][c].value = value,
                            }
                            changed = true;
                            if self.verbose {
                                println!("Updating ({}, {}) to {}", r, c, self.cells[r][c].value);
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
            .any(|cell| cell.value == CellValue::Empty)
        {
            true => Err((count, "Could not solve the board")),
            false => Ok(count),
        }
    }
    fn possible_values(&self, r: usize, c: usize) -> HashSet<CellValue> {
        let all_possibilities: HashSet<CellValue> = (1..=9).map(CellValue::from).collect();
        // Remove values in the same row, column, and 3x3 square
        let possibilities = remove_rcs_possibilities(self, r, c, all_possibilities.clone());
        // Consider the remaining possibilities, check if they are also possible after remove_rcs_possibilities for all other cells in the
        // row, column and 3x3 square. If a possibility candidate is not possible for any other cell in its row, its column, or in its 3x3
        // square, then it is the only possibility for the cell, remove all other possibilities. Do not also do this check when assessing
        // possibilities for other cells.
        let mut other_possibilities = HashSet::<CellValue>::new();
        // Compute the naive possibilities for the other cells in the same row and column and accumulate the possibilities as the set union.
        for i in 0..9 {
            if i != c {
                let cell_possibilities: HashSet<CellValue> =
                    remove_rcs_possibilities(self, r, i, all_possibilities.clone());
                other_possibilities = other_possibilities.union(&cell_possibilities).collect();
            }
            if i != r {
                let cell_possibilities: HashSet<CellValue> =
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
                    let cell_possibilities: HashSet<CellValue> =
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
    mut possibilities: HashSet<CellValue>,
) -> HashSet<CellValue> {
    for i in 0..9 {
        possibilities.remove(&board.cells[r][i].value);
        possibilities.remove(&board.cells[i][c].value);
    }
    let r_start = r / 3 * 3;
    let c_start = c / 3 * 3;
    for r in r_start..r_start + 3 {
        for c in c_start..c_start + 3 {
            possibilities.remove(&board.cells[r][c].value);
        }
    }
    possibilities
}
