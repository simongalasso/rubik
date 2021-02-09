extern crate rubik;

use crate::pruning::pruning::{Pruning};

use rubik::cubie_cube::{CubieCube};
use rubik::enums::*;
use std::cmp;

// this version stops at the first solution found
pub fn solve(state: &mut CubieCube, max_depth: u8, pruning_tables: Pruning) -> Option<Vec<(CubieCube, u8)>> {
    let mut sequence: Vec<usize> = vec![];
    for bound in 0..max_depth {
        if search_phase1(state, 0, bound, max_depth, &mut sequence, &pruning_tables) {
            return Some(sequence);
        }
    }
    return None;
}

fn search_phase1(state: &CubieCube, depth: u8, bound: u8, max_depth: u8, sequence: &mut Vec<usize>>, pruning_tables: &Pruning) -> bool {
    if depth == bound {
        if state.is_part_of_g1() {
            println!("to G1: {}", sequence.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" "));
            for bound_phase2 in 0..(max_depth - depth) {
                if search_phase2(state, 0, bound_phase2, sequence) {
                    return true;
                }
            }
            panic!("error, phase2 didn't found a solution"); // FIXME, find other way
        }
    } else if cmp::max(pruning_tables.slice_flip_pruning_table[state.get_flip_coord()], pruning_tables.slice_twist_pruning_table[state.get_twist_coord()]) as u8 <= depth {
        for action in ACTIONS.iter() {
            sequence.push(action.clone());
            let new_state: CubieCube = state.multiply(&ACTIONS_LIST[*action].0, ACTIONS_LIST[*action].1);
            if search_phase1(&new_state, depth + 1, bound, max_depth, sequence, &pruning_tables) {
                return true;
            }
            sequence.pop();
        }
    }
    return false;
}

fn search_phase2(state: &CubieCube, depth: u8, bound: u8, sequence: &mut Vec<usize>) -> bool {
    if depth == bound {
        if state.is_solved() {
            return true;
        }
    } else /* if prunning_table_phase2[state] <= depth */ {
        for action in G1_ACTIONS.iter() {
            sequence.push(action.clone());
            let new_state: CubieCube = state.multiply(&ACTIONS_LIST[*action].0, ACTIONS_LIST[*action].1);
            if search_phase2(&new_state, depth + 1, bound, sequence) {
                return true;
            }
            sequence.pop();
        }
    }
    return false;
}