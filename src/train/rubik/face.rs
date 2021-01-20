extern crate rand;

use rand::Rng;

#[derive(Debug, Clone)]
pub enum Face {
    U, R, F, D, L, B
}

impl Face {
    pub fn get_faces() -> [Face; 6] {
        return [Face::U, Face::R, Face::F, Face::D, Face::L, Face::B];
    }

    pub fn pick_random() -> Face {
        let mut rng = rand::thread_rng();
        return Face::get_faces()[rng.gen_range(0, 6)].clone();
    }
}