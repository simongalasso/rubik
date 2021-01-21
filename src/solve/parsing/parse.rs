use crate::parsing::args::*;
use crate::graphics::action::*;

pub fn parse(config: &Config) -> Vec<Action> {
	let mut input_sequence: Vec<Action> = Vec::new();
	for value in config.input.trim().split_whitespace() {
		match value.len() {
			1 | 2 => {
				let face: Face = Face::from_char(value.chars().nth(0).unwrap()).expect("error: bad character");
				let rotation: f32 = get_rotation(value.chars().nth(1).unwrap_or('\0')).expect("error: bad character");
				input_sequence.push(Action::new(face, rotation));
			},
			_ => panic!("error: bad input format")
		}
	}
	return input_sequence;
}

fn get_rotation(value: char) -> Option<f32> {
	return match value {
		'\0' => Some(90.0),
		'2' => Some(180.0),
        '\'' => Some(-90.0),
        _ => None
	}
}