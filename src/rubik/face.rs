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
    
    pub fn from_char(val: char) -> Option<Face> {
        return match val {
            'U' => Some(Face::U),
            'R' => Some(Face::R),
            'F' => Some(Face::F),
            'D' => Some(Face::D),
            'L' => Some(Face::L),
            'B' => Some(Face::B),
            _ => None
        }
    }

}