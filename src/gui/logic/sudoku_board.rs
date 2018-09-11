use ::std::collections::HashSet;
use conrod::{self, widget, color, Colorable, Sizeable, Borderable, Positionable, Widget};
use ::game;

/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct SudokuBoard<'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
	
	board_original: Option<&'a game::SudokuBoard>,
	board_playing: Option<&'a mut game::SudokuBoard>,
	is_being_solved: Option<bool>
}

// We use `#[derive(WidgetStyle)] to vastly simplify the definition and implementation of the
// widget's associated `Style` type. This generates an implementation that automatically
// retrieves defaults from the provided theme in the following order:
//
// 1. If the field is `None`, falls back to the style stored within the `Theme`.
// 2. If there are no style defaults for the widget in the `Theme`, or if the
//    default field is also `None`, falls back to the expression specified within
//    the field's `#[conrod(default = "expr")]` attribute.

// not using the macro because I can't get lists to work by using the macro
struct Ids {
    text_boxes: conrod::widget::id::List,
	borders_background: conrod::widget::id::Id
}

impl Ids {
    pub fn new(mut generator: conrod::widget::id::Generator) -> Self {
        let mut ids = Ids {
            text_boxes: conrod::widget::id::List::new(),
			borders_background: generator.next()
        };
        ids.text_boxes.resize(81, &mut generator);
        ids
    }
}

/// Represents the unique, cached state for our widget.
pub struct State {
    ids: Ids
}

impl<'a> SudokuBoard<'a> {
    /// Create a context to be built upon.
    pub fn new() -> Self {
        SudokuBoard {
            common: widget::CommonBuilder::default(),
			board_original: None,
			board_playing: None,
			is_being_solved: None
        }
    }
	
	pub fn set_board(mut self : Self, board_original : &'a game::SudokuBoard, board_playing : &'a mut game::SudokuBoard, is_being_solved : bool) -> Self {
		self.board_original = Some(board_original);
		self.board_playing = Some(board_playing);
		self.is_being_solved = Some(is_being_solved);
		self
	}
	
	fn check_board(&self) -> (Option<Vec<game::SudokuBoardError>>, bool) {
		let errors = game::check_for_errors(self.board_playing.as_ref().unwrap());
		if errors.len() == 0 {
			if game::find_unassigned_location(self.board_playing.as_ref().unwrap()).is_none() {
				// board is solved
				(None, true)
			} else {
				(None, false)
			}
		} else {
			(Some(errors), false)
		}
	}
	
	fn get_incorrect_cell_indeces(board_errors : &Option<Vec<game::SudokuBoardError>>) -> HashSet<u8> {
		let mut incorrect_cell_indeces : HashSet<u8> = HashSet::new();
		if let Some(ref board_errors) = board_errors {
			for error in board_errors.iter() {
				match error {
					game::SudokuBoardError::Row(index) => {
						for i in (9 * index)..(9 * (index + 1)) {
							incorrect_cell_indeces.insert(i as u8);
						}
					},
					game::SudokuBoardError::Column(index) => {
						for i in 0..9 {
							incorrect_cell_indeces.insert((i * 9 + index) as u8);
						}
					},
					game::SudokuBoardError::Quadrant(qi, qj) => {
						for i in (3 * qi)..(3 * (qi + 1)) {
							for j in (3 * qj)..(3 * (qj + 1)) {
								incorrect_cell_indeces.insert((i * 9 + j) as u8);
							}
						}
					}
				}
			}
		}
		incorrect_cell_indeces
	}
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a> Widget for SudokuBoard<'a> {
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = ();
    /// The event produced by instantiating the widget - () if the game is finished, otherwise no event.
    type Event = Option<()>;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen)
        }
    }

    fn style(&self) -> Self::Style {
        ()
    }

    /// Optionally specify a function to use for determining whether or not a point is over a
    /// widget, or if some other widget's function should be used to represent this widget.
    ///
    /// This method is optional to implement. By default, the bounding rectangle of the widget
    /// is used.
    /// fn is_over(&self) -> widget::IsOverFn {
    ///   ...
    /// }

    /// Update the state of the widget by handling any input that has occurred since the last update.
    fn update(mut self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, rect, ui, .. } = args;

        assert_eq!(rect.w().round(), rect.h().round(), "SudokuBoard rect is not a square");

        let border_thick_width = 0.007 * rect.w();
        let border_thin_width = 0.003 * rect.w();
        let text_box_dimension = (rect.w() - 4.0 * border_thick_width - 6.0 * border_thin_width) / 9.0;
		let text_box_font_size = (0.65 * text_box_dimension).round() as u32;
		
		// draw borders
		// borders are drawn as a background rect
		widget::primitive::shape::rectangle::Rectangle::fill_with([rect.w(), rect.h()], color::BLACK)
				.x_y_relative_to(id, 0.0, 0.0)
				.set(state.ids.borders_background, ui);
		
		// get board errors and status
		let (board_errors, board_done) = self.check_board();
		
		// draw text boxes
		let incorrect_cell_indeces = Self::get_incorrect_cell_indeces(&board_errors);
		let mut row_start_text_box_id : Option<conrod::widget::id::Id> = None;
		let mut previous_text_box_id : Option<conrod::widget::id::Id> = None;
		for (index, &text_box_id) in state.ids.text_boxes.iter().enumerate() {
			let board_i = index / 9;
			let board_j = index % 9;			
			let text_box_value = self.board_playing.as_ref().unwrap()[board_i][board_j];
			let mut text_box_text : String;
			if text_box_value == 0 {
				text_box_text = String::new();
			} else {
				text_box_text = text_box_value.to_string();
			}
			let text_color = if self.board_original.as_ref().unwrap()[board_i][board_j] == 0 {
				color::rgb(0.46667, 0.46667, 0.86667)
			} else {
				color::BLACK
			};
			let background_color = if incorrect_cell_indeces.contains(&(index as u8)) {
				color::rgb(1.0, 0.74902, 0.74902)
			} else {
				color::WHITE
			};
			
			let mut text_box = widget::TextBox::new(text_box_text.as_str())
				.center_justify()
				.font_size(text_box_font_size)
				.text_color(text_color)
				.color(background_color)
				.w_h(text_box_dimension, text_box_dimension)
				.border(0.0);

			// calculate paddings for cell
			let mut horizontal_padding;
			let mut vertical_padding;
			if index % 3 == 0 {
				horizontal_padding = border_thick_width;
			} else {
				horizontal_padding = border_thin_width;
			}
			if (index / 9) % 3 == 0 {
				vertical_padding = border_thick_width;
			} else {
				vertical_padding = border_thin_width;
			}

			// positioning
			if index == 0 {
				text_box = text_box
					.top_left_with_margins_on(state.ids.borders_background, horizontal_padding, vertical_padding);
				row_start_text_box_id = Some(text_box_id);
			} else if index % 9 == 0 {
				text_box = text_box
					.down_from(row_start_text_box_id.unwrap(), vertical_padding);
				row_start_text_box_id = Some(text_box_id);
			} else {
				text_box = text_box
					.right_from(previous_text_box_id.unwrap(), horizontal_padding);
			}
			previous_text_box_id = Some(text_box_id);

			for edit in text_box.set(text_box_id, ui) {
				if let widget::text_box::Event::Update(text) = edit {						
					// prevent changing values when game solving has been requested and prevent changing of given cell values
					if !self.is_being_solved.unwrap() && self.board_original.as_ref().unwrap()[board_i][board_j] == 0 {
						// find the last character in this text box, if there is one
						if let Some(value) = text.chars().next_back() {
							// convert the character to a number
							if let Some(value) = value.to_digit(10) {
								// set the field value to this number
								self.board_playing.as_mut().unwrap()[board_i][board_j] = value as u8;
							}
						} else {
							// mark the field as empty
							self.board_playing.as_mut().unwrap()[board_i][board_j] = 0;
						}
					}
				}
			}
		}
		
		// return board status event
		if board_done {
			Some(())
		} else {
			None
		}
    }
}