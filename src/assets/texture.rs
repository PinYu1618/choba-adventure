use super::*;

#[derive(Resource, AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/tiles.png")]
    pub tiles: Handle<Image>,
}
