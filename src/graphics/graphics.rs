// extern crate rulinalg;
extern crate nalgebra;
extern crate kiss3d;

// use rulinalg::matrix::{Matrix, BaseMatrixMut, BaseMatrix};
use nalgebra::{Translation3, Point3, Vector3, UnitQuaternion, Unit};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::camera::{FirstPerson, ArcBall, Camera};
use std::cell::RefCell;
use std::rc::Rc;

const C_GREY: (f32, f32, f32) = (0.11, 0.11, 0.11);
const C_RED: (f32, f32, f32) = (1.0, 0.0, 0.0);
const C_GREEN: (f32, f32, f32) = (0.0, 1.0, 0.0);
const C_BLUE: (f32, f32, f32) = (0.0, 0.0, 1.0);
const C_WHITE: (f32, f32, f32) = (1.0, 1.0, 1.0);
const C_YELLOW: (f32, f32, f32) = (1.0, 1.0, 0.0);
const C_ORANGE: (f32, f32, f32) = (1.0, 0.647, 0.0);

struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    fn from(x: f32, y: f32, z: f32) -> Vec3 {
        return Vec3 {
            x, y, z
        }
    }
}

enum Face {
    U, F, L, D, R, B
}

pub fn display_graphics() {
    let mut window: Window = Window::new("Rubik");
    window.set_background_color(C_GREY.0, C_GREY.1, C_GREY.2);
    let mut camera = ArcBall::new(Point3::new(-20.0, 10.0, -20.0), Point3::origin());
    window.set_light(Light::StickToCamera);

    let mut rubik: Vec<SceneNode> = Vec::new();

    let rubik_size: f32 = 10.0;
    let cubie_size: f32 = rubik_size / 3.0;
    let gap: f32 = cubie_size / 50.0;

    let mut index: usize = 0;
    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                let offset: f32 = (3.0 - 1.0) * cubie_size / 2.0;
                let pos: Vec3 = Vec3::from(
                    x as f32 * cubie_size - offset,
                    y as f32 * cubie_size - offset,
                    z as f32 * cubie_size - offset);
                let mut cubie: SceneNode = window.add_cube(cubie_size - gap, cubie_size - gap, cubie_size - gap);
                cubie.append_translation(&Translation3::new(pos.x, pos.y, pos.z));
                rubik.push(cubie);
                index += 1;
            }
        }
    }

    rubik[9].set_color(C_RED.0, C_RED.1, C_RED.2);


    let sequence: Vec<Face> = vec![Face::F, Face::R, Face::U, Face::B, Face::D];

    let speed: u8 = 1;
    let mut moves: usize = 0;
    let mut animating: bool = false;
    let mut target_angle: f32 = 0.0;
    let mut current_angle: f32 = 0.0;
    let mut current: ([usize; 9], Unit::<Vector3::<f32>>) = get_face_cubies(&sequence[moves]); // doublon
    while window.render_with_camera(&mut camera) {
        if !animating && moves < sequence.len() {
            current = get_face_cubies(&sequence[moves]); // doublon
            target_angle = 90.0; // to unmock, can be 180
            current_angle = 0.0;
            animating = true;
        } else if animating {
            let rot = UnitQuaternion::from_axis_angle(&current.1, radian(speed as f32));
            for index in 0..rubik.len() {
                if current.0.contains(&index) {
                    rubik[index].append_rotation(&rot);
                }
            }
            current_angle += speed as f32;
            if current_angle == target_angle {
                animating = false;
                moves += 1;
            }
        }
    }
}

fn get_face_cubies(face: &Face) -> ([usize; 9], Unit::<Vector3::<f32>>) {
    match face {
        Face::U => ([26, 17, 8, 25, 16, 7, 24, 15, 6], Vector3::<f32>::y_axis()),
        Face::F => ([24, 15, 6, 21, 12, 3, 18, 9, 0], Vector3::<f32>::z_axis()),
        Face::L => ([26, 25, 24, 23, 22, 21, 20, 19, 18], Vector3::<f32>::x_axis()),
        Face::D => ([20, 11, 2, 19, 10, 1, 18, 9, 0], Vector3::<f32>::y_axis()),
        Face::R => ([6, 7, 8, 3, 4, 5, 0, 1, 2], Vector3::<f32>::x_axis()),
        Face::B => ([26, 17, 8, 22, 14, 5, 20, 11, 2], Vector3::<f32>::z_axis()),
    }
}

fn radian(d_angle: f32) -> f32 {
    return d_angle * std::f32::consts::PI / 180.0;
}

fn degree(r_angle: f32) -> f32 {
    return r_angle * 180.0 / std::f32::consts::PI;
}

// fn add_cubie(window: &mut Window, pos: Vec3, cubie_size: f32) -> SceneNode {    
//     // let r: f32 = cubie_size / 2.0;
//     // let mut cubie: SceneNode = self.add_geom_with_name("cube", Vector3::new(wx, wy, wz)).expect("Unable to load the default cube geometry.");

//     // // cubie U
//     // let mut quad = cubie.add_quad(cubie_size, cubie_size, 1, 1);
//     // quad.append_rotation(&UnitQuaternion::from_axis_angle(&Vector3::x_axis(), radian(90.0)));
//     // quad.append_translation(&Translation3::new(0.0, -r, 0.0));
//     // quad.set_color(C_BLUE.0, C_BLUE.1, C_BLUE.2);

//     // // cubie F
//     // let mut quad = cubie.add_quad(cubie_size, cubie_size, 1, 1);
//     // quad.append_translation(&Translation3::new(0.0, 0.0, -r));
//     // quad.set_color(C_WHITE.0, C_WHITE.1, C_WHITE.2);

//     // // cubie L
//     // let mut quad = cubie.add_quad(cubie_size, cubie_size, 1, 1);
//     // quad.append_rotation(&UnitQuaternion::from_axis_angle(&Vector3::y_axis(), radian(90.0)));
//     // quad.append_translation(&Translation3::new(-r, 0.0, 0.0));
//     // quad.set_color(C_ORANGE.0, C_ORANGE.1, C_ORANGE.2);

//     // // cubie D
//     // let mut quad = cubie.add_quad(cubie_size, cubie_size, 1, 1);
//     // quad.append_rotation(&UnitQuaternion::from_axis_angle(&Vector3::x_axis(), radian(90.0)));
//     // quad.append_translation(&Translation3::new(0.0, r, 0.0));
//     // quad.set_color(C_GREEN.0, C_GREEN.1, C_GREEN.2);

//     // // cubie B
//     // let mut quad = cubie.add_quad(cubie_size, cubie_size, 1, 1);
//     // quad.append_translation(&Translation3::new(0.0, 0.0, r));
//     // quad.set_color(C_YELLOW.0, C_YELLOW.1, C_YELLOW.2);

//     // // cubie R
//     // let mut quad = cubie.add_quad(cubie_size, cubie_size, 1, 1);
//     // quad.append_rotation(&UnitQuaternion::from_axis_angle(&Vector3::y_axis(), radian(90.0)));
//     // quad.append_translation(&Translation3::new(r, 0.0, 0.0));
//     // quad.set_color(C_RED.0, C_RED.1, C_RED.2);

//     cubie.append_translation(&Translation3::new(pos.x, pos.y, pos.z));
//     return cubie;
// }