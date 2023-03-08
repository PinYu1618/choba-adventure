use bevy::reflect::TypeUuid;

use super::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, serde::Deserialize, Debug)]
pub enum MobType {
    GobLin,
    Orc,
}

#[derive(serde::Deserialize, TypeUuid, Reflect)]
#[uuid = "82f8783f-af2e-4cad-bcaa-b484df6506c8"]
pub struct Mob {
    pub index: u32,
    pub fg: Color,
}

#[derive(Resource, Reflect, Default, AssetCollection)]
pub struct MobData {
    #[asset(path = "data/mob/goblin.mob.ron")]
    pub goblin: Handle<Mob>,
    #[asset(path = "data/mob/orc.mob.ron")]
    pub orc: Handle<Mob>,
}

impl MobData {
    pub fn select(&self, mobtype: &MobType) -> Handle<Mob> {
        use MobType::*;

        match *mobtype {
            Goblin => self.goblin.clone(),
            Orc => self.orc.clone(),
        }
    }
}
