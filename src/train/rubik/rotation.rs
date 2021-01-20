extern crate rand;

use rand::Rng;

#[derive(Debug, Clone)]
pub enum Rotation {
    L, R, D
}

impl Rotation {
    pub fn get_rotations() -> [Rotation; 3] {
        return [Rotation::L, Rotation::R, Rotation::D];
    }

    pub fn pick_random() -> Rotation {
        let mut rng = rand::thread_rng();
        return Rotation::get_rotations()[rng.gen_range(0, 3)].clone();
    }
}