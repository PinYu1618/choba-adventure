use crate::prelude::*;

pub fn player_input(_kb: Res<Input<KeyCode>>, mut player_q: Query<&mut TilePos, With<Player>>) {
    /*let delta = match key {
        KeyCode::Left => Point::new(-1, 0),
        KeyCode::Right => Point::new(1, 0),
        KeyCode::Up => Point::new(0, -1),
        KeyCode::Down => Point::new(0, 1),
        _ => Point::new(0, 0),
    };*/

    let delta = IVec2::new(1, 0);

    if delta.x != 0 || delta.y != 0 {
        player_q.iter_mut().for_each(|_pos| {
            //(3)
            //let destination = *pos + delta;
            //if can_enter_tile(destination) {
            //    *pos = destination;
            //camera.on_player_move(destination);
            //}
        });
    }
}

fn _can_enter_tile(_pos: &TilePos) -> bool {
    true
}
