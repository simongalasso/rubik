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
        /*for _move in _moves.iter() {
            apply_move(&mut _cube, &_move);
        }*/
    }

    /* FR corner */
    /* Ex:
    [T, T, T, T, D, D, D, D] -> []
    */
    /*fn front_rot_r_corner{

    }

    /* FR edge */
    fn front_rot_r_edge{

    }

    fn apply_move(_cube: &mut Cube, _move: &Move) {
        // apply matrix
    }*/
}