extern crate rubik;
extern crate kiss3d;

mod parsing;
mod graphics;
mod display;
mod algo;

use std::cmp::Ordering;
use graphics::cubie::Cubie;
use graphics::graphics::*;
use algo::kociemba::*;
use rubik::face::*;
use rubik::rotation::*;
use rubik::rubik_state::*;
use rubik::action::*;
use parsing::args::*;
use parsing::parse::*;
use display::output::*;
use kiss3d::window::Window;
use kiss3d::camera::{ArcBall};
use nalgebra::{Point3, Vector3, UnitQuaternion, Unit};
use kiss3d::light::Light;
use kiss3d::event::{WindowEvent, Key};
use kiss3d::scene::SceneNode;

fn main() {
    let config: Config = Config::new();
    let input_sequence: Vec<Action> = parse(&config);

    display_sequence("shuffle: ", Some(input_sequence.clone())); // do something better than clone
    println!("visualisator: {}{}", config.visualisator, if config.visualisator { format!(" | speed: {}", config.speed_selection) } else { String::from("") });

    // let solution: Option<Vec<Action>> = solve(&state);
    // display_sequence("solution: ", solution);

    if config.visualisator {
        let mut rubik_graphics: RubikGraphics = RubikGraphics::new("Rubik", C_GREY);
        let (mut rubik_state, mut rubik, mut cubies) = rubik_graphics.g_rubik_setup();

        let rubik_rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), (0.5 as f32).to_radians());
        let speed: f32 = match &config.speed_selection[..] { // doit impérativement être un diviseur de 180
            "slow" => 1.0,
            "normal" => 3.0,
            "fast" => 6.0,
            _ => 6.0
        };
        let mut moves: usize = 0;
        let mut started: bool = false;
        let mut solve_started: bool = false;
        let mut rotating: bool = true;
        let mut animating: bool = false;
        let mut current_angle: f32 = 0.0;
        let mut current_cubies: (Vec<Cubie>, Unit::<Vector3::<f32>>) = RubikGraphics::get_face_cubies(&cubies, &input_sequence[moves].face); // stupid init, find something else
        let mut next_action: Action = Action::new(Face::F, Rotation::R); // stupid init, find something else
        while rubik_graphics.window.render_with_camera(&mut rubik_graphics.camera) {
            for mut event in rubik_graphics.window.events().iter() {
                match event.value {
                    WindowEvent::Key(button, kiss3d::event::Action::Release, _) => {
                        event.inhibited = true;
                        match button {
                            Key::Escape => rubik_graphics.window.close(),
                            Key::Return => {
                                if !started {
                                    started = true;
                                } else {
                                    solve_started = true;
                                }
                            },
                            Key::Space => {
                                rotating = !rotating;
                            },
                            _ => {}
                        }
                        
                    },
                    _ => {}
                }
            }
            if started {
                if moves < input_sequence.len() {
                    if !animating {
                        current_cubies = RubikGraphics::get_face_cubies(&cubies, &input_sequence[moves].face);
                        current_angle = 0.0;
                        animating = true;
                    } else if animating {
                        let angle = input_sequence[moves].rot.to_angle().unwrap().signum() * speed;
                        let rot: UnitQuaternion<f32> = UnitQuaternion::from_axis_angle(&(current_cubies.1), angle.to_radians());
                        for cubie in current_cubies.0.iter_mut() {
                            cubie.rotate(rot);
                        }
                        current_angle += angle;
                        if current_angle == input_sequence[moves].rot.to_angle().unwrap() {
                            rubik_state = input_sequence[moves].apply_to(&rubik_state);
                            animating = false;
                            moves += 1;
                        }
                    }
                } else if solve_started && rubik_state != SOLVED_STATE {
                    if !animating {
                        next_action = Action::pick_random();
                        current_cubies = RubikGraphics::get_face_cubies(&cubies, &next_action.face);
                        current_angle = 0.0;
                        animating = true;
                    } else if animating {
                        let angle = next_action.rot.to_angle().unwrap().signum() * speed;
                        let rot: UnitQuaternion<f32> = UnitQuaternion::from_axis_angle(&(current_cubies.1), angle.to_radians());
                        for cubie in current_cubies.0.iter_mut() {
                            cubie.rotate(rot);
                        }
                        current_angle += angle;
                        if current_angle == next_action.rot.to_angle().unwrap() {
                            rubik_state = next_action.apply_to(&rubik_state);
                            animating = false;
                        }
                    }
                }
            }
            if rotating {
                rubik.append_rotation(&rubik_rot);
            }
        }
    }
}