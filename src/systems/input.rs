use crate::prelude::*;

pub fn player_input(
    mut kb: ResMut<Input<KeyCode>>,
    mut player_q: Query<&mut TilePos, With<Player>>,
) {
    let mut pos = player_q.single_mut();

    let mut new_position = pos.clone();

    let key = kb.get_pressed().next().cloned();
    if let Some(key) = key {
        match key {
            KeyCode::Left => new_position.x -= 1,
            KeyCode::Right => new_position.x += 1,
            KeyCode::Down => new_position.y -= 1,
            KeyCode::Up => new_position.y += 1,
            _ => {}
        }

        // move to new position
        if new_position != *pos {
            // placeholder to know if it just a move or an attack
            //let mut hit_something = false;
            // check if there is an enemy at the destination position
            /*enemies
            .iter()
            .filter(|(_, pos)| **pos == new_position)
            // if there's an enemy, say you hit something and send a WantsToAttack
            .for_each(|(victim, _)| {
                hit_something = true;

                commands.spawn(WantsToAttack {
                    attacker: player_ent,
                    victim,
                });
            });*/

            // if it did not hit then it is just a movement
            //if !hit_something {
            /*commands.spawn(WantsToMove {
                entity: player_ent,
                destination: new_position,
            });*/
            //}
            *pos = new_position;
            info!("Moved");
        }

        kb.reset(key);

        /*if action {
            cmds.insert_resource(CurrentState(TurnState::Running));
        }*/
    }
}

fn _can_enter_tile(_pos: &TilePos) -> bool {
    true
}
