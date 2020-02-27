#![allow(dead_code)]

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

fn main() {
    let _args: Vec<String> = env::args().collect();
    let mut _moves: Vec<Move> = Vec::new();

    if _args.len() != 2 {
        println!("[!] Error, Rubik should receive one argument");
        return ;
    }

    let _arg_list: Vec<&str> = _args[1].trim().split_whitespace().collect();

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
                    return ;
                },
            },
            action: match &_arg_list[index][1..] {
                "" => Action::RotRight,
                "'" => Action::RotLeft,
                "2" => Action::DoubleRot,
                _ => {
                    println!("[!] Error, Patern does not match: {}", _arg_list[index]);
                    return ;
                },
            }
        })
    }

    for _move in _moves {
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
}
