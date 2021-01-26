extern crate rubik;

use rubik::rubik_state::{RubikState};
use rubik::action::{Action};

pub fn solve(initial_state: &RubikState) -> Option<Vec<Action>> {
    Action::pick_random().apply_to(initial_state);
    return Some(vec![Action::pick_random(), Action::pick_random(), Action::pick_random(), Action::pick_random(), Action::pick_random()]);
}