extern crate pbr;
extern crate rulinalg;
extern crate rubik;

use nn::{NN, HaltCondition};

use std::cmp::Ordering;
use pbr::ProgressBar;
use rulinalg::matrix::{Matrix};

use rubik::neuralnet::*;
use rubik::rubik_state::*;
use rubik::face::*;
use rubik::rotation::*;
use rubik::action::*;

// const MAX_ITER: usize = 1000; // nb of ADI procedure
// const N_SHUFFLE: usize = 100; // training set size

const MAX_ADI: usize = 1000;
const TRAINING_INPUTS_NB: usize = 100;

fn main() {

    // let mut rubik_state: RubikState = RubikState::new_solved();
    // println!("{:?}", rubik_state);
    // rubik_state = Action::new(Face::F, Rotation::R).apply_to(&rubik_state);
    // println!("{:?}", rubik_state);
    // rubik_state = Action::new(Face::F, Rotation::R).apply_to(&rubik_state);
    // println!("{:?}", rubik_state);
    // rubik_state = Action::new(Face::F, Rotation::R).apply_to(&rubik_state);
    // println!("{:?}", rubik_state);
    // rubik_state = Action::new(Face::F, Rotation::R).apply_to(&rubik_state);
    // println!("{:?}", rubik_state);

    let mut progress_bar = ProgressBar::new((MAX_ADI) as u64);
    progress_bar.format("|██░|");
    progress_bar.set_width(Some(80));

    // let mut nn = NeuralNetwork::new(40, 40, 1 + 18); // can reduce input to 24 (https://arxiv.org/pdf/1805.07470.pdf) // hidden node is arbitrary
    let mut nn = NN::new(&[40, 40, 1 + 18]);
    for _ in 0..MAX_ADI {
        let mut training_inputs: Vec<RubikState> = vec![SOLVED_STATE];
        for i in 1..TRAINING_INPUTS_NB {
            training_inputs.push(Action::pick_random().apply_to(&training_inputs[i - 1]));
        }
        let mut targets: Vec<Vec<f64>> = Vec::new();
        for state in training_inputs.iter() {
            let mut children_state_values: Vec<f64> = Vec::new();
            let mut children_state_policies: Vec<Vec<f64>> = Vec::new();
            for action in Action::get_actions().iter() {
                let child_state: RubikState = action.apply_to(&state);
                let reward: f64 = match child_state == SOLVED_STATE {
                    true => 1.0,
                    false => -1.0
                };
                let results: Vec<f64> = nn.run(&(child_state.aligned_format())[..]);
                children_state_values.push(results[0] + reward);
                children_state_policies.push((&results[1..]).to_vec());
            }
            let index: usize = children_state_values.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal)).map(|(index, _)| index).unwrap(); // handle unwrap
            let mut state_value_and_policies_target: Vec<f64> = vec![children_state_values[index]]; // maybe have to normalize (depend on the nn algo used) [!]
            state_value_and_policies_target.append(&mut children_state_policies[index]);
            targets.push(state_value_and_policies_target);
        }
        let training_set: Vec<(Vec<f64>, Vec<f64>)> = (0..TRAINING_INPUTS_NB).map(|i| (training_inputs[i].aligned_format(), targets[i].clone())).collect::<Vec<(Vec<f64>, Vec<f64>)>>();
        nn.train(&training_set)
            .halt_condition( HaltCondition::Epochs(1) )
            .momentum( 0.1 )
            .rate( 0.3 )
            .go();
        progress_bar.inc();
    }
    progress_bar.set_width(None);
    progress_bar.finish_print("training terminated");
    // nn.export_weights();
    NeuralNetwork::export_string(nn.to_json());
}

/*

// ISSUES
- https://arxiv.org/pdf/1805.07470.pdf (page 5 - weighted samples)

// OPTIMISATIONS
- possible states (no state callback, no two times the same action)

*/