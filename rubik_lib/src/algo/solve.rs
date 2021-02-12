
use pruning::pruning::{Pruning};
use rubik::cubie_cube::{CubieCube};
use rubik::enums::*;

const MAX_DEPTH: u8 = 30;

// this version stops at the first solution found
pub fn solve(state: &mut CubieCube, pruning_tables: Pruning) -> Option<Vec<usize>> {
    let mut sequence: Vec<usize> = vec![];
    for bound in 0..MAX_DEPTH {
        if search_phase1(state, 0, bound, &mut sequence, &pruning_tables) {
            return Some(sequence);
        }
    }
    return None;
}

fn search_phase1(state: &CubieCube, depth: u8, bound: u8, sequence: &mut Vec<usize>, pruning_tables: &Pruning) -> bool {
    if depth == bound {
        if state.is_part_of_g1() {
            println!("to G1: {}", sequence.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" "));
            for bound_phase2 in 0..(MAX_DEPTH - depth) {
                if search_phase2(state, 0, bound_phase2, sequence, &pruning_tables) {
                    return true;
                }
            }
            panic!("error, phase2 didn't found a solution"); // FIXME, find other way
        }
    } else if (bound - depth) >= pruning_tables.flip_pruning_table[state.get_flip_coord()] && (bound - depth) >= pruning_tables.twist_pruning_table[state.get_twist_coord()] && (bound - depth) >= pruning_tables.uds_e_location_pruning_table[state.get_uds_e_location_coord()] {
        for action in ACTIONS.iter() {
            if sequence.last().is_none() || ACTIONS_LIST[*sequence.last().unwrap()].0 != ACTIONS_LIST[*action].0 && ACTIONS_LIST[*sequence.last().unwrap()].0 != ACTIONS_LIST[ACTION_INVERSE[(*action as f32 / 3.0).floor() as usize]].0 { // FIXME, possible optimisations
                sequence.push(action.clone());
                let new_state: CubieCube = state.multiply(&ACTIONS_LIST[*action].0, ACTIONS_LIST[*action].1);
                if search_phase1(&new_state, depth + 1, bound, sequence, &pruning_tables) {
                    return true;
                }
                sequence.pop();
            }
        }
    }
    return false;
}

fn search_phase2(state: &CubieCube, depth: u8, bound: u8, sequence: &mut Vec<usize>, pruning_tables: &Pruning) -> bool {
    if depth == bound {
        if state.is_solved() {
            return true;
        }
    } else if (bound - depth) >= pruning_tables.c_p_pruning_table[state.get_c_p_coord()] && (bound - depth) >= pruning_tables.ud_e_p_pruning_table[state.get_ud_e_p_coord()] && (bound - depth) >= pruning_tables.uds_e_sorted_pruning_table[state.get_uds_e_sorted_coord()] {
        for action in G1_ACTIONS.iter() {
            if sequence.last().is_none() || ACTIONS_LIST[*sequence.last().unwrap()].0 != ACTIONS_LIST[*action].0 && ACTIONS_LIST[*sequence.last().unwrap()].0 != ACTIONS_LIST[ACTION_INVERSE[(*action as f32 / 3.0).floor() as usize]].0 { // FIXME, possible optimisations
                sequence.push(action.clone());
                let new_state: CubieCube = state.multiply(&ACTIONS_LIST[*action].0, ACTIONS_LIST[*action].1);
                if search_phase2(&new_state, depth + 1, bound, sequence, &pruning_tables) {
                    return true;
                }
                sequence.pop();
            }
        }
    }
    return false;
}