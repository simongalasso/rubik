use crate::graphics::action::*;

pub fn display_sequence(prepend: &str, sequence: &Vec<Action>) {
    print!("{}", prepend);
    for action in sequence.iter() {
        print!("{} ", action.to_string());
    }
    println!();
}