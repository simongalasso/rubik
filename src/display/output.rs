use crate::parsing::parse::*;

pub fn display_sequence(prepend: &str, sequence: &Vec<Action>) {
    print!("{}", prepend);
    for action in sequence.iter() {
        print!("{}{} ", action.face.to_string(), action.rot.to_string());
    }
    println!();
}