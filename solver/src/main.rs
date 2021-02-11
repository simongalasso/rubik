use std::time::{Instant};

extern crate rubik_lib;

use rubik_lib::rubik::cubie_cube::{CubieCube};
use rubik_lib::pruning::pruning::{Pruning};
use rubik_lib::algo::solve::*;
use rubik_lib::rubik::enums::{ACTIONS_STR_LIST};

mod parsing;

use parsing::parse::*;
use parsing::args::{Config};
 
fn main() {
    let config: Config = Config::new();
    let pruning_tables: Pruning = Pruning::new();
    
    let input_sequence: Vec<usize> = parse_inputs(&config);
    println!("visualisator: {}{}", config.visualisator, if config.visualisator { format!(" | speed: {}", config.speed_selection) } else { String::from("") });
    println!("sequence: {}", input_sequence.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" "));

    let mut cb_cube: CubieCube = CubieCube::new_solved();
    cb_cube.apply_sequence(&input_sequence);
    let very_start_time: std::time::Instant = Instant::now();
    match solve(&mut cb_cube, pruning_tables) {
        Some(solution) => {
            eprintln!("solution: {}", solution.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" "));
            eprintln!("solved: {:?}", very_start_time.elapsed());
        },
        None => println!("Search timed out without finding any solution")
    }
}