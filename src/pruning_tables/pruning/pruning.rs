extern crate rubik;

use rubik::cubie_cube::{CubieCube};
use rubik::coord_cube::{CoordCube};

/// Let's get a coffee and try to understand all of those constants :)
const N_PERM_4: i32 = 24;
const N_CHOOSE_8_4: i32 = 70;
const N_MOVE: i32 = 18;  // number of possible face moves

const N_TWIST: i32 = 2187;  // 3^7 possible corner orientations in phase 1
const N_FLIP: i32 = 2048;  // 2^11 possible edge orientations in phase 1
const N_SLICE_SORTED: i32 = 11880;  // 12*11*10*9 possible positions of the FR, FL, BL, BR edges in phase 1
const N_SLICE: i32 = N_SLICE_SORTED; // N_PERM_4 // we ignore the permutation of FR, FL, BL, BR in phase 1
const N_FLIPSLICE_CLASS: i32 = 64430;  // number of equivalence classes for combined flip+slice concerning symmetry group D4h

// const N_U_EDGES_PHASE2: i32 = 1680;  // number of different positions of the edges UR, UF, UL and UB in phase 2
// const N_CORNERS: i32 = 40320;  // 8! corner permutations in phase 2
// const N_CORNERS_CLASS: i32 = 2768;  // number of equivalence classes concerning symmetry group D4h
// const N_UD_EDGES: i32 = 40320;  // 8! permutations of the edges in the U-face and D-face in phase 2

// const N_SYM: i32 = 48;  // number of cube symmetries of full group Oh
// const N_SYM_D4h: i32 = 16;  // Number of symmetries of subgroup D4h

#[derive(Debug)]
pub struct Pruning {
    pub phase1: String,
    pub phase2: String,
}

impl Pruning {
    pub fn new() -> Pruning { 
        return Pruning {
            phase1: Self::create_phase_1(),
            phase2: Self::create_phase_2()
        };
    }

    pub fn create_phase_1() -> String {
        println!("I'm starting to create the pruning tables for the phase 2");
        let mut phase1: String = "phase1".to_string();
        return phase1;
    }

    pub fn create_phase_2() -> String {
        println!("I'm starting to create the pruning tables for the phase 2");
        let mut phase2: String = "phase2".to_string();
        return phase2;
    }

}