use rulinalg::matrix::{Matrix, BaseMatrix};
use std::cmp::Ordering;
use rand::prelude::*;

use crate::algo::neuralnet::*;
use super::action::*;

const STATE_REF: [usize; 40] = [
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
];

const ACTION_LIST: [Action; 18] = [
    Action { face: Face::U, rot: Rotation::L }, Action { face: Face::U, rot: Rotation::R }, Action { face: Face::U, rot: Rotation::D },
    Action { face: Face::F, rot: Rotation::L }, Action { face: Face::F, rot: Rotation::R }, Action { face: Face::F, rot: Rotation::D },
    Action { face: Face::L, rot: Rotation::L }, Action { face: Face::L, rot: Rotation::R }, Action { face: Face::L, rot: Rotation::D },
    Action { face: Face::D, rot: Rotation::L }, Action { face: Face::D, rot: Rotation::R }, Action { face: Face::D, rot: Rotation::D },
    Action { face: Face::R, rot: Rotation::L }, Action { face: Face::R, rot: Rotation::R }, Action { face: Face::R, rot: Rotation::D },
    Action { face: Face::B, rot: Rotation::L }, Action { face: Face::B, rot: Rotation::R }, Action { face: Face::B, rot: Rotation::D },
];

pub struct Rubik {
    pub nn: NeuralNetwork,
    pub state: [usize; 40],
}

impl Rubik {
    pub fn new_shuffled(iterations: usize) -> Rubik {
        let mut rubik: Rubik = Rubik {
            nn: NeuralNetwork::new(8 + 8 + 12 + 12, 20, 18), // experiment differents hidden nodes
            state: STATE_REF,
        };
        let sequence: Vec<Action> = (0..iterations).map(|_| {
            let faces: Vec<Face> = vec![Face::U, Face::F, Face::L, Face::D, Face::R, Face::B];
            let rotations: Vec<Rotation> = vec![Rotation::L, Rotation::R, Rotation::D];
            let current_face: Face = faces[rand::thread_rng().gen_range(0, faces.len())].clone();
            let current_rot: Rotation = rotations[rand::thread_rng().gen_range(0, rotations.len())].clone();
            return Action::new(current_face, current_rot);
        }).collect::<Vec<Action>>();
        rubik.shuffle(sequence);
        return rubik;
    }

    pub fn shuffle(&mut self, sequence: Vec<Action>) {
        for action in sequence.iter() {
            action.apply_to(self);
        }
    }

    pub fn next_action(&mut self) -> Action {
        let inputs: Vec<f64> = self.state.iter().map(|val| *val as f64).collect::<Vec<f64>>();
        let output: Matrix<f64> = self.nn.feedforward(Matrix::new(self.state.len(), 1, inputs));
        let max_index: Option<usize> = output.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal)).map(|(index, _)| index);
        return ACTION_LIST[max_index.unwrap()].clone(); // handle exception
    }
}