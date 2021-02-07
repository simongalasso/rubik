use super::corner::{Corner, CORNERS};
use super::edge::{Edge, EDGES};
use super::utils::{c_nk};

/// U action (is replaced by representation)
pub const U: CubieCube = CubieCube {
    c_p: [Corner::UBR, Corner::URF, Corner::UFL, Corner::ULB, Corner::DFR, Corner::DLF, Corner::DBL, Corner::DRB],
    c_o: [0, 0, 0, 0, 0, 0, 0, 0],
    e_p: [Edge::UB, Edge::UR, Edge::UF, Edge::UL, Edge::DR, Edge::DF, Edge::DL, Edge::DB, Edge::FR, Edge::FL, Edge::BL, Edge::BR],
    e_o: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
};

/// R action (is replaced by representation)
pub const R: CubieCube = CubieCube {
    c_p: [Corner::DFR, Corner::UFL, Corner::ULB, Corner::URF, Corner::DRB, Corner::DLF, Corner::DBL, Corner::UBR],
    c_o: [2, 0, 0, 1, 1, 0, 0, 2],
    e_p: [Edge::FR, Edge::UF, Edge::UL, Edge::UB, Edge::BR, Edge::DF, Edge::DL, Edge::DB, Edge::DR, Edge::FL, Edge::BL, Edge::UR],
    e_o: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
};

/// F action (is replaced by representation)
pub const F: CubieCube = CubieCube {
    c_p: [Corner::UFL, Corner::DLF, Corner::ULB, Corner::UBR, Corner::URF, Corner::DFR, Corner::DBL, Corner::DRB],
    c_o: [1, 2, 0, 0, 2, 1, 0, 0],
    e_p: [Edge::UR, Edge::FL, Edge::UL, Edge::UB, Edge::DR, Edge::FR, Edge::DL, Edge::DB, Edge::UF, Edge::DF, Edge::BL, Edge::BR],
    e_o: [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0]
};

/// D action (is replaced by representation)
pub const D: CubieCube = CubieCube {
    c_p: [Corner::URF, Corner::UFL, Corner::ULB, Corner::UBR, Corner::DLF, Corner::DBL, Corner::DRB, Corner::DFR],
    c_o: [0, 0, 0, 0, 0, 0, 0, 0],
    e_p: [Edge::UR, Edge::UF, Edge::UL, Edge::UB, Edge::DF, Edge::DL, Edge::DB, Edge::DR, Edge::FR, Edge::FL, Edge::BL, Edge::BR],
    e_o: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
};

/// L action (is replaced by representation)
pub const L: CubieCube = CubieCube {
    c_p: [Corner::URF, Corner::ULB, Corner::DBL, Corner::UBR, Corner::DFR, Corner::UFL, Corner::DLF, Corner::DRB],
    c_o: [0, 1, 2, 0, 0, 2, 1, 0],
    e_p: [Edge::UR, Edge::UF, Edge::BL, Edge::UB, Edge::DR, Edge::DF, Edge::FL, Edge::DB, Edge::FR, Edge::UL, Edge::DL, Edge::BR],
    e_o: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
};

/// B action (is replaced by representation)
pub const B: CubieCube = CubieCube {
    c_p: [Corner::URF, Corner::UFL, Corner::UBR, Corner::DRB, Corner::DFR, Corner::DLF, Corner::ULB, Corner::DBL],
    c_o: [0, 0, 1, 2, 0, 0, 2, 1],
    e_p: [Edge::UR, Edge::UF, Edge::UL, Edge::BR, Edge::DR, Edge::DF, Edge::DL, Edge::BL, Edge::FR, Edge::FL, Edge::UB, Edge::DB],
    e_o: [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1]
};

/// All 90 degree actions
pub const BASIC_ACTIONS: [CubieCube; 6] = [U, R, F, D, L, B];

#[derive(Debug, Clone, PartialEq)]
pub struct CubieCube {
    pub c_p: [Corner; 8],
    pub c_o: [u8; 8],
    pub e_p: [Edge; 12],
    pub e_o: [u8; 12]
}

impl CubieCube {
    /// Creates a new solved CubieCube
    pub fn new_solved() -> CubieCube {
        return CubieCube {
            c_p: [Corner::URF, Corner::UFL, Corner::ULB, Corner::UBR, Corner::DFR, Corner::DLF, Corner::DBL, Corner::DRB],
            c_o: [0, 0, 0, 0, 0, 0, 0, 0],
            e_p: [Edge::UR, Edge::UF, Edge::UL, Edge::UB, Edge::DR, Edge::DF, Edge::DL, Edge::DB, Edge::FR, Edge::FL, Edge::BL, Edge::BR],
            e_o: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        };
    }

    pub fn apply_sequence(&mut self, sequence: &Vec<CubieCube>) {
        for a in sequence.iter() {
            self.multiply(a);
        }
    }

    /// Multiplies itself with another CubieCube
    pub fn multiply(&mut self, b: &CubieCube) {
        self.corner_multiply(b);
        self.edge_multiply(b);
    }

