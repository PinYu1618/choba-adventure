use crate::prelude::*;

const IBM_BIOS_FONT: &str = "fonts/IBM_BIOS.ttf";
const ATLAS: &str = "textures/monochrome-transparent.png";

#[derive(Resource, Deref)]
pub struct UiFont(Handle<Font>);
#[derive(Resource, Deref)]
pub struct Atlas(Handle<Image>);

pub fn load_fonts(mut cmds: Commands, ass: Res<AssetServer>) {
    let handle: Handle<Font> = ass.load(IBM_BIOS_FONT);
    cmds.insert_resource(UiFont(handle));
}

pub fn load_atlas(mut cmds: Commands, ass: Res<AssetServer>) {
    let handle: Handle<Image> = ass.load(ATLAS);
    cmds.insert_resource(Atlas(handle));
}
