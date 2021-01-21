extern crate rubik;

mod parsing;
mod graphics;
mod display;

use rubik::neuralnet::*;
use rubik::action::*;
use parsing::args::*;
use parsing::parse::*;
use graphics::graphics::*;
use display::output::*;

fn main() {
    let config: Config = Config::new();
    let input_sequence: Vec<Action> = parse(&config);

    let mut nn: NeuralNetwork = NeuralNetwork::new(40, 40, 1 + 18);
    nn.import_weights(&config.weights_file);

    display_sequence("shuffle: ", &input_sequence);
    println!("weights file: {}", config.weights_file);
    println!("visualisator: {}{}", config.visualisator, if config.visualisator { format!(" | speed: {}", config.speed_selection) } else { String::from("") });

    // let mut rubik: Rubik = Rubik::new();
    // rubik.shuffle(input_sequence);

    // let output_sequence: Vec<Action> = rubik.solve();
    // display_sequence("solution: ", &output_sequence);

    if config.visualisator {
        display_graphics(&input_sequence, config.speed_selection, &mut nn);
    }
}