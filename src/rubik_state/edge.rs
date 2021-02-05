/// All Rubik's edge cubies
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Edge {
    UR = 0, UF = 1, UL = 2, UB = 3, DR = 4, DF = 5, DL = 6, DB = 7, FR = 8, FL = 9, BL = 10, BR = 11
}

pub const EDGES: [Edge; 12] = [Edge::UR, Edge::UF, Edge::UL, Edge::UB, Edge::DR, Edge::DF, Edge::DL, Edge::DB, Edge::FR, Edge::FL, Edge::BL, Edge::BR];