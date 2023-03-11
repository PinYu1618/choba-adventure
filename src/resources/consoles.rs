use crate::prelude::*;

#[derive(Resource, Deref, Debug)]
pub struct MapConsole(pub Entity);

#[derive(Resource, Debug)]
pub struct EntityConsole(pub Entity);
