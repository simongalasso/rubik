// use crate::parsing::args::*;
// use crate::action::action::*;

// pub fn parse(config: &Config) -> Vec<Action>{
// 	let mut input_sequence: Vec<Action> = Vec::new();
// 	for value in config.input.trim().split_whitespace() {
// 		match value.len() {
// 			1 | 2 => {
// 				let face: Face = Face::from(value.chars().nth(0).unwrap()).expect("error: bad character");
// 				let rotation: u8 = get_rotation(value.chars().nth(1).unwrap_or(' ')).expect("error: bad character");
// 				input_sequence.push(Action::from(face, rotation));
// 			},
// 			_ => panic!("error: bad input format")
// 		}
// 	}
// 	return input_sequence;
// }

// fn get_rotation(value: char) {
// 	return match value {
// 		' ' => 1,
// 		'2' => 2,
// 		'\'' => 3
// 	}
// }