extern crate rubik;

use rubik::rubik_state::*;
use rubik::action::*;

use nn::{NN};

struct Node {
    visited: bool,
    a: Vec<Node>,
    n: [usize; 18],
    l: [f64; 18],
    w: [f64; 18],
    p: [f64; 18]
}

impl Node {
    pub fn new() -> Node {
        return Node {
            visited: false,
            a: Vec::new(),
            n: [0; 18],
            w: [0.0; 18],
            l: [0.0; 18],
            p: [0.0; 18]
        }
    }
}

pub const MAX_TRY: usize = 1000;
const C: f64 = 5.0; // ?

pub fn solve(initial_state: &RubikState, nn: &NN) -> Option<Vec<Action>> {
    let mut root: Node = Node::new();
    let mut history: Vec<Action> = Vec::new();
    for _ in 0..MAX_TRY {
        let mut state: RubikState = initial_state.clone();
        search(&mut history, &mut root, &mut state, nn);
        if state == SOLVED_STATE {
            return Some(history);
        }
    }
    return None;
}

fn search(history: &mut Vec<Action>, node: &mut Node, state: &mut RubikState, nn: &NN) -> f64 {
    if node.visited {
        let sum_n: usize = node.n.iter().sum();
        let mut u: [f64; 18] = [0.0; 18]; // find other way to initialize
        for i in 0..18 {
            u[i] = C * node.p[i] * (sum_n as f64).sqrt() / (1.0 + node.n[i] as f64); 
        }
        let mut q: [f64; 18] = [0.0; 18]; // find other way to initialize
        for i in 0..18 {
            q[i] = node.w[i] - node.l[i]; 
        }
        let mut max_index: usize = 0;
        for i in 0..18 {
            if u[i] + q[i] > u[max_index] + q[max_index] {
                max_index = i;
            }
        }
        let action = Action::get_actions()[max_index].clone(); // do something else than cloning
        *state = action.apply_to(&state);
        history.push(action);
        let predicted_value: f64 = search(history, &mut node.a[max_index], state, nn);
        node.w[max_index] = node.w[max_index].max(predicted_value);
        node.n[max_index] += 1;
        return predicted_value;
    } else {
        node.visited = true;
        node.a = (0..18).map(|_| Node::new()).collect::<Vec<Node>>();
        let results: Vec<f64> = nn.run(&(state.aligned_format())[..]);
        node.w = [results[0]; 18];
        return results[0];
    }
}

/*

const EXPLORATION_PARAMETER: f64 = 5.0;
const VIRTUAL_LOSS_HYPERPARAMETER: f64 = 0.2;

struct Node {
    is_root: bool,
    visited: bool,
    parent: usize;
    state: RubikState,
    children: [Option<usize>; 18],
    used_action: [usize; 18], // the nb of times an action has been used from this state
    max_action_value: [f64; 18], // the max value of each action from this state
    action_virtual_loss: [f64; 18], // the current virtual loss of each action from this state, it's use to prevent the tree search to visit two times the same state
    action_prior_probability: [f64; 18] // the prior probability of each action from this state
}

impl Node {
    pub fn new(parent: usize, state: RubikState) -> Node {
        return Node {
            visited: false,
            parent: parent,
            state: state,
            children: [None; 18],
            used_actions: [0; 18],
            action_virtual_loss: [f64; 18],
            action_prior_probability: [f64; 18]
        }
    }
}

pub fn mcts(state: RubikState) {
    let list: Vec<Node> = vec![Node::new(0, state)];
    while _ in MAX_TRY {
        let leaf: usize = selection(&mut list, 0);
        expansion(&mut list, leaf);
        simulation_result: (f64; [f64; 18]) = simulation(&mut list, leaf);
        backpropagation(leaf, simulation_result.0);
    }
    return path; // need to store it during process
}

fn selection(list: &mut Vec<Node>, node_idx: usize) -> usize {
    while list[node_idx].visited {
        let used_action_sum: usize = list[node_idx].used_action.iter().sum();
        let mut u: [f64; 18] = [0.0; 18];
        for i in 0..18 {
            u[i] = EXPLORATION_PARAMETER * list[node_idx].action_prior_probability[i] * (used_action_sum as f64).sqrt() / (1.0 + list[node_idx].used_action[i] as f64);
        }
        let mut q: [f64; 18] = [0.0; 18];
        for i in 0..18 {
            q[i] = list[node_idx].max_action_value[i] - list[node_idx].action_virtual_loss[i]; 
        }
        let mut max_index: usize = 0;
        for i in 0..18 {
            if u[i] + q[i] > u[max_index] + q[max_index] {
                max_index = i;
            }
        }
        list[node_idx].action_virtual_loss[max_index] += VIRTUAL_LOSS_HYPERPARAMETER;
        node_idx = node_idx.children[max_index];
    }
    return node_idx;
}

fn expansion(list: &mut Vec<Node>, leaf_idx: usize) {
    for (i, action) in Action::get_actions().iter().enumerate() {
        let new_node: Node = Node::new(leaf_idx, action.apply_to(&list[leaf_idx].state));
        let results: Vec<f64> = nn.run(&(new_node.state.aligned_format())[..]);
        new_node.action_prior_probability[i] = &results[1..];
        list.push(new_node);
        list[leaf_idx].children[i] = list.len();
    }
}

fn simulation(list: &mut Vec<Node>, leaf_idx: usize) -> (f64; [f64; 18]) {
    let results: Vec<f64> = nn.run(&(list[leaf].state.aligned_format())[..]);
    return (results[0], &results[1..]);
}

fn backpropagation(list: &mut Vec<Node>, node_idx: usize, simulation_value: f64) {
    if node_idx == 0 {
        return;
    }
    for (i, action) in Action::get_actions().iter().enumerate() {
        list[node_idx].max_action_value[i] = list[node_idx].max_action_value[i].max(simulation_result);
        list[node_idx].used_action[i] += 1.0;
        list[node_idx].action_virtual_loss[i] -= VIRTUAL_LOSS_HYPERPARAMETER;
    }
    backpropagation(list, node.parent, simulation_result);
}

*/