extern crate rand;

use super::SudokuBoard;
use self::rand::{Rng, ThreadRng};
use std::fs::File;

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    VeryHard
}

struct Database<> {
    easy: Vec<String>,
    medium: Vec<String>,
    hard: Vec<String>,
    very_hard: Vec<String>
}

static mut DATABASE : Option<Box<Database>> = Option::None;
static mut PRNG_THREAD : Option<ThreadRng> = Option::None;

/// Initializes the game database from resource files
/// # Panics 
/// If any of the following files does not exist: resources/games/easy.sud, resources/games/medium.sud, resources/games/hard.sud, resources/games/very_hard.sud; and by implication, on any of read_database_sud_file panic condition
pub fn initialize_database() {
    let mut dir_path = ::util::exe_dir();
    dir_path.push("resources");
    dir_path.push("games");

    let mut easy_path = dir_path.clone();
    easy_path.push("easy.sud");
    let mut easy_file = File::open(easy_path.as_path()).expect("easy.sud not found");

    let mut medium_path = dir_path.clone();
    medium_path.push("medium.sud");
    let mut medium_file = File::open(medium_path.as_path()).expect("medium.sud not found");

    let mut hard_path = dir_path.clone();
    hard_path.push("hard.sud");
    let mut hard_file = File::open(hard_path.as_path()).expect("hard.sud not found");

    let mut very_hard_path = dir_path.clone();
    very_hard_path.push("very_hard.sud");
    let mut very_hard_file = File::open(very_hard_path.as_path()).expect("very_hard.sud not found");

    let (easy, medium, hard, very_hard) = (
        read_database_sud_file(&mut easy_file),
        read_database_sud_file(&mut medium_file),
        read_database_sud_file(&mut hard_file),
        read_database_sud_file(&mut very_hard_file)
    );
    let database = Box::new(Database{easy, medium, hard, very_hard});

    unsafe{
        DATABASE = Some(database);
    }
}

/// Reads a single .sud game file
/// # Panics
/// If a file is invalid, that is, if it does not exclusively consist of digit rows of length 81
fn read_database_sud_file(file: &mut File) -> Vec<String> {
    use std::io::BufReader;
    use std::io::BufRead;
    use ::util::is_numeric;

    let mut games : Vec<String> = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_string = line.expect("IO error while reading .sud file");
        if line_string.len() != 81 || !is_numeric(line_string.as_str()) {
            panic!(format!("Line {} is invalid", line_string));
        }
        games.push(line_string);
    }
    games
}

/// It randomly picks a predefined game
pub fn generate_game(difficulty : Difficulty) -> (SudokuBoard, usize) {
    unsafe{
        let database = DATABASE.as_ref().unwrap();
        match difficulty {
            Difficulty::Easy => pick_game(&database.easy),
            Difficulty::Medium => pick_game(&database.medium),
            Difficulty::Hard => pick_game(&database.hard),
            Difficulty::VeryHard => pick_game(&database.very_hard)
        }
    }
}

fn pick_game(games_vec : &Vec<String>) -> (SudokuBoard, usize) {
    unsafe {
        // lazy initialization of a static variable
        // another option would be to use the lazy_static crate, but it does not work in this case due to the object being a thread
        if PRNG_THREAD.is_none() {
            PRNG_THREAD = Some(rand::thread_rng());
        }
        let prng = PRNG_THREAD.as_mut().unwrap();
        let index : usize = prng.gen_range(0, games_vec.len());
        let selected_game = &games_vec[index];
        (SudokuBoard::new(selected_game.as_str()), index)
    }
}