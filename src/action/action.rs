// use std::fmt;

// use crate::rubik::rubik::*;

// // //the positional changes of the cornercubies by faceturns (clockwise) (is_replaced_by representation)
// // CornerCubieMove: [[Cubie; 8]; 6] = [
// //     ((c:DFR;o:2),(c:UFL;o:0),(c:ULB;o:0),(c:URF;o:1),(c:DRB;o:1),(c:DLF;o:0),(c:DBL;o:0),(c:UBR;o:2)), //R
// //     ((c:UFL;o:1),(c:DLF;o:2),(c:ULB;o:0),(c:UBR;o:0),(c:URF;o:2),(c:DFR;o:1),(c:DBL;o:0),(c:DRB;o:0)), //F
// //     ((c:URF;o:0),(c:UFL;o:0),(c:ULB;o:0),(c:UBR;o:0),(c:DLF;o:0),(c:DBL;o:0),(c:DRB;o:0),(c:DFR;o:0)), //D
// //     ((c:URF;o:0),(c:ULB;o:1),(c:DBL;o:2),(c:UBR;o:0),(c:DFR;o:0),(c:UFL;o:2),(c:DLF;o:1),(c:DRB;o:0)), //L
// //     ((c:URF;o:0),(c:UFL;o:0),(c:UBR;o:1),(c:DRB;o:2),(c:DFR;o:0),(c:DLF;o:0),(c:ULB;o:2),(c:DBL;o:1))  //B
// // ];

// // //the positional changes of the edgecubies by faceturns (clockwise) (is_replaced_by representation)
// // EdgeCubieMove: [[Cubie; 12]; 6] = [
// //     ((e:UB;o:0;oA:1),(e:UR;o:0;oA:1),(e:UF;o:0;oA:1),(e:UL;o:0;oA:1),(e:DR;o:0;oA:0),(e:DF;o:0;oA:0),(e:DL;o:0;oA:0),(e:DB;o:0;oA:0),(e:FR;o:0;oA:0),(e:FL;o:0;oA:0),(e:BL;o:0;oA:0),(e:BR;o:0;oA:0)), // U
// //     ((e:FR;o:0;oA:1),(e:UF;o:0;oA:0),(e:UL;o:0;oA:0),(e:UB;o:0;oA:0),(e:BR;o:0;oA:1),(e:DF;o:0;oA:0),(e:DL;o:0;oA:0),(e:DB;o:0;oA:0),(e:DR;o:0;oA:1),(e:FL;o:0;oA:0),(e:BL;o:0;oA:0),(e:UR;o:0;oA:1)), // R
// //     ((e:UR;o:0;oA:0),(e:FL;o:1;oA:1),(e:UL;o:0;oA:0),(e:UB;o:0;oA:0),(e:DR;o:0;oA:0),(e:FR;o:1;oA:1),(e:DL;o:0;oA:0),(e:DB;o:0;oA:0),(e:UF;o:1;oA:1),(e:DF;o:1;oA:1),(e:BL;o:0;oA:0),(e:BR;o:0;oA:0)), // F
// //     ((e:UR;o:0;oA:0),(e:UF;o:0;oA:0),(e:UL;o:0;oA:0),(e:UB;o:0;oA:0),(e:DF;o:0;oA:1),(e:DL;o:0;oA:1),(e:DB;o:0;oA:1),(e:DR;o:0;oA:1),(e:FR;o:0;oA:0),(e:FL;o:0;oA:0),(e:BL;o:0;oA:0),(e:BR;o:0;oA:0)), // D
// //     ((e:UR;o:0;oA:0),(e:UF;o:0;oA:0),(e:BL;o:0;oA:1),(e:UB;o:0;oA:0),(e:DR;o:0;oA:0),(e:DF;o:0;oA:0),(e:FL;o:0;oA:1),(e:DB;o:0;oA:0),(e:FR;o:0;oA:0),(e:UL;o:0;oA:1),(e:DL;o:0;oA:1),(e:BR;o:0;oA:0)), // L
// //     ((e:UR;o:0;oA:0),(e:UF;o:0;oA:0),(e:UL;o:0;oA:0),(e:BR;o:1;oA:1),(e:DR;o:0;oA:0),(e:DF;o:0;oA:0),(e:DL;o:0;oA:0),(e:BL;o:1;oA:1),(e:FR;o:0;oA:0),(e:FL;o:0;oA:0),(e:UB;o:1;oA:1),(e:DB;o:1;oA:1))  // B
// // );

