#[derive(Debug, Clone)]
pub enum Face {
    U, F, L, D, R, B
}

impl Face {
    pub fn from_char(val: char) -> Option<Face> {
        return match val {
            'U' => Some(Face::U),
            'F' => Some(Face::F),
            'L' => Some(Face::L),
            'D' => Some(Face::D),
            'R' => Some(Face::R),
            'B' => Some(Face::B),
            _ => None
        }
    }
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

    pub fn to_string(&self) -> String {
        let rot: char = match self.rot {
            180.0 => '2',
            -90.0 => '\'',
            _ => '\0'
        };
        return format!("{:?}{}", self.face, rot);
    }
}