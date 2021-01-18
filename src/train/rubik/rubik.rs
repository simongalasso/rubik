use rulinalg::matrix::{Matrix, BaseMatrix};

use crate::algo::neuralnet::*;

const STATE_REF: [usize; 40] = [
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
];

pub struct Rubik {
    pub nn: NeuralNetwork,
    pub state: [usize; 40],
}

impl Rubik {
    pub fn new_shuffled(iterations: u16) -> Rubik {
        let mut rubik: Rubik = Rubik {
            nn: NeuralNetwork::new(8 + 8 + 12 + 12, 20, 18), // experiment differents hidden nodes
            state: STATE_REF,
        };
        // let sequence: Vec<Action> = /* */;
        // rubik.shuffle(sequence);
        return rubik;
    }

    // pub fn shuffle(&mut self, sequence: Vec<Action>) {
    //     for action in sequence.iter() {
    //         action.apply(self);
    //     }
    // }

    pub fn next_move(&mut self) {
        let output: Matrix<f64> = self.nn.feedforward(Matrix::new(self.state.len(), 1, self.state.iter().map(|val| *val as f64).collect::<Vec<f64>>()));
    }
}