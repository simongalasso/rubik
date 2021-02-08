use super::cubie_cube::{CubieCube};

pub struct CoordCube {
    /// orientations of every corners as coordinate (used in phase1)
    pub twist_coord: usize,
    /// orientations of every edges as coordinate (used in phase1)
    pub flip_coord: usize,
    /// UD slice arrangement as coordinate (used in phase1)
    pub uds_e_location_coord: usize,

    /// onlypermutation of every corners as coordinate (used in phase2)
    pub c_p_coord: usize,
    /// permutation of every edges as coordinate (used in phase2)
    pub ud_e_p_coord: usize,
    /// UD slice coordinate (used in phase2)
    pub uds_e_sorted_coord: usize
}

impl CoordCube {
    /// Creates a CoordCube from a CubieCube
    pub fn from_cubie_cube_phase1(cb_cube: &CubieCube) -> CoordCube {
        return CoordCube {
            twist_coord: cb_cube.get_twist_coord(),
            flip_coord: cb_cube.get_flip_coord(),
            uds_e_location_coord: cb_cube.get_uds_e_location_coord(),
            c_p_coord: 0,
            ud_e_p_coord: 0,
            uds_e_sorted_coord: 0
        };
    }

    pub fn from_cubie_cube_phase2(cb_cube: &CubieCube) -> CoordCube {
        return CoordCube {
            twist_coord: 0,
            flip_coord: 0,
            uds_e_location_coord: 0,
            c_p_coord: cb_cube.get_c_p_coord(),
            ud_e_p_coord: cb_cube.get_ud_e_p_coord(),
            uds_e_sorted_coord: cb_cube.get_uds_e_sorted_coord()
        };
    }

    /// Returns true if this state is part of G1 group
    pub fn is_part_of_g1(&self) -> bool {
        return self.twist_coord == 0 && self.flip_coord == 0 && self.uds_e_location_coord == 0;
    }

    /// Returns true if this state is the solved state (valid if is part of G1 group)
    pub fn is_solved(&self) -> bool {
        return self.c_p_coord == 0 && self.ud_e_p_coord == 0 && self.uds_e_sorted_coord == 0;
    }
}