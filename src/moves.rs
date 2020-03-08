#![allow(dead_code)]

use crate::cube::Cube;

// Left, Right, Double
pub enum Action {
    L, R, D,
}

// Front, Right, Up, Bot, Left, Down
pub enum Face {
    F, R, U, B, L, D,
}

pub struct Move {
    pub face: Face,
    pub action: Action
}

impl Move {
    pub fn get_moves_list(_args: &Vec<String>) -> Option<Vec<Move>> {
        let _arg_list: Vec<&str> = _args[1].trim().split_whitespace().collect();
        let mut _moves: Vec<Move> = Vec::new();
    
        for index in 0.._arg_list.len() {
            _moves.push(Move {
                face: match _arg_list[index].chars().next().unwrap() {
                    'F' => Face::F,
                    'R' => Face::R,
                    'U' => Face::U,
                    'B' => Face::B,
                    'L' => Face::L,
                    'D' => Face::D,
                    _ => {
                        println!("[!] Error, Patern does not match: {}", _arg_list[index]);
                        return None;
                    },
                },
                action: match &_arg_list[index][1..] {
                    "" => Action::R,
                    "'" => Action::L,
                    "2" => Action::D,
                    _ => {
                        println!("[!] Error, Patern does not match: {}", _arg_list[index]);
                        return None;
                    },
                }
            })
        }
        Some(_moves)
    }

    pub fn cube_shuffle(mut _cube: &mut Cube, _moves: &Vec<Move>) {
        for _move in _moves.iter() {
            Move::apply_move(&mut _cube, &_move);
        }
    }

    fn apply_move(_cube: &mut Cube, _move: &Move) {
        // code here
    }

    /*
    [A, B, C, D, E, F, G, H],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [A, B, C, D, E, F, G, H, I, J, K, L],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    =
    [B, F, C, D, A, E, G, H],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [A, J, C, D, E, I, G, H, B, F, K, L],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    */
    fn front_rot_r() {
        cube.c[0] = cube.c[1];
        cube.c[1] = cube.c[5];
        cube.c[4] = cube.c[0];
        cube.c[5] = cube.c[4];

        cube.e[1] = cube.e[9];
        cube.e[5] = cube.e[8];
        cube.e[8] = cube.e[1];
        cube.e[9] = cube.e[5];
    }

    /*
    [A, B, C, D, E, F, G, H],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [A, B, C, D, E, F, G, H, I, J, K, L],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    =
    [E, A, C, D, F, B, G, H],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [A, I, C, D, E, J, G, H, F, B, K, L],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    */
    fn front_rot_l() {
        cube.c[0] = cube.c[4];
        cube.c[1] = cube.c[0];
        cube.c[4] = cube.c[5];
        cube.c[5] = cube.c[1];

        cube.e[1] = cube.e[8];
        cube.e[5] = cube.e[9];
        cube.e[8] = cube.e[5];
        cube.e[9] = cube.e[1];
    }

    /*
    [A, B, C, D, E, F, G, H],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [A, B, C, D, E, F, G, H, I, J, K, L],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    =
    [F, E, C, D, B, A, G, H],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [A, B, C, D, E, F, G, H, I, J, K, L],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    */
    fn front_rot_d() {
        cube.c[0] = cube.c[5];
        cube.c[1] = cube.c[4];
        cube.c[4] = cube.c[1];
        cube.c[5] = cube.c[0];
    }
}