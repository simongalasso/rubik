use pruning::moves::{Moves};
use pruning::pruning::{Pruning};
use rubik::cubie_cube::{CubieCube};
use rubik::enums::*;

// use std::io::{self, Write};

const MAX_P1_DEPTH: u8 = 10;
const MAX_P2_DEPTH: u8 = 10;

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
    fn from_cb_cube_p1(state: &CubieCube) -> Self {
        return Self {
            twist: state.get_twist_coord(),
            flip: state.get_flip_coord(),
            uds_e_l: state.get_uds_e_location_coord(),
            c_p: 0, ud_e_p: 0, uds_e_s: 0
        }
    }
}

// this version stops at the first solution found
pub fn solve(state: &CubieCube, ptables: &Pruning, moves_tables: &Moves, start_time: std::time::Instant) -> Result<Vec<usize>, String> {
    let mut max_p1_depth: u8 = MAX_P1_DEPTH;
    let mut max_p2_depth: u8 = MAX_P2_DEPTH;
    loop /* && set a timeout condition */ {
        let coord_state: CoordState = CoordState::from_cb_cube_p1(state);
        let mut sequence: Vec<usize> = vec![];
        let mut bound: u8 = ptables.twist_pruning_table[coord_state.twist].max(ptables.flip_pruning_table[coord_state.flip]).max(ptables.uds_e_location_pruning_table[coord_state.uds_e_l]);
        while bound < max_p1_depth {
            match search_phase1(&coord_state, 0, bound, &mut sequence, ptables, moves_tables, &mut state.clone(), max_p2_depth, start_time) {
                None => return Ok(sequence),
                Some(cost) => {
                    bound = cost;
                }
            }
        }
        if max_p1_depth < 12 {
            max_p1_depth += 1;
        }
        if max_p2_depth < 18 {
            max_p2_depth += 1;
        }
        // println!("- - - - - - -\n# new max depths ({}) ({})", max_p1_depth, max_p2_depth);
    }
    // return Err(String::from("Search timed out without finding any solution"));
}

fn search_phase1(coord_state: &CoordState, depth: u8, bound: u8, sequence: &mut Vec<usize>, ptables: &Pruning, mtables: &Moves, state: &mut CubieCube, max_p2_depth: u8, start_time: std::time::Instant) -> Option<u8> {
    let cost = depth + ptables.twist_pruning_table[coord_state.twist].max(ptables.flip_pruning_table[coord_state.flip]).max(ptables.uds_e_location_pruning_table[coord_state.uds_e_l]);
    if cost > bound {
        return Some(cost);
    }
    if coord_state.twist == 0 && coord_state.flip == 0 && coord_state.uds_e_l == 0 {
        let mut cb_cube: CubieCube = state.clone();
        cb_cube.apply_sequence(&sequence);
        let mut new_coord_state: CoordState = coord_state.clone();
        new_coord_state.c_p = cb_cube.get_c_p_coord();
        new_coord_state.ud_e_p = cb_cube.get_ud_e_p_coord();
        new_coord_state.uds_e_s = cb_cube.get_uds_e_sorted_coord();
        let mut bound_phase2: u8 = ptables.c_p_pruning_table[coord_state.c_p].max(ptables.ud_e_p_pruning_table[coord_state.ud_e_p]).max(ptables.uds_e_sorted_pruning_table[coord_state.uds_e_s]);
        while bound_phase2 < max_p2_depth {
            // print!("[{}]", bound_phase2);
            // io::stdout().flush().expect("error: can't flush stdout");
            match search_phase2(&new_coord_state, 0, bound_phase2, sequence, ptables, mtables) {
                None => {
                    // println!();
                    return None
                },
                Some(cost) => {
                    bound_phase2 = cost;
                    // io::stdout().flush().expect("error: can't flush stdout");
                }
            }
        }
        // println!();
    }
    let mut min: u8 = std::u8::MAX;
    for action in ACTIONS.iter() {
        if sequence.last().is_none() || (
            ACTIONS_LIST[*sequence.last().unwrap()].0 != ACTIONS_LIST[*action].0
            &&
            ACTIONS_LIST[*sequence.last().unwrap()].0 != ACTIONS_LIST[ACTION_INVERSE[(*action as f32 / 3.0).floor() as usize]].0)
        {
            sequence.push(action.clone());
            let mut new_coord_state: CoordState = coord_state.clone();
            new_coord_state.twist = mtables.twist_moves[18 * coord_state.twist + 3 * (action / 3) + (ACTIONS_LIST[*action].1 as usize - 1)] as usize;
            new_coord_state.flip = mtables.flip_moves[18 * coord_state.flip + 3 * (action / 3) + (ACTIONS_LIST[*action].1 as usize - 1)] as usize;
            new_coord_state.uds_e_l = mtables.uds_e_location_moves[18 * coord_state.uds_e_l + 3 * (action / 3) + (ACTIONS_LIST[*action].1 as usize - 1)] as usize;
            match search_phase1(&new_coord_state, depth + 1, bound, sequence, ptables, mtables, state, max_p2_depth, start_time) {
                None => { return None },
                Some(cost_ret) => if cost_ret < min {
                    min = cost_ret
                },
            }
            sequence.pop();
        }
    }
    return Some(min);
}

fn search_phase2(coord_state: &CoordState, depth: u8, bound: u8, sequence: &mut Vec<usize>, ptables: &Pruning, mtables: &Moves) -> Option<u8> {
    let cost = depth + ptables.c_p_pruning_table[coord_state.c_p].max(ptables.ud_e_p_pruning_table[coord_state.ud_e_p]).max(ptables.uds_e_sorted_pruning_table[coord_state.uds_e_s]);
    if cost > bound {
        return Some(cost);
    }
    if coord_state.c_p == 0 && coord_state.ud_e_p == 0 && coord_state.uds_e_s == 0 {
        return None;
    }
    let mut min: u8 = std::u8::MAX;
    for action in G1_ACTIONS.iter() {
        if sequence.last().is_none() || (
            ACTIONS_LIST[*sequence.last().unwrap()].0 != ACTIONS_LIST[*action].0
            &&
            ACTIONS_LIST[*sequence.last().unwrap()].0 != ACTIONS_LIST[ACTION_INVERSE[(*action as f32 / 3.0).floor() as usize]].0)
        {
            sequence.push(action.clone());
            let mut new_coord_state: CoordState = coord_state.clone();
            new_coord_state.c_p = mtables.c_p_moves[18 * coord_state.c_p + 3 * (action / 3) + (ACTIONS_LIST[*action].1 as usize - 1)] as usize;
            new_coord_state.ud_e_p = mtables.ud_e_p_moves[18 * coord_state.ud_e_p + 3 * (action / 3) + (ACTIONS_LIST[*action].1 as usize - 1)] as usize;
            new_coord_state.uds_e_s = mtables.uds_e_sorted_moves[18 * coord_state.uds_e_s + 3 * (action / 3) + (ACTIONS_LIST[*action].1 as usize - 1)] as usize;
            match search_phase2(&new_coord_state, depth + 1, bound, sequence, ptables, mtables) {
                None => { return None },
                Some(cost_ret) => if cost_ret < min {
                    min = cost_ret
                },
            }
            sequence.pop();
        }
    }
    return Some(min);
}