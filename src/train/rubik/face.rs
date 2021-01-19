#[derive(Debug, Clone)]
pub enum Face {
    U, R, F, D, L, B
}

impl Face {
    pub fn get_faces() -> [Face; 6] {
        return [Face::U, Face::R, Face::F, Face::D, Face::L, Face::B];
    }
}