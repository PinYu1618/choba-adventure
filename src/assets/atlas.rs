use super::*;

#[derive(Resource, AssetCollection)]
pub struct AtlasAssets {
    #[asset(texture_atlas(
        tile_size_x = 16.,
        tile_size_y = 16.,
        columns = 49,
        rows = 22,
        padding_x = 1.
    ))]
    #[asset(path = "textures/tiles.png")]
    pub onebit_mono: Handle<TextureAtlas>,
}
