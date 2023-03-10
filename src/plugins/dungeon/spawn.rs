use crate::prelude::*;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(DungeonState::Ticking)
                .with_system(Player::spawn)
                .into(),
        );
    }
}
