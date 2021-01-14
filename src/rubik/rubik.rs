use crate::parsing::parse::*;

// UpRightFront, UpFrontLeft, ...
enum Corner {
    URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB,
}

// UpRigt, UpFront, UpLeft, ...
enum Edge {
    UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR,
}

// enum Cubie {
//     Corner, Edge
// }

// impl Cubie {
//     fn new(id: &str, c: u8, o: u8) -> Cubie {
//         return Cubie {
//             id, c, o
//         }
//     }
// }

// fn test {
//     let cubie: Cubie = Cubie::Corner::new("URF", 0, 2);
// }

pub struct Rubik {
    c: [Corner; 8],
    cdir: [u32; 8],
    e: [Edge; 12],
    edir: [u32; 12],
}

impl Rubik {
    pub fn new() -> Rubik {
        return Rubik {
            c: [Corner::URF, Corner::UFL, Corner::ULB, Corner::UBR, Corner::DFR, Corner::DLF, Corner::DBL, Corner::DRB],
            cdir: [0, 0, 0, 0, 0, 0, 0, 0], // 0 (ref), 1 (twisted clockwise) or 2 twisted anticlockwise
            e: [Edge::UR, Edge::UF, Edge::UL, Edge::UB, Edge::DR, Edge::DF, Edge::DL, Edge::DB, Edge::FR, Edge::FL, Edge::BL, Edge::BR],
            edir: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 0 (ref) or 1 (flipped)
        }
    }

    pub fn shuffle(&mut self, sequence: Vec<Action>) {
        for action in sequence.iter() {
            self.permutate(action);
        }
    }

    pub fn permutate(&mut self, action: &Action) {
        // corner permutation: (from, to)
        // (URF, UFL), (UFL, DLF), ()
    }

    pub fn solve(&mut self) -> Vec<Action> {
        let mut output_sequence: Vec<Action> = Vec::new();
        // do stuff
        return output_sequence;
    }

    pub fn get_cubies(&self) {
        // convert cubies to visualisator format

    }
}

/* ---------------------------------------------------------------------------------------------
    2 * 3                           * 3 *                           T T T
    * * *                           2 * 0                           T T T
    1 * 0                       |   * 1 *                           T T T
                                |
    1 * 0  0 * 3  3 * 2  2 * 1  |   * 1 *  * 0 *  * 3 *  * 2 *  |   F F F  R R R  B B B  L L L
    * * *  * * *  * * *  * * *  |   9 * 8  8 * B  B * A  A * 9  |   F F F  R R R  B B B  L L L
    5 * 4  4 * 7  7 * 6  6 * 5  |   * 5 *  * 4 *  * 7 *  * 6 *  |   F F F  R R R  B B B  L L L
                                |
    5 * 4                           * 5 *                           D D D
    * * *                           6 * 4                           D D D
    6 * 7                           * 7 *                           D D D   
--------------------------------------------------------------------------------------------- */