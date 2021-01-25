extern crate rubik;

use rubik::action::*;

pub fn display_sequence(prepend: &str, sequence: Option<Vec<Action>>) {
    if sequence.is_none() {
        println!("No solution found"); // handle this in the main instead of here, this function should be generic
    } else {
        print!("{}", prepend);
        for action in sequence.unwrap().iter() {
            print!("{} ", action.to_string());
        }
        println!();
    }
}