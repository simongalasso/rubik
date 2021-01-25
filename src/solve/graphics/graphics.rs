extern crate nalgebra;
extern crate kiss3d;
extern crate rubik;

use std::cmp::Ordering;
use nalgebra::{Point3, Vector3, UnitQuaternion, Unit};
use kiss3d::light::Light;
use kiss3d::window::Window;
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

pub struct RubikGraphics {
    pub window: Window,
    pub camera: ArcBall,
}

impl RubikGraphics {
    pub fn new(title: &str, color: (f32, f32, f32)) -> RubikGraphics {
        let mut window: Window = Window::new("Rubik");
        window.set_framerate_limit(Some(60));
        window.set_background_color(C_GREY.0, C_GREY.1, C_GREY.2);
        let mut camera: ArcBall = ArcBall::new(Point3::new(-6.0, 3.0, -6.0), Point3::origin());
        camera.rebind_drag_button(None);
        camera.set_min_dist(5.0);
        camera.set_max_dist(50.0);
        camera.set_dist_step(5.0);
        window.set_light(Light::StickToCamera);
        return RubikGraphics {
            window, camera
        }
    }
    
    pub fn g_rubik_setup(&mut self) -> (RubikState, SceneNode, Vec<Cubie>) {
        let mut rubik_state: RubikState = SOLVED_STATE;
        let mut rubik: SceneNode = self.window.add_group();
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
        return (rubik_state, rubik, cubies);
    }
    
    pub fn get_face_cubies(cubies: &Vec<Cubie>, face: &Face) -> (Vec<Cubie>, Unit::<Vector3::<f32>>) { // To optimise
        match face {
            Face::U => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().y) == 1.0).collect::<Vec<Cubie>>(), -Vector3::<f32>::y_axis()),
            Face::R => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().x) == -1.0).collect::<Vec<Cubie>>(), Vector3::<f32>::x_axis()),
            Face::F => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().z) == -1.0).collect::<Vec<Cubie>>(), Vector3::<f32>::z_axis()),
            Face::D => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().y) == -1.0).collect::<Vec<Cubie>>(), Vector3::<f32>::y_axis()),
            Face::L => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().x) == 1.0).collect::<Vec<Cubie>>(), -Vector3::<f32>::x_axis()),
            Face::B => (cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().z) == 1.0).collect::<Vec<Cubie>>(), -Vector3::<f32>::z_axis()),
        }
    }
}