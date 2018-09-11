use super::super::SudokuBoard;
use super::super::check_for_errors;
use super::super::find_unassigned_location;
use super::solve;

fn solve_game_test_helper(board_str : &str) {
    let board = SudokuBoard::new(board_str);
    if let Some(solution) = solve(&board) {
        let errors = check_for_errors(&solution);
        let unassigned_location = find_unassigned_location(&solution);
        if !(errors.len() == 0 && unassigned_location.is_none()) {
            panic!("game::solver::solve() produces invalid solution");
        }
    } else {
        panic!("game::solver::solve() fails to solve board");
    }
}

#[test]
fn solve_game_easy() {
    solve_game_test_helper("081000000900040000023571609194086007200000008805207040540710003702000400000400790");
}

#[test]
fn solve_game_medium() {
    solve_game_test_helper("040300600000064000809002054250001000900000070000400910070098030600103500000006100");
}

#[test]
fn solve_game_hard() {
    solve_game_test_helper("000080001400000050000006027001400000070900300500000000060008170000305006009040508");
}

#[test]
fn solve_game_very_hard() {
    solve_game_test_helper("000060007000080019000570400001000020000006704400002003060803000007000000304090000");
}

#[test]
#[should_panic(expected = "game::solver::solve() fails to solve board")]
fn solve_game_unsolvable_invalid() {
    solve_game_test_helper("110000000000000000000000000000000000000000000000000000000000000000000000000000000");
}

#[test]
#[should_panic(expected = "game::solver::solve() fails to solve board")]
fn solve_game_unsolvable_valid() {
    solve_game_test_helper("516849732307605000809700065135060907472591006968370050253186074684207500791050608");
}

// the empty board is solveable by the specific algorithm in place
#[test]
fn solve_game_empty() {
    solve_game_test_helper("000000000000000000000000000000000000000000000000000000000000000000000000000000000");
}