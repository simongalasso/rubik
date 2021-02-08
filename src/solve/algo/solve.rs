extern crate rubik;

use rubik::cubie_cube::{CubieCube};
use rubik::coord_cube::{CoordCube};

/* # Kociemba Algorithm ######################################### */

// const MAX_LEN: u16 = 25;
// const CURRENT_DEPTH: u16 = /* ? */;

// pub fn solve(cubestring: String) {
//     let fc_cube: FaceletCube = FaceletCube::from_string(cubestring);
//     let cb_cube = fc_cube.to_cubie_cube();
//     let co_cube = cb_cube.to_coord_cube();
//     phase1_start(co_cube);
// }

// fn phase1_start(state: CoordCube) {
//     for depth in 0..MAX_LEN {
//         self.phase1(co_cube, depth);
//     }
// }

// fn phase1(state: CoordCube, depth: u16) {
//     if depth == 0 && state.is_part_of_g1() && /* last move was not part of g1 actions, so it should be either F, F', B, B', R, R', L, or L' */ {
//         phase2_start(state);
//     } else if depth > 0 && prune1[state] <= depth {
//         let relevant_actions = /* ? */;
//         for a in relevant_actions.iter() {
//             phase1(a.apply_to(&state), depth - 1);
//         }
//     }
// }

// fn phase2_start(state: CoordCube) {
//     for depth in 0..(MAX_LEN - CURRENT_DEPTH) {
//         phase2(state, depth);
//     }
// }

// fn phase2(state: CoordCube, depth: u16) {
//     if depth == 0 && state.is_solved() {
//         MAX_LEN = CURRENT_DEPTH - 1;
//     } else if depth > 0 {
//         let relevant_actions = /* ? */;
//         for a in relevant_actions.iter() {
//             phase2(a.apply_to(&state), depth - 1);
//         }
//     }
// }

// this version stops at the first solution found
pub fn solve(state: &mut CubieCube, max_depth: u8) -> Option<Vec<CubieCube>> {
    // let database: Vec<(CubieCube, Option<CubieCube>)> = generate_database();
    let mut sequence: Vec<CubieCube> = vec![];
    for bound in 0..max_depth {
        if search_phase1(state, 0, bound, max_depth, &mut sequence, /* database */) {
            return Some(sequence);
        }
    }
    return None;
}

fn search_phase1(state: &CubieCube, depth: u8, bound: u8, max_depth: u8, sequence: &mut Vec<CubieCube>, /* database: Vec<> */) -> bool {
    if depth == bound {
        if CoordCube::from_cubie_cube_phase1(state).is_part_of_g1() {
            println!("to G1: {}", sequence.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(" "));
            for bound_phase2 in 0..(max_depth - depth) {
                if search_phase2(state, 0, bound_phase2, sequence, /* database */) {
                    return true;
                }
            }
            panic!("error, phase2 didn't found a solution"); // FIXME, find other way
        }
    } else /* if prunning_table_phase_1[state] <= depth */ {
        for action in CubieCube::get_actions().iter() {
            sequence.push(action.clone());
            let mut new_state: CubieCube = state.clone();
            new_state.multiply(action);
            if search_phase1(&new_state, depth + 1, bound, max_depth, sequence) {
                return true;
            }
            sequence.pop();
        }
    }
    return false;
}

fn search_phase2(state: &CubieCube, depth: u8, bound: u8, sequence: &mut Vec<CubieCube>, /* database: Vec<> */) -> bool {
    if depth == bound {
        if CoordCube::from_cubie_cube_phase2(state).is_solved() {
            return true;
        }
    } else /* if prunning_table_phase2[state] <= depth */ {
        for action in CubieCube::get_g1_actions() {
            sequence.push(action.clone());
            let mut new_state: CubieCube = state.clone();
            new_state.multiply(&action);
            if search_phase2(&new_state, depth + 1, bound, sequence) {
                return true;
            }
            sequence.pop();
        }
    }
    return false;
}

// use std::collections::VecDeque;

// fn generate_database() -> Vec<(CubieCube, Option<CubieCube>)> {
//     let mut database: Vec<(CubieCube, Option<CubieCube>)> = vec![];
//     let mut queue: VecDeque<(CubieCube, Option<CubieCube>)> = VecDeque::new();
//     queue.push_back((CubieCube::new_solved(), None));
//     while let Some(el) = queue.pop_front() {
//         for action in CubieCube::get_actions().iter() {
//             let mut new_state: CubieCube = el.0.clone();
//             new_state.multiply(action);
//             queue.push_back((new_state.clone(), Some(action.clone())));
//             database.push((new_state.clone(), Some(action.inverse())));
//         }
//     }
//     return database;
// }
