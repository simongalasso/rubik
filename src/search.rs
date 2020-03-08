#![allow(dead_code)]

use crate::cube::Cube;
use crate::moves::Move;

pub struct Search {

}

impl Search {
    pub fn cube_solver(_cube: &mut Cube) -> Vec<Move> {
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