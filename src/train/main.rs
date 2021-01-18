extern crate rulinalg;

mod algo;
mod rubik;

use rubik::rubik::*;
use rubik::action::*;

fn main() {
    let mut rubik: Rubik = Rubik::new_shuffled(10);
    let next_action: Action = rubik.next_action();
    println!("chosen action: {:?}", next_action);
}