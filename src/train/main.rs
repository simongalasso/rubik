extern crate pbr;
extern crate rulinalg;
extern crate rubik;

use std::cmp::Ordering;
use pbr::ProgressBar;
use rulinalg::matrix::{Matrix};

use rubik::neuralnet::*;
use rubik::rubik_state::*;
use rubik::face::*;
use rubik::rotation::*;
use rubik::action::*;

const MAX_ITER: usize = 1000; // nb of ADI procedure
const N_SHUFFLE: usize = 100; // training set size

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

    let mut progress_bar = ProgressBar::new((MAX_ITER) as u64);
    progress_bar.format("|██░|");
    progress_bar.set_width(Some(80));

    let mut nn = NeuralNetwork::new(40, 40, 1 + 18); // can reduce input to 24 (https://arxiv.org/pdf/1805.07470.pdf) // hidden node is arbitrary
    for _ in 0..MAX_ITER {
        let mut shuffled_states: Vec<RubikState> = vec![SOLVED_STATE];
        for n in 1..N_SHUFFLE {
            let action: Action = Action::new(Face::pick_random(), Rotation::pick_random());
            let new_state = action.apply_to(&shuffled_states[n - 1]);
            shuffled_states.push(new_state);
        }
        let mut set_values: Vec<(f64, usize)> = Vec::new();
        for state in shuffled_states.iter() {
            let reward: f64 = match *state == SOLVED_STATE {
                true => 1.0,
                false => -1.0
            };
            let mut values: Vec<f64> = Vec::new();
            for a in Action::get_actions().iter() {
                let a_state = a.apply_to(&state);
                let result: Matrix<f64> = nn.feedforward(Matrix::new(40, 1, a_state.aligned_format()));
                let v: f64 = result.data()[0];
                let p: Vec<f64> = (&result.data()[1..]).to_vec(); // something probably incoherent in p here and below
                values.push(v * reward);
            }
            let (v_target, p_target): (f64, usize) = values.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal)).map(|(index, val)| (*val, index)).unwrap(); // handle unwrap
            set_values.push((v_target, p_target));
        }
        for n in 0..N_SHUFFLE {
            let inputs: Matrix<f64> = Matrix::new(40, 1, shuffled_states[n].aligned_format());
            let targets: Matrix<f64> = Matrix::new(1 + 18, 1, (0..19).map(|i| if i == 0 { set_values[n].0 } else if i == set_values[n].1 { 1.0 } else { 0.0 }).collect::<Vec<f64>>());
            nn.train(&inputs, &targets);
        }
        progress_bar.inc();
    }
    progress_bar.set_width(None);
    progress_bar.finish_print("training terminated");
    nn.export_weights();
}

/*

// ISSUES
- https://arxiv.org/pdf/1805.07470.pdf (page 5 - weighted samples)

// OPTIMISATIONS
- possible states (no state callback, no two times the same action)

*/