// extern crate nalgebra;
// extern crate kiss3d;

// use nalgebra::{Point3};
// use kiss3d::light::{Light};
// use kiss3d::window::{Window};
// use kiss3d::camera::{ArcBall};
// use kiss3d::event::{WindowEvent, Key};
// use crate::parsing::args::{Config};

// pub const C_GREY: (f32, f32, f32) = (0.09, 0.09, 0.09);
// pub const C_RED: (f32, f32, f32) = (1.0, 0.15, 0.15);
// pub const C_GREEN: (f32, f32, f32) = (0.0, 0.60784, 0.28235);
// pub const C_BLUE: (f32, f32, f32) = (0.0, 0.27450, 0.67843);
// pub const C_WHITE: (f32, f32, f32) = (1.0, 1.0, 1.0);
// pub const C_YELLOW: (f32, f32, f32) = (1.0, 1.0, 0.0);
// pub const C_ORANGE: (f32, f32, f32) = (1.0, 0.4, 0.1);
// pub const C_BLACK: (f32, f32, f32) = (0.0, 0.0, 0.0);

// pub struct Display {
//     pub window: Window,
//     pub camera: ArcBall,
//     pub started: bool,
//     pub rotating: bool,
//     pub animating: bool,
//     pub speed: f32,
//     pub moves: usize
// }

// impl Display {
//     pub fn new(config: &Config) -> Display {
//         let mut window: Window = Window::new("Rubik");
//         window.set_framerate_limit(Some(60));
//         window.set_background_color(C_GREY.0, C_GREY.1, C_GREY.2);
//         let mut camera: ArcBall = ArcBall::new(Point3::new(-6.0, 3.0, -6.0), Point3::origin());
//         camera.rebind_drag_button(None);
//         camera.set_min_dist(5.0);
//         camera.set_max_dist(50.0);
//         camera.set_dist_step(5.0);
//         window.set_light(Light::StickToCamera);
//         return Display {
//             window, camera,
//             started: false,
//             rotating: true,
//             animating: false,
//             speed: match &config.speed_selection[..] { // doit impérativement être un diviseur de 180
//                 "slow" => 1.0,
//                 "normal" => 3.0,
//                 "fast" => 6.0,
//                 _ => 6.0
//             },
//             moves: 0
//         }
//     }

//     pub fn handle_events(&mut self) {
//         for mut event in self.window.events().iter() {
//             match event.value {
//                 WindowEvent::Key(button, kiss3d::event::Action::Release, _) => {
//                     event.inhibited = true;
//                     match button {
//                         Key::Escape => self.window.close(),
//                         Key::Return => {
//                             if !self.started {
//                                 self.started = true;
//                             }
//                         },
//                         Key::Space => {
//                             self.rotating = !self.rotating;
//                         },
//                         _ => {}
//                     }
                    
//                 },
//                 _ => {}
//             }
//         }
//     }
// }