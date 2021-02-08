extern crate rubik;

use rubik::cubie_cube::{CubieCube};
use rubik::coord_cube::{CoordCube};

// this version stops at the first solution found
pub fn solve(state: &mut CubieCube, max_depth: u8) -> Option<Vec<CubieCube>> {
    let mut sequence: Vec<CubieCube> = vec![];
    for bound in 0..max_depth {
        if search_phase1(state, 0, bound, max_depth, &mut sequence) {
            return Some(sequence);
        }
    }
    return None;
}

fn search_phase1(state: &CubieCube, depth: u8, bound: u8, max_depth: u8, sequence: &mut Vec<CubieCube>) -> bool {
    if depth == bound {
        if CoordCube::from_cubie_cube_phase1(state).is_part_of_g1() {
            println!("to G1: {}", sequence.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(" "));
            for bound_phase2 in 0..(max_depth - depth) {
                if search_phase2(state, 0, bound_phase2, sequence) {
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

fn search_phase2(state: &CubieCube, depth: u8, bound: u8, sequence: &mut Vec<CubieCube>) -> bool {
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