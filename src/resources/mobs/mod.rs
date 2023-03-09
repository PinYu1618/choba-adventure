use super::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Deserialize, Debug)]
pub enum MobType {
    Goblin,
    Orc,
}

#[derive(serde::Deserialize, TypeUuid, Reflect)]
#[uuid = "82f8783f-af2e-4cad-bcaa-b484df6506c8"]
pub struct MobData {
    pub index: u32,
    pub fg: Color,
}

#[derive(Resource, Reflect, Default, AssetCollection)]
pub struct Mobset {
    #[asset(path = "data/mob/goblin.mob.ron")]
    pub goblin: Handle<MobData>,
    #[asset(path = "data/mob/orc.mob.ron")]
    pub orc: Handle<MobData>,
}

impl Mobset {
    #[allow(unused)]
    pub fn select(&self, mobtype: &MobType) -> Handle<MobData> {
        use MobType::*;

        match *mobtype {
            Goblin => self.goblin.clone(),
            Orc => self.orc.clone(),
        }
    }
}
