use crate::prelude::*;

const IBM_BIOS_FONT: &str = "fonts/IBM_BIOS.ttf";
const TILES: &str = "textures/tiles.png";

#[derive(Resource, Deref)]
pub struct UiFont(Handle<Font>);
#[derive(Resource, Deref)]
pub struct TilesImage(Handle<Image>);
#[derive(Resource, Deref)]
pub struct TilesAtlas(Handle<TextureAtlas>);

pub fn load_fonts(mut cmds: Commands, ass: Res<AssetServer>) {
    let handle: Handle<Font> = ass.load(IBM_BIOS_FONT);
    cmds.insert_resource(UiFont(handle));
}

pub fn load_textures(
    mut cmds: Commands,
    ass: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tiles_image: Handle<Image> = ass.load(TILES);
    let tiles_atlas = TextureAtlas::from_grid(
        tiles_image.clone(),
        Vec2::new(16.0, 16.0),
        49,
        22,
        Some(Vec2::ONE),
        Some(Vec2::ONE),
    );
    let texture_atlas_handle = texture_atlases.add(tiles_atlas);
    cmds.insert_resource(TilesImage(tiles_image));
    cmds.insert_resource(TilesAtlas(texture_atlas_handle));
}
