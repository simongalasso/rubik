#[derive(Debug, Clone)]
pub enum Rotation {
    L, R, D
}

impl Rotation {
    pub fn get_rotations() -> [Rotation; 3] {
        return [Rotation::L, Rotation::R, Rotation::D];
    }
}