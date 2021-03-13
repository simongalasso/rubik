use std::io::{self, Write};

use pruning::moves::{Moves};
use pruning::pruning::{Pruning};
use rubik::cubie_cube::{CubieCube};
use rubik::enums::*;

const MAX_DEPTH: u8 = 24;

#[derive(Clone)]
struct CoordState {
    twist: usize,
    flip: usize,
    uds_e_l: usize,
    c_p: usize,
    ud_e_p: usize,
    uds_e_s: usize
}

impl CoordState {
    fn from_cb_cube_p1(state: &CubieCube) -> CoordState {
        return CoordState {
            twist: state.get_twist_coord(),
            flip: state.get_flip_coord(),
            uds_e_l: state.get_uds_e_location_coord(),
            c_p: 1, ud_e_p: 1, uds_e_s: 1
        }
    }
}

// this version stops at the first solution found
pub fn solve(state: &mut CubieCube, ptables: &Pruning, moves_tables: &Moves) -> Option<Vec<usize>> {
    let coord_state: CoordState = CoordState::from_cb_cube_p1(state);
    let mut sequence: Vec<usize> = vec![];
    print!("G1: ");
    io::stdout().flush().unwrap();
    for bound in 0..MAX_DEPTH {
        if search_phase1(&coord_state, 0, bound, &mut sequence, ptables, moves_tables) {
            println!("");
            return Some(sequence);
        }
    }
    println!("");
    return None;
}

fn search_phase1(coord_state: &CoordState, depth: u8, bound: u8, sequence: &mut Vec<usize>, ptables: &Pruning, mtables: &Moves) -> bool {
    // println!("P1 - [{}] = [{}]", depth, bound);
    if depth == bound {
        // println!("depth == bound");
        // print!("[{}]", depth);
        // io::stdout().flush().unwrap();
        if coord_state.twist == 0 && coord_state.flip == 0 && coord_state.uds_e_l == 0 /*&& !G1_ACTIONS.contains(sequence.last().unwrap())*/ {
            println!("to G1: {}", sequence.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" "));
            // io::stdout().flush().unwrap();
            for bound_phase2 in 0..(MAX_DEPTH - depth) {
                if search_phase2(coord_state, 0, bound_phase2, sequence, ptables, mtables) {
                    return true;
                }
            }
        }
    } else if
        (bound - depth) >= ptables.twist_pruning_table[coord_state.twist]
        &&
        (bound - depth) >= ptables.flip_pruning_table[coord_state.flip]
        &&
        (bound - depth) >= ptables.uds_e_location_pruning_table[coord_state.uds_e_l]
    {
        // println!("P1 ptables passed");
        for action in ACTIONS.iter() {
            if sequence.last().is_none() || (
                ACTIONS_LIST[*sequence.last().unwrap()].0 != ACTIONS_LIST[*action].0
                &&
                ACTIONS_LIST[*sequence.last().unwrap()].0 != ACTIONS_LIST[ACTION_INVERSE[(*action as f32 / 3.0).floor() as usize]].0)
            {
                sequence.push(action.clone());
                let mut new_coord_state: CoordState = coord_state.clone();
                new_coord_state.twist = mtables.twist_moves[18 * coord_state.twist + 3 * (action / 3) + ACTIONS_LIST[*action].1 as usize - 1] as usize;
                new_coord_state.flip = mtables.flip_moves[18 * coord_state.flip + 3 * (action / 3) + ACTIONS_LIST[*action].1 as usize - 1] as usize;
                new_coord_state.uds_e_l = mtables.uds_e_location_moves[18 * coord_state.uds_e_l + 3 * (action / 3) + ACTIONS_LIST[*action].1 as usize - 1] as usize;
                if search_phase1(&new_coord_state, depth + 1, bound, sequence, ptables, mtables) {
                    return true;
                }
                sequence.pop();
            }
        }
    }
    return false;
}

fn search_phase2(coord_state: &CoordState, depth: u8, bound: u8, sequence: &mut Vec<usize>, ptables: &Pruning, mtables: &Moves) -> bool {
    // println!("P2 - [{}] = [{}]", depth, bound);
    if depth == bound {
        if coord_state.c_p == 0 && coord_state.ud_e_p == 0 && coord_state.uds_e_s == 0 {
            return true;
        }
    } else if
        (bound - depth) >= ptables.c_p_pruning_table[coord_state.c_p]
        &&
        (bound - depth) >= ptables.ud_e_p_pruning_table[coord_state.ud_e_p]
        &&
        (bound - depth) >= ptables.uds_e_sorted_pruning_table[coord_state.uds_e_s]
    {
        // println!("P2 ptables passed");
        for action in G1_ACTIONS.iter() {
            if sequence.last().is_none() || (
                ACTIONS_LIST[*sequence.last().unwrap()].0 != ACTIONS_LIST[*action].0
                &&
                ACTIONS_LIST[*sequence.last().unwrap()].0 != ACTIONS_LIST[ACTION_INVERSE[(*action as f32 / 3.0).floor() as usize]].0)
            {
                sequence.push(action.clone());
                let mut new_coord_state: CoordState = coord_state.clone();
                new_coord_state.c_p = mtables.c_p_moves[18 * coord_state.c_p + 3 * (action / 3) + ACTIONS_LIST[*action].1 as usize - 1] as usize;
                new_coord_state.ud_e_p = mtables.ud_e_p_moves[18 * coord_state.ud_e_p + 3 * (action / 3) + ACTIONS_LIST[*action].1 as usize - 1] as usize;
                new_coord_state.uds_e_s = mtables.uds_e_sorted_moves[18 * coord_state.uds_e_s + 3 * (action / 3) + ACTIONS_LIST[*action].1 as usize - 1] as usize;
                if search_phase2(&new_coord_state, depth + 1, bound, sequence, ptables, mtables) {
                    return true;
                }
                sequence.pop();
            }
        }
    }
    return false;
}