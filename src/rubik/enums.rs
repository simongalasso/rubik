use super::cubie_cube::{CubieCube};

/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/*  Corners                                                                 */
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */

pub const CORNERS_NB: usize = 8;

pub const URF: usize = 0;
pub const UFL: usize = 1;
pub const ULB: usize = 2;
pub const UBR: usize = 3;
pub const DFR: usize = 4;
pub const DLF: usize = 5;
pub const DBL: usize = 6;
pub const DRB: usize = 7;

/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/*  Edges                                                                   */
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */

pub const EDGES_NB: usize = 12;

pub const UR: usize = 0;
pub const UF: usize = 1;
pub const UL: usize = 2;
pub const UB: usize = 3;
pub const DR: usize = 4;
pub const DF: usize = 5;
pub const DL: usize = 6;
pub const DB: usize = 7;
pub const FR: usize = 8;
pub const FL: usize = 9;
pub const BL: usize = 10;
pub const BR: usize = 11;

/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
/*  Actions                                                                 */
/* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */

/// CubieCube U action (is replaced by representation)
pub const U: CubieCube = CubieCube {
    c_p: [UBR, URF, UFL, ULB, DFR, DLF, DBL, DRB],
    c_o: [0, 0, 0, 0, 0, 0, 0, 0],
    e_p: [UB, UR, UF, UL, DR, DF, DL, DB, FR, FL, BL, BR],
    e_o: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
};

/// CubieCube R action (is replaced by representation)
pub const R: CubieCube = CubieCube {
    c_p: [DFR, UFL, ULB, URF, DRB, DLF, DBL, UBR],
    c_o: [2, 0, 0, 1, 1, 0, 0, 2],
    e_p: [FR, UF, UL, UB, BR, DF, DL, DB, DR, FL, BL, UR],
    e_o: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
};

/// CubieCube F action (is replaced by representation)
pub const F: CubieCube = CubieCube {
    c_p: [UFL, DLF, ULB, UBR, URF, DFR, DBL, DRB],
    c_o: [1, 2, 0, 0, 2, 1, 0, 0],
    e_p: [UR, FL, UL, UB, DR, FR, DL, DB, UF, DF, BL, BR],
    e_o: [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0]
};

/// CubieCube D action (is replaced by representation)
pub const D: CubieCube = CubieCube {
    c_p: [URF, UFL, ULB, UBR, DLF, DBL, DRB, DFR],
    c_o: [0, 0, 0, 0, 0, 0, 0, 0],
    e_p: [UR, UF, UL, UB, DF, DL, DB, DR, FR, FL, BL, BR],
    e_o: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
};

/// CubieCube L action (is replaced by representation)
pub const L: CubieCube = CubieCube {
    c_p: [URF, ULB, DBL, UBR, DFR, UFL, DLF, DRB],
    c_o: [0, 1, 2, 0, 0, 2, 1, 0],
    e_p: [UR, UF, BL, UB, DR, DF, FL, DB, FR, UL, DL, BR],
    e_o: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
};

/// CubieCube B action (is replaced by representation)
pub const B: CubieCube = CubieCube {
    c_p: [URF, UFL, UBR, DRB, DFR, DLF, ULB, DBL],
    c_o: [0, 0, 1, 2, 0, 0, 2, 1],
    e_p: [UR, UF, UL, BR, DR, DF, DL, BL, FR, FL, UB, DB],
    e_o: [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1]
};

/// The list of actions represented as &str
pub const ACTIONS_STR_LIST: [&str; 18] = [
    "U", "U2", "U'", "R", "R2", "R'", "F", "F2", "F'",
    "D", "D2", "D'", "L", "L2", "L'", "B", "B2", "B'"
];

/// The list of actions represented as a tuple of the basic_action and its factor
pub const ACTIONS_LIST: [(CubieCube, u8); 18] = [
    (U, 1), (U, 2), (U, 3), (R, 1), (R, 2), (R, 3), (F, 1), (F, 2), (F, 3),
    (D, 1), (D, 2), (D, 3), (L, 1), (L, 2), (L, 3), (B, 1), (B, 2), (B, 3)
];

/// The indexes of the list of actions
pub const ACTIONS: [usize; 18] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];

/// The indexes of the list of actions inverses
pub const ACTION_INVERSE: [usize; 18] = [9, 10, 11, 12, 13, 14, 15, 16, 17, 0, 1, 2, 3, 4, 5, 6, 7, 8]; // FIXME, can optimise by turn function

/// The indexes of the list of G1 actions
pub const G1_ACTIONS: [usize; 10] = [0, 1, 2, 4, 7, 9, 10, 11, 13, 16];