extern crate rubik_lib;
use rubik_lib::rubik::enums::{ACTIONS_STR_LIST};

pub fn parse_inputs(input: &String) -> Result<Vec<usize>, String> {
	let mut input_sequence: Vec<usize> = Vec::new();
	for value in input.trim().split_whitespace() {
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