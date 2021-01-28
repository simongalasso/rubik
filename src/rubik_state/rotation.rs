extern crate rand;

use rand::{Rng};

#[derive(Debug, Clone)]
pub enum Rotation {
    L, R, D
}

pub const ROTATION_LIST: [Rotation; 3] = [Rotation::L, Rotation::R, Rotation::D];

impl Rotation {
    /// Pick a random Rotation and returns it
    ///
    /// # Basic usage
    ///
    /// ```
    /// extern crate rubik;
    /// use rubik::rotation::{Rotation};
    ///
    /// fn main() {
    ///     let rotation: Rotation = Rotation::pick_random();
    /// }
    /// ```
    pub fn pick_random() -> Rotation {
        let mut rng = rand::thread_rng();
        return ROTATION_LIST[rng.gen_range(0, 3)].clone();
    }

    /// Take a char and return its associated `Rotation`
    ///
    /// # Basic usage
    ///
    /// ```
    /// extern crate rubik;
    /// use rubik::rotation::{Rotation};
    ///
    /// fn main() {
    ///     let rotation: Option<Rotation> = Rotation::from_char('2');
    ///     assert_eq!(rotation.unwrap(), Rotation::D);
    /// }
    /// ```
    pub fn from_char(value: char) -> Option<Rotation> {
        return match value {
            '\0' => Some(Rotation::R),
            '2' => Some(Rotation::D),
            '\'' => Some(Rotation::L),
            _ => None
        }
    }

    /// Converts `Rotation` to an angle
    ///
    /// # Basic usage
    ///
    /// ```
    /// extern crate rubik;
    /// use rubik::rotation::{Rotation};
    ///
    /// fn main() {
    ///     let rotation: Rotation = Rotation::L;
    ///     let angle: Option<f32> = rotation.to_angle('2');
    ///     assert_eq!(angle.unwrap(), -90.0);
    /// }
    /// ```
    pub fn to_angle(&self) -> Option<f32> {
        return match self {
            Rotation::R => Some(90.0),
            Rotation::D => Some(180.0),
            Rotation::L => Some(-90.0),
        }
    }
}