use std::convert::TryInto;

use super::rubik_state::*;
use super::face::*;
use super::rotation::*;
use super::corner::*;
use super::edge::*;

// U action (is replaced by representation)
const U: RubikState = RubikState {
    c_p: [Corner::UBR, Corner::URF, Corner::UFL, Corner::ULB, Corner::DFR, Corner::DLF, Corner::DBL, Corner::DRB],
    c_o: [0, 0, 0, 0, 0, 0, 0, 0],
    e_p: [Edge::UB, Edge::UR, Edge::UF, Edge::UL, Edge::DR, Edge::DF, Edge::DL, Edge::DB, Edge::FR, Edge::FL, Edge::BL, Edge::BR],
    e_o: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
};

// R action (is replaced by representation)
const R: RubikState = RubikState {
    c_p: [Corner::DFR, Corner::UFL, Corner::ULB, Corner::URF, Corner::DRB, Corner::DLF, Corner::DBL, Corner::UBR],
    c_o: [2, 0, 0, 1, 1, 0, 0, 2],
    e_p: [Edge::FR, Edge::UF, Edge::UL, Edge::UB, Edge::BR, Edge::DF, Edge::DL, Edge::DB, Edge::DR, Edge::FL, Edge::BL, Edge::UR],
    e_o: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
};

// F action (is replaced by representation)
const F: RubikState = RubikState {
    c_p: [Corner::UFL, Corner::DLF, Corner::ULB, Corner::UBR, Corner::URF, Corner::DFR, Corner::DBL, Corner::DRB],
    c_o: [1, 2, 0, 0, 2, 1, 0, 0],
    e_p: [Edge::UR, Edge::FL, Edge::UL, Edge::UB, Edge::DR, Edge::FR, Edge::DL, Edge::DB, Edge::UF, Edge::DF, Edge::BL, Edge::BR],
    e_o: [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0]
};

// D action (is replaced by representation)
const D: RubikState = RubikState {
    c_p: [Corner::URF, Corner::UFL, Corner::ULB, Corner::UBR, Corner::DLF, Corner::DBL, Corner::DRB, Corner::DFR],
    c_o: [0, 0, 0, 0, 0, 0, 0, 0],
    e_p: [Edge::UR, Edge::UF, Edge::UL, Edge::UB, Edge::DF, Edge::DL, Edge::DB, Edge::DR, Edge::FR, Edge::FL, Edge::BL, Edge::BR],
    e_o: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
};

// L action (is replaced by representation)
const L: RubikState = RubikState {
    c_p: [Corner::URF, Corner::ULB, Corner::DBL, Corner::UBR, Corner::DFR, Corner::UFL, Corner::DLF, Corner::DRB],
    c_o: [0, 1, 2, 0, 0, 2, 1, 0],
    e_p: [Edge::UR, Edge::UF, Edge::BL, Edge::UB, Edge::DR, Edge::DF, Edge::FL, Edge::DB, Edge::FR, Edge::UL, Edge::DL, Edge::BR],
    e_o: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
};

// B action (is replaced by representation)
const B: RubikState = RubikState {
    c_p: [Corner::URF, Corner::UFL, Corner::UBR, Corner::DRB, Corner::DFR, Corner::DLF, Corner::ULB, Corner::DBL],
    c_o: [0, 0, 1, 2, 0, 0, 2, 1],
    e_p: [Edge::UR, Edge::UF, Edge::UL, Edge::BR, Edge::DR, Edge::DF, Edge::DL, Edge::BL, Edge::FR, Edge::FL, Edge::UB, Edge::DB],
    e_o: [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1]
};

#[derive(Debug, Clone)]
pub struct Action {
    pub data: RubikState,
    pub rot: Rotation
}

impl Action {
    pub fn new(face: Face, rot: Rotation) -> Action {
        return Action {
            data: match face {
                Face::U => U,
                Face::R => R,
                Face::F => F,
                Face::D => D,
                Face::L => L,
                Face::B => B,
            },
            rot: rot
        }
    }

    pub fn get_actions() -> [Action; 18] {
        return [
            Action::new(Face::U, Rotation::R), Action::new(Face::U, Rotation::L), Action::new(Face::U, Rotation::D),
            Action::new(Face::R, Rotation::R), Action::new(Face::R, Rotation::L), Action::new(Face::R, Rotation::D),
            Action::new(Face::F, Rotation::R), Action::new(Face::F, Rotation::L), Action::new(Face::F, Rotation::D),
            Action::new(Face::D, Rotation::R), Action::new(Face::D, Rotation::L), Action::new(Face::D, Rotation::D),
            Action::new(Face::L, Rotation::R), Action::new(Face::L, Rotation::L), Action::new(Face::L, Rotation::D),
            Action::new(Face::B, Rotation::R), Action::new(Face::B, Rotation::L), Action::new(Face::B, Rotation::D),
        ];
    }

    pub fn apply_to(&self, state: &mut RubikState) {
        println!("action");
        let tmp: RubikState = state.clone();
        for i in 0..8 {
            let index: usize = self.data.c_p[i].clone() as usize;
            state.c_p[i] = tmp.c_p[index].clone();
            state.c_o[i] = (tmp.c_o[index].clone() + self.data.c_o[i]) % 3;
        }
        for i in 0..12 {
            let index: usize = self.data.e_p[i].clone() as usize;
            state.e_p[i] = tmp.e_p[index].clone();
            state.e_o[i] = (tmp.e_o[index].clone() + self.data.e_o[i]) % 2;
        }
    }
}