extern crate rubik;

mod pruning;
use pruning::pruning::{Pruning};

fn main() {
    println!("Hey! Let's create some pruning tables");
    let mut pruning_tables: Pruning = Pruning::new();
    println!("slice_flip_pruning_table : {:?}", pruning_tables.slice_flip_pruning_table);
    println!("slice_twist_pruning_table : {:?}", pruning_tables.slice_twist_pruning_table);
    println!("phase2 : {:?}", pruning_tables.phase2);
}