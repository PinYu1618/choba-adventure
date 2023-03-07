use crate::prelude::*;

const IBM_BIOS_FONT: &str = "fonts/IBM_BIOS.ttf";
const TILES_IMAGE: &str = "textures/monochrome-transparent.png";

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

pub fn load_textures(mut cmds: Commands, ass: Res<AssetServer>) {
    let tiles_image: Handle<Image> = ass.load(TILES_IMAGE);
    let tiles_atlas: Handle<TextureAtlas> = ass.load(TILES_IMAGE);
    cmds.insert_resource(TilesImage(tiles_image));
    cmds.insert_resource(TilesAtlas(tiles_atlas));
}
