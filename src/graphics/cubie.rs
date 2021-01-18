use nalgebra::{Translation3, Point3, Vector3, UnitQuaternion, Unit, Quaternion};
use kiss3d::scene::SceneNode;

#[derive(Clone)]
pub struct Cubie {
    pub node: SceneNode,
    pub scale: f32,
    pub gap: f32
}

impl Cubie {
    pub fn new(rubik: &mut SceneNode, size: Vector3<f32>, pos: Vector3<f32>, scale: f32, gap: f32) -> Cubie {
        let mut cubie: Cubie = Cubie {
            node: rubik.add_cube(size.x - gap, size.y - gap, size.z - gap),
            scale: scale,
            gap: gap
        };
        cubie.node.append_translation(&Translation3::new(pos.x, pos.y, pos.z));
        return cubie;
    }

    pub fn rotate(&mut self, rot: UnitQuaternion<f32>) {
        self.node.append_rotation(&rot);
    }
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