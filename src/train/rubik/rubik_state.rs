extern crate rand;

use rand::Rng;

use super::action::*;

const STATE_REF: [usize; 24] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0
];

pub struct RubikState([usize; 24]);

impl RubikState {
    pub fn from_array(arr: [usize; 24]) -> RubikState {
        return RubikState(arr);
    }
    
    pub fn new_random(iteration: usize) -> RubikState {
        let mut state: RubikState = RubikState::from_array(STATE_REF);
        let sequence: Vec<Action> = (0..iteration).map(|_| {
            let faces: Vec<Face> = vec![Face::U, Face::F, Face::L, Face::D, Face::R, Face::B];
            let rotations: Vec<Rotation> = vec![Rotation::L, Rotation::R, Rotation::D];
            let current_face: Face = faces[rand::thread_rng().gen_range(0, faces.len())].clone();
            let current_rot: Rotation = rotations[rand::thread_rng().gen_range(0, rotations.len())].clone();
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
}