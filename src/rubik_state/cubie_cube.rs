use super::enums::{Corner, CORNERS, Edge, EDGES};
use super::utils::{c_nk, rotate_left};

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

    /// Applies a sequence of action on itself
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
    pub fn corner_multiply(&mut self, b: &CubieCube) { // FIXME, refactor, opti
        let mut c_p_tmp: [Corner; 8] = [Corner::URF; 8]; // FIXME, stupide initialisation, trouver une autre methode
        let mut c_o_tmp: [u8; 8] = [0; 8];
        let mut ori: i32 = 0;
        for i in 0..8 {
            c_p_tmp[i] = self.c_p[b.c_p[i] as usize];
            let ori_a: i32 = self.c_o[b.c_p[i] as usize] as i32;
            let ori_b: i32 = b.c_o[i] as i32;
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
    pub fn get_actions() -> Vec<CubieCube> { // FIXME, refactor, only need to do it once at start
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
    pub fn from_action_str(s: &str) -> CubieCube { // FIXME, refactor the architecture
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

    /// Returns the inverse of itself (ex, if self is F then returns F')
    pub fn inverse(&self) -> CubieCube { // FIXME, optimisation, refactor
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

    /// Returns as a number from 0 to 2186 (3^7 - 1) the twist of every corners
    pub fn get_twist_coord(&self) -> usize {
        let mut twist: usize = 0;
        for i in (Corner::URF as usize)..(Corner::DRB as usize) {
            twist = twist * 3 + self.c_o[i] as usize;
        }
        return twist;
    }

    /// Returns as a number from 0 to 2047 (2^11 - 1) the flip of every edges
    pub fn get_flip_coord(&self) -> usize {
        let mut flip: usize = 0;
        for i in (Edge::UR as usize)..(Edge::BR as usize) {
            flip = flip * 2 + self.e_o[i] as usize;
        }
        return flip;
    }

    /// Returns as a number from 0 to 494 the location state of the 4 UD slice edges
    pub fn get_uds_e_location_coord(&self) -> usize {
        let mut uds_e_sorted: usize = 0;
        let mut x: usize = 0;
        for i in ((Edge::UR as usize)..(Edge::BR as usize + 1)).rev() { // FIXME, bien check si la range correspond à celle de kociemba
            if Edge::FR as usize <= self.e_p[i] as usize && self.e_p[i] as usize <= Edge::BR as usize {
                uds_e_sorted += c_nk(11 - i, x + 1);
                x += 1;
            }
        }
        return uds_e_sorted;
    }

    /// Returns as a number from 0 to 40319 (8! - 1) the permutation of every corners (unused in phase1)
    pub fn get_c_p_coord(&self) -> usize {
        let mut perm: Vec<Corner> = self.c_p.to_vec();
        let mut c_p_coord: usize = 0;
        for j in ((Corner::URF as usize + 1)..(Corner::DRB as usize + 1)).rev() { // FIXME, bien check si la range correspond à celle de kociemba
            let mut k: usize = 0;
            while perm[j] != CORNERS[j] {
                rotate_left::<Corner>(&mut perm, 0, j);
                k += 1;
            }
            c_p_coord = (j + 1) * c_p_coord + k;
        }
        return c_p_coord;
    }

    /// Returns as a number from 0 to 40319 the permutation of every U edges and every D edges (undefined in phase1)
    pub fn get_ud_e_p_coord(&self) -> usize {
        let mut perm: Vec<Edge> = Vec::from(&self.e_p[..8]);
        let mut ud_e_p_coord: usize = 0;
        for j in ((Edge::UR as usize + 1)..(Edge::DB as usize + 1)).rev() { // FIXME, bien check si la range correspond à celle de kociemba
            let mut k: usize = 0;
            while perm[j] != EDGES[j] {
                rotate_left::<Edge>(&mut perm, 0, j);
                k += 1;
            }
            ud_e_p_coord = (j + 1) * ud_e_p_coord + k;
        }
        return ud_e_p_coord;
    }
    /// Returns as a number from 0 to 23 the location and permutation state of the 4 UD slice edges (unused in phase1)
    pub fn get_uds_e_sorted_coord(&self) -> usize {
        let mut a: usize = 0;
        let mut x: usize = 0;
        let mut edge4: Vec<Edge> = Vec::from(&[Edge::UR; 4][..]); // FIXME, stupid init, refactor
        // First compute the index a < (12 choose 4) and the permutation array perm
        for j in ((Edge::UR as usize)..(Edge::BR as usize + 1)).rev() { // FIXME, bien check si la range correspond à celle de kociemba
            if Edge::FR as usize <= self.e_p[j] as usize && self.e_p[j] as usize <= Edge::BR as usize {
                a += c_nk(11 - j, x + 1);
                edge4[3 - x] = self.e_p[j];
                x += 1;
            }
        }
        // Then compute the index b < 4! for the permutation in edge4
        let mut b: usize = 0;
        for j in (1..4).rev() { // FIXME, bien check si la range correspond à celle de kociemba
            let mut k: usize = 0;
            while edge4[j] != EDGES[j + 8] {
                rotate_left::<Edge>(&mut edge4, 0, j);
                k += 1;
            }
            b = (j + 1) * b + k;
        }
        return 24 * a + b;
    }
}