extern crate rulinalg;
extern crate nalgebra;
extern crate kiss3d;
extern crate rubik;

use nn::{NN};

use std::cmp::Ordering;
use nalgebra::{Point3, Vector3, UnitQuaternion, Unit};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::camera::{ArcBall};
use kiss3d::event::{WindowEvent, Key};
use rubik::face::*;
use rubik::rotation::*;
use rubik::rubik_state::*;

use super::cubie::Cubie;
use rubik::action::*;

pub const C_GREY: (f32, f32, f32) = (0.09, 0.09, 0.09);
pub const C_RED: (f32, f32, f32) = (1.0, 0.15, 0.15);
pub const C_GREEN: (f32, f32, f32) = (0.0, 0.60784, 0.28235);
pub const C_BLUE: (f32, f32, f32) = (0.0, 0.27450, 0.67843);
pub const C_WHITE: (f32, f32, f32) = (1.0, 1.0, 1.0);
pub const C_YELLOW: (f32, f32, f32) = (1.0, 1.0, 0.0);
pub const C_ORANGE: (f32, f32, f32) = (1.0, 0.4, 0.1);
pub const C_BLACK: (f32, f32, f32) = (0.0, 0.0, 0.0);

pub fn display_graphics(sequence: &Vec<Action>, speed_selection: String, nn: &mut NN/*NeuralNetwork*/) {
    let mut window: Window = Window::new("Rubik");
    window.set_background_color(C_GREY.0, C_GREY.1, C_GREY.2);
    window.set_framerate_limit(Some(60));
    let mut camera = ArcBall::new(Point3::new(-6.0, 3.0, -6.0), Point3::origin());
    camera.rebind_drag_button(None);
    camera.set_min_dist(5.0);
    camera.set_max_dist(50.0);
    camera.set_dist_step(5.0);
    window.set_light(Light::StickToCamera);
    
    // could be nice to merge the two rubik representations
    let mut rubik_state: RubikState = SOLVED_STATE;
    let mut rubik: SceneNode = window.add_group();
    let mut cubies: Vec<Cubie> = Vec::new();
    let scale: f32 = 3.0;
    let gap: f32 = 0.05;

    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                let pos: Vector3<f32> = Vector3::new(x as f32, y as f32, z as f32);
                cubies.push(Cubie::new(&mut rubik, 1.0, pos, scale, gap));
            }
        }
    }

    let rubik_rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), (0.5 as f32).to_radians());

    let speed: f32 = match &speed_selection[..] { // doit impérativement être un diviseur de 180
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
    let mut current_cubies: (Vec<Cubie>, Unit::<Vector3::<f32>>) = get_face_cubies(&cubies, &sequence[moves].face); // stupid init, find something else
    let mut next_action: Action = Action::new(Face::F, Rotation::R); // stupid init, find something else
    while window.render_with_camera(&mut camera) {
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(button, kiss3d::event::Action::Release, _) => {
                    event.inhibited = true;
                    match button {
                        Key::Escape => window.close(),
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
            if moves < sequence.len() {
                if !animating {
                    current_cubies = get_face_cubies(&cubies, &sequence[moves].face);
                    current_angle = 0.0;
                    animating = true;
                } else if animating {
                    let angle = sequence[moves].rot.to_angle().unwrap().signum() * speed;
                    let rot: UnitQuaternion<f32> = UnitQuaternion::from_axis_angle(&(current_cubies.1), angle.to_radians());
                    for cubie in current_cubies.0.iter_mut() {
                        cubie.rotate(rot);
                    }
                    current_angle += angle;
                    if current_angle == sequence[moves].rot.to_angle().unwrap() {
                        rubik_state = sequence[moves].apply_to(&rubik_state);
                        animating = false;
                        moves += 1;
                    }
                }
            } else if solve_started && rubik_state != SOLVED_STATE {
                if !animating {
                    // println!("{:.5?}", rubik_state.aligned_format());
                    // let results: Matrix<f64> = nn.feedforward(Matrix::new(40, 1, rubik_state.aligned_format()));
                    let results = nn.run(&(rubik_state.aligned_format())[..]);
                    let p: Vec<f64> = (&results[1..]).to_vec();
                    // let p: Vec<f64> = (&results.data()[1..]).to_vec();
                    let policy: usize = p.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal)).map(|(index, _)| index).unwrap(); // handle unwrap
                    println!("policy: {}", policy);
                    next_action = Action::get_actions()[policy].clone();
                    println!("next_action: {}", next_action.to_string());
                    current_cubies = get_face_cubies(&cubies, &next_action.face);
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

fn get_face_cubies(cubies: &Vec<Cubie>, face: &Face) -> (Vec<Cubie>, Unit::<Vector3::<f32>>) { // To optimise
    match face {
        Face::U => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().y) == 1.0).collect::<Vec<Cubie>>(), -Vector3::<f32>::y_axis()),
        Face::R => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().x) == -1.0).collect::<Vec<Cubie>>(), Vector3::<f32>::x_axis()),
        Face::F => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().z) == -1.0).collect::<Vec<Cubie>>(), Vector3::<f32>::z_axis()),
        Face::D => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().y) == -1.0).collect::<Vec<Cubie>>(), Vector3::<f32>::y_axis()),
        Face::L => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().x) == 1.0).collect::<Vec<Cubie>>(), -Vector3::<f32>::x_axis()),
        Face::B => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().z) == 1.0).collect::<Vec<Cubie>>(), -Vector3::<f32>::z_axis()),
    }
}