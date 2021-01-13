mod parsing;
mod display;
mod rubik;

use parsing::args::*;
use parsing::parse::*;
use display::output::*;
use rubik::rubik::*;

fn main() {
    let config: Config = Config::new();
    let input_sequence: Vec<Action> = parse(&config);
    display_sequence("shuffle: ", &input_sequence);

    let mut rubik: Rubik = Rubik::new();
    rubik.shuffle(input_sequence);

    let output_sequence: Vec<Action> = rubik.solve();
    display_sequence("solution: ", &output_sequence);
}