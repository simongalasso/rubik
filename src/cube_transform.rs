pub fn cube_shuffle(mut _cube: &mut Cube, _moves: &Vec<Move>) {
    for _move in _moves.iter() {
        apply_move(&mut _cube, &_move);
    }
}

fn apply_move(_cube: &mut Cube, _move: &Move) {
    // apply matrix
}