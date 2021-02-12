extern crate rubik_lib;
use rubik_lib::rubik::enums::{ACTIONS_STR_LIST};

use super::args::{Config};

pub fn parse_inputs(config: &Config) -> Vec<usize> {
	let mut input_sequence: Vec<usize> = Vec::new();
	for value in config.input.trim().split_whitespace() {
		match value.len() {
			1 | 2 => {
				input_sequence.push(ACTIONS_STR_LIST.iter().position(|v| *v == value).unwrap())
			},
			_ => panic!("error: parse_inputs(), bad input format")
		}
	}
	return input_sequence;
}