extern crate rand;
extern crate rulinalg;

use std::fs::File;
use std::path::Path;
use std::fs::metadata;
use std::io::prelude::*;
use std::io::BufReader;
use rulinalg::matrix::{Matrix, BaseMatrixMut, BaseMatrix};
use rand::Rng;

/* Math ------------------------------------------ */

pub fn sigmoid(x: f64) -> f64 {
    return 1.0 / (1.0 + (-x).exp());
}

pub fn dsigmoid(y: f64) -> f64 {
    return y * (1.0 - y);
}

/* Data ------------------------------------------ */

#[derive(Debug)]
struct Data {
    inputs: Matrix<f64>,
    targets: Matrix<f64>
}

/* Neural Network -------------------------------- */

#[derive(Clone)]
pub struct NeuralNetwork {
    nb_inputs: usize,
    nb_hidden: usize,
    nb_outputs: usize,
    weights_ih: Matrix<f64>,
    weights_ho: Matrix<f64>,
    bias_h: Matrix<f64>,
    bias_o: Matrix<f64>,
    learning_rate: f64
}

impl NeuralNetwork {
    pub fn new(nb_inputs: usize, nb_hidden: usize, nb_outputs: usize) -> NeuralNetwork {
        let mut rng = rand::thread_rng();
        NeuralNetwork {
            nb_inputs: nb_inputs,
            nb_hidden: nb_hidden,
            nb_outputs: nb_outputs,
            weights_ih: Matrix::new(nb_hidden, nb_inputs, (0..(nb_hidden * nb_inputs)).map(|_| rng.gen_range(-1.0, 1.0)).collect::<Vec<f64>>()),
            weights_ho: Matrix::new(nb_outputs, nb_hidden, (0..(nb_outputs * nb_hidden)).map(|_| rng.gen_range(-1.0, 1.0)).collect::<Vec<f64>>()),
            bias_h: Matrix::new(nb_hidden, 1, (0..nb_hidden).map(|_| rng.gen_range(-1.0, 1.0)).collect::<Vec<f64>>()),
            bias_o: Matrix::new(nb_outputs, 1, (0..nb_outputs).map(|_| rng.gen_range(-1.0, 1.0)).collect::<Vec<f64>>()),
            learning_rate: 0.1
        }
    }

    pub fn feedforward(&mut self, inputs: Matrix<f64>) -> Matrix<f64> {
        // Feed forward inputs -> hidden
        let mut hidden: Matrix<f64> = &self.weights_ih * &inputs;
        hidden = &hidden + &self.bias_h;
        hidden = hidden.apply(&sigmoid);
        // Feed forward hidden -> outputs
        let mut outputs: Matrix<f64> = &self.weights_ho * &hidden;
        outputs = &outputs + &self.bias_o;
        outputs = outputs.apply(&sigmoid);
        return outputs;
    }

    pub fn train(&mut self, inputs: &Matrix<f64>, targets: &Matrix<f64>) { 
        // Feed forward inputs -> hidden
        let mut hidden: Matrix<f64> = &self.weights_ih * inputs;
        hidden = &hidden + &self.bias_h;
        hidden = hidden.apply(&sigmoid);
        // Feed forward hidden -> outputs
        let mut outputs: Matrix<f64> = &self.weights_ho * &hidden;
        outputs = &outputs + &self.bias_o;
        outputs = outputs.apply(&sigmoid);

        // Computing outputs errors
        let output_errors: Matrix<f64> = targets - &outputs;
        // Computing outputs gradient
        let mut outputs_gradients: Matrix<f64> = outputs.apply(&dsigmoid);
        outputs_gradients = outputs_gradients.elemul(&output_errors);
        outputs_gradients = &outputs_gradients * &self.learning_rate;
        // Computing hidden -> outputs deltas
        let hidden_transposed: Matrix<f64> = hidden.transpose();
        let weights_ho_deltas: Matrix<f64> = &outputs_gradients * hidden_transposed;
        // Update hidden -> outputs weights
        self.weights_ho = &self.weights_ho + &weights_ho_deltas;
        // Update outputs bias
        self.bias_o = &self.bias_o + outputs_gradients;

        // Computing hidden errors
        let weights_ho_transposed: Matrix<f64> = self.weights_ho.transpose();
        let hidden_errors: Matrix<f64> = &weights_ho_transposed * &output_errors;
        // Computing hidden gradient
        let mut hidden_gradients: Matrix<f64> = hidden.apply(&dsigmoid);
        hidden_gradients = hidden_gradients.elemul(&hidden_errors);
        hidden_gradients = &hidden_gradients * &self.learning_rate;
        // Computing inputs -> hidden deltas
        let inputs_transposed: Matrix<f64> = inputs.transpose(); 
        let weights_ih_deltas: Matrix<f64> = &hidden_gradients * inputs_transposed;
        // Update inputs -> hidden weights
        self.weights_ih = &self.weights_ih + &weights_ih_deltas;
        // Update outputs bias
        self.bias_h = &self.bias_h + hidden_gradients;
    }

