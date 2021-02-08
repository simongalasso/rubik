extern crate rubik;

use super::args::{Config};
use rubik::cubie_cube::{CubieCube};

pub fn parse_inputs(config: &Config) -> Vec<(CubieCube, u8)> {
	let mut input_sequence: Vec<(CubieCube, u8)> = Vec::new();
	for value in config.input.trim().split_whitespace() {
		match value.len() {
			1 | 2 => {
				input_sequence.push(CubieCube::from_action_str(value));
			},
			_ => panic!("error: parse_inputs(), bad input format")
		}
	}
	return input_sequence;
}