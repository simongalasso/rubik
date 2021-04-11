use std::path::Path;
use rubik::cubie_cube::{CubieCube};
use rubik::enums::*;

use super::file_utils::{write_u32_vec, read_u32_vec, get_current_path, create_dir};

const N_TWIST: i32 = 2187;
const N_FLIP: i32 = 2048;
const N_SLICE_SORTED: i32 = 11880;
const N_UD_EDGES: i32 = 40320;
const N_CORNERS: i32 = 40320;
const N_UDS_E_LOCATION: i32 = 495;
const N_MOVE: i32 = 18;


#[derive(Debug)]
pub struct Moves {
    pub twist_moves: Vec<u32>,
    pub flip_moves: Vec<u32>,
    pub uds_e_location_moves: Vec<u32>,
    pub c_p_moves: Vec<u32>,
    pub ud_e_p_moves: Vec<u32>,
    pub uds_e_sorted_moves: Vec<u32>,
}

impl Moves {
    pub fn new() -> Self {
        let path: &str = &format!("{}{}" , get_current_path(), "/moves-tables");
        create_dir(path);
        return Self {
            twist_moves: Self::create_twist_moves(path),
            flip_moves: Self::create_flip_moves(path),
            uds_e_sorted_moves: Self::create_uds_e_sorted_moves(path),
            ud_e_p_moves: Self::create_ud_e_p_moves(path),
            uds_e_location_moves: Self::create_uds_e_location_moves(path),
            c_p_moves: Self::create_c_p_moves(path),
        };
    }

    pub fn create_twist_moves(path: &str) -> Vec<u32> {
        let mut twist_moves: Vec<u32> = Vec::new();
        if Path::new(&format!("{}{}" , path, "/twist_moves.pr")).exists() {
            twist_moves = read_u32_vec(&format!("{}{}" , path, "/twist_moves.pr"));
        } else {
            println!("[moves-tables] twist_moves.pr doesn't exist, creating it...");
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            for _ in 0..(N_TWIST * N_MOVE) {
                twist_moves.push(0);
            }
            for i in 0..N_TWIST {
                cb_cube.set_twist_coord(i as usize);
                for j in 0..6 /* 6 faces */ {
                    for k in 0..3 /* 3 moves for each face */ {
                        cb_cube.corner_multiply(&BASIC_ACTIONS_LIST[j]);
                        twist_moves[N_MOVE as usize * i as usize + 3 * j + k] = cb_cube.get_twist_coord() as u32;
                    }
                    cb_cube.corner_multiply(&BASIC_ACTIONS_LIST[j]);
                }
            }
            write_u32_vec(&format!("{}{}" , path, "/twist_moves.pr"), &twist_moves);
            println!("[moves-tables] twist_moves.pr created!");
        }
        return twist_moves;
    }

