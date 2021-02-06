extern crate rubik;

use rubik::cubie_cube::{CubieCube};
use rubik::edge::{Edge};

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
// db_max break the function, added for testing
pub fn solve(state: &mut CubieCube, max_depth: u8, db_max: i32) -> Option<Vec<CubieCube>> {
    // let database: Vec<(CubieCube, Option<CubieCube>)> = generate_database(db_max);
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
        if contained_in_g1(state) {
            for bound_phase2 in 0..(max_depth - depth) {
                if search_phase2(state, 0, bound_phase2, sequence, /* database */) {
                    return true;
                }
            }
            panic!("error, phase2 didn't found a solution");
        }
    } else /* if prunning_table_phase_1[state] <= depth */ {
        for action in CubieCube::get_actions().iter() {
            if !sequence.contains(action) {
                sequence.push(action.clone());
                let mut new_state: CubieCube = state.clone();
                new_state.multiply(action);
                if search_phase1(&new_state, depth + 1, bound, max_depth, sequence) {
                    return true;
                }
                sequence.pop();
            }
        }
    }
    return false;
}

fn contained_in_g1(state: &CubieCube) -> bool { // FIXME, a optimiser
    let o_sum: u8 = state.e_o.iter().sum::<u8>() + state.c_o.iter().sum::<u8>();
    let fr_index: usize = state.e_p.iter().position(|el| *el == Edge::FR).unwrap();
    let fl_index: usize = state.e_p.iter().position(|el| *el == Edge::FL).unwrap();
    let bl_index: usize = state.e_p.iter().position(|el| *el == Edge::BL).unwrap();
    let br_index: usize = state.e_p.iter().position(|el| *el == Edge::BR).unwrap();
    let valid_pos: [usize; 4] = [8, 9, 10, 11];
    return o_sum == 0 && valid_pos.contains(&fr_index) && valid_pos.contains(&fl_index) && valid_pos.contains(&bl_index) && valid_pos.contains(&br_index);
}

fn search_phase2(state: &CubieCube, depth: u8, bound: u8, sequence: &mut Vec<CubieCube>, /* database: Vec<> */) -> bool {
    if depth == bound {
        if *state == CubieCube::new_solved() {
            return true;
        }
    } else /* if prunning_table_phase2[state] <= depth */ {
        for action in CubieCube::get_actions().iter() {
            if !sequence.contains(action) {
                sequence.push(action.clone());
                let mut new_state: CubieCube = state.clone();
                new_state.multiply(action);
                if search_phase2(&new_state, depth + 1, bound, sequence) {
                    return true;
                }
                sequence.pop();
            }
        }
    }
    return false;
}

// use std::collections::VecDeque;

// fn generate_database(mut db_max: i32) -> Vec<(CubieCube, Option<CubieCube>)> {
//     let mut database: Vec<(CubieCube, Option<CubieCube>)> = vec![];
//     let mut queue: VecDeque<(CubieCube, Option<CubieCube>)> = VecDeque::new();
//     queue.push_back((CubieCube::new_solved(), None));
//     while let Some(el) = queue.pop_front() {
//         if db_max <= 0 {
//             break;
//         }
//         for action in CubieCube::get_actions().iter() {
//             let mut new_state: CubieCube = el.0.clone();
//             new_state.multiply(action);
//             queue.push_back((new_state.clone(), Some(action.clone())));
//             database.push((new_state.clone(), Some(action.inverse())));
//             db_max -= 1;
//         }
//     }
//     return database;
// }
