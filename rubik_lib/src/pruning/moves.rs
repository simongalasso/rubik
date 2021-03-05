use std::path::Path;
use rubik::cubie_cube::{CubieCube};
use rubik::enums::*;

use super::file_utils::{write_u32_vec, read_u32_vec, get_current_path, create_dir};

const N_TWIST: i32 = 2187;  // 3^7 possible corner orientations in phase 1
const N_FLIP: i32 = 2048;  // 2^11 possible edge orientations in phase 1
const N_SLICE_SORTED: i32 = 11880;  // 12*11*10*9 possible positions of the FR, FL, BL, BR edges in phase 1
const N_UD_EDGES: i32 = 40320;  // 8! permutations of the edges in the U-face and D-face in phase 2
const N_CORNERS: i32 = 40320;  // # 8! corner permutations in phase 2

const N_MOVE: i32 = 18;

#[derive(Debug)]
pub struct Moves {
    // moves
    pub twist_moves: Vec<u32>,
    pub flip_moves: Vec<u32>,
    pub slice_sorted_moves: Vec<u32>,
    // pub u_edges_moves: Vec<u32>,
    // pub d_edges_moves: Vec<u32>,
    pub ud_edges_moves: Vec<u32>,
    pub corners_moves: Vec<u32>,
    // pub parity_moves: Vec<u32>,
}

