use nalgebra::{Translation3, Point3, Vector3, UnitQuaternion, Unit, Quaternion};
use kiss3d::scene::SceneNode;

use super::cubie::*;

pub struct Facelet {
    node: SceneNode
}

impl Facelet {
    pub fn new(cubie: &mut Cubie, size: f32, rot: Option<UnitQuaternion<f32>>, pos: Option<Translation3<f32>>, color: (f32, f32, f32)) -> Facelet {
        let mut node: SceneNode = cubie.node.add_cube(size, size, 0.0);
        if rot.is_some() {
            node.append_rotation(&rot.unwrap());
        }
        if pos.is_some() {
            node.append_translation(&pos.unwrap());
        }
        node.set_color(color.0, color.1, color.2);
        return Facelet {
            node: node,
        }
    }
}