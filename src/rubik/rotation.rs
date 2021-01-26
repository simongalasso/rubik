extern crate rand;

use rand::{Rng};

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

    pub fn from_char(value: char) -> Option<Rotation> {
        return match value {
            '\0' => Some(Rotation::R),
            '2' => Some(Rotation::D),
            '\'' => Some(Rotation::L),
            _ => None
        }
    }

    pub fn to_angle(&self) -> Option<f32> {
        return match self {
            Rotation::R => Some(90.0),
            Rotation::D => Some(180.0),
            Rotation::L => Some(-90.0),
        }
    }
}