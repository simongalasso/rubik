extern crate rubik_lib;
extern crate rand;

use rand::Rng;
use rubik_lib::rubik::enums::*;
use super::args::{Config};

const MAX_SCRAMBLE: u8 = 50;

pub fn parse_inputs(config: &Config) -> Result<Vec<usize>, String> {
	match &config.input {
		Some(input) => {
			if !["slow", "normal", "fast"].iter().any(|keyword| *keyword == &config.speed_selection) {
				return Err(String::from("error: use a valid speed selection keyword"));
			}
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
		},
		None => {
			let mut random_sequence: Vec<usize> = vec![];
			for _ in 0..rand::thread_rng().gen_range(1, MAX_SCRAMBLE) {
				let available_actions: Vec<usize> = (0..18).filter(|i| {
					random_sequence.last().is_none() || (
						ACTIONS_LIST[*random_sequence.last().unwrap()].0 != ACTIONS_LIST[*i].0
						&& ACTIONS_LIST[*random_sequence.last().unwrap()].0 != ACTIONS_LIST[ACTION_INVERSE[*i]].0)
				}).collect();
				let rand_action: usize = available_actions[rand::thread_rng().gen_range(0, available_actions.len())];
				random_sequence.push(rand_action);
			}
			return Ok(random_sequence);
		}
	}
}