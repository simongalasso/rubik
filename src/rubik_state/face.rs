extern crate rand;

use rand::{Rng};

#[derive(Debug, Clone)]
pub enum Face {
    U = 0, R = 1, F = 2, D = 3, L = 4, B = 5
}

pub const FACE_LIST: [Face; 6] = [Face::U, Face::R, Face::F, Face::D, Face::L, Face::B];

impl Face {
    /// Picks a random Face and returns it
    ///
    /// # Basic usage
    ///
    /// ```
    /// extern crate rubik;
    /// use rubik::face::{Face};
    ///
    /// fn main() {
    ///     let face: Face = Face::pick_random();
    /// }
    /// ```
    pub fn pick_random() -> Face {
        let mut rng = rand::thread_rng();
        return FACE_LIST[rng.gen_range(0, 6)].clone();
    }
    
    /// Takes a char and return its associated `Face`
    ///
    /// # Basic usage
    ///
    /// ```
    /// extern crate rubik;
    /// use rubik::face::{Face};
    ///
    /// fn main() {
    ///     let face: Option<Face> = Face::from_char('U');
    ///     assert_eq!(face.unwrap(), Face::U);
    /// }
    /// ```
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