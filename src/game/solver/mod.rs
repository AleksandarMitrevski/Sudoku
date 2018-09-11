use super::SudokuBoard;

/// Solves the given board if it is solvable and returns the solution found, else it returns None
pub fn solve(board : &SudokuBoard) -> Option<SudokuBoard> {
    use super::check_for_errors;
    if check_for_errors(board).is_empty() {
        let mut board_clone = board.clone();
        if solve_board(&mut board_clone) {
            Some(board_clone)
        } else {
            None
        }
    } else {
        None
    }
}

/// This is a simple backtracking algorithm based on https://www.geeksforgeeks.org/sudoku-backtracking-7/ (visit the site for algorithm explanation, or check other/solver_algorithm/solve_board_algorithm.cpp in the GitHub repository)
fn solve_board(board : &mut SudokuBoard) -> bool {
    match super::find_unassigned_location(board) {
        None => {
            return true;
        },
        Some((row, col)) => {
            for num in 1..=9 {
                if is_safe(board, row, col, num) {
                    board[row][col] = num;
                    if solve_board(board) {
                        return true;
                    }
                    board[row][col] = 0;
                }
            }
            return false;
        }
    };
}

fn used_in_row(board : &SudokuBoard, row : usize, value : u8) -> bool {
    for el in board[row].iter() {
        if *el == value {
            return true;
        }
    }
    false
}

fn used_in_col(board : &SudokuBoard, col : usize, value : u8) -> bool {
    for i in 0..9 {
        if board[i][col] == value {
            return true;
        }
    }
    false
}

fn used_in_quadrant(board : &SudokuBoard, qi : usize, qj : usize, value : u8) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if board[qi + i][qj + j] == value {
                return true;
            }
        }
    }
    false
}

fn is_safe(board : &SudokuBoard, row : usize, col : usize, value : u8) -> bool {
    !used_in_row(board, row, value) && !used_in_col(board, col, value) && !used_in_quadrant(board, row - row % 3, col - col % 3, value)
}

// Declare tests module
#[cfg(test)]
mod tests;