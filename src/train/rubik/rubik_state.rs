extern crate rand;

use rand::Rng;

use super::action::*;
use super::face::*;
use super::rotation::*;
use super::corner::*;
use super::edge::*;

pub const SOLVED_STATE: RubikState = RubikState {
    c_p: [Corner::URF, Corner::UFL, Corner::ULB, Corner::UBR, Corner::DFR, Corner::DLF, Corner::DBL, Corner::DRB],
    c_o: [0, 0, 0, 0, 0, 0, 0, 0],
    e_p: [Edge::UR, Edge::UF, Edge::UL, Edge::UB, Edge::DR, Edge::DF, Edge::DL, Edge::DB, Edge::FR, Edge::FL, Edge::BL, Edge::BR],
    e_o: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
};

#[derive(Debug, Clone, PartialEq)]
pub struct RubikState {
    pub c_p: [Corner; 8],
    pub c_o: [u8; 8],
    pub e_p: [Edge; 12],
    pub e_o: [u8; 12]
}

impl RubikState {
    pub fn new_random(iteration: usize) -> RubikState {
        let mut state: RubikState = SOLVED_STATE;
        let sequence: Vec<Action> = (0..iteration).map(|_| {
            return Action::new(Face::pick_random(), Rotation::pick_random());
        }).collect::<Vec<Action>>();
        state.shuffle(sequence);
        return state;
    }

    pub fn shuffle(&mut self, sequence: Vec<Action>) {
        for action in sequence.iter() {
            *self = action.apply_to(self);
        }
    }

    // pub fn is_solvable(&self) -> bool {
    //     // do code
    //     return true;
    // }
}