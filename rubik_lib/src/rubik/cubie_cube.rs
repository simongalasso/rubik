use super::utils::{c_nk, rotate_left, rotate_right};
use super::enums::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CubieCube {
    pub c_p: [usize; 8],
    pub c_o: [u8; 8],
    pub e_p: [usize; 12],
    pub e_o: [u8; 12]
}

impl CubieCube {
    /// Creates a new solved CubieCube
    pub fn new_solved() -> CubieCube {
        return CubieCube {
            c_p: [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
            c_o: [0, 0, 0, 0, 0, 0, 0, 0],
            e_p: [UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR],
            e_o: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        };
    }

    /// Applies a sequence of action on itself
    pub fn apply_sequence(&mut self, sequence: &Vec<usize>) {
        for a in sequence.iter() {
            *self = self.multiply(&ACTIONS_LIST[*a].0, ACTIONS_LIST[*a].1);
        }
    }

    /// Creates a CubieCube by multiplying itself with another CubieCube
    pub fn multiply(&self, b: &CubieCube, repeat: u8) -> CubieCube {
        let mut new_cb_cube: CubieCube = self.clone();
        for _ in 0..repeat {
            new_cb_cube.corner_multiply(b);
            new_cb_cube.edge_multiply(b);
        }
        return new_cb_cube;
    }

    /// Multiplies itself with another CubieCube (edges not affected)
    pub fn corner_multiply(&mut self, b: &CubieCube) { // FIXME, refactor, opti
        let mut c_p_tmp: [usize; 8] = [0; 8];
        let mut c_o_tmp: [u8; 8] = [0; 8];
        let mut ori: i32 = 0;
        for i in 0..8 {
            c_p_tmp[i] = self.c_p[b.c_p[i]];
            let ori_a: i32 = self.c_o[b.c_p[i]] as i32;
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
        let mut e_p_tmp: [usize; 12] = [0; 12];
        let mut e_o_tmp: [u8; 12] = [0; 12];
        for i in 0..12 {
            e_p_tmp[i] = self.e_p[b.e_p[i]];
            e_o_tmp[i] = (b.e_o[i] + self.e_o[b.e_p[i]]) % 2;
        }
        for i in 0..12 {
            self.e_p[i] = e_p_tmp[i];
            self.e_o[i] = e_o_tmp[i];
        }
    }

    /// Returns as a number from 0 to 2186 (3^7 - 1) the twist of every corners
    pub fn get_twist_coord(&self) -> usize {
        let mut twist: usize = 0;
        for i in URF..DRB {
            twist = twist * 3 + self.c_o[i] as usize;
        }
        return twist;
    }

    /// Sets the twist of every corners from a number from 0 to 2186 (3^7 - 1)
    pub fn set_twist_coord(&mut self, mut twist: usize) {
        let mut twistparity: usize = 0;
        for i in (URF..DRB).rev() {
            self.c_o[i] = (twist % 3) as u8;
            twistparity += self.c_o[i] as usize;
            twist = twist / 3;
        }
        self.c_o[DRB] = ((3 - twistparity % 3) % 3) as u8;
    }

    /// Returns as a number from 0 to 2047 (2^11 - 1) the flip of every edges
    pub fn get_flip_coord(&self) -> usize {
        let mut flip: usize = 0;
        for i in UR..BR {
            flip = flip * 2 + self.e_o[i] as usize;
        }
        return flip;
    }

    /// Sets the flip of every edges from a number from 0 to 2047 (2^11 - 1)
    pub fn set_flip_coord(&mut self, mut flip: usize) {
        let mut flipparity: usize = 0;
        for i in (UR..BR).rev() {
            self.e_o[i] = (flip % 2) as u8;
            flipparity += self.e_o[i] as usize;
            flip = flip / 2;
        }
        self.e_o[BR] = ((2 - flipparity % 2) % 2) as u8;
    }

    /// Returns as a number from 0 to 494 the location state of the 4 UD slice edges
    pub fn get_uds_e_location_coord(&self) -> usize {
        let mut uds_e_sorted: usize = 0;
        let mut x: usize = 0;
        for i in (UR..(BR + 1)).rev() {
            if FR <= self.e_p[i] && self.e_p[i] <= BR {
                uds_e_sorted += c_nk(11 - i, x + 1);
                x += 1;
            }
        }
        return uds_e_sorted;
    }

    /// Sets the 4 UD slice edges from a number from 0 to 494
    pub fn set_uds_e_location_coord(&mut self, index: usize) {
        let slice_edge = [FR, FL, BL, BR];
        let other_edge = [UR, UF, UL, UB, DR, DF, DL, DB];
        let mut a: i32 = index as i32;
        self.e_p = [12; EDGES_NB]; // Invalidate every edge positions
        let mut x: usize = 4;
        for j in 0..EDGES_NB {
            if a - c_nk(11 - j, x) as i32 >= 0 {
                self.e_p[j] = slice_edge[4 - x];
                a -= c_nk(11 - j, x) as i32;
                x -= 1;
            }
        }
        x = 0;
        for k in 0..EDGES_NB {
            if self.e_p[k] == 12 {
                self.e_p[k] = other_edge[x];
                x += 1;
            }
        }
    }

    /// Returns as a number from 0 to 40319 (8! - 1) the permutation of every corners (unused in phase1)
    pub fn get_c_p_coord(&self) -> usize {
        let mut perm: Vec<usize> = self.c_p.to_vec();
        let mut c_p_coord: usize = 0;
        for j in ((URF + 1)..(DRB + 1)).rev() {
            let mut k: usize = 0;
            while perm[j] != j {
                rotate_left(&mut perm, 0, j);
                k += 1;
            }
            c_p_coord = (j + 1) * c_p_coord + k;
        }
        return c_p_coord;
    }

    /// Sets the permutation of every corners from a number from 0 to 40319 (8! - 1)  (unused in phase1)
    pub fn set_c_p_coord(&mut self, mut index: usize) {
        let mut perm: Vec<usize> = vec![URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB];
        for j in 0..CORNERS_NB {
            let mut k: usize = index % (j + 1);
            index = index / (j + 1);
            while k > 0 {
                rotate_right(&mut perm, 0, j);
                k -= 1;
            }
        }
        for i in 0..CORNERS_NB {
            self.c_p[i] = perm[i];
        }
    }

    /// Returns as a number from 0 to 40319 the permutation of every U edges and every D edges (undefined in phase1)
    pub fn get_ud_e_p_coord(&self) -> usize {
        let mut perm: Vec<usize> = Vec::from(&self.e_p[..8]);
        let mut ud_e_p_coord: usize = 0;
        for j in ((UR + 1)..(DB + 1)).rev() {
            let mut k: usize = 0;
            while perm[j] != j {
                rotate_left(&mut perm, 0, j);
                k += 1;
            }
            ud_e_p_coord = (j + 1) * ud_e_p_coord + k;
        }
        eprintln!("E");
        return ud_e_p_coord;
    }

    /// Sets the permutation of every U edges and every D edges from a number from 0 to 40319 
    pub fn set_ud_e_p_coord(&mut self, mut index: usize) {
        let mut perm: Vec<usize> = self.e_p.to_vec();
        for i in 0..8 {
            perm[i] = i;
        }
        for j in 0..8 {
            let mut k: usize = index % (j + 1);
            index = index / (j + 1);
            while k > 0 {
                rotate_right(&mut perm, 0, j);
                k -= 1;
            }
        }
        for i in 0..12 {
            self.e_p[i] = perm[i];
        }
    }

    /// Returns as a number from 0 to 23 the location and permutation state of the 4 UD slice edges (unused in phase1)
    pub fn get_uds_e_sorted_coord(&self) -> usize {
        let mut a: usize = 0;
        let mut x: usize = 0;
        let mut edge4: Vec<usize> = vec![0; 4];
        // First compute the index a < (12 choose 4) and the permutation array perm
        for j in (UR..(BR + 1)).rev() {
            if FR <= self.e_p[j] && self.e_p[j] <= BR {
                a += c_nk(11 - j, x + 1);
                edge4[3 - x] = self.e_p[j];
                x += 1;
            }
        }
        // Then compute the index b < 4! for the permutation in edge4
        let mut b: usize = 0;
        for j in (1..4).rev() {
            let mut k: usize = 0;
            while edge4[j] != j + 8 {
                rotate_left(&mut edge4, 0, j);
                k += 1;
            }
            b = (j + 1) * b + k;
        }
        return 24 * a + b;
    }

    /// Sets the location and permutation state of the 4 UD slice edges from a number from 0 to 23 
    pub fn set_uds_e_sorted_coord(&mut self, index: usize) {
        let mut slice_edge: Vec<usize> = vec![FR, FL, BL, BR];
        let other_edge: Vec<usize> = vec![UR, UF, UL, UB, DR, DF, DL, DB];
        let mut b: usize = index % 24;
        let mut a: usize = index / 24;
        self.e_p = [EDGES_NB; EDGES_NB]; // Invalidate all edge positions
        let mut l: usize = 1;
        while l < 4 {
            let mut k: usize = b % (l + 1);
            b = b / (l + 1);
            while k > 0 {
                rotate_right(&mut slice_edge, 0, l);
                k -= 1;
            }
            l += 1;
        }
        let mut x: usize = 4;
        for j in 0..EDGES_NB {
            if a as i32 - c_nk(11 - j, x) as i32 >= 0 {
                self.e_p[j] = slice_edge[4 - x];
                a -= c_nk(11 - j, x);
                x -= 1;
            }
        }
        x = 0;
        for j in 0..EDGES_NB {
            if self.e_p[j] == EDGES_NB {
                self.e_p[j] = other_edge[x];
                x += 1;
            }
        }
    }
    
    /// Returns true if this state is part of G1 group
    pub fn is_part_of_g1(&self) -> bool {
        return self.get_twist_coord() == 0 && self.get_flip_coord() == 0 && self.get_uds_e_location_coord() == 0;
    }
}