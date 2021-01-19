extern crate rand;

use rand::Rng;

use super::action::*;
use super::face::*;
use super::rotation::*;
use super::corner::*;
use super::edge::*;

#[derive(Debug, Clone)]
pub struct RubikState {
    pub c_p: [Corner; 8],
    pub c_o: [u8; 8],
    pub e_p: [Edge; 12],
    pub e_o: [u8; 12]
}

impl RubikState {
    pub fn new_solved() -> RubikState {
        return RubikState {
            c_p: [Corner::URF, Corner::UFL, Corner::ULB, Corner::UBR, Corner::DFR, Corner::DLF, Corner::DBL, Corner::DRB],
            c_o: [0, 0, 0, 0, 0, 0, 0, 0],
            e_p: [Edge::UR, Edge::UF, Edge::UL, Edge::UB, Edge::DR, Edge::DF, Edge::DL, Edge::DB, Edge::FR, Edge::FL, Edge::BL, Edge::BR],
            e_o: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        };
    }

    pub fn new_random(iteration: usize) -> RubikState {
        let mut state: RubikState = RubikState::new_solved();
        let sequence: Vec<Action> = (0..iteration).map(|_| {
            let current_face: Face = Face::get_faces()[rand::thread_rng().gen_range(0, Face::get_faces().len())].clone();
            let current_rot: Rotation = Rotation::get_rotations()[rand::thread_rng().gen_range(0, Rotation::get_rotations().len())].clone();
            return Action::new(current_face, current_rot);
        }).collect::<Vec<Action>>();
        state.shuffle(sequence);
        return state;
    }

    pub fn shuffle(&mut self, sequence: Vec<Action>) {
        for action in sequence.iter() {
            action.apply_to(self);
        }
    }

    pub fn is_solvable(&self) -> bool {
        // do code
        return true;
    }
}