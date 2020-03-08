#![allow(dead_code)]

use crate::moves::Move;
use crate::moves::Face;
use crate::moves::Action;

// [!] Debug : Display given Moves
pub fn display_moves(_moves: &Vec<Move>) {
    for _move in _moves.iter() {
        match _move.face {
            Face::F => print!("Front"),
            Face::R => print!("Right"),
            Face::U => print!("Up"),
            Face::B => print!("Bot"),
            Face::L => print!("Left"),
            Face::D => print!("Down"),
        }
        print!(" | ");
        match _move.action {
            Action::L => println!("RotLeft"),
            Action::R => println!("RotRight"),
            Action::D => println!("DoubleRot"),
        }
    }
}

pub fn display_solution(_solution: &Vec<Move>) {
    for _move in _solution.iter() {
        match _move.face {
            Face::F => print!("F"),
            Face::R => print!("R"),
            Face::U => print!("U"),
            Face::B => print!("B"),
            Face::L => print!("L"),
            Face::D => print!("D"),
        }
        match _move.action {
            Action::R => print!(""),
            Action::L => print!("'"),
            Action::D => print!("2"),
        }
        print!(" ");
    }
}