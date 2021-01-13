use std::fmt;

use crate::parsing::args::*;

#[derive(Debug, PartialEq)]
pub enum Face {
	F, R, U, B, L, D
}

impl Face {
	fn from(value: char) -> Option<Face> {
		return match value {
			'F' => Some(Face::F),
			'R' => Some(Face::R),
			'U' => Some(Face::U),
			'B' => Some(Face::B),
			'L' => Some(Face::L),
			'D' => Some(Face::D),
			_ => None
		}
	}
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub enum Rotation {
	R, L, D
}

impl Rotation {
	fn from(value: char) -> Option<Rotation> {
		return match value {
			' ' => Some(Rotation::R),
			'\'' => Some(Rotation::L),
			'2' => Some(Rotation::D),
			_ => None
		}
	}
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
			Rotation::R => '\0',
			Rotation::L => '\'',
			Rotation::D => '2'
		})
    }
}

#[derive(Debug, PartialEq)]
pub struct Action {
	pub face: Face,
	pub rot: Rotation
}

impl Action {
	fn from(face: Face, rot: Rotation) -> Action {
		return Action {
			face: face,
			rot: rot
		}
	}
}

pub fn parse(config: &Config) -> Vec<Action>{
	let mut input_sequence: Vec<Action> = Vec::new();
	for value in config.input.trim().split_whitespace() {
		match value.len() {
			1 | 2 => {
				let face: Face = Face::from(value.chars().nth(0).unwrap()).expect("error: bad character");
				let rotation: Rotation = Rotation::from(value.chars().nth(1).unwrap_or(' ')).expect("error: bad character");
				input_sequence.push(Action::from(face, rotation));
			},
			_ => panic!("error: bad input format")
		}
	}
	return input_sequence;
}