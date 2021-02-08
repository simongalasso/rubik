extern crate rubik;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use rubik::cubie_cube::{CubieCube};
// use rubik::coord_cube::{CoordCube};
use super::file_utils::{write_u32_vec, read_u32_vec};

/// Let's get a coffee and try to understand all of those constants :)
/// Let's try with phase1
const N_MOVE: i32 = 18;  // number of possible face moves
const N_TWIST: i32 = 2187;  // 3^7 possible corner orientations in phase 1
const N_FLIP: i32 = 2048;  // 2^11 possible edge orientations in phase 1
const N_SLICE: i32 = 495; // N_PERM_4 // we ignore the permutation of FR, FL, BL, BR in phase 1

#[derive(Debug)]
pub struct Pruning {
    // phase 1 -> test
    pub slice_twist_pruning_table: Vec<u32>,
    pub slice_flip_pruning_table: Vec<u32>,
    // phase 2 -> temporary
    pub phase2: Vec<u32>,
}

impl Pruning {
    pub fn new() -> Pruning {
        return Pruning {
            slice_twist_pruning_table: Self::create_slice_twist(),
            slice_flip_pruning_table: Self::create_slice_flip(),
            phase2: Self::create_phase_2()
        };
    }


    /// PHASE 1 ///
    pub fn create_slice_twist() -> Vec<u32> {
        println!("/// CREATE_PHASE_1_TWIST ///");
        println!("I'm checking wether the file exists or if I have to generate the pruning tables for phase 1");
        let mut slice_twist_pruning_table: Vec<u32>;
        if Path::new("pruning_slice_twist.pr").exists() {
            println!("Pruning tables for phase 1 twist exists!");
            println!("Let's load the variable!");
            slice_twist_pruning_table = read_u32_vec("./pruning_slice_twist.pr");
        } else {
            println!("Pruning tables for phase 1 twist doesn't exists!");
            println!("Creating the file");

            let mut cb_cube: CubieCube = CubieCube::new_solved();
            let mut depth: i32 = 0;
            
            // for i in 0..N_TWIST {
                // for j in 0..6 {
                //     for k in 0..3 {

                //         cornerMultiply(a, &moveCube[j]);
                //         twistMove[i][3 * j + k] = getTwist(a);
                //     }
                //     cornerMultiply(a, &moveCube[j]);// 4. faceturn restores
                // }
            // }

            // for i in 0..(N_SLICE * N_TWIST / 2 + 1) {
            //     println!("i = {}", i);
            // }

            slice_twist_pruning_table = vec![1, 2, 3];
            write_u32_vec("./pruning_slice_twist.pr", &slice_twist_pruning_table);
        }
        return slice_twist_pruning_table;
    }

    pub fn create_slice_flip() -> Vec<u32> {
        println!("/// CREATE_PHASE_1_FLIP ///");
        println!("I'm checking wether the file exists or if I have to generate the pruning tables for phase 1");
        let mut slice_flip_pruning_table: Vec<u32>;
        if Path::new("pruning_slice_flip.pr").exists() {
            println!("Pruning tables for phase 1 flip exists!");
            println!("Let's load the variable!");
            slice_flip_pruning_table = read_u32_vec("./pruning_slice_flip.pr");
        } else {
            println!("Pruning tables for phase 1 flip doesn't exists!");
            println!("Creating the file");
            slice_flip_pruning_table = vec![1, 2, 3];
            write_u32_vec("./pruning_slice_flip.pr", &slice_flip_pruning_table);
        }
        return slice_flip_pruning_table;
    }

    /// PHASE 2 ///
    pub fn create_phase_2() -> Vec<u32> {
        println!("/// CREATE_PHASE_2 ///");
        println!("I'm checking wether the file exists or if I have to generate the pruning tables for phase 2");
        let mut phase2: Vec<u32>;
        if Path::new("pruning_phase2.pr").exists() {
            println!("Pruning tables for phase 2 exists!");
            println!("Let's load the variable!");
            phase2 = read_u32_vec("./pruning_phase2.pr");
        } else {
            println!("Pruning tables for phase 2 doesn't exists!");
            println!("Creating the file");
            phase2 = vec![1, 2, 3];
            write_u32_vec("./pruning_phase2.pr", &phase2);
        }
        return phase2;
    }

}