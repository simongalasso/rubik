// use nalgebra::{Point3, Vector3, Unit};
// use kiss3d::window::{Window};
// use kiss3d::scene::{SceneNode};
// use rubik::face::{Face};
// use super::cubie::{Cubie};
 
// pub struct GlRubik {
//     pub scene_node: SceneNode,
//     pub cubies: Vec<Cubie>
// }

// impl GlRubik {
//     pub fn new(window: &mut Window) -> GlRubik {
//         let mut rubik: SceneNode = window.add_group();
//         let mut cubies: Vec<Cubie> = Vec::new();
//         let gap: f32 = 0.05;
    
//         for x in -1..2 {
//             for y in -1..2 {
//                 for z in -1..2 {
//                     cubies.push(Cubie::new(&mut rubik, 1.0, Point3::new(x as f32, y as f32, z as f32), gap));
//                 }
//             }
//         }

//         return GlRubik {
//             scene_node: rubik,
//             cubies: cubies
//         }
//     }

//     pub fn get_face_cubies(&self, face: char) -> Vec<Cubie> { // To optimise
//         match face {
//             'U' => self.cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().y) == 1.0).collect::<Vec<Cubie>>(),
//             'R' => self.cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().x) == -1.0).collect::<Vec<Cubie>>(),
//             'F' => self.cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().z) == -1.0).collect::<Vec<Cubie>>(),
//             'D' => self.cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().y) == -1.0).collect::<Vec<Cubie>>(),
//             'L' => self.cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().x) == 1.0).collect::<Vec<Cubie>>(),
//             'B' => self.cubies.iter().cloned().filter(|cubie| f32::round(cubie.node.data().local_translation().z) == 1.0).collect::<Vec<Cubie>>()
//         }
//     }
    
//     pub fn get_face_axis(&self, face: &Face) -> Unit::<Vector3::<f32>> {
//         match face {
//             Face::U => -Vector3::<f32>::y_axis(),
//             Face::R => Vector3::<f32>::x_axis(),
//             Face::F => Vector3::<f32>::z_axis(),
//             Face::D => Vector3::<f32>::y_axis(),
//             Face::L => -Vector3::<f32>::x_axis(),
//             Face::B => -Vector3::<f32>::z_axis(),
//         }
//     }
// }