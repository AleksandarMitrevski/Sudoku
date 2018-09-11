# Sudoku

To those lacking inherent appreciation of [DICSS](https://github.com/AleksandarMitrevski/DICSS) I offer an altogether beautiful and touching alternative.

![Title screen](https://github.com/AleksandarMitrevski/sudoku/blob/master/other/game_images/game-0.png)

![In-game 1](https://github.com/AleksandarMitrevski/sudoku/blob/master/other/game_images/game-1.png)

![In-game 2](https://github.com/AleksandarMitrevski/sudoku/blob/master/other/game_images/game-2.png)

![Solved game](https://github.com/AleksandarMitrevski/sudoku/blob/master/other/game_images/game-3.png)

## Building

Run `cargo build --release` and then copy the `/resources` folder to the directory of the generated EXE.

## Runtime dependencies

The application runs on all platforms that can be targeted by Rust. Rendering the GUI requires OpenGL support. The specific compiler runtime dependencies need to be installed if building dynamically.

## Implementation details

The GUI is built on top of [Conrod](https://github.com/PistonDevelopers/conrod) v0.61.1 as to me it seemed the simplest, easiest-to-use GUI library for Rust at the time of selection.

The game features four difficulty levels: easy, medium, hard and very hard, and it comes with fourty predefined games in each - see `/resources/games/*.sud`; these files can be modified and / or expanded to include more games. The provided predefined games were generated using the online game generators of [Open Sudoku](https://opensudoku.moire.org/).

The game-solving algorithm is a direct adaptation of `/other/solver_algorithm/solve_board_algorithm.cpp`, which comes from [GeeksforGeeks](https://www.geeksforgeeks.org/sudoku-backtracking-7/).

As for code organization, there are two important submodules of the binary root module:

- The responsibilities of the `game` module are new game selection, error-checking logic, solver algorithm, game saving, and there are unit tests for some of the algorithms.

- The `gui` module provides a thin interface for the root module and handles some of the usual setup with Conrod, its `logic` submodule and this module's `sudoku_board` custom widget submodule is where the game GUI is created.