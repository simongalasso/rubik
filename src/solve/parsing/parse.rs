extern crate rubik;

use super::args::*;
use rubik::action::*;
use rubik::face::*;
use rubik::rotation::*;

pub fn parse(config: &Config) -> Vec<Action> {
	let mut input_sequence: Vec<Action> = Vec::new();
	for value in config.input.trim().split_whitespace() {
		match value.len() {
			1 | 2 => {
				let face: Face = Face::from_char(value.chars().nth(0).unwrap()).expect("error: bad character"); // handle first unwrap
				let rotation: Rotation = Rotation::from_char(value.chars().nth(1).unwrap_or('\0')).expect("error: bad character");
				input_sequence.push(Action::new(face, rotation));
			},
			_ => panic!("error: bad input format")
		}
	}
	return input_sequence;
}