#![allow(dead_code)]

mod cube_solver;
mod cube_transform;

use std::env;

enum Action {
    RotLeft,
    RotRight,
    DoubleRot
}

enum Face {
    Front,
    Right,
    Up,
    Bot,
    Left,
    Down
}

struct Move {
    face: Face,
    action: Action
}

struct Cubicube {
    name: u32, // 0 1 2 3 4 5 6 7 8
    orientation: char // x y z
}

struct Cubiface {
    cubicubes: [Cubicube; 9]
}

struct Cube {
    faces: [Cubiface; 6] // F R U B L D
}

impl Cube {
    fn initialize_cube() -> Cube {
        Cube{
            faces: [
                Cubiface{
                    cubicubes: [
                        Cubicube{name: 0, orientation: 'x'}, Cubicube{name: 1, orientation: 'x'}, Cubicube{name: 2, orientation: 'x'},
                        Cubicube{name: 3, orientation: 'x'}, Cubicube{name: 4, orientation: 'x'}, Cubicube{name: 5, orientation: 'x'},
                        Cubicube{name: 6, orientation: 'x'}, Cubicube{name: 7, orientation: 'x'}, Cubicube{name: 8, orientation: 'x'}]
                },
                Cubiface{
                    cubicubes: [
                        Cubicube{name: 0, orientation: 'x'}, Cubicube{name: 1, orientation: 'x'}, Cubicube{name: 2, orientation: 'x'},
                        Cubicube{name: 3, orientation: 'x'}, Cubicube{name: 4, orientation: 'x'}, Cubicube{name: 5, orientation: 'x'},
                        Cubicube{name: 6, orientation: 'x'}, Cubicube{name: 7, orientation: 'x'}, Cubicube{name: 8, orientation: 'x'}]
                },
                Cubiface{
                    cubicubes: [
                        Cubicube{name: 0, orientation: 'x'}, Cubicube{name: 1, orientation: 'x'}, Cubicube{name: 2, orientation: 'x'},
                        Cubicube{name: 3, orientation: 'x'}, Cubicube{name: 4, orientation: 'x'}, Cubicube{name: 5, orientation: 'x'},
                        Cubicube{name: 6, orientation: 'x'}, Cubicube{name: 7, orientation: 'x'}, Cubicube{name: 8, orientation: 'x'}]
                },
                Cubiface{
                    cubicubes: [
                        Cubicube{name: 0, orientation: 'x'}, Cubicube{name: 1, orientation: 'x'}, Cubicube{name: 2, orientation: 'x'},
                        Cubicube{name: 3, orientation: 'x'}, Cubicube{name: 4, orientation: 'x'}, Cubicube{name: 5, orientation: 'x'},
                        Cubicube{name: 6, orientation: 'x'}, Cubicube{name: 7, orientation: 'x'}, Cubicube{name: 8, orientation: 'x'}]
                },
                Cubiface{
                    cubicubes: [
                        Cubicube{name: 0, orientation: 'x'}, Cubicube{name: 1, orientation: 'x'}, Cubicube{name: 2, orientation: 'x'},
                        Cubicube{name: 3, orientation: 'x'}, Cubicube{name: 4, orientation: 'x'}, Cubicube{name: 5, orientation: 'x'},
                        Cubicube{name: 6, orientation: 'x'}, Cubicube{name: 7, orientation: 'x'}, Cubicube{name: 8, orientation: 'x'}]
                },
                Cubiface{
                    cubicubes: [
                        Cubicube{name: 0, orientation: 'x'}, Cubicube{name: 1, orientation: 'x'}, Cubicube{name: 2, orientation: 'x'},
                        Cubicube{name: 3, orientation: 'x'}, Cubicube{name: 4, orientation: 'x'}, Cubicube{name: 5, orientation: 'x'},
                        Cubicube{name: 6, orientation: 'x'}, Cubicube{name: 7, orientation: 'x'}, Cubicube{name: 8, orientation: 'x'}]
                },
            ]
        }
    }
}

fn main() {
    let mut _cube: Cube = Cube::initialize_cube();

    // Getting arguments
    let _args: Vec<String> = env::args().collect();
    if _args.len() != 2 {
        println!("[!] Error, Rubik should receive one argument");
        return ;
    }

    // Make a list of Moves based on arguments
    let _moves: Vec<Move> = match get_moves_list(&_args) {
        Some(value) => value,
        None => return
    };

    // [!] Debug : Display given Moves
    for _move in _moves.iter() {
        match _move.face {
            Face::Front => print!("Front"),
            Face::Right => print!("Right"),
            Face::Up => print!("Up"),
            Face::Bot => print!("Bot"),
            Face::Left => print!("Left"),
            Face::Down => print!("Down"),
        }
        print!(" | ");
        match _move.action {
            Action::RotLeft => println!("RotLeft"),
            Action::RotRight => println!("RotRight"),
            Action::DoubleRot => println!("DoubleRot"),
        }
    }

    // Shuffle the cube
    cube_shuffle(&mut _cube, &_moves);

    // Cube solver
    let _solution: Vec<Move> = cube_solver(&mut _cube);

    for _move in _solution.iter() {
        match _move.face {
            Face::Front => print!("F"),
            Face::Right => print!("R"),
            Face::Up => print!("U"),
            Face::Bot => print!("B"),
            Face::Left => print!("L"),
            Face::Down => print!("D"),
        }
        match _move.action {
            Action::RotLeft => print!("'"),
            Action::DoubleRot => print!("2"),
        }
        print!(" ");
    }
}

fn get_moves_list(_args: &Vec<String>) -> Option<Vec<Move>> {
    let _arg_list: Vec<&str> = _args[1].trim().split_whitespace().collect();
    let mut _moves: Vec<Move> = Vec::new();

    for index in 0.._arg_list.len() {
        _moves.push(Move {
            face: match _arg_list[index].chars().next().unwrap() {
                'F' => Face::Front,
                'R' => Face::Right,
                'U' => Face::Up,
                'B' => Face::Bot,
                'L' => Face::Left,
                'D' => Face::Down,
                _ => {
                    println!("[!] Error, Patern does not match: {}", _arg_list[index]);
                    return None;
                },
            },
            action: match &_arg_list[index][1..] {
                "" => Action::RotRight,
                "'" => Action::RotLeft,
                "2" => Action::DoubleRot,
                _ => {
                    println!("[!] Error, Patern does not match: {}", _arg_list[index]);
                    return None;
                },
            }
        })
    }
    Some(_moves)
}