    /// Multiplies itself with another CubieCube (edges not affected)
    pub fn corner_multiply(&mut self, b: &CubieCube) { // FIXME, comprendre le fonctionnement de cette fonction et la simplifier !!!
        let mut c_p_tmp: [Corner; 8] = [Corner::URF; 8]; // FIXME, stupide initialisation, trouver une autre methode
        let mut c_o_tmp: [u8; 8] = [0; 8];
        let mut ori: i32 = 0; // FIXME, j'ai mis i32 par defaut, verifier si c'est pertinent
        for i in 0..8 {
            c_p_tmp[i] = self.c_p[b.c_p[i] as usize];
            let ori_a: i32 = self.c_o[b.c_p[i] as usize] as i32; // FIXME, j'ai mis i32 par defaut, verifier si c'est pertinent
            let ori_b: i32 = b.c_o[i] as i32; // FIXME, j'ai mis i32 par defaut, verifier si c'est pertinent
            if ori_a < 3 && ori_b < 3 { // two regular cubes
                ori = ori_a + ori_b;
                if ori >= 3 {
                    ori -= 3;
                }
            } else if ori_a < 3 && ori_b >= 3 { // cube b is in a mirrored state
                ori = ori_a + ori_b;
                if ori >= 6 {
                    ori -= 3; // the composition also is in a mirrored state
                }
            } else if ori_a >= 3 && ori_b < 3 { // cube a is in a mirrored state
                ori = ori_a - ori_b;
                if ori < 3 {
                    ori += 3; // the composition is a mirrored cube
                }
            } else if ori_a >= 3 && ori_b >= 3 { // if both cubes are in mirrored states
                ori = ori_a - ori_b;
                if ori < 0 {
                    ori += 3; // the composition is a regular cube
                }
            }
            c_o_tmp[i] = ori as u8;
        }
        for i in 0..8 {
            self.c_p[i] = c_p_tmp[i];
            self.c_o[i] = c_o_tmp[i];
        }
    }

    /// Multiply itself with another CubieCube (corners not affected)
    pub fn edge_multiply(&mut self, b: &CubieCube) {
        let mut e_p_tmp: [Edge; 12] = [Edge::UR; 12];
        let mut e_o_tmp: [u8; 12] = [0; 12];
        for i in 0..12 {
            e_p_tmp[i] = self.e_p[b.e_p[i] as usize];
            e_o_tmp[i] = (b.e_o[i] + self.e_o[b.e_p[i] as usize]) % 2;
        }
        for i in 0..12 {
            self.e_p[i] = e_p_tmp[i];
            self.e_o[i] = e_o_tmp[i];
        }
    }

    /// Returns a vector of all actions (U, U2, U', R, R2, R', F, F2, F', D, D2, D', L, L2, L', B, B2, B')
    pub fn get_actions() -> Vec<CubieCube> { // FIXME, do it once at start
        let mut actions: Vec<CubieCube> = vec![];
        for a in 0..6 {
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            for _ in 0..3 {
                cb_cube.multiply(&BASIC_ACTIONS[a]);
                actions.push(cb_cube.clone());
            }
        }
        return actions;
    }

    /// Returns a CubieCube from action char
    pub fn from_action_str(s: &str) -> CubieCube {
        let actions: Vec<CubieCube> = CubieCube::get_actions();
        let index: usize = match s.chars().nth(0).expect("error, from_action_str(), bad input") {
            'U' => 0,
            'R' => 3,
            'F' => 6,
            'D' => 9,
            'L' => 12,
            _ => 15 // B
        };
        return actions[index + match s.chars().nth(1) {
            None => 0,
            Some(v) if v == '2' => 1,
            Some(v) if v == '\'' => 2,
            Some(_) => panic!("error, from_action_str(), bad input"),
        }].clone();
    }

    /// Returns the action string or None if not corresponding to any available actions
    pub fn to_string(&self) -> String {
        let s_actions: [&str; 18] = ["U", "U2", "U\'", "R", "R2", "R\'", "F", "F2", "F\'", "D", "D2", "D\'", "L", "L2", "L\'", "B", "B2", "B\'"];
        let actions: Vec<CubieCube> = CubieCube::get_actions();
        return match actions.iter().position(|a| a == self) {
            Some(index) => String::from(s_actions[index]),
            None => String::from("None")
        }
    }

    /// Returns the inverse of itself
    pub fn inverse(&self) -> CubieCube { // FIXME, comprendre le fonctionnement de cette fonction, et l'optimiser / simplifier !!!
        let mut inverse: CubieCube = CubieCube::new_solved();
        for (i, e) in EDGES.iter().enumerate() {
            inverse.e_p[self.e_p[i] as usize] = *e;
        }
        for i in 0..EDGES.len() {
            inverse.e_o[i] = self.e_o[inverse.e_p[i] as usize];
        }
        for (i, c) in CORNERS.iter().enumerate() {
            inverse.c_p[self.c_p[i] as usize] = *c;
        }
        for i in 0..CORNERS.len() {
            let ori: i32 = self.c_o[inverse.c_p[i] as usize] as i32;
            if ori >= 3 {
                inverse.c_o[i] = ori as u8;
            } else {
                if -ori < 0 {
                    inverse.c_o[i] = (-ori + 3) as u8;
                }
            }
        }
        return inverse;
    }

    pub fn get_twist(&self) -> usize { // FIXME, bof compris
        let mut twist: usize = 0;
        for i in (Corner::URF as usize)..(Corner::DRB as usize) {
            twist = twist * 3 + self.c_o[i] as usize;
        }
        return twist;
    }

    pub fn get_flip(&self) -> usize { // FIXME, bof compris
        let mut flip: usize = 0;
        for i in (Edge::UR as usize)..(Edge::BR as usize) {
            flip = flip * 2 + self.e_o[i] as usize;
        }
        return flip;
    }

    pub fn get_uds_e_sorted(&self) -> usize { // FIXME, bof compris
        let mut uds_e_sorted: usize = 0;
        let mut x: usize = 0;
        for i in (0..(11 + 1)).rev() {
            if Edge::FR as usize <= self.e_p[i] as usize && self.e_p[i] as usize <= Edge::BR as usize {
                uds_e_sorted += c_nk(11 - i, x + 1);
                x += 1;
            }
        }
        return uds_e_sorted;
    }
}