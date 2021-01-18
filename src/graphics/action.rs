#[derive(Debug, Clone)]
pub enum Face {
    U, F, L, D, R, B
}

#[derive(Debug, Clone)]
pub struct Action {
    pub face: Face,
    pub rot: f32
}

impl Action {
    pub fn new(face: Face, rot: f32) -> Action {
        return Action {
            face: face,
            rot: rot
        }
    }
}