extern crate kiss3d;
extern crate rubik;

mod parsing;
mod display;
mod algo;
mod pruning;

use pruning::pruning::{Pruning};
use algo::solve::*;
use parsing::parse::*;
use display::display::{Display};
use display::gl_rubik::{GlRubik};
use display::cubie::{Cubie};
use parsing::args::{Config};
use rubik::cubie_cube::{CubieCube};
use rubik::enums::*;
use nalgebra::{Vector3, UnitQuaternion, Unit};
use rubik::enums::{ACTIONS_STR_LIST};
use std::time::{Instant};

pub const ANGLES: [f32; 3] = [90.0, 180.0, -90.0];
 
fn main() {
    let config: Config = Config::new();
    let pruning_tables: Pruning = Pruning::new();
    
    // println!("flip_pruning_table : {:?}", pruning_tables.flip_pruning_table);
    // println!("twist_pruning_table : {:?}", pruning_tables.twist_pruning_table);
    // println!("uds_e_location_pruning_table : {:?}", pruning_tables.uds_e_location_pruning_table);
    // println!("c_p_pruning_table : {:?}", pruning_tables.c_p_pruning_table);
    // println!("ud_e_p_pruning_table : {:?}", pruning_tables.ud_e_p_pruning_table);
    // println!("uds_e_sorted_pruning_table : {:?}", pruning_tables.uds_e_sorted_pruning_table);

    // for i in 0..pruning_tables.ud_e_p_pruning_table.len() {
    //     if  pruning_tables.ud_e_p_pruning_table[i] > 8 {
    //         println!("depth = {}",  pruning_tables.ud_e_p_pruning_table[i]);
    //     }
    // }

    let input_sequence: Vec<usize> = parse_inputs(&config);
    println!("visualisator: {}{}", config.visualisator, if config.visualisator { format!(" | speed: {}", config.speed_selection) } else { String::from("") });
    println!("sequence: {}", input_sequence.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" "));

    let mut cb_cube: CubieCube = CubieCube::new_solved();
    cb_cube.apply_sequence(&input_sequence);
    let very_start_time: std::time::Instant = Instant::now();
    let mut solution: Vec<usize> = Vec::new();
    match solve(&mut cb_cube, pruning_tables) {
        Some(s) => {
            eprintln!("solution: {}", s.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" "));
            eprintln!("solved: {:?}", very_start_time.elapsed());
            solution = s.clone();
        },
        None => println!("Search timed out without finding any solution")
    }

    if config.visualisator {
        let mut display: Display = Display::new(&config);
        let mut gl_rubik: GlRubik = GlRubik::new(&mut display.window);

        let rubik_rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), (0.5 as f32).to_radians());

        let mut sequence: Vec<usize> = input_sequence.clone();

        let mut current_angle: f32 = 0.0;
        let mut current_cubies: Vec<Cubie> = gl_rubik.get_face_cubies(&ACTIONS_STR_LIST[sequence[display.moves]]); // stupid init, find something else
        let mut current_axis: Unit::<Vector3::<f32>> = gl_rubik.get_face_axis(&ACTIONS_STR_LIST[sequence[display.moves]]); // stupid init, find something else
        while display.window.render_with_camera(&mut display.camera) {
            display.handle_events();
            if display.started {
                if !display.animating {
                    if display.moves == sequence.len() {
                        display.started = false;
                        display.moves = 0;
                        sequence = solution.clone();
                    }
                    current_cubies = gl_rubik.get_face_cubies(&ACTIONS_STR_LIST[sequence[display.moves]]);
                    current_axis = gl_rubik.get_face_axis(&ACTIONS_STR_LIST[sequence[display.moves]]);
                    current_angle = 0.0;
                    display.animating = true;
                } else if display.animating {
                    let angle = ANGLES[(ACTIONS_LIST[sequence[display.moves]].1 - 1) as usize].signum() * display.speed;
                    let rot: UnitQuaternion<f32> = UnitQuaternion::from_axis_angle(&current_axis, angle.to_radians());
                    for cubie in current_cubies.iter_mut() {
                        cubie.rotate(rot);
                    }
                    current_angle += angle;
                    if current_angle == ANGLES[(ACTIONS_LIST[sequence[display.moves]].1 - 1) as usize] {
                        display.animating = false;
                        display.moves += 1;
                    }
                }
            }
            if display.rotating {
                gl_rubik.scene_node.append_rotation(&rubik_rot);
            }
        }
    }
}