extern crate rubik;

use super::args::{Config};
use rubik::cubie_cube::{CubieCube};

pub fn parse_inputs(config: &Config) -> Vec<CubieCube> {
	let mut input_sequence: Vec<CubieCube> = Vec::new();
	for value in config.input.trim().split_whitespace() {
		match value.len() {
			1 | 2 => {
				input_sequence.push(CubieCube::from_action_str(value));
			},
			_ => panic!("error: bad input format")
		}
	}
	return input_sequence;
}