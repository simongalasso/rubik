mod parsing;
mod display;
mod action;
mod rubik;
mod graphics;

use parsing::args::*;
// use parsing::parse::*;
// use display::output::*;
// use action::action::*;
// use rubik::rubik::*;
use graphics::graphics::*;

fn main() {
    let config: Config = Config::new();
    // let input_sequence: Vec<Action> = parse(&config);
    // display_sequence("shuffle: ", &input_sequence);

    // let mut rubik: Rubik = Rubik::new();
    // rubik.shuffle(input_sequence);

    // let output_sequence: Vec<Action> = rubik.solve();
    // display_sequence("solution: ", &output_sequence);

    if config.visualisator {
        display_graphics();
    }
}