use super::cubie_cube::{CubieCube};

pub struct CoordCube {
    pub twist: usize, // orientations if every corners as usize
    pub flip: usize, // orientations if every edges as usize
    pub uds_e_sorted: usize // UD slice arrangement as usize
}

// ref (https://www.jaapsch.net/puzzles/compindx.htm#orient) and (http://www.kociemba.org/math/UDSliceCoord.htm)
impl CoordCube {
    pub fn from_cubie_cube(cb_cube: &CubieCube) -> CoordCube {
        return CoordCube {
            twist: cb_cube.get_twist(),
            flip: cb_cube.get_flip(),
            uds_e_sorted: cb_cube.get_uds_e_sorted()
        };
    }
}