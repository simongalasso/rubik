/// All Rubik's corner cubies
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Corner {
    URF = 0, UFL = 1, ULB = 2, UBR = 3, DFR = 4, DLF = 5, DBL = 6, DRB = 7
}

pub const CORNERS: [Corner; 8] = [Corner::URF, Corner::UFL, Corner::ULB, Corner::UBR, Corner::DFR, Corner::DLF, Corner::DBL, Corner::DRB];