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


    fn find_empty_cell(&self) -> Option<(usize, usize)> {
        for i in 0..9 {
            for j in 0..0 {
                if self.board[i][j] == 0{
                    return Some((i, j));
                }
            }
        }

        None
    }

    fn check(&self, index_tuple: (usize, usize), value: u8) -> bool {
        let (y, x) = index_tuple;


        for i in 0..9 {
            if self.board[i][x] == value {
                return false;
            }
        }

        for i in 0..9 {
            if self.board[y][i] == value { 
                return false;
            }
        }

        let sec_row = y / 3;
        let sec_col = x / 3;

        for i in (sec_row * 3)..(sec_row * 3 + 3) {
            for j in (sec_col * 3)..(sec_col * 3 + 3) {
                if self.board[i][j] == value {
                    return false;
                }
            }
        }
        ture

    }