// // is_replaced_by representation
// pub enum Permutation {
//     U = {
//         corners: [
//             Cubie::from(Corner::UBR, 0), Cubie::from(Corner::URF, 0), Cubie::from(Corner::UFL, 0), Cubie::from(Corner::ULB, 0),
//             Cubie::from(Corner::DFR, 0), Cubie::from(Corner::DLF, 0), Cubie::from(Corner::DBL, 0), Cubie::from(Corner::DRB, 0)
//         ],
//         edges: [
//             Cubie::from(Edge::UR, 0), Cubie::from(Edge::UF, 0), Cubie::from(Edge::UL, 0), Cubie::from(Edge::UB, 0),
//             Cubie::from(Edge::DR, 0), Cubie::from(Edge::DF, 0), Cubie::from(Edge::DL, 0), Cubie::from(Edge::DB, 0),
//             Cubie::from(Edge::FR, 0), Cubie::from(Edge::FL, 0), Cubie::from(Edge::BL, 0), Cubie::from(Edge::BR, 0),
//         ]
//     },
//     R = {
//         corners: [
//             Cubie::from(Corner::URF, 0), Cubie::from(Corner::UFL, 0), Cubie::from(Corner::ULB, 0), Cubie::from(Corner::UBR, 0),
//             Cubie::from(Corner::DFR, 0), Cubie::from(Corner::DLF, 0), Cubie::from(Corner::DBL, 0), Cubie::from(Corner::DRB, 0)
//         ],
//         edges: [
//             Cubie::from(Edge::UR, 0), Cubie::from(Edge::UF, 0), Cubie::from(Edge::UL, 0), Cubie::from(Edge::UB, 0),
//             Cubie::from(Edge::DR, 0), Cubie::from(Edge::DF, 0), Cubie::from(Edge::DL, 0), Cubie::from(Edge::DB, 0),
//             Cubie::from(Edge::FR, 0), Cubie::from(Edge::FL, 0), Cubie::from(Edge::BL, 0), Cubie::from(Edge::BR, 0),
//         ]
//     },
//     F = {
//         corners: [
//             Cubie::from(Corner::URF, 0), Cubie::from(Corner::UFL, 0), Cubie::from(Corner::ULB, 0), Cubie::from(Corner::UBR, 0),
//             Cubie::from(Corner::DFR, 0), Cubie::from(Corner::DLF, 0), Cubie::from(Corner::DBL, 0), Cubie::from(Corner::DRB, 0)
//         ],
//         edges: [
//             Cubie::from(Edge::UR, 0), Cubie::from(Edge::UF, 0), Cubie::from(Edge::UL, 0), Cubie::from(Edge::UB, 0),
//             Cubie::from(Edge::DR, 0), Cubie::from(Edge::DF, 0), Cubie::from(Edge::DL, 0), Cubie::from(Edge::DB, 0),
//             Cubie::from(Edge::FR, 0), Cubie::from(Edge::FL, 0), Cubie::from(Edge::BL, 0), Cubie::from(Edge::BR, 0),
//         ]
//     },
//     D = {
//         corners: [
//             Cubie::from(Corner::URF, 0), Cubie::from(Corner::UFL, 0), Cubie::from(Corner::ULB, 0), Cubie::from(Corner::UBR, 0),
//             Cubie::from(Corner::DFR, 0), Cubie::from(Corner::DLF, 0), Cubie::from(Corner::DBL, 0), Cubie::from(Corner::DRB, 0)
//         ],
//         edges: [
//             Cubie::from(Edge::UR, 0), Cubie::from(Edge::UF, 0), Cubie::from(Edge::UL, 0), Cubie::from(Edge::UB, 0),
//             Cubie::from(Edge::DR, 0), Cubie::from(Edge::DF, 0), Cubie::from(Edge::DL, 0), Cubie::from(Edge::DB, 0),
//             Cubie::from(Edge::FR, 0), Cubie::from(Edge::FL, 0), Cubie::from(Edge::BL, 0), Cubie::from(Edge::BR, 0),
//         ]
//     },
//     L = {
//         corners: [
//             Cubie::from(Corner::URF, 0), Cubie::from(Corner::UFL, 0), Cubie::from(Corner::ULB, 0), Cubie::from(Corner::UBR, 0),
//             Cubie::from(Corner::DFR, 0), Cubie::from(Corner::DLF, 0), Cubie::from(Corner::DBL, 0), Cubie::from(Corner::DRB, 0)
//         ],
//         edges: [
//             Cubie::from(Edge::UR, 0), Cubie::from(Edge::UF, 0), Cubie::from(Edge::UL, 0), Cubie::from(Edge::UB, 0),
//             Cubie::from(Edge::DR, 0), Cubie::from(Edge::DF, 0), Cubie::from(Edge::DL, 0), Cubie::from(Edge::DB, 0),
//             Cubie::from(Edge::FR, 0), Cubie::from(Edge::FL, 0), Cubie::from(Edge::BL, 0), Cubie::from(Edge::BR, 0),
//         ]
//     },
//     B = {
//         corners: [
//             Cubie::from(Corner::URF, 0), Cubie::from(Corner::UFL, 0), Cubie::from(Corner::ULB, 0), Cubie::from(Corner::UBR, 0),
//             Cubie::from(Corner::DFR, 0), Cubie::from(Corner::DLF, 0), Cubie::from(Corner::DBL, 0), Cubie::from(Corner::DRB, 0)
//         ],
//         edges: [
//             Cubie::from(Edge::UR, 0), Cubie::from(Edge::UF, 0), Cubie::from(Edge::UL, 0), Cubie::from(Edge::UB, 0),
//             Cubie::from(Edge::DR, 0), Cubie::from(Edge::DF, 0), Cubie::from(Edge::DL, 0), Cubie::from(Edge::DB, 0),
//             Cubie::from(Edge::FR, 0), Cubie::from(Edge::FL, 0), Cubie::from(Edge::BL, 0), Cubie::from(Edge::BR, 0),
//         ]
//     }
// }

