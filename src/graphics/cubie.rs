use nalgebra::{Translation3, Point3, Vector3, UnitQuaternion, Unit, Quaternion};
use kiss3d::scene::SceneNode;

use super::graphics::*;
use super::facelet::*;

#[derive(Clone)]
pub struct Cubie {
    pub node: SceneNode,
    pub scale: f32,
    pub gap: f32
}

impl Cubie {
    pub fn new(rubik: &mut SceneNode, size: f32, pos: Vector3<f32>, scale: f32, gap: f32) -> Cubie {
        let mut cubie: Cubie = Cubie {
            node: rubik.add_group(),
            scale: scale,
            gap: gap
        };

        let r: f32 = (size - gap) / 2.0;
        // cubie U
        let color: (f32, f32, f32) = if pos.y == 1.0 { C_BLUE } else { C_BLACK };
        let rot: UnitQuaternion<f32> = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), (90.0 as f32).to_radians());
        Facelet::new(&mut cubie, size - gap, Some(rot), Some(Translation3::new(0.0, r, 0.0)), color);
        // cubie F
        let color: (f32, f32, f32) = if pos.z == -1.0 { C_WHITE } else { C_BLACK };
        Facelet::new(&mut cubie, size - gap, None, Some(Translation3::new(0.0, 0.0, -r)), color);
        // cubie L
        let color: (f32, f32, f32) = if pos.x == 1.0 { C_ORANGE } else { C_BLACK };
        let rot: UnitQuaternion<f32> = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), (90.0 as f32).to_radians());
        Facelet::new(&mut cubie, size - gap, Some(rot), Some(Translation3::new(r, 0.0, 0.0)), color);
        // cubie D
        let color: (f32, f32, f32) = if pos.y == -1.0 { C_GREEN } else { C_BLACK };
        let rot: UnitQuaternion<f32> = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), (90.0 as f32).to_radians());
        Facelet::new(&mut cubie, size - gap, Some(rot), Some(Translation3::new(0.0, -r, 0.0)), color);
        // cubie B
        let color: (f32, f32, f32) = if pos.z == 1.0 { C_YELLOW } else { C_BLACK };
        Facelet::new(&mut cubie, size - gap, None, Some(Translation3::new(0.0, 0.0, r)), color);
        // cubie R
        let color: (f32, f32, f32) = if pos.x == -1.0 { C_RED } else { C_BLACK };
        let rot: UnitQuaternion<f32> = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), (90.0 as f32).to_radians());
        Facelet::new(&mut cubie, size - gap, Some(rot), Some(Translation3::new(-r, 0.0, 0.0)), color);

        cubie.node.append_translation(&Translation3::new(pos.x, pos.y, pos.z));
        return cubie;
    }

    pub fn rotate(&mut self, rot: UnitQuaternion<f32>) {
        self.node.append_rotation(&rot);
    }
}