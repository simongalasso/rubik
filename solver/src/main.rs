extern crate rand;

use rand::Rng;

use rubik_lib::rubik::cubie_cube::{CubieCube};
use rubik_lib::pruning::pruning::{Pruning};
use rubik_lib::pruning::moves::{Moves};

use rubik_lib::algo::solve::*;
use rubik_lib::rubik::enums::{ACTIONS_STR_LIST};
use nalgebra::{Vector3, Point3, Point2, UnitQuaternion, Unit};
use kiss3d::text::{Font};
use std::time::{Instant};

mod parsing;
mod display;

use parsing::parse::*;
use parsing::args::{Config};
use display::display::*;
use display::gl_rubik::{GlRubik};
use display::cubie::{Cubie};

pub const ANGLES: [f32; 3] = [90.0, 180.0, -90.0];

fn main() {
    let config: Config = Config::new();
    let pruning_tables: Pruning = Pruning::new();
    let moves_tables: Moves = Moves::new();
    let mut input_sequence: Vec<usize> = parse_inputs(&config);
    println!("visualisator: {}{}", config.visualisator, if config.visualisator { format!(" | speed: {}", config.speed_selection) } else { String::from("") });
    println!("sequence: {}", input_sequence.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" "));

    // for i in 0..moves_tables.ud_e_p_moves.len() {
    //     if moves_tables.ud_e_p_moves[i] == 0 {
    //         println!("error");
    //     }
    // }

    const LOOPS: usize = 1;
    const MAX_SCRAMBLE: usize = 50;

    let very_start_time: std::time::Instant = Instant::now();
    for loop_idx in 0..LOOPS {
        eprintln!("- {} -\ninput_sequence: {:?}", loop_idx, input_sequence);
        let mut cb_cube: CubieCube = CubieCube::new_solved();
        cb_cube.apply_sequence(&input_sequence);
        let start_time: std::time::Instant = Instant::now();
        let mut solution: Vec<usize> = Vec::new();
        match solve(&mut cb_cube, &pruning_tables, &moves_tables) {
            Some(s) => {
                eprintln!("solution: {}", s.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" "));
                eprintln!("duration: {:?}", start_time.elapsed());
                solution = s.clone();
            },
            None => println!("Search timed out without finding any solution")
        }

        if config.visualisator {
            let mut display: Display = Display::new(&config);
            let mut gl_rubik: GlRubik = GlRubik::new(&mut display.window);
            let font = Font::default();

            let rubik_rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), (0.5 as f32).to_radians());
            let mut sequence: Vec<usize> = input_sequence.clone();

            let mut current_angle: f32 = 0.0;
            let mut current_cubies: Vec<Cubie> = gl_rubik.get_face_cubies(&ACTIONS_STR_LIST[sequence[display.moves]]); // stupid init, find something else
            let mut current_axis: Unit::<Vector3::<f32>> = gl_rubik.get_face_axis(&ACTIONS_STR_LIST[sequence[display.moves]]); // stupid init, find something else

            while display.window.render_with_camera(&mut display.camera) {
                // display.window.draw_text(&(sequence.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" "))[..], &Point2::new(15.0, 15.0), 42.0, &font, &Point3::new(1.0, 1.0, 1.0));
                match display.status {
                    STATUS_ORDERED => display.window.draw_text("press [enter] to scramble", &Point2::new(15.0, 15.0), 42.0, &font, &Point3::new(1.0, 1.0, 1.0)),
                    STATUS_SCRAMBLING => display.window.draw_text("scrambling...", &Point2::new(15.0, 15.0), 42.0, &font, &Point3::new(1.0, 1.0, 1.0)),
                    STATUS_SCRAMBLED => display.window.draw_text("press [enter] to solve", &Point2::new(15.0, 15.0), 42.0, &font, &Point3::new(1.0, 1.0, 1.0)),
                    STATUS_ORDERING => display.window.draw_text("ordering...", &Point2::new(15.0, 15.0), 42.0, &font, &Point3::new(1.0, 1.0, 1.0)),
                    _ => {}
                }
                display.handle_events();
                if display.started {
                    if !display.animating {
                        if display.moves == sequence.len() {
                            match display.status {
                                STATUS_ORDERED => display.status = STATUS_SCRAMBLING,
                                STATUS_SCRAMBLING => display.status = STATUS_SCRAMBLED,
                                STATUS_SCRAMBLED => display.status = STATUS_ORDERING,
                                STATUS_ORDERING => display.status = STATUS_ORDERED,
                                _ => {}
                            }
                            display.started = false;
                            display.moves = 0;
                            sequence = solution.clone();
                        }
                    }
                    if display.rotating {
                        gl_rubik.scene_node.append_rotation(&rubik_rot);
                    }
                }
            }
        }
        input_sequence = (0..rand::thread_rng().gen_range(1, MAX_SCRAMBLE)).map(|_| rand::thread_rng().gen_range(0, 17)).collect();
    }
    eprintln!("total duration: {:?}", very_start_time.elapsed());
}