// #[derive(Debug, PartialEq)]
// pub enum Face {
// 	F, R, U, B, L, D
// }

// impl Face {
// 	fn from(value: char) -> Option<Face> {
// 		return match value {
// 			'F' => Some(Face::F),
// 			'R' => Some(Face::R),
// 			'U' => Some(Face::U),
// 			'B' => Some(Face::B),
// 			'L' => Some(Face::L),
// 			'D' => Some(Face::D),
// 			_ => None
// 		}
// 	}
// }

// impl fmt::Display for Face {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// #[derive(Debug, PartialEq)]
// pub struct Action {
// 	pub face: Face,
// 	pub rot: u8
// }

// // impl fmt::Display for Rotation {
// //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// //         write!(f, "{}", match self {
// // 			self.rot == 1 '\0',
// // 			Rotation::L => '\'',
// // 			Rotation::D => '2'
// // 		})
// //     }
// // }

// impl Action {
// 	pub fn from(face: Face, rot: u8) -> Action {
// 		return Action {
// 			face: face,
// 			rot: rot
// 		}
//     }
    
//     pub fn apply(&self, rubik: &mut Rubik) {
//         for _ in 0..self.rot {
//             rubik.corner.iter().map(|corner| );
//             for corner in rubik.corner.iter() {
//                 corner.
//                 // [(c:DFR;o:2), (c:UFL;o:0), (c:ULB;o:0), (c:URF;o:1), (c:DRB;o:1), (c:DLF;o:0), (c:DBL;o:0), (c:UBR;o:2)]
//             }
//             // Corner = (URF,UFL,ULB,UBR,DFR,DLF,DBL,DRB);
//         }
//     }
// }