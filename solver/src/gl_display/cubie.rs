use nalgebra::{Translation3, Point3, Vector3, UnitQuaternion};
use kiss3d::scene::{SceneNode};
use crate::gl_display::gl_display::*;

#[derive(Clone)]
pub struct Cubie {
    pub node: SceneNode,
    pub gap: f32
}

impl Cubie {
    pub fn new(rubik: &mut SceneNode, size: f32, pos: Point3<f32>, gap: f32) -> Self {
        let mut cubie: Self = Self {
            node: rubik.add_group(),
            gap: gap
        };

        let r: f32 = (size - gap) / 2.0;
        // cubie U
        let color: (f32, f32, f32) = if pos.y == 1.0 { C_BLUE } else { C_BLACK };
        let rot: UnitQuaternion<f32> = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), (90.0 as f32).to_radians());
        cubie.add_facelet(size - gap, Some(rot), Some(Translation3::new(0.0, r, 0.0)), color);
        // cubie R
        let color: (f32, f32, f32) = if pos.x == -1.0 { C_RED } else { C_BLACK };
        let rot: UnitQuaternion<f32> = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), (90.0 as f32).to_radians());
        cubie.add_facelet(size - gap, Some(rot), Some(Translation3::new(-r, 0.0, 0.0)), color);
        // cubie F
        let color: (f32, f32, f32) = if pos.z == -1.0 { C_YELLOW } else { C_BLACK };
        cubie.add_facelet(size - gap, None, Some(Translation3::new(0.0, 0.0, -r)), color);
        // cubie D
        let color: (f32, f32, f32) = if pos.y == -1.0 { C_GREEN } else { C_BLACK };
        let rot: UnitQuaternion<f32> = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), (90.0 as f32).to_radians());
        cubie.add_facelet(size - gap, Some(rot), Some(Translation3::new(0.0, -r, 0.0)), color);
        // cubie L
        let color: (f32, f32, f32) = if pos.x == 1.0 { C_ORANGE } else { C_BLACK };
        let rot: UnitQuaternion<f32> = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), (90.0 as f32).to_radians());
        cubie.add_facelet(size - gap, Some(rot), Some(Translation3::new(r, 0.0, 0.0)), color);
        // cubie B
        let color: (f32, f32, f32) = if pos.z == 1.0 { C_WHITE } else { C_BLACK };
        cubie.add_facelet(size - gap, None, Some(Translation3::new(0.0, 0.0, r)), color);

        cubie.node.append_translation(&Translation3::new(pos.x, pos.y, pos.z));
        return cubie;
    }

    fn add_facelet(&mut self, size: f32, rot: Option<UnitQuaternion<f32>>, pos: Option<Translation3<f32>>, color: (f32, f32, f32)) {
        let mut node: SceneNode = self.node.add_cube(size, size, 0.0);
        if rot.is_some() {
            node.append_rotation(&rot.unwrap());
        }
        if pos.is_some() {
            node.append_translation(&pos.unwrap());
        }
        node.set_color(color.0, color.1, color.2);
    }

    pub fn rotate(&mut self, rot: UnitQuaternion<f32>) {
        self.node.append_rotation(&rot);
    }
}