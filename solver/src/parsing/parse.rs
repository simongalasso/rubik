extern crate rubik_lib;

use rubik_lib::rubik::enums::{ACTIONS_STR_LIST};
use super::args::{Config};

pub fn parse_inputs(config: &Config) -> Result<Vec<usize>, String> {
	if !["slow", "normal", "fast"].iter().any(|keyword| *keyword == &config.speed_selection) {
		return Err(String::from("error: use a valid speed selection keyword"));
	}
	let mut input_sequence: Vec<usize> = Vec::new();
	for value in config.input.trim().split_whitespace() {
		match value.len() {
			1 | 2 => {
				match ACTIONS_STR_LIST.iter().position(|v| *v == value) {
					Some(index) => input_sequence.push(index),
					None => return Err(String::from("error: the sequence contains a bad value"))
				}
			},
			_ => return Err(format!("error: the sequence contains a bad value \"{}\"", value))
		}
	}
	return match !input_sequence.is_empty() {
		true => Ok(input_sequence),
		false => Err(String::from("error: the sequence should not be empty"))
	};
}