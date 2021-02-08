extern crate rubik;

mod pruning;

use rubik::cubie_cube::{CubieCube};
use pruning::pruning::{create_phase1_prun_table};

fn main() {
    println!("Hey! Let's create some pruning tables");
    create_phase1_prun_table();
}