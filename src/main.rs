#[macro_use]
extern crate conrod;
#[macro_use]
extern crate conrod_derive;

mod game;
mod gui;
mod util;

fn main() {
    // read in files for new game generation
    game::initialize_database();

    // initialize GUI and game state, then start the event loop
    gui::initialize();
    gui::start();
}