    pub fn create_flip_moves(path: &str) -> Vec<u32> {
        let mut flip_moves: Vec<u32> = Vec::new();
        if Path::new(&format!("{}{}" , path, "/flip_moves.pr")).exists() {
            flip_moves = read_u32_vec(&format!("{}{}" , path, "/flip_moves.pr"));
        } else {
            println!("[moves-tables] flip_moves.pr doesn't exist, creating it...");
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            for _ in 0..(N_FLIP * N_MOVE) {
                flip_moves.push(0);
            }
            for i in 0..N_FLIP {
                cb_cube.set_flip_coord(i as usize);
                for j in 0..6 {
                    for k in 0..3 {
                        cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j]);
                        flip_moves[N_MOVE as usize * i as usize + 3 * j + k] = cb_cube.get_flip_coord() as u32;
                    }
                    cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j]);
                }
            }
            write_u32_vec(&format!("{}{}" , path, "/flip_moves.pr"), &flip_moves);
            println!("[moves-tables] flip_moves.pr created!");
        }
        return flip_moves;
    }

    pub fn create_uds_e_sorted_moves(path: &str) -> Vec<u32> {
        let mut uds_e_sorted_moves: Vec<u32> = Vec::new();
        if Path::new(&format!("{}{}" , path, "/uds_e_sorted_moves.pr")).exists() {
            uds_e_sorted_moves = read_u32_vec(&format!("{}{}" , path, "/uds_e_sorted_moves.pr"));
        } else {
            println!("[moves-tables] uds_e_sorted_moves.pr doesn't exist, creating it...");
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            for _ in 0..(N_SLICE_SORTED * N_MOVE) {
                uds_e_sorted_moves.push(0);
            }
            for i in 0..N_SLICE_SORTED {
                cb_cube.set_uds_e_sorted_coord(i as usize);
                for j in 0..6 {
                    for k in 0..3 {
                        cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j]);
                        uds_e_sorted_moves[N_MOVE as usize * i as usize + 3 * j + k] = cb_cube.get_uds_e_sorted_coord() as u32;
                    }
                    cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j]);
                }
            }
            write_u32_vec(&format!("{}{}" , path, "/uds_e_sorted_moves.pr"), &uds_e_sorted_moves);
            println!("[moves-tables] uds_e_sorted_moves.pr created!");
        }
        return uds_e_sorted_moves;
    }

    pub fn create_ud_e_p_moves(path: &str) -> Vec<u32> {
        let mut ud_e_p_moves: Vec<u32> = Vec::new();
        if Path::new(&format!("{}{}" , path, "/ud_e_p_moves.pr")).exists() {
            ud_e_p_moves = read_u32_vec(&format!("{}{}" , path, "/ud_e_p_moves.pr"));
        } else {
            println!("[moves-tables] ud_e_p_moves.pr doesn't exist, creating it...");
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            for _ in 0..(N_UD_EDGES * N_MOVE) {
                ud_e_p_moves.push(0);
            }
            for i in 0..N_UD_EDGES {
                cb_cube.set_ud_e_p_coord(i as usize);
                for j in 0..6 {
                    for k in 0..3 {
                        cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j]);
                        if j == 1 && k != 1 || j == 2 && k != 1 || j == 4 && k != 1 || j == 5 && k != 1 {
                            continue;
                        }
                        ud_e_p_moves[N_MOVE as usize * i as usize + 3 * j + k] = cb_cube.get_ud_e_p_coord() as u32;
                    }
                    cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j]);
                }
            }
            write_u32_vec(&format!("{}{}" , path, "/ud_e_p_moves.pr"), &ud_e_p_moves);
            println!("[moves-tables] ud_e_p_moves.pr created!");
        }
        return ud_e_p_moves;
    }

    pub fn create_uds_e_location_moves(path: &str) -> Vec<u32> {
        let mut uds_e_location_moves: Vec<u32> = Vec::new();
        if Path::new(&format!("{}{}" , path, "/uds_e_location_moves.pr")).exists() {
            uds_e_location_moves = read_u32_vec(&format!("{}{}" , path, "/uds_e_location_moves.pr"));
        } else {
            println!("[moves-tables] uds_e_location_moves.pr doesn't exist, creating it...");
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            for _ in 0..(N_UDS_E_LOCATION * N_MOVE) {
                uds_e_location_moves.push(0);
            }
            for i in 0..N_UDS_E_LOCATION {
                cb_cube.set_uds_e_location_coord(i as usize);
                for j in 0..6 {
                    for k in 0..3 {
                        cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j]);
                        uds_e_location_moves[N_MOVE as usize * i as usize + 3 * j + k] = cb_cube.get_uds_e_location_coord() as u32;
                    }
                    cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j]);
                }
            }
            write_u32_vec(&format!("{}{}" , path, "/uds_e_location_moves.pr"), &uds_e_location_moves);
            println!("[moves-tables] uds_e_location_moves.pr created!");
        }
        return uds_e_location_moves;
    }
    
    pub fn create_c_p_moves(path: &str) -> Vec<u32> {
        let mut c_p_moves: Vec<u32> = Vec::new();
        if Path::new(&format!("{}{}" , path, "/c_p_moves.pr")).exists() {
            c_p_moves = read_u32_vec(&format!("{}{}" , path, "/c_p_moves.pr"));
        } else {
            println!("[moves-tables] c_p_moves.pr doesn't exist, creating it...");
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            for _ in 0..(N_CORNERS * N_MOVE) {
                c_p_moves.push(0);
            }
            for i in 0..N_CORNERS {
                cb_cube.set_c_p_coord(i as usize);
                for j in 0..6 {
                    for k in 0..3 {
                        cb_cube.corner_multiply(&BASIC_ACTIONS_LIST[j]);
                        c_p_moves[N_MOVE as usize * i as usize + 3 * j + k] = cb_cube.get_c_p_coord() as u32;
                    }
                    cb_cube.corner_multiply(&BASIC_ACTIONS_LIST[j]);
                }
            }
            write_u32_vec(&format!("{}{}" , path, "/c_p_moves.pr"), &c_p_moves);
            println!("[moves-tables] c_p_moves.pr created!");
        }
        return c_p_moves;
    }

}