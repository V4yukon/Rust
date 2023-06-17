/*
 *      
 *      A Rust implementation of Sudoku solver using Backtracking.
 *      
 */


pub struct Sudoku {
    board: [[u8; 9]; 9],
}

impl Sudoku {
    pub fn new(board: [[u8; 9]; 9]) -> Sudoku {
        Sudoku {board}
    }


    fn 
