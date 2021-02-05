extern crate rubik;

use rubik::cubie_cube::{CubieCube};

/* # Kociemba Algorithm ######################################### */

// const MAX_LEN: u16 = 25;
// const CURRENT_DEPTH: u16 = /* ? */;

// pub fn solve(cubestring: String) {
//     let fc_cube: FaceletCube = FaceletCube::from_string(cubestring);
//     let cb_cube = fc_cube.to_cubie_cube();
//     let co_cube = cb_cube.to_coord_cube();
//     phase1_start(co_cube);
// }

// fn phase1_start(state: CoordCube) {
//     for depth in 0..MAX_LEN {
//         self.phase1(co_cube, depth);
//     }
// }

// fn phase1(state: CoordCube, depth: u16) {
//     if depth == 0 && state.is_part_of_g1() && /* last move was not part of g1 actions, so it should be either F, F', B, B', R, R', L, or L' */ {
//         phase2_start(state);
//     } else if depth > 0 && prune1[state] <= depth {
//         let relevant_actions = /* ? */;
//         for a in relevant_actions.iter() {
//             phase1(a.apply_to(&state), depth - 1);
//         }
//     }
// }

// fn phase2_start(state: CoordCube) {
//     for depth in 0..(MAX_LEN - CURRENT_DEPTH) {
//         phase2(state, depth);
//     }
// }

// fn phase2(state: CoordCube, depth: u16) {
//     if depth == 0 && state.is_solved() {
//         MAX_LEN = CURRENT_DEPTH - 1;
//     } else if depth > 0 {
//         let relevant_actions = /* ? */;
//         for a in relevant_actions.iter() {
//             phase2(a.apply_to(&state), depth - 1);
//         }
//     }
// }

/* # God's Algorithm ############################################ */

pub fn gods_algorithm(state: &mut CubieCube) -> Vec<CubieCube> {
    let database: Vec<(CubieCube, Option<CubieCube>)> = generate_database();
    let sequence: Vec<CubieCube> = vec![];
    while *state != CubieCube::new_solved() {
        let action: CubieCube = database.iter().find(|&&row| row.0 == *state).unwrap().1.unwrap();
        state.multiply(&action);
        sequence.push(action);
    }
    return sequence;
}

fn generate_database() -> Vec<(CubieCube, Option<CubieCube>)> {
    let database: Vec<(CubieCube, Option<CubieCube>)> = vec![(CubieCube::new_solved(), None)];
    let queue: Vec<&(CubieCube, Option<CubieCube>)> = vec![database.last().unwrap()];
    for (state, _) in queue.iter() {
        queue.pop();
        for action in CubieCube::get_actions().iter() {
            let new_state: CubieCube = state.clone();
            new_state.multiply(action);
            if !database.iter().any(|&row| row.0 == new_state) {
                database.push((new_state, Some(action.inverse())));
                queue.push(database.last().unwrap());
            }
        }
    }
    return database;
}
