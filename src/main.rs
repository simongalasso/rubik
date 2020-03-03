#![allow(dead_code)]

use std::env;

/* - MOVES ------------------------------------ */

// Left, Right, Double
enum Action {
    L, R, D,
}

// Front, Right, Up, Bot, Left, Down
enum Face {
    F, R, U, B, L, D,
}

struct Move {
    face: Face,
    action: Action
}

/* - CUBE SIMULATION -------------------------- */

// UpRightFront, UpFrontLeft, ...
enum Corner {
    URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB,
}

// UpRigt, UpFront, UpLeft, ...
enum Edge {
    UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR,
}

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

/* - PERMUTATIONS FUNCTIONS ------------------- */

fn apply_move(_cube: &mut Cube, _move: &Move) {
    // apply matrix
}

fn cube_shuffle(mut _cube: &mut Cube, _moves: &Vec<Move>) {
    for _move in _moves.iter() {
        apply_move(&mut _cube, &_move);
    }
}

/* - ARGUMENT PARSING ------------------------- */

fn get_moves_list(_args: &Vec<String>) -> Option<Vec<Move>> {
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

/* - RUBIK'S SOLVER --------------------------- */

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

    // Shuffle the cube
    cube_shuffle(&mut _cube, &_moves);

    // Cube solver
    let _solution: Vec<Move> = cube_solver(&mut _cube);

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