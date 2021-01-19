use super::rubik_state::*;

#[derive(Debug, Clone)]
pub enum Face {
    U, F, L, D, R, B
}

#[derive(Debug, Clone)]
pub enum Rotation {
    L, R, D
}

#[derive(Debug, Clone)]
pub struct Action {
    pub face: Face,
    pub rot: Rotation
}

impl Action {
    pub fn new(face: Face, rot: Rotation) -> Action {
        return Action {
            face: face,
            rot: rot
        }
    }

    pub fn apply_to(&self, state: &mut RubikState) {
        // do permutation
    }
}