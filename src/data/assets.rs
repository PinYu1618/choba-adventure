use super::*;

#[derive(Resource, AssetCollection)]
pub struct Fonts {
    #[asset(path = "fonts/IBM_BIOS.ttf")]
    pub ibm_bios: Handle<Font>,
}

#[derive(Resource, AssetCollection)]
pub struct Textures {
    #[asset(path = "textures/tiles.png")]
    pub tiles: Handle<Image>,
}

#[derive(Resource, AssetCollection)]
pub struct Atlases {
    #[asset(texture_atlas(
        tile_size_x = 16.,
        tile_size_y = 16.,
        columns = 49,
        rows = 22,
        padding_x = 1.,
        padding_y = 1.
    ))]
    #[asset(path = "textures/tiles.png")]
    pub onebit_mono: Handle<TextureAtlas>,
}
