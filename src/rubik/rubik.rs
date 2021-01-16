// use std::collections::HashMap;

// use crate::action::action::*;

// pub enum Corner {
//     URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB
// }

// pub enum Edge {
//     UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR
// }

// pub struct Cubie {
//     c: u8,
//     o: u8
// }

// impl Cubie {
//     fn from(c: u8, o: u8) -> Cubie {
//         return Cubie {
//             c: 0,
//             o: 0
//         }
//     }
// }

// pub struct Rubik {
//     corners: [Cubie; 8],
//     edges: [Cubie; 12]
// }

// impl Rubik {
//     pub fn new() -> Rubik {
//         return Rubik {
//             corners: [
//                 Cubie::from(Corner::URF, 0), Cubie::from(Corner::UFL, 0), Cubie::from(Corner::ULB, 0), Cubie::from(Corner::UBR, 0),
//                 Cubie::from(Corner::DFR, 0), Cubie::from(Corner::DLF, 0), Cubie::from(Corner::DBL, 0), Cubie::from(Corner::DRB, 0)
//             ],
//             edges: [
//                 Cubie::from(Edge::UR, 0), Cubie::from(Edge::UF, 0), Cubie::from(Edge::UL, 0), Cubie::from(Edge::UB, 0),
//                 Cubie::from(Edge::DR, 0), Cubie::from(Edge::DF, 0), Cubie::from(Edge::DL, 0), Cubie::from(Edge::DB, 0),
//                 Cubie::from(Edge::FR, 0), Cubie::from(Edge::FL, 0), Cubie::from(Edge::BL, 0), Cubie::from(Edge::BR, 0),
//             ]
//         }
//     }

//     pub fn shuffle(&mut self, sequence: Vec<Action>) {
//         for action in sequence.iter() {
//             action.apply(self);
//         }
//     }

//     pub fn solve(&mut self) -> Vec<Action> {
//         let mut output_sequence: Vec<Action> = Vec::new();
//         // do stuff
//         return output_sequence;
//     }

//     pub fn get_cubies(&self) {
//         // convert cubies to visualisator format

//     }
// }