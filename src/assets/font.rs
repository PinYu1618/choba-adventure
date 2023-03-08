use super::*;

#[derive(Resource, AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/IBM_BIOS.ttf")]
    pub ibm_bios: Handle<Font>,
}
