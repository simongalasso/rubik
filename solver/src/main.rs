use std::time::{Instant};

use rubik_lib::rubik::cubie_cube::{CubieCube};
use rubik_lib::pruning::pruning::{Pruning};
use rubik_lib::pruning::moves::{Moves};
use rubik_lib::algo::solve::*;
use rubik_lib::rubik::enums::*;

mod parsing;
mod gl_display;

use parsing::parse::*;
use parsing::args::{Config};
use gl_display::gl_display::{GlDisplay};

fn main() {
    let config: Config = Config::new();
    match parse_inputs(&config) {
        Ok(input_sequence) => {
            let pruning_tables: Pruning = Pruning::new();
            let moves_tables: Moves = Moves::new();
            println!("visualisator: {}{}", config.visualisator, match config.visualisator {
                true => format!(" | speed: {}", config.speed_selection),
                _ => String::from("")
            });
            println!("sequence: ({}) {}", input_sequence.len(), input_sequence.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" "));
            let mut cb_cube: CubieCube = CubieCube::new_solved();
            cb_cube.apply_sequence(&input_sequence);
            let start_time: std::time::Instant = Instant::now();
            match solve(&mut cb_cube, &pruning_tables, &moves_tables, start_time) {
                Ok(s) => {
                    println!("solution: ({}) {}", s.len(), s.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" "));
                    println!("duration: {:?}", start_time.elapsed());
                    if config.visualisator {
                        let mut gl_display: GlDisplay = GlDisplay::new(&config);
                        gl_display.launch(&input_sequence, s);
                    }
                },
                Err(error) => println!("{}", error)
            }
        },
        Err(error) => println!("{}", error)
    }
}