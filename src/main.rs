use sudoku::board::SudokuBoard;

fn main() {
    let input = include_str!("../board.txt");
    let mut board = SudokuBoard::from(input);
    board.verbose = true;
    println!("{}", board);
    match board.solve() {
        Ok(count) => println!("Solved after {} checks", count),
        Err((count, msg)) => println!("Took {} checks. {}", count, msg),
    }
}