    pub fn export_weights(&self) {
        let path = Path::new("weights.txt");
        let display = path.display();
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        file.write_all(b"#weights_ih\n").expect(&format!("couldn't write to {}", display));
        for i in 0..(self.nb_inputs * self.nb_hidden) {
            file.write_all(&format!("{}\n", self.weights_ih.data()[i]).as_bytes()).expect(&format!("couldn't write to {}", display));
        }
        file.write_all(b"#weights_ho\n").expect(&format!("couldn't write to {}", display));
        for i in 0..(self.nb_hidden * self.nb_outputs) {
            file.write_all(&format!("{}\n", self.weights_ho.data()[i]).as_bytes()).expect(&format!("couldn't write to {}", display));
        }
        file.write_all(b"#bias_h\n").expect(&format!("couldn't write to {}", display));
        for i in 0..(self.nb_hidden) {
            file.write_all(&format!("{}\n", self.bias_h.data()[i]).as_bytes()).expect(&format!("couldn't write to {}", display));
        }
        file.write_all(b"#bias_o\n").expect(&format!("couldn't write to {}", display));
        for i in 0..(self.nb_outputs) {
            file.write_all(&format!("{}\n", self.bias_o.data()[i]).as_bytes()).expect(&format!("couldn't write to {}", display));
        }
        println!("successfully wrote to {}", display);
    }

    pub fn import_weights(&mut self, file: &String) {
        if !metadata(&file).expect("error: A problem occured with the file").is_file() {
            panic!("error: please provide a valid file");
        }
        let file = File::open(&file).expect("error: file not found");
        let lines: Vec<_> = BufReader::new(file).lines().collect();
    
        // [!] ALERT SHITTY PARSING [!] TO REFACTOR !!!
        let mut weights_ih: Vec<f64> = Vec::new();
        let mut weights_ho: Vec<f64> = Vec::new();
        let mut bias_h: Vec<f64> = Vec::new();
        let mut bias_o: Vec<f64> = Vec::new();
        let mut current: String = String::from("");
        for (i, line) in lines.into_iter().enumerate() {
            if let Ok(content) = line {
                if content.chars().next().unwrap() == '#' {
                    current = content;
                } else {
                    match &current[..] {
                        "#weights_ih" => {
                            weights_ih.push(content.parse::<f64>().expect("error: bad character"));
                        },
                        "#weights_ho" => {
                            weights_ho.push(content.parse::<f64>().expect("error: bad character"));
                        },
                        "#bias_h" => {
                            bias_h.push(content.parse::<f64>().expect("error: bad character"));
                        },
                        "#bias_o" => {
                            bias_o.push(content.parse::<f64>().expect("error: bad character"));
                        },
                        _ => panic!("error: bad weights file format")
                    }
                }
            }
        }
        // check nb of weights and bias
        self.weights_ih = Matrix::new(self.nb_hidden, self.nb_inputs, weights_ih);
        self.weights_ho = Matrix::new(self.nb_outputs, self.nb_hidden, weights_ho);
        self.bias_h = Matrix::new(self.nb_hidden, 1, bias_h);
        self.bias_o = Matrix::new(self.nb_outputs, 1, bias_o);
    }
}