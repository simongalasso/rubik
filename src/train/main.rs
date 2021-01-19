extern crate rulinalg;

mod algo;
mod rubik;

use rubik::rubik_state::*;
use rubik::action::*;

// fn main() {
//     let mut rubik: Rubik = Rubik::new_shuffled(10);
//     let next_action: Action = rubik.next_action();
//     println!("chosen action: {:?}", next_action);
// }

const ACTION_LIST: [Action; 18] = [
    Action { face: Face::U, rot: Rotation::L }, Action { face: Face::U, rot: Rotation::R }, Action { face: Face::U, rot: Rotation::D },
    Action { face: Face::F, rot: Rotation::L }, Action { face: Face::F, rot: Rotation::R }, Action { face: Face::F, rot: Rotation::D },
    Action { face: Face::L, rot: Rotation::L }, Action { face: Face::L, rot: Rotation::R }, Action { face: Face::L, rot: Rotation::D },
    Action { face: Face::D, rot: Rotation::L }, Action { face: Face::D, rot: Rotation::R }, Action { face: Face::D, rot: Rotation::D },
    Action { face: Face::R, rot: Rotation::L }, Action { face: Face::R, rot: Rotation::R }, Action { face: Face::R, rot: Rotation::D },
    Action { face: Face::B, rot: Rotation::L }, Action { face: Face::B, rot: Rotation::R }, Action { face: Face::B, rot: Rotation::D },
];

const I: usize = 100; // shuffle iterations
const L: usize = 100; // nb of iterations
const K: usize = 10; // training set size

fn main() {
    for i in 0..L {
        let training_set: Vec<RubikState> = (0..K).map(|_| RubikState::new_random(I)).collect::<Vec<RubikState>>();
        // for state in training_set.iter() {
        //     for (i, action) in ACTIONS_LIST.iter().enumerate() {
        //         child_state = action.apply_to(state);
        //         (values[i], policies[i]) = nn.guess(child_state);
        //         results[i] = values[i] + reward(child_state);
        //     }
        // }
    }
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