impl Moves {
    pub fn new() -> Moves {
        let path: &str = &format!("{}{}" , get_current_path(), "/moves-tables");
        create_dir(path);
        return Moves {
            // moves
            twist_moves: Self::create_twist_moves(path),
            flip_moves: Self::create_flip_moves(path),
            slice_sorted_moves: Self::create_slice_sorted_moves(path),
            // u_edges_moves: Self::create_u_edges_moves(path),
            // d_edges_moves: Self::create_d_edges_moves(path),
            ud_edges_moves: Self::create_ud_edges_moves(path),
            corners_moves: Self::create_corners_moves(path),
            // parity_moves: Self::create_parity_moves(path),
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
                for j in 0..6 {
                    for k in 0..3 {
                        cb_cube.corner_multiply(&BASIC_ACTIONS_LIST[j].0);
                        twist_moves[N_MOVE as usize * i as usize + 3 * j + k] = cb_cube.get_twist_coord() as u32;
                    }
                    cb_cube.corner_multiply(&BASIC_ACTIONS_LIST[j].0);
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
                        cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j].0);
                        flip_moves[N_MOVE as usize * i as usize + 3 * j + k] = cb_cube.get_flip_coord() as u32;
                    }
                    cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j].0);
                }
            }
            write_u32_vec(&format!("{}{}" , path, "/flip_moves.pr"), &flip_moves);
            println!("[moves-tables] flip_moves.pr created!");
        }
        return flip_moves;
    }

    pub fn create_slice_sorted_moves(path: &str) -> Vec<u32> {
        let mut slice_sorted_moves: Vec<u32> = Vec::new();
        if Path::new(&format!("{}{}" , path, "/slice_sorted_moves.pr")).exists() {
            slice_sorted_moves = read_u32_vec(&format!("{}{}" , path, "/slice_sorted_moves.pr"));
        } else {
            println!("[moves-tables] slice_sorted_moves.pr doesn't exist, creating it...");
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            for _ in 0..(N_SLICE_SORTED * N_MOVE) {
                slice_sorted_moves.push(0);
            }
            for i in 0..N_SLICE_SORTED {
                cb_cube.set_uds_e_sorted_coord(i as usize);
                for j in 0..6 {
                    for k in 0..3 {
                        cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j].0);
                        slice_sorted_moves[N_MOVE as usize * i as usize + 3 * j + k] = cb_cube.get_uds_e_sorted_coord() as u32;
                    }
                    cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j].0);
                }
            }
            write_u32_vec(&format!("{}{}" , path, "/slice_sorted_moves.pr"), &slice_sorted_moves);
            println!("[moves-tables] slice_sorted_moves.pr created!");
        }
        return slice_sorted_moves;
    }

    //  pub fn create_u_edges_moves(path: &str) -> Vec<u32> {
    //     let mut u_edges_moves: Vec<u32> = Vec::new();
    //     if Path::new(&format!("{}{}" , path, "/u_edges_moves.pr")).exists() {
    //         u_edges_moves = read_u32_vec(&format!("{}{}" , path, "/u_edges_moves.pr"));
    //     } else {
    //         println!("[moves-tables] u_edges_moves.pr doesn't exist, creating it...");
    //         let mut cb_cube: CubieCube = CubieCube::new_solved();
    //         for _ in 0..(N_SLICE_SORTED * N_MOVE) {
    //             u_edges_moves.push(0);
    //         }
    //         for i in 0..N_SLICE_SORTED {
    //             cb_cube.set(i as usize);
    //             for j in 0..6 {
    //                 for k in 0..3 {
    //                     cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j].0);
    //                     u_edges_moves[N_MOVE as usize * i as usize + 3 * j + k] = cb_cube.get_uds_e_sorted_coord() as u32;
    //                 }
    //                 cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j].0);
    //             }
    //         }
    //         write_u32_vec(&format!("{}{}" , path, "/u_edges_moves.pr"), &u_edges_moves);
    //         println!("[moves-tables] u_edges_moves.pr created!");
    //     }
    //     return u_edges_moves;
    // }

    // pub fn create_d_edges_moves(path: &str) -> Vec<u32> {
    //     let mut u_edges_moves: Vec<u32> = Vec::new();
    //     if Path::new(&format!("{}{}" , path, "/u_edges_moves.pr")).exists() {
    //         u_edges_moves = read_u32_vec(&format!("{}{}" , path, "/u_edges_moves.pr"));
    //     } else {
    //         println!("[moves-tables] u_edges_moves.pr doesn't exist, creating it...");
    //         let mut cb_cube: CubieCube = CubieCube::new_solved();
    //         for _ in 0..(N_SLICE_SORTED * N_MOVE) {
    //             u_edges_moves.push(0);
    //         }
    //         for i in 0..N_SLICE_SORTED {
    //             cb_cube.set(i as usize);
    //             for j in 0..6 {
    //                 for k in 0..3 {
    //                     cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j].0);
    //                     u_edges_moves[N_MOVE as usize * i as usize + 3 * j + k] = cb_cube.get_uds_e_sorted_coord() as u32;
    //                 }
    //                 cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j].0);
    //             }
    //         }
    //         write_u32_vec(&format!("{}{}" , path, "/u_edges_moves.pr"), &u_edges_moves);
    //         println!("[moves-tables] u_edges_moves.pr created!");
    //     }
    //     return u_edges_moves;
    // }

    pub fn create_ud_edges_moves(path: &str) -> Vec<u32> {
        let mut ud_edges_moves: Vec<u32> = Vec::new();
        if Path::new(&format!("{}{}" , path, "/ud_edges_moves.pr")).exists() {
            ud_edges_moves = read_u32_vec(&format!("{}{}" , path, "/ud_edges_moves.pr"));
        } else {
            println!("[moves-tables] ud_edges_moves.pr doesn't exist, creating it...");
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            for _ in 0..(N_UD_EDGES * N_MOVE) {
                ud_edges_moves.push(0);
            }
            for i in 0..N_UD_EDGES {
                cb_cube.set_ud_e_p_coord(i as usize);
                for j in 0..6 {
                    for k in 0..3 {
                        cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j].0);
                        if j != 0 && j != 3 && k == 1 {
                            ud_edges_moves[N_MOVE as usize * i as usize + 3 * j + k] = cb_cube.get_ud_e_p_coord() as u32;
                        }
                    }
                    cb_cube.edge_multiply(&BASIC_ACTIONS_LIST[j].0);
                }
            }
            write_u32_vec(&format!("{}{}" , path, "/ud_edges_moves.pr"), &ud_edges_moves);
            println!("[moves-tables] ud_edges_moves.pr created!");
        }
        return ud_edges_moves;
    }

     pub fn create_corners_moves(path: &str) -> Vec<u32> {
        let mut corners_moves: Vec<u32> = Vec::new();
        if Path::new(&format!("{}{}" , path, "/corners_moves.pr")).exists() {
            corners_moves = read_u32_vec(&format!("{}{}" , path, "/corners_moves.pr"));
        } else {
            println!("[moves-tables] corners_moves.pr doesn't exist, creating it...");
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            for _ in 0..(N_CORNERS * N_MOVE) {
                corners_moves.push(0);
            }
            for i in 0..N_CORNERS {
                cb_cube.set_c_p_coord(i as usize);
                for j in 0..6 {
                    for k in 0..3 {
                        cb_cube.corner_multiply(&BASIC_ACTIONS_LIST[j].0);
                        corners_moves[N_MOVE as usize * i as usize + 3 * j + k] = cb_cube.get_c_p_coord() as u32;
                    }
                    cb_cube.corner_multiply(&BASIC_ACTIONS_LIST[j].0);
                }
            }
            write_u32_vec(&format!("{}{}" , path, "/corners_moves.pr"), &corners_moves);
            println!("[moves-tables] corners_moves.pr created!");
        }
        return corners_moves;
    }
}