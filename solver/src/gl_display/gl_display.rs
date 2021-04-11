extern crate rand;
extern crate nalgebra;
extern crate kiss3d;

use nalgebra::{Vector3, Point3, Point2, UnitQuaternion, Unit};
use kiss3d::text::{Font};
use kiss3d::light::{Light};
use kiss3d::window::{Window};
use kiss3d::camera::{ArcBall};
use kiss3d::event::{WindowEvent, Key};

use crate::parsing::args::{Config};

use super::gl_rubik::{GlRubik};
use super::cubie::{Cubie};

use rubik_lib::rubik::enums::*;

pub const C_GREY: (f32, f32, f32) = (0.09, 0.09, 0.09);
pub const C_RED: (f32, f32, f32) = (1.0, 0.15, 0.15);
pub const C_GREEN: (f32, f32, f32) = (0.0, 0.60784, 0.28235);
pub const C_BLUE: (f32, f32, f32) = (0.0, 0.27450, 0.67843);
pub const C_WHITE: (f32, f32, f32) = (1.0, 1.0, 1.0);
pub const C_YELLOW: (f32, f32, f32) = (1.0, 1.0, 0.0);
pub const C_ORANGE: (f32, f32, f32) = (1.0, 0.4, 0.1);
pub const C_BLACK: (f32, f32, f32) = (0.0, 0.0, 0.0);

pub const STATUS_ORDERED: u8 = 0;
pub const STATUS_SCRAMBLED: u8 = 2;

const ANGLES: [f32; 3] = [90.0, 180.0, -90.0];

pub struct GlDisplay {
    pub window: Window,
    pub camera: ArcBall,
    pub started: bool,
    pub rotating: bool,
    pub animating: bool,
    pub speed: f32,
    pub moves: usize,
    pub status: u8
}

impl GlDisplay {
    pub fn new(config: &Config) -> Self {
        let mut window: Window = Window::new("Rubik");
        window.set_framerate_limit(Some(60));
        window.set_background_color(C_GREY.0, C_GREY.1, C_GREY.2);
        let mut camera: ArcBall = ArcBall::new(Point3::new(-6.0, 3.0, -6.0), Point3::origin());
        camera.rebind_drag_button(None);
        camera.set_min_dist(5.0);
        camera.set_max_dist(50.0);
        camera.set_dist_step(5.0);
        window.set_light(Light::StickToCamera);
        return Self {
            window, camera,
            started: false,
            rotating: true,
            animating: false,
            speed: match &config.speed_selection[..] { // doit impérativement être un diviseur de 180
                "slow" => 1.0,
                "normal" => 3.0,
                "fast" => 6.0,
                _ => 6.0
            },
            moves: 0,
            status: STATUS_ORDERED
        }
    }

    pub fn handle_events(&mut self) {
        for mut event in self.window.events().iter() {
            match event.value {
                WindowEvent::Key(button, kiss3d::event::Action::Release, _) => {
                    event.inhibited = true;
                    match button {
                        Key::Escape => self.window.close(),
                        Key::Return if !self.started => {
                            self.status = match self.status {
                                STATUS_ORDERED => STATUS_SCRAMBLED,
                                _ => STATUS_ORDERED
                            };
                            self.started = true;
                        },
                        Key::Space => self.rotating = !self.rotating,
                        _ => {}
                    }
                    
                },
                _ => {}
            }
        }
    }

    pub fn launch(&mut self, input_sequence: &Vec<usize>, solution: Vec<usize>) {
        let mut gl_rubik: GlRubik = GlRubik::new(&mut self.window);
        let font = Font::default();
        let rubik_rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), (0.5 as f32).to_radians());
        let mut sequence: Vec<usize> = input_sequence.clone();
        let mut current_angle: f32 = 0.0;
        let mut current_cubies: Vec<Cubie> = gl_rubik.get_face_cubies(&ACTIONS_STR_LIST[sequence[self.moves]]); // stupid init, find something else
        let mut current_axis: Unit::<Vector3::<f32>> = gl_rubik.get_face_axis(&ACTIONS_STR_LIST[sequence[self.moves]]); // stupid init, find something else
        while self.window.render_with_camera(&mut self.camera) {
            self.handle_events();
            if !self.started {
                self.window.draw_text(&format!("press [enter] to {}", match self.status {
                    STATUS_ORDERED => "launch the shuffle sequence",
                    _ => "order the rubik's cub"
                }), &Point2::new(15.0, 15.0), 38.0, &font, &Point3::new(1.0, 1.0, 1.0));
            } else {
                self.window.draw_text("waiting...", &Point2::new(15.0, 15.0), 38.0, &font, &Point3::new(1.0, 1.0, 1.0));
                if !self.animating {
                    if self.moves == sequence.len() {
                        sequence = match self.status {
                            STATUS_SCRAMBLED => solution.clone(),
                            _ => input_sequence.clone()
                        };
                        self.started = false;
                        self.moves = 0;
                    }
                    current_cubies = gl_rubik.get_face_cubies(&ACTIONS_STR_LIST[sequence[self.moves]]);
                    current_axis = gl_rubik.get_face_axis(&ACTIONS_STR_LIST[sequence[self.moves]]);
                    current_angle = 0.0;
                    self.animating = true;
                } else {
                    let angle = ANGLES[(ACTIONS_LIST[sequence[self.moves]].1 - 1) as usize].signum() * self.speed;
                    let rot: UnitQuaternion<f32> = UnitQuaternion::from_axis_angle(&current_axis, angle.to_radians());
                    for cubie in current_cubies.iter_mut() {
                        cubie.rotate(rot);
                    }
                    current_angle += angle;
                    if current_angle == ANGLES[(ACTIONS_LIST[sequence[self.moves]].1 - 1) as usize] {
                        self.animating = false;
                        self.moves += 1;
                    }
                }
            }
            if self.rotating {
                gl_rubik.scene_node.append_rotation(&rubik_rot);
            }
        }
    }
}