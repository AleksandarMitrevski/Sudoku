mod sudoku_board;

use ::game::{self, SudokuBoard, Difficulty};

/// Handles all state
struct State {
    in_game: bool,
	board_original: Option<SudokuBoard>,
	board_playing: Option<SudokuBoard>,
	game_title: Option<String>,
	is_solving: bool,
	board_solving: Option<Option<SudokuBoard>>
}

static mut STATE : Option<State> = Option::None;

/// Initialize game state
pub fn initialize() {
    unsafe {
		// load saved game if there is one
		if let Some((board_original, board_playing, game_title)) = game::load_from_file() {
			game::remove_saved_game();	// consuming the save file
			STATE = Some(State{
				in_game: true,
				board_original: Some(board_original),
				board_playing: Some(board_playing),
				game_title: Some(game_title),
				is_solving: false,
				board_solving: None
			});
		} else {
			STATE = Some(State{
				in_game: false,
				board_original: None,
				board_playing: None,
				game_title: None,
				is_solving: false,
				board_solving: None
			});
		}
    }
}

pub fn check_initialized(error_message : &'static str) {
	unsafe {
		assert!(STATE.is_some(), error_message);
	}
}

fn start_new_game(difficulty : Difficulty) {
    unsafe {
        if let Some(ref mut state) = STATE {
			let difficulty_string = match difficulty {
				Difficulty::Easy => "Easy",
				Difficulty::Medium => "Medium",
				Difficulty::Hard => "Hard",
				Difficulty::VeryHard => "Very hard"
			};
			let (board_original, game_index) = game::generate_game(difficulty);
			let board_playing = board_original.clone();
			
            state.in_game = true;
			state.board_original = Some(board_original);
			state.board_playing = Some(board_playing);
			state.game_title = Some(format!("{} game #{}", difficulty_string, game_index + 1));
			state.is_solving = false;
			state.board_solving = None;
        } else {
            panic!("game::make_new_game() called with STATE being None");
        }
    }
}

fn reset_game() {
    unsafe {
        if let Some(ref mut state) = STATE {
			state.is_solving = false;
			state.board_playing = Some(state.board_original.unwrap().clone());
		} else {
            panic!("game::reset_game() called with STATE being None");
        }
	}
}

fn solve_game() {
	use ::std::thread;
	
    unsafe {
        if let Some(ref mut state) = STATE {
			reset_game();
			
			// given that we have reset, board_playing = board_original
			state.is_solving = true;
			state.board_solving = None;
			
			let board = state.board_playing.unwrap().clone();
			let is_solving : &bool = &state.is_solving;
			let board_option : &mut Option<Option<SudokuBoard>> = &mut state.board_solving;
			
			// running the solver in a new thread so as not to block the UI, it takes a bit of time for Very Hard games
			thread::spawn(move || {
				let board_solution = ::game::solve(&board);
				
				// only if solving has not been interrupted do the assignment
				if *is_solving {
					*board_option = Some(board_solution);
				}
			});
		} else {
            panic!("game::solve_game() called with STATE being None");
        }
	}
}

fn quit_game() {
    unsafe {
        if let Some(ref mut state) = STATE {
            state.in_game = false;
			state.board_original = None;
			state.board_playing = None;
			state.game_title = None;
			state.is_solving = false;
			state.board_solving = None;
        } else {
            panic!("game::quit_game() called with STATE being None");
        }
    }
}

pub fn on_exit_event() {
	unsafe {
		if let Some(ref mut state) = STATE {
			// if in-game, saving it to hard disk
			if state.in_game {
				game::save_to_file(state.board_original.as_ref().unwrap(), state.board_playing.as_ref().unwrap(), state.game_title.as_ref().unwrap().as_str());
			}
		} else {
            panic!("game::on_exit_event() called with STATE being None");
        }
	}
}

use conrod::{self, widget, Colorable, Positionable, Sizeable, Labelable, Widget};

widget_ids!(
	pub struct Ids {
		text_top_padding,
		text_title,
		button_easy,
		button_medium,
		text_button_alignment,
		button_hard,
		button_very_hard,
		text_game_title,
		text_game_status,
		sudoku_board,
		button_reset,
		button_solve,
		button_quit_game
	}
);

pub fn generate_widget_ids(ui : &mut conrod::Ui) -> Ids {
	// Generate the widget identifiers.	
	Ids::new(ui.widget_id_generator())
}

pub fn draw_ui(ui : &mut conrod::UiCell, ids : &Ids) {
	unsafe {
		if let Some(ref mut state) = STATE {
			// calculating paddings based on window size
			let padding_vertical_bigger = 0.0175 * ui.win_h;
			let padding_vertical_smaller = padding_vertical_bigger / 2.0;
			let padding_vertical_biggest = padding_vertical_smaller + padding_vertical_bigger;
							
			// ugly, is there a better way?
			widget::Text::new("")
				.mid_top()
				.font_size(padding_vertical_bigger.round() as u32)
				.set(ids.text_top_padding, ui);

			if state.in_game {
				let smaller_window_dimension = if ui.win_w < ui.win_h {
					ui.win_w
				} else {
					ui.win_h
				};
				let board_size : f64 = 0.825 * smaller_window_dimension;
				let mut game_button_width : f64;
				let mut game_button_height : f64;
				if ui.win_w < ui.win_h {
					game_button_width = ui.win_w * 0.215;
					game_button_height = game_button_width / 4.5;
				} else {
					game_button_height = ui.win_h * 0.045;
					game_button_width = game_button_height * 4.5;
				}
				let game_button_font_size = (game_button_height * 0.75).round() as u32;
				let status_text_font_size = (0.0255 * ui.win_h).round() as u32;
			
				widget::Text::new(state.game_title.as_ref().unwrap().as_str())
					.down_from(ids.text_top_padding, 0.0)
					.align_middle_x()
					.color(conrod::color::BLACK)
					.font_size(status_text_font_size)
					.set(ids.text_game_title, ui);
				
				if state.is_solving {
					match state.board_solving {
						Some(Some(board_solution)) => {
							// board is solved, copy solution and reset state
							state.board_playing = Some(board_solution);
							state.board_solving = None;
							state.is_solving = false;
						},
						Some(None) => {
							// board is unsolveable, reset state
							state.board_solving = None;
							state.is_solving = false;
						},
						_ => ()
					}
				}
				let mut game_is_finished = false;
				for _finished in sudoku_board::SudokuBoard::new()
					.set_board(state.board_original.as_ref().unwrap(), state.board_playing.as_mut().unwrap(), state.is_solving)
					.down_from(ids.text_game_title, padding_vertical_smaller)
					.align_middle_x()
					.w_h(board_size, board_size)
					.set(ids.sudoku_board, ui) {
						game_is_finished = true;
					}
				let game_is_finished = game_is_finished;
				
				let mut control_buttons_down_from : widget::id::Id;
				let mut control_buttons_padding : f64;
				if !game_is_finished {
					control_buttons_down_from = ids.sudoku_board;
					control_buttons_padding = padding_vertical_biggest;
				} else {
					widget::Text::new("You won. Congratulations!")
						.down_from(ids.sudoku_board, padding_vertical_smaller)
						.align_middle_x()
						.color(conrod::color::BLACK)
						.font_size(status_text_font_size)
						.set(ids.text_game_status, ui);
					control_buttons_down_from = ids.text_game_status;
					control_buttons_padding = padding_vertical_bigger;
				}

				for _click in widget::Button::new()
					.down_from(control_buttons_down_from, control_buttons_padding)
					.align_middle_x()
					.w_h(game_button_width, game_button_height)
					.label("Solve it")
					.label_font_size(game_button_font_size)
					.set(ids.button_solve, ui) {
						// do not solve the board if it has already been solved
						if !game_is_finished {
							solve_game();
						}
					}

				for _click in widget::Button::new()
					.left_from(ids.button_solve, padding_vertical_smaller)
					.w_h(game_button_width, game_button_height)
					.label("Reset")
					.label_font_size(game_button_font_size)
					.set(ids.button_reset, ui) {
						reset_game();
					}

				for _click in widget::Button::new()
					.right_from(ids.button_solve, padding_vertical_smaller)
					.w_h(game_button_width, game_button_height)
					.label("Quit game")
					.label_font_size(game_button_font_size)
					.set(ids.button_quit_game, ui) {
						quit_game();
					}
			} else {
				let smaller_window_dimension = if ui.win_w < ui.win_h {
					ui.win_w
				} else {
					ui.win_h
				};
				let mut title_button_height : f64;
				let mut title_button_width : f64;
				if ui.win_w < ui.win_h {
					title_button_width = ui.win_w * 0.15;
					title_button_height = title_button_width / 4.0;
				} else {
					title_button_height = ui.win_h * 0.036;
					title_button_width = title_button_height * 4.0;
				}
				let title_font_size = ::std::cmp::min(120, (smaller_window_dimension * 0.25).round() as u32);	// the upper bound on font size prevents glium panicking with NoRoomForWholeQueue
				let title_padding_top = 0.5 * ui.win_h - (title_font_size as f64);
				let title_button_font_size = (title_button_height * 0.75).round() as u32;
				let padding_title_buttons_larger = 0.011 * smaller_window_dimension;
				let padding_title_buttons_smaller = padding_title_buttons_larger / 2.0;

				widget::Text::new("Sudoku")
					.down_from(ids.text_top_padding, title_padding_top)
					.align_middle_x()
					.color(conrod::color::BLACK)
					.font_size(title_font_size)
					.set(ids.text_title, ui);

				// again, ugly
				widget::Text::new("")
					.down_from(ids.text_title, padding_vertical_biggest * 1.5)
					.align_middle_x()
					.font_size(title_button_font_size)
					.set(ids.text_button_alignment, ui);

				for _click in widget::Button::new()
					.left_from(ids.button_medium, padding_title_buttons_larger)
					.w_h(title_button_width, title_button_height)
					.label("Easy")
					.label_font_size(title_button_font_size)
					.set(ids.button_easy, ui) {
						start_new_game(Difficulty::Easy);
					};

				for _click in widget::Button::new()
					.left_from(ids.text_button_alignment, padding_title_buttons_smaller)
					.w_h(title_button_width, title_button_height)
					.label("Medium")
					.label_font_size(title_button_font_size)
					.set(ids.button_medium, ui) {
						start_new_game(Difficulty::Medium);
					};

				for _click in widget::Button::new()
					.right_from(ids.text_button_alignment, padding_title_buttons_smaller)
					.w_h(title_button_width, title_button_height)
					.label("Hard")
					.label_font_size(title_button_font_size)
					.set(ids.button_hard, ui) {
						start_new_game(Difficulty::Hard);
					};

				for _click in widget::Button::new()
					.right_from(ids.button_hard, padding_title_buttons_larger)
					.w_h(title_button_width, title_button_height)
					.label("Very Hard")
					.label_font_size(title_button_font_size)
					.set(ids.button_very_hard, ui) {
						start_new_game(Difficulty::VeryHard);
					};
			}
		} else {
			panic!("gui::logic::draw_ui() called with STATE being None");
		}
	}
}