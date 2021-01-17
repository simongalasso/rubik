// extern crate rulinalg;
extern crate nalgebra;
extern crate kiss3d;

// use rulinalg::matrix::{Matrix, BaseMatrixMut, BaseMatrix};
use nalgebra::{Translation3, Point3, Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;

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

pub fn display_graphics() {
    let mut window: Window = Window::new("Rubik");
    window.set_background_color(C_GREY.0, C_GREY.1, C_GREY.2);
    window.set_light(Light::StickToCamera);

    let mut rubik: Vec<SceneNode> = Vec::new();

    let rubik_size: f32 = 10.0;
    let cubie_size: f32 = rubik_size;

    let mut index: usize = 0;
    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                let offset: f32 = (3.0 - 1.0) * cubie_size / 2.0;
                let pos: Vec3 = Vec3::from(
                    x as f32 * cubie_size - offset,
                    y as f32 * cubie_size - offset,
                    z as f32 * cubie_size - offset);
                let mut cubie: SceneNode = window.add_cube(cubie_size - 0.5, cubie_size - 0.5, cubie_size - 0.5);
                cubie.append_translation(&Translation3::new(pos.x, pos.y, pos.z));
                rubik.push(cubie);
                index += 1;
            }
        }
    }

    rubik[0].set_color(C_RED.0, C_RED.1, C_RED.2);

    let mut nb = 0;
    let speed: u8 = 1;
    let mut animating: bool = false;
    let mut current_cubies: &mut [SceneNode] = &mut rubik[0..9];
    let mut target_angle: f32 = 0.0;
    let mut current_angle: f32 = 0.0;
    while window.render() {
        if !animating {
            current_cubies = &mut rubik[0..9]; // to unmock
            target_angle = 90.0; // to unmock, can be 180
            current_angle = 0.0;
            animating = true;
        } else if (nb < 1) {
            let rot = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), radian(speed as f32));
            for cubie in current_cubies.iter_mut() {
                (*cubie).append_rotation(&rot);
            }
            current_angle += speed as f32;
            if current_angle == target_angle {
                animating = false;
                nb += 1;
            }
        }
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