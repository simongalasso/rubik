#![allow(dead_code)]

use std::env;

/* Moves */

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

/* Cube simulation */

enum Corner {
    URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB,
}

enum Edge {
    UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR,
}

/* [c : corner] [cdir : corner dir] [e : edge] [edir : edge dir] */
struct Cube {
    c: [Corner; 8],
    cdir: [u32; 8],
    e: [Edge; 12],
    edir: [u32; 12],
}

impl Cube {
    fn initialize_cube() -> Cube {
        Cube {
            c: [Corner::URF, Corner::UFL, Corner::ULB, Corner::UBR, Corner::DFR, Corner::DLF, Corner::DBL, Corner::DRB],
            cdir: [0, 0, 0, 0, 0, 0, 0, 0],
            e: [Edge::UR, Edge::UF, Edge::UL, Edge::UB, Edge::DR, Edge::DF, Edge::DL, Edge::DB, Edge::FR, Edge::FL, Edge::BL, Edge::BR],
            edir: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
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
            Action::RotRight => print!(""),
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

fn cube_shuffle(mut _cube: &mut Cube, _moves: &Vec<Move>) {
    for _move in _moves.iter() {
        apply_move(&mut _cube, &_move);
    }
}

fn apply_move(_cube: &mut Cube, _move: &Move) {
    // apply matrix
}

fn cube_solver(_cube: &mut Cube) -> Vec<Move> {
    let _solution: Vec<Move> = Vec::new();
    _solution
}

/* [!] This is currently a DFS */
fn graph_iterate(_current_state: &Cube, _solution: &mut Vec<Move>) -> bool {
    for _face_idx in 0..5 {
        for _action_idx in 0..3 {
            /*if (graph_iterate() == true) {
                return true
            }*/
        }
    }
    false
}

/*

// TEST ALGO

// Possible optimisations :
//  _path could be type of &Node pointing on _frontier element

use std::collections::VecDeque;

#[derive(Copy, Clone)]
struct Node {
    //code here   
}

fn solver_algo(start: &Node, _goal: &Node) -> Vec<Node> {
    let mut frontier: VecDeque<Node> = VecDeque::new();
    let mut visited: Vec<Node> = Vec::new();
    let path: Vec<Node> = Vec::new();

    frontier.push_back(*start);

    while !frontier.is_empty() {
        let current_node = frontier.pop_front().unwrap();

        // if goal reached
            // break ;

        for neighbour in &mut find_neighbors(&current_node) {
            if /* !visited.contains(*neighbour) */ {
                visited.push(*neighbour);
                frontier.push_back(*neighbour);
            }
        }
    }
    return path
}

fn find_neighbors(_current_node: &Node) -> Vec<Node> {
    let mut neighbors: Vec<Node> = Vec::new();

    /*
    for move in Move {
        if (move != last_move) {
            neighbors.push(apply_move(_current_node, move));
        }
    }
    */

    return neighbors
}

*/