mod parsing;
mod display;

use parsing::args::*;
use parsing::parse::*;
use display::output::*;

fn main() {
    let config: Config = Config::new();
    let input_sequence: Vec<Action> = parse(&config);
    display_sequence("shuffle: ", input_sequence);
}