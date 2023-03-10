use bevy_ecs_tilemap::prelude::fill_tilemap;
use bracket_random::prelude::RandomNumberGenerator;

use crate::prelude::*;

pub struct MainGamePlugin;

impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, setup_consoles)
            .add_exit_system(AppState::InGame, despawn_with::<GameCleanup>);
    }
}

fn setup_consoles(mut cmds: Commands, textures: Res<Textures>) {
    // Layer 1
    let mut tile_storage = TileStorage::empty(MAP_SIZE);
    let tilemap_entity = cmds.spawn(Name::new("Map Console")).id();
    cmds.insert_resource(MapConsole(tilemap_entity));

    fill_tilemap(
        TileTextureIndex(0),
        MAP_SIZE,
        TilemapId(tilemap_entity),
        &mut cmds,
        &mut tile_storage,
    );

    cmds.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: GRID_SIZE,
        map_type: MAP_TYPE,
        size: MAP_SIZE,
        storage: tile_storage,
        texture: TilemapTexture::Single(textures.cp437.clone()),
        tile_size: TILE_SIZE,
        transform: get_tilemap_center_transform(&MAP_SIZE, &GRID_SIZE, &MAP_TYPE, 0.0),
        ..Default::default()
    });

    // Layer 2
    let mut tile_storage = TileStorage::empty(MAP_SIZE);
    let tilemap_entity = cmds.spawn(Name::new("Entity Console")).id();
    cmds.insert_resource(EntityConsole(tilemap_entity));

    fill_tilemap(
        TileTextureIndex(0),
        MAP_SIZE,
        TilemapId(tilemap_entity),
        &mut cmds,
        &mut tile_storage,
    );

    cmds.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: GRID_SIZE,
        map_type: MAP_TYPE,
        size: MAP_SIZE,
        storage: tile_storage,
        texture: TilemapTexture::Single(textures.cp437.clone()),
        tile_size: TILE_SIZE,
        transform: get_tilemap_center_transform(&MAP_SIZE, &GRID_SIZE, &MAP_TYPE, 1.0),
        ..Default::default()
    });
}

fn _setup_dungeon(mut cmds: Commands) {
    let mut rng = RandomNumberGenerator::new();
    let schema = Schema::new(&mut rng);
    //cmds.spawn((Player, Position(schema.player_start)));
    cmds.insert_resource(schema.map);
    //cmds.insert_resource(BCamera::new(schema.player_start));
    cmds.insert_resource(NextState(TurnState::AwaitInput));
    info!("Setup finished.");
}
