extern crate rubik;

mod pruning;
use pruning::pruning::{Pruning};

fn main() {
    println!("Hey! Let's create some pruning tables");
    let mut pruning_tables: Pruning = Pruning::new();
    println!("Phase1 : {:?}", pruning_tables.phase1);
    println!("Phase2 : {:?}", pruning_tables.phase2);
}