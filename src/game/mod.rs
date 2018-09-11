// Declare the submodules
mod database;
mod solver;

use std::ops::{Deref, DerefMut};
use std::fmt;
use std::fs::File;

/// Type alias for the board matrix
type SudokuBoardMatrix = [[u8; 9]; 9];

/// Struct type that contains a Sudoku board
/// 1 to 9 are valid cell values, 0 means abscense
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SudokuBoard( SudokuBoardMatrix );

impl SudokuBoard {
    pub fn new(game_string : &str) -> SudokuBoard {
        assert_eq!(game_string.len(), 81);
        let mut board = [[0u8; 9]; 9];
        for (i, ch) in game_string.chars().enumerate() {
            board[i / 9][i % 9] = ch.to_digit(10).unwrap() as u8;
        }
        SudokuBoard(board)
    }
}

// The following two enable direct indexing for SudokuBoard type
impl Deref for SudokuBoard {
    type Target = SudokuBoardMatrix;

    fn deref(&self) -> &SudokuBoardMatrix {
        &self.0
    }
}
impl DerefMut for SudokuBoard {
    fn deref_mut(&mut self) -> &mut SudokuBoardMatrix {
        &mut self.0
    }
}

// Prettier debug printing
impl fmt::Debug for SudokuBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("SudokuBoard {\n");
        for row in self.iter() {
            result.push_str("   "); // why three spaces? Because that looks best in my console. Also see: https://stackoverflow.com/a/219177
            for elem in row.iter() {
                result.push_str(elem.to_string().as_str());
            }
            result.push('\n');
        }
        result.push_str("}");
        write!(f, "{}", result)
    }
}

// Used to save a game to a file
impl ToString for SudokuBoard {
    fn to_string(&self) -> String {
        use std::char::from_digit;
        let mut result = String::new();
        for row in self.iter() {
            for el in row.iter() {
                result.push(from_digit(*el as u32, 10).unwrap());
            }
        }
        result
    }
}

// Re-export the Difficulty enum from the database module
pub use self::database::Difficulty;

// Re-export game database initialization from the database module
pub use self::database::initialize_database;

// Re-export generate game functionality from the database module
pub use self::database::generate_game;

/// Internal enum to represent the types of errors that are shown to the user
pub enum SudokuBoardError {
    Row(u8),
    Column(u8),
    Quadrant(u8, u8)
}

/// Checks the board for mistakes and returns all it finds, otherwise returns empty vector
pub fn check_for_errors(board : &SudokuBoard) -> Vec<SudokuBoardError> {
    let check_unique = |elements : &[u8]| -> bool {
        use std::collections::HashSet;
        let mut set : HashSet<u8> = HashSet::new();
        for el in elements.iter() {
            if *el != 0 {
                if set.contains(el) {
                    return false;
                } else {
                    set.insert(*el);
                }
            }
        }
        true
    };
    let mut errors : Vec<SudokuBoardError> = Vec::new();
    // check for row errors
    for (i, row) in board.iter().enumerate() {
        if !check_unique(row) {
            errors.push(SudokuBoardError::Row(i as u8));
        }
    }
    // check for col errors
    for j in 0..9 {
        let mut col = [0u8; 9];
        for i in 0..9 {
            col[i] = board[i][j];
        }
        if !check_unique(&col) {
            errors.push(SudokuBoardError::Column(j as u8));
        }
    }
    // check for quadrant errors
    for qi in 0..3 {
        for qj in 0..3 {
            let mut q = [0u8; 9];
            let mut q_i = 0;
            for i in (3*qi)..(3*(qi+1)) {
                for j in (3*qj)..(3*(qj+1)) {
                    q[q_i] = board[i][j];
                    q_i += 1;
                }
            }
            if !check_unique(&q) {
                errors.push(SudokuBoardError::Quadrant(qi as u8, qj as u8));
            }
        }
    }
    errors
}

/// Finds an unassigned board location
pub fn find_unassigned_location(board : &SudokuBoard) -> Option<(usize, usize)> {
    for (i, row) in board.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 0 {
                return Some((i, j));
            }
        }
    }
    None
}

// Re-export solve game functionality from the solver module
pub use self::solver::solve;

const FILENAME_SAVED_GAME : &str = "saved_game.sud";

/// Save game to file
/// Fails silently
pub fn save_to_file(board_original : &SudokuBoard, board_playing : &SudokuBoard, title : &str) {
    use std::io::BufWriter;
    use std::io::Write;
    let mut path_buf = ::util::exe_dir();
    path_buf.push(FILENAME_SAVED_GAME);
    if let Ok(file) = File::create(path_buf.as_path()){
        let mut writer = BufWriter::new(file);
        let mut output_data = board_original.to_string();
		output_data.push('\n');
		output_data.push_str(board_playing.to_string().as_str());
		output_data.push('\n');
		output_data.push_str(title);
        let _result = writer.write_all(output_data.as_bytes()); // assign to unused variable to avoid the warning
    }
}

/// Load saved game from file
/// Fails silently by returning None
pub fn load_from_file() -> Option<(SudokuBoard, SudokuBoard, String)> {
    use std::io::BufReader;
    use std::io::BufRead;
    use ::util::is_numeric;
    let mut path_buf = ::util::exe_dir();
    path_buf.push(FILENAME_SAVED_GAME);
    if let Ok(file) = File::open(path_buf.as_path()){
		let mut file_reader = BufReader::new(file);
		let mut board_original : Option<SudokuBoard> = None;
		let mut board_playing : Option<SudokuBoard> = None;
		let mut title : Option<String> = None;
		for (index, line) in file_reader.lines().enumerate() {
			if let Ok(line) = line {
				if index == 0 || index == 1 {
					if line.len() == 81 && is_numeric(line.as_str()) {
						if index == 0 {
							board_original = Some(SudokuBoard::new(line.as_str()));
						} else {
							board_playing = Some(SudokuBoard::new(line.as_str()));
						}
					}
				} else if index == 2 {
					title = Some(String::from(line.as_str()));
				} else {
					break;
				}
			} else {
				break;
			}
		}
		if board_original.is_some() && board_playing.is_some() && title.is_some() {
			Some((board_original.unwrap(), board_playing.unwrap(), title.unwrap()))
		} else {
			None
		}
    } else {
		None
	}
}

pub fn remove_saved_game() {
	let mut path_buf = ::util::exe_dir();
    path_buf.push(FILENAME_SAVED_GAME);
	let _ = ::std::fs::remove_file(path_buf.as_path());
}

// Declares the test module
mod tests;