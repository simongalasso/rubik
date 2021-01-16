extern crate kiss3d;
extern crate nalgebra;

use nalgebra::{Translation3, Point3, Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;

mod parsing;
mod display;
mod action;
mod rubik;

use parsing::args::*;
// use parsing::parse::*;
// use display::output::*;
// use action::action::*;
// use rubik::rubik::*;

const C_GREY: (f32, f32, f32) = (0.11, 0.11, 0.11);

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

fn main() {
    let config: Config = Config::new();
    // let input_sequence: Vec<Action> = parse(&config);
    // display_sequence("shuffle: ", &input_sequence);

    // let mut rubik: Rubik = Rubik::new();
    // rubik.shuffle(input_sequence);

    // let output_sequence: Vec<Action> = rubik.solve();
    // display_sequence("solution: ", &output_sequence);

    if config.visualisator {
        let mut window: Window = Window::new("Rubik");
        window.set_background_color(C_GREY.0, C_GREY.1, C_GREY.2);
        let mut rubik = window.add_group();

        let rubik_size: f32 = 10.0;
        let cubie_size: f32 = rubik_size;

        for z in 0..3 {
            for y in 0..3 {
                for x in 0..3 {
                    let offset: f32 = (3.0 - 1.0) * cubie_size / 2.0;
                    let pos: Vec3 = Vec3::from(
                        x as f32 * cubie_size - offset,
                        y as f32 * cubie_size - offset,
                        z as f32 * cubie_size - offset);
                    let cubie: SceneNode = add_cubie(&mut rubik, pos, cubie_size);
                }
            }
        }

        window.set_light(Light::StickToCamera);
        while window.render() {}
    }
}

fn add_cubie(rubik: &mut SceneNode, pos: Vec3, cubie_size: f32) -> SceneNode {
    let color_u = (1.0, 1.0, 1.0);
    let color_f = (0.0, 1.0, 0.0);
    let color_l = (1.0, 0.647, 0.0);
    let color_d = (0.0, 0.0, 1.0);
    let color_r = (1.0, 0.0, 0.0);
    let color_b = (1.0, 1.0, 0.0);

    let r: f32 = cubie_size / 2.0;
    let mut cubie: SceneNode = rubik.add_group();

    // cubie U
    let mut quad = cubie.add_quad(cubie_size, cubie_size, 1, 1);
    quad.append_rotation(&UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 90.0 * std::f32::consts::PI / 180.0));
    quad.append_translation(&Translation3::new(0.0, -r, 0.0));
    quad.set_color(color_u.0, color_u.1, color_u.2);

    // cubie F
    let mut quad = cubie.add_quad(cubie_size, cubie_size, 1, 1);
    quad.append_translation(&Translation3::new(0.0, 0.0, -r));
    quad.set_color(color_f.0, color_f.1, color_f.2);

    // cubie L
    let mut quad = cubie.add_quad(cubie_size, cubie_size, 1, 1);
    quad.append_rotation(&UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 90.0 * std::f32::consts::PI / 180.0));
    quad.append_translation(&Translation3::new(-r, 0.0, 0.0));
    quad.set_color(color_l.0, color_l.1, color_l.2);

    // cubie D
    let mut quad = cubie.add_quad(cubie_size, cubie_size, 1, 1);
    quad.append_rotation(&UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 90.0 * std::f32::consts::PI / 180.0));
    quad.append_translation(&Translation3::new(0.0, r, 0.0));
    quad.set_color(color_d.0, color_d.1, color_d.2);

    // cubie B
    let mut quad = cubie.add_quad(cubie_size, cubie_size, 1, 1);
    quad.append_translation(&Translation3::new(0.0, 0.0, r));
    quad.set_color(color_b.0, color_b.1, color_b.2);

    // cubie R
    let mut quad = cubie.add_quad(cubie_size, cubie_size, 1, 1);
    quad.append_rotation(&UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 90.0 * std::f32::consts::PI / 180.0));
    quad.append_translation(&Translation3::new(r, 0.0, 0.0));
    quad.set_color(color_r.0, color_r.1, color_r.2);

    cubie.append_translation(&Translation3::new(pos.x, pos.y, pos.z));
    return cubie;
}