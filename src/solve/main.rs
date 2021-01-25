extern crate rubik;

use nn::{NN};

mod parsing;
mod graphics;
mod display;
mod algo;

use rubik::neuralnet::*;
use rubik::rubik_state::*;
use rubik::action::*;
use parsing::args::*;
use parsing::parse::*;
use display::output::*;
use algo::mcts::*;

fn main() {
    let config: Config = Config::new();
    let input_sequence: Vec<Action> = parse(&config);

    // let mut nn: NeuralNetwork = NeuralNetwork::new(40, 40, 1 + 18);
    // nn.import_weights(&config.weights_file);
    let nn = NN::from_json(&(NeuralNetwork::import_file(&config.weights_file))[..]);

    display_sequence("shuffle: ", Some(input_sequence.clone())); // do something better than clone
    println!("weights file: {}", config.weights_file);
    println!("visualisator: {}{}", config.visualisator, if config.visualisator { format!(" | speed: {}", config.speed_selection) } else { String::from("") });

    let mut state: RubikState = SOLVED_STATE;
    state.shuffle(input_sequence);

    let solution: Option<Vec<Action>> = solve(&state, &nn);
    display_sequence("solution: ", solution);

    // if config.visualisator {
    //     display_graphics(&input_sequence, config.speed_selection, &mut nn);
    // }
}