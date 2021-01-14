extern crate kiss3d;
extern crate nalgebra;

use nalgebra::{Translation3, Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;

mod parsing;
mod display;
mod rubik;

use parsing::args::*;
use parsing::parse::*;
use display::output::*;
use rubik::rubik::*;

fn main() {
    let config: Config = Config::new();
    let input_sequence: Vec<Action> = parse(&config);
    display_sequence("shuffle: ", &input_sequence);

    let mut rubik: Rubik = Rubik::new();
    // println!("{}", rubik.edges[0].c);
    rubik.shuffle(input_sequence);

    let output_sequence: Vec<Action> = rubik.solve();
    display_sequence("solution: ", &output_sequence);

    if config.visualisator {
        let mut window: Window = Window::new("Rubik");
        let mut rubik = window.add_group();
        // for cubie in rubik.ges_cubies().iter() {
        //     let mut cubie1 = g1.add_cube(1.0, 1.0, 1.0);
        //     // calc color
        //     g1.set_color(1.0, 0.0, 0.0);
        //     // calc translation
        //     cubie1.append_translation(&Translation3::new(2.0f32, 0.0, 0.0));
        // }
        let mut g0 = rubik.add_group();
        let mut c_0_1 = g0.add_cube(1.0, 1.0, 1.0);
        c_0_1.append_translation(&Translation3::new(-2.0, -2.0, -2.0));
        let mut c_0_2 = g0.add_cube(1.0, 1.0, 1.0);
        c_0_2.append_translation(&Translation3::new(0.0, -2.0, -2.0));
        let mut c_0_3 = g0.add_cube(1.0, 1.0, 1.0);
        c_0_3.append_translation(&Translation3::new(2.0, -2.0, -2.0));
        let mut c_0_4 = g0.add_cube(1.0, 1.0, 1.0);
        c_0_4.append_translation(&Translation3::new(-2.0, 0.0, -2.0));
        let mut c_0_5 = g0.add_cube(1.0, 1.0, 1.0);
        c_0_5.append_translation(&Translation3::new(0.0, 0.0, -2.0));
        let mut c_0_6 = g0.add_cube(1.0, 1.0, 1.0);
        c_0_6.append_translation(&Translation3::new(2.0, 0.0, -2.0));
        let mut c_0_7 = g0.add_cube(1.0, 1.0, 1.0);
        c_0_7.append_translation(&Translation3::new(-2.0, 2.0, -2.0));
        let mut c_0_8 = g0.add_cube(1.0, 1.0, 1.0);
        c_0_8.append_translation(&Translation3::new(0.0, 2.0, -2.0));
        let mut c_0_9 = g0.add_cube(1.0, 1.0, 1.0);
        c_0_9.append_translation(&Translation3::new(2.0, 2.0, -2.0));
        
        let mut g1 = rubik.add_group();
        let mut c_1_1 = g1.add_cube(1.0, 1.0, 1.0);
        c_1_1.append_translation(&Translation3::new(-2.0, -2.0, 0.0));
        let mut c_1_2 = g1.add_cube(1.0, 1.0, 1.0);
        c_1_2.append_translation(&Translation3::new(0.0, -2.0, 0.0));
        let mut c_1_3 = g1.add_cube(1.0, 1.0, 1.0);
        c_1_3.append_translation(&Translation3::new(2.0, -2.0, 0.0));
        let mut c_1_4 = g1.add_cube(1.0, 1.0, 1.0);
        c_1_4.append_translation(&Translation3::new(-2.0, 0.0, 0.0));
        let mut c_1_6 = g1.add_cube(1.0, 1.0, 1.0);
        c_1_6.append_translation(&Translation3::new(2.0, 0.0, 0.0));
        let mut c_1_7 = g1.add_cube(1.0, 1.0, 1.0);
        c_1_7.append_translation(&Translation3::new(-2.0, 2.0, 0.0));
        let mut c_1_8 = g1.add_cube(1.0, 1.0, 1.0);
        c_1_8.append_translation(&Translation3::new(0.0, 2.0, 0.0));
        let mut c_0_9 = g1.add_cube(1.0, 1.0, 1.0);
        c_0_9.append_translation(&Translation3::new(2.0, 2.0, 0.0));

        let mut g2 = rubik.add_group();
        let mut c_2_1 = g2.add_cube(1.0, 1.0, 1.0);
        c_2_1.append_translation(&Translation3::new(-2.0, -2.0, 2.0));
        let mut c_2_2 = g2.add_cube(1.0, 1.0, 1.0);
        c_2_2.append_translation(&Translation3::new(0.0, -2.0, 2.0));
        let mut c_2_3 = g2.add_cube(1.0, 1.0, 1.0);
        c_2_3.append_translation(&Translation3::new(2.0, -2.0, 2.0));
        let mut c_2_4 = g2.add_cube(1.0, 1.0, 1.0);
        c_2_4.append_translation(&Translation3::new(-2.0, 0.0, 2.0));
        let mut c_2_5 = g2.add_cube(1.0, 1.0, 1.0);
        c_2_5.append_translation(&Translation3::new(0.0, 0.0, 2.0));
        let mut c_2_6 = g2.add_cube(1.0, 1.0, 1.0);
        c_2_6.append_translation(&Translation3::new(2.0, 0.0, 2.0));
        let mut c_2_7 = g2.add_cube(1.0, 1.0, 1.0);
        c_2_7.append_translation(&Translation3::new(-2.0, 2.0, 2.0));
        let mut c_2_8 = g2.add_cube(1.0, 1.0, 1.0);
        c_2_8.append_translation(&Translation3::new(0.0, 2.0, 2.0));
        let mut c_2_9 = g2.add_cube(1.0, 1.0, 1.0);
        c_2_9.append_translation(&Translation3::new(2.0, 2.0, 2.0));

        window.set_light(Light::StickToCamera);
        while window.render() {}
    }
}