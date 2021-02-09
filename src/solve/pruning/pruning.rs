extern crate rubik;

use std::path::Path;
use rubik::cubie_cube::{CubieCube};
use rubik::enums::*;

use super::file_utils::{write_u8_vec, read_u8_vec};

/// Let's try with phase1
const N_MOVE: i32 = 18;  // number of possible face moves
const N_TWIST: i32 = 2187;  // 3^7 possible corner orientations in phase 1
const N_FLIP: i32 = 2048;  // 2^11 possible edge orientations in phase 1
const N_SLICE: i32 = 495; // N_PERM_4 // we ignore the permutation of FR, FL, BL, BR in phase 1

#[derive(Debug)]
pub struct Pruning {
    pub twist_pruning_table: Vec<u8>,
    pub flip_pruning_table: Vec<u8>,
    pub uds_e_location_pruning_table: Vec<u8>,
    // pub c_p_coord_pruning_table: Vec<u8>,
    pub phase2: Vec<u8>,
}

impl Pruning {
    pub fn new() -> Pruning {
        return Pruning {
            twist_pruning_table: Self::create_twist(),
            flip_pruning_table: Self::create_flip(),
            uds_e_location_pruning_table: Self::create_uds_e_location(),
            phase2: Self::create_phase_2()
        };
    }

    pub fn create_twist() -> Vec<u8> {
        let mut twist_pruning_table: Vec<u8> = Vec::new();
        if Path::new("pruning_twist.pr").exists() {
            twist_pruning_table = read_u8_vec("./pruning_twist.pr");
        } else {
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            let mut depth: u8 = 0;
            let mut done: i32 = 0;

            for i in 0..N_TWIST {
                twist_pruning_table.push(255);
            }
            twist_pruning_table[0] = 0;
            while done != N_TWIST-1 {
                for i in 0..N_TWIST {
                    if twist_pruning_table[i as usize] == depth {
                        cb_cube.set_twist_coord(i as usize);
                        for action in ACTIONS.iter() {
                            let new_state: CubieCube = cb_cube.multiply(&action.0, action.1);
                            let new_twist = new_state.get_twist_coord();
                            if twist_pruning_table[new_twist as usize] == 255 {
                                twist_pruning_table[new_twist as usize] = depth + 1;
                                done += 1;
                            }
                        }
                    }
                }
                depth += 1;
            }
            write_u8_vec("./pruning_twist.pr", &twist_pruning_table);
        }
        return twist_pruning_table;
    }

    pub fn create_flip() -> Vec<u8> {
        let mut flip_pruning_table: Vec<u8> = Vec::new();
        if Path::new("pruning_flip.pr").exists() {
            flip_pruning_table = read_u8_vec("./pruning_flip.pr");
        } else {
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            let mut depth: u8 = 0;
            let mut done: i32 = 0;

            for i in 0..N_FLIP {
                flip_pruning_table.push(255);
            }
            flip_pruning_table[0] = 0;
            while done != N_FLIP-1 {
                for i in 0..N_FLIP {
                    if flip_pruning_table[i as usize] == depth {
                        cb_cube.set_flip_coord(i as usize);
                        for action in ACTIONS.iter() {
                            let new_state: CubieCube = cb_cube.multiply(&action.0, action.1);
                            let new_flip = new_state.get_flip_coord();
                            if flip_pruning_table[new_flip as usize] == 255 {
                                flip_pruning_table[new_flip as usize] = depth + 1;
                                done += 1;
                            }
                        }
                    }
                }
                depth += 1;
            }
            write_u8_vec("./pruning_flip.pr", &flip_pruning_table);
        }
        return flip_pruning_table;
    }

    pub fn create_uds_e_location() -> Vec<u8> {
        let mut uds_e_location_pruning_table: Vec<u8> = Vec::new();
        if Path::new("pruning_uds_e_location.pr").exists() {
            uds_e_location_pruning_table = read_u8_vec("./pruning_uds_e_location.pr");
        } else {
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            let mut depth: u8 = 0;
            let mut done: i32 = 0;

            for i in 0..N_SLICE {
                uds_e_location_pruning_table.push(255);
            }
            uds_e_location_pruning_table[0] = 0;
            while done != N_SLICE-1 {
                for i in 0..N_SLICE {
                    if uds_e_location_pruning_table[i as usize] == depth {
                        cb_cube.set_uds_e_location_coord(i as usize);
                        for action in ACTIONS.iter() {
                            let new_state: CubieCube = cb_cube.multiply(&action.0, action.1);
                            let new_uds_e_location = new_state.get_uds_e_location_coord();
                            if uds_e_location_pruning_table[new_uds_e_location as usize] == 255 {
                                uds_e_location_pruning_table[new_uds_e_location as usize] = depth + 1;
                                done += 1;
                            }
                        }
                    }
                }
                depth += 1;
            }
            write_u8_vec("./pruning_uds_e_location.pr", &uds_e_location_pruning_table);
        }
        return uds_e_location_pruning_table;
    }
    

    pub fn create_phase_2() -> Vec<u8> {
        let mut phase2: Vec<u8>;
        if Path::new("pruning_phase2.pr").exists() {
            phase2 = read_u8_vec("./pruning_phase2.pr");
        } else {
            phase2 = vec![1, 2, 3];
            write_u8_vec("./pruning_phase2.pr", &phase2)
        }
        return phase2;
    }

}