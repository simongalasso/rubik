extern crate pbr;
extern crate rulinalg;

use pbr::ProgressBar;

mod algo;
mod rubik;

use rubik::rubik_state::*;
use rubik::face::*;
use rubik::rotation::*;
use rubik::action::*;

// fn main() {
//     let mut rubik: Rubik = Rubik::new_shuffled(10);
//     let next_action: Action = rubik.next_action();
//     println!("chosen action: {:?}", next_action);
// }

const MAX_ITER: usize = 1000; // nb of ADI procedure
const SET_SIZE: usize = 100; // training set size
const N_SHUFFLE: usize = 100; // training set size

const RAND_IT: usize = 100; // rand cube shuffle iterations

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

    let mut progress_bar = ProgressBar::new(MAX_ITER as u64);
    progress_bar.format("|██░|");
    progress_bar.set_width(Some(80));

    // let mut nn = NeuralNetwork::new(/* */);
    for _ in 0..MAX_ITER {
        for _ in 0..SET_SIZE {
            let mut state: RubikState = SOLVED_STATE;
            for n in 0..N_SHUFFLE {
                let action: Action = Action::new(Face::pick_random(), Rotation::pick_random());
                state = action.apply_to(&state);
                let reward: f64 = match state == SOLVED_STATE {
                    true => 1.0,
                    false => -1.0
                };
                let mut values: Vec<f64> = Vec::new();
                for a in Action::get_actions().iter() {
                    state = a.apply_to(&state);
                    // let (v, p): (f64, ?) = nn.feedforward()
                    values.push(/* p + */reward);
                }
                // let (p_max, v_max): (usize, f64) = output.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal)).map(|(index, _)| index);
            }
        }
        progress_bar.inc();
    }
    progress_bar.set_width(None);
    progress_bar.finish_print("training done");
    // export weights to file : weights.csv
}

/*

// ISSUES
- https://arxiv.org/pdf/1805.07470.pdf (page 5 - weighted samples)

// OPTIMISATIONS
- possible states (no state callback)

*/