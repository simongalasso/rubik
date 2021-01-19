extern crate pbr;
extern crate rulinalg;

use pbr::ProgressBar;
use std::thread;

mod algo;
mod rubik;

use rubik::rubik_state::*;
use rubik::action::*;
use rubik::face::*;
use rubik::rotation::*;

// fn main() {
//     let mut rubik: Rubik = Rubik::new_shuffled(10);
//     let next_action: Action = rubik.next_action();
//     println!("chosen action: {:?}", next_action);
// }

const I: usize = 100; // shuffle iterations
const L: usize = 100; // nb of iterations
const K: usize = 10; // training set size

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

    let mut progress_bar = ProgressBar::new(L as u64);
    progress_bar.format("|██░|");
    progress_bar.tick_format("-\\|/");
    progress_bar.set_width(Some(80));

    let mut nn = NeuralNetwork::new(/* */);
    for i in 0..L {
        let targets: Vec<(f64, f64)> = Vec::new();
        let training_set: Vec<RubikState> = (0..K).map(|_| RubikState::new_random(I)).collect::<Vec<RubikState>>();
        for state in training_set.iter() {
            for (i, action) in Action::get_actions().iter().enumerate() {
                let child_state: RubikState = action.apply_to(state);
                // (values[i], policies[i]) = nn.guess(child_state);
                // results[i] = values[i] + reward(child_state);
            }
            let value_target: f64 = max(/* */); // max value
            let policy_target: f64 = ; // the action that led to that max value
            targets.push((value_target, policy_target));
        }
        nn.train(training_set, targets);
        progress_bar.inc();
    }
    progress_bar.set_width(None);
    progress_bar.finish_print("training done");
}

/*

// TRAINING (ADI)
for i in 0..training_iterations {
    generate training_set sequence of random actions from the solved cube // this ensure to have a positive reward
    foreach training_set state {
        for (i, action) in ACTIONS_LIST {
            child_state = action.apply_to(state);
            (values[i], policies[i]) = nn.guess(child_state);
            results[i] = values[i] + reward(child_state);
        }
        new_value = max(results);
        new_policy = ACTIONS_LIST[new_value];
    }
    nn.train(training_set, (new_value, new_policy));
}

// ISSUES
- divergent behavior -> https://arxiv.org/pdf/1805.07470.pdf (page 5 - weighted samples)

// OPTIMISATIONS
- possible states (no state callback)
-

// SOLVER (Asynchronous MCTS)
loop {
    if state.is_leaf {
        
    }
    state = state.neighbours[nn.guess.policy];
}

*/