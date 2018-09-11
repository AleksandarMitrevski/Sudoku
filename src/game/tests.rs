use super::SudokuBoard;
use super::SudokuBoardError;
use super::check_for_errors;

#[test]
fn board_marshal_unmarshal_inverse_check() {
    let board_1 = SudokuBoard::new("000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    let board_2 = SudokuBoard::new(board_1.to_string().as_str());
    assert_eq!(board_1, board_2, "marshalling and unmarshalling of game::SudokuBoard are not inverse operations");
}

#[test]
fn check_board_errors_none() {
    let board = SudokuBoard::new("000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    let errors = check_for_errors(&board);
    assert_eq!(errors.len(), 0, "game::check_for_errors() returns error(s) when there are not any");
}

#[test]
fn check_board_errors_row() {
    let board = SudokuBoard::new("100100000000000000000000000000000000000000000000000000000000000000000000000000000");
    let errors = check_for_errors(&board);
    assert_eq!(errors.len(), 1, "game::check_for_errors() does not return one error");
    match errors[0] {
        SudokuBoardError::Row(_) => (),
        _ => panic!("game::check_for_errors() returns incorrect error")
    };
}

#[test]
fn check_board_errors_col() {
    let board = SudokuBoard::new("100000000000000000000000000100000000000000000000000000000000000000000000000000000");
    let errors = check_for_errors(&board);
    assert_eq!(errors.len(), 1, "game::check_for_errors() does not return one error");
    match errors[0] {
        SudokuBoardError::Column(_) => (),
        _ => panic!("game::check_for_errors() returns incorrect error")
    };
}

#[test]
fn check_board_errors_quadrant() {
    let board = SudokuBoard::new("100000000010000000000000000000000000000000000000000000000000000000000000000000000");
    let errors = check_for_errors(&board);
    assert_eq!(errors.len(), 1, "game::check_for_errors() does not return one error");
    match errors[0] {
        SudokuBoardError::Quadrant(_, _) => (),
        _ => panic!("game::check_for_errors() returns incorrect error")
    };
}

#[test]
fn check_board_errors_row_column() {
    let board = SudokuBoard::new("100100000000000000000000000100000000000000000000000000000000000000000000000000000");
    let errors = check_for_errors(&board);
    assert_eq!(errors.len(), 2, "game::check_for_errors() does not return two errors");
    match (&errors[0], &errors[1]) {
        (SudokuBoardError::Row(_), SudokuBoardError::Column(_)) => (),
        _ => panic!("game::check_for_errors() returns incorrect errors")
    };
}

#[test]
fn check_board_errors_row_quadrant() {
    let board = SudokuBoard::new("110000000000000000000000000000000000000000000000000000000000000000000000000000000");
    let errors = check_for_errors(&board);
    assert_eq!(errors.len(), 2, "game::check_for_errors() does not return two errors");
    match (&errors[0], &errors[1]) {
        (SudokuBoardError::Row(_), SudokuBoardError::Quadrant(_, _)) => (),
        _ => panic!("game::check_for_errors() returns incorrect errors")
    };
}

#[test]
fn check_board_errors_column_quadrant() {
    let board = SudokuBoard::new("100000000100000000000000000000000000000000000000000000000000000000000000000000000");
    let errors = check_for_errors(&board);
    assert_eq!(errors.len(), 2, "game::check_for_errors() does not return two errors");
    match (&errors[0], &errors[1]) {
        (SudokuBoardError::Column(_), SudokuBoardError::Quadrant(_, _)) => (),
        _ => panic!("game::check_for_errors() returns incorrect errors")
    };
}

#[test]
fn check_board_errors_row_column_quadrant() {
    let board = SudokuBoard::new("110000000100000000000000000000000000000000000000000000000000000000000000000000000");
    let errors = check_for_errors(&board);
    assert_eq!(errors.len(), 3, "game::check_for_errors() does not return three errors");
    match (&errors[0], &errors[1], &errors[2]) {
        (SudokuBoardError::Row(_), SudokuBoardError::Column(_), SudokuBoardError::Quadrant(_, _)) => (),
        _ => panic!("game::check_for_errors() returns incorrect errors")
    };
}