use std::path::Path;
use rubik::cubie_cube::{CubieCube};
use rubik::enums::*;

use super::file_utils::{write_u8_vec, read_u8_vec, get_current_path, create_dir};

const N_TWIST: i32 = 2187;  // 3^7 possible corner orientations in phase 1
const N_FLIP: i32 = 2048;  // 2^11 possible edge orientations in phase 1
const N_UDS_E_LOCATION: i32 = 495;
const N_C_P: i32 = 40320;
const N_UD_E_P: i32 = 40320;
const N_UDS_E_SORTED: i32 = 24;

#[derive(Debug)]
pub struct Pruning {
    // phase 1
    pub twist_pruning_table: Vec<u8>,
    pub flip_pruning_table: Vec<u8>,
    pub uds_e_location_pruning_table: Vec<u8>,
    // phase 2
    pub c_p_pruning_table: Vec<u8>,
    pub ud_e_p_pruning_table: Vec<u8>,
    pub uds_e_sorted_pruning_table: Vec<u8>,
}

impl Pruning {
    pub fn new() -> Pruning {
        let path: &str = &format!("{}{}" , get_current_path(), "/pruning-tables");
        create_dir(path);
        return Pruning {
            // phase 1
            twist_pruning_table: Self::create_twist(path),
            flip_pruning_table: Self::create_flip(path),
            uds_e_location_pruning_table: Self::create_uds_e_location(path),
            // phase 2
            c_p_pruning_table: Self::create_c_p(path),
            ud_e_p_pruning_table: Self::create_ud_e_p(path),
            uds_e_sorted_pruning_table: Self::create_uds_e_sorted(path),
        };
    }

    pub fn create_twist(path: &str) -> Vec<u8> {
        let mut twist_pruning_table: Vec<u8> = Vec::new();
        if Path::new(&format!("{}{}" , path, "/pruning_twist.pr")).exists() {
            twist_pruning_table = read_u8_vec(&format!("{}{}" , path, "/pruning_twist.pr"));
        } else {
            println!("[pruning-tables] pruning_twist.pr doesn't exist, creating it...");
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            let mut depth: u8 = 0;
            let mut done: i32 = 0;

            for _ in 0..N_TWIST {
                twist_pruning_table.push(255);
            }
            twist_pruning_table[0] = 0;
            while done != N_TWIST-1 {
                for i in 0..N_TWIST {
                    if twist_pruning_table[i as usize] == depth {
                        cb_cube.set_twist_coord(i as usize);
                        for action in ACTIONS.iter() {
                            let new_state: CubieCube = cb_cube.multiply(&ACTIONS_LIST[*action].0, ACTIONS_LIST[*action].1);
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
            write_u8_vec(&format!("{}{}" , path, "/pruning_twist.pr"), &twist_pruning_table);
            println!("[pruning-tables] pruning_twist.pr created!");
        }
        return twist_pruning_table;
    }

    pub fn create_flip(path: &str) -> Vec<u8> {
        let mut flip_pruning_table: Vec<u8> = Vec::new();
        if Path::new(&format!("{}{}" , path, "/pruning_flip.pr")).exists() {
            flip_pruning_table = read_u8_vec(&format!("{}{}" , path, "/pruning_flip.pr"));
        } else {
            println!("[pruning-tables] pruning_flip.pr doesn't exist, creating it...");
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            let mut depth: u8 = 0;
            let mut done: i32 = 0;

            for _ in 0..N_FLIP {
                flip_pruning_table.push(255);
            }
            flip_pruning_table[0] = 0;
            while done != N_FLIP-1 {
                for i in 0..N_FLIP {
                    if flip_pruning_table[i as usize] == depth {
                        cb_cube.set_flip_coord(i as usize);
                        for action in ACTIONS.iter() {
                            let new_state: CubieCube = cb_cube.multiply(&ACTIONS_LIST[*action].0, ACTIONS_LIST[*action].1);
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
            write_u8_vec(&format!("{}{}" , path, "/pruning_flip.pr"), &flip_pruning_table);
            println!("[pruning-tables] pruning_flip.pr created!");
        }
        return flip_pruning_table;
    }

    pub fn create_uds_e_location(path: &str) -> Vec<u8> {
        let mut uds_e_location_pruning_table: Vec<u8> = Vec::new();
        if Path::new(&format!("{}{}" , path, "/pruning_uds_e_location.pr")).exists() {
            uds_e_location_pruning_table = read_u8_vec(&format!("{}{}" , path, "/pruning_uds_e_location.pr"));
        } else {
            println!("[pruning-tables] pruning_uds_e_location.pr.pr doesn't exist, creating it...");
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            let mut depth: u8 = 0;
            let mut done: i32 = 0;

            for _ in 0..N_UDS_E_LOCATION {
                uds_e_location_pruning_table.push(255);
            }
            uds_e_location_pruning_table[0] = 0;
            while done != N_UDS_E_LOCATION-1 {
                for i in 0..N_UDS_E_LOCATION {
                    if uds_e_location_pruning_table[i as usize] == depth {
                        cb_cube.set_uds_e_location_coord(i as usize);
                        for action in ACTIONS.iter() {
                            let new_state: CubieCube = cb_cube.multiply(&ACTIONS_LIST[*action].0, ACTIONS_LIST[*action].1);
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
            write_u8_vec(&format!("{}{}" , path, "/pruning_uds_e_location.pr"), &uds_e_location_pruning_table);
            println!("[pruning-tables] pruning_uds_e_location.pr created!");
        }
        return uds_e_location_pruning_table;
    }

    pub fn create_c_p(path: &str) -> Vec<u8> {
        let mut c_p_pruning_table: Vec<u8> = Vec::new();
        if Path::new(&format!("{}{}" , path, "/pruning_c_p.pr")).exists() {
            c_p_pruning_table = read_u8_vec(&format!("{}{}" , path, "/pruning_c_p.pr"));
        } else {
            println!("[pruning-tables] pruning_c_p.pr doesn't exist, creating it...");
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            let mut depth: u8 = 0;
            let mut done: i32 = 0;

            for _ in 0..N_C_P {
                c_p_pruning_table.push(255);
            }
            c_p_pruning_table[0] = 0;
            while done != N_C_P-1 {
                for i in 0..N_C_P {
                    if c_p_pruning_table[i as usize] == depth {
                        cb_cube.set_c_p_coord(i as usize);
                        for action in ACTIONS.iter() {
                            let new_state: CubieCube = cb_cube.multiply(&ACTIONS_LIST[*action].0, ACTIONS_LIST[*action].1);
                            let new_c_p = new_state.get_c_p_coord();
                            if c_p_pruning_table[new_c_p as usize] == 255 {
                                c_p_pruning_table[new_c_p as usize] = depth + 1;
                                done += 1;
                            }
                        }
                    }
                }
                depth += 1;
            }
            write_u8_vec(&format!("{}{}" , path, "/pruning_c_p.pr"), &c_p_pruning_table);
            println!("[pruning-tables] pruning_c_p.pr created!");
        }
        return c_p_pruning_table;
    }

    pub fn create_uds_e_sorted(path: &str) -> Vec<u8> {
        let mut uds_e_sorted_pruning_table: Vec<u8> = Vec::new();
        if Path::new(&format!("{}{}" , path, "/pruning_uds_e_sorted.pr")).exists() {
            uds_e_sorted_pruning_table = read_u8_vec(&format!("{}{}" , path, "/pruning_uds_e_sorted.pr"));
        } else {
            println!("[pruning-tables] pruning_uds_e_sorted.pr doesn't exist, creating it...");
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            let mut depth: u8 = 0;
            let mut done: i32 = 0;

            for _ in 0..N_UDS_E_SORTED {
                uds_e_sorted_pruning_table.push(255);
            }
            uds_e_sorted_pruning_table[0] = 0;
            while done != N_UDS_E_SORTED-1 {
                for i in 0..N_UDS_E_SORTED {
                    if uds_e_sorted_pruning_table[i as usize] == depth {
                        cb_cube.set_uds_e_sorted_coord(i as usize);
                        for action in ACTIONS.iter() {
                            let new_state: CubieCube = cb_cube.multiply(&ACTIONS_LIST[*action].0, ACTIONS_LIST[*action].1);
                            if new_state.is_part_of_g1() {
                                let new_uds_e_sorted = new_state.get_uds_e_sorted_coord();
                                if uds_e_sorted_pruning_table[new_uds_e_sorted as usize] == 255 {
                                    uds_e_sorted_pruning_table[new_uds_e_sorted as usize] = depth + 1;
                                    done += 1;
                                }
                            }
                        }
                    }
                }
                depth += 1;
            }
            write_u8_vec(&format!("{}{}" , path, "/pruning_uds_e_sorted.pr"), &uds_e_sorted_pruning_table);
            println!("[pruning-tables] pruning_uds_e_sorted.pr created!");
        }
        return uds_e_sorted_pruning_table;
    }
    
    pub fn create_ud_e_p(path: &str) -> Vec<u8> {
        let mut ud_e_p_pruning_table: Vec<u8> = Vec::new();
        if Path::new(&format!("{}{}" , path, "/pruning_ud_e_p.pr")).exists() {
            ud_e_p_pruning_table = read_u8_vec(&format!("{}{}" , path, "/pruning_ud_e_p.pr"));
        } else {
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            let mut depth: u8 = 0;
            let mut done: i32 = 0;
            let mut uds_e_sorted_pruning_table: Vec<u8> = Vec::new();
            for _ in 0..N_UDS_E_SORTED {
                uds_e_sorted_pruning_table.push(255);
            }
            uds_e_sorted_pruning_table[0] = 0;
            while done != N_UDS_E_SORTED-1 {
                for i in 0..N_UDS_E_SORTED {
                    if uds_e_sorted_pruning_table[i as usize] == depth {
                        cb_cube.set_uds_e_sorted_coord(i as usize);
                        for action in ACTIONS.iter() {
                            let new_state: CubieCube = cb_cube.multiply(&ACTIONS_LIST[*action].0, ACTIONS_LIST[*action].1);
                            if new_state.is_part_of_g1() {
                                let new_uds_e_sorted = new_state.get_uds_e_sorted_coord();
                                if uds_e_sorted_pruning_table[new_uds_e_sorted as usize] == 255 {
                                    uds_e_sorted_pruning_table[new_uds_e_sorted as usize] = depth + 1;
                                    done += 1;
                                }
                            }
                        }
                    }
                }
                depth += 1;
            }
            println!("[pruning-tables] pruning_ud_e_p.pr doesn't exist, creating it...");
            for _ in 0..N_UD_E_P * N_UDS_E_SORTED {
                ud_e_p_pruning_table.push(255);
            }
            for ud in 0..N_UDS_E_SORTED {
                let mut cb_cube: CubieCube = CubieCube::new_solved();
                cb_cube.set_uds_e_sorted_coord(ud as usize);

                let mut depth: u8 = uds_e_sorted_pruning_table[ud as usize];
                let mut done: i32 = 0;
                ud_e_p_pruning_table[ud as usize * (N_UD_E_P as usize)] = uds_e_sorted_pruning_table[ud as usize];
                while done != N_UD_E_P-1 {
                    for i in 0..N_UD_E_P {
                        if ud_e_p_pruning_table[ud as usize * N_UD_E_P as usize + i as usize] == depth {
                            cb_cube.set_ud_e_p_coord(i as usize);
                            for action in ACTIONS.iter() {
                                let new_state: CubieCube = cb_cube.multiply(&ACTIONS_LIST[*action].0, ACTIONS_LIST[*action].1);
                                if new_state.is_part_of_g1() {
                                    let new_ud_e_p = new_state.get_ud_e_p_coord();
                                    if ud_e_p_pruning_table[ud as usize * N_UD_E_P as usize + new_ud_e_p as usize] == 255 {
                                        ud_e_p_pruning_table[ud as usize * N_UD_E_P as usize + new_ud_e_p as usize] = depth + 1 as u8;
                                        done += 1;
                                    }
                                }
                            }
                        }
                    }
                    depth += 1;
                }
            }
            write_u8_vec(&format!("{}{}" , path, "/pruning_ud_e_p.pr"), &ud_e_p_pruning_table);
            println!("[pruning-tables] pruning_ud_e_p.pr created!");
        }
        return ud_e_p_pruning_table;
    }

}