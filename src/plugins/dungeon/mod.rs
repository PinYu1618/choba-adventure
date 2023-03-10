mod spawn;

use bracket_random::prelude::RandomNumberGenerator;

use self::spawn::SpawnPlugin;
use crate::prelude::*;

pub struct DungeonCrawlPlugin;

impl Plugin for DungeonCrawlPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::DungeonCrawl, setup_layers_and_start_mapgen)
            .add_exit_system(AppState::DungeonCrawl, despawn_with::<GameCleanup>)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(DungeonState::Mapgen)
                    .with_system(mapgen)
                    .into(),
            )
            .add_plugin(SpawnPlugin);
    }
}

fn setup_layers_and_start_mapgen(mut cmds: Commands, textures: Res<Textures>) {
    // Layer 1
    let mut tile_storage = TileStorage::empty(MAP_SIZE);
    let tilemap_entity = cmds.spawn((Name::new("Map Console"), GameCleanup)).id();
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
        ..default()
    });

    // Layer 2
    let emap_storage = TileStorage::empty(MAP_SIZE);
    let emap_entity = cmds.spawn(Name::new("Entity Console")).id();
    cmds.insert_resource(EntityConsole(emap_entity));

    cmds.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: GRID_SIZE,
        map_type: MAP_TYPE,
        size: MAP_SIZE,
        storage: emap_storage,
        texture: TilemapTexture::Single(textures.cp437.clone()),
        tile_size: TILE_SIZE,
        transform: get_tilemap_center_transform(&MAP_SIZE, &GRID_SIZE, &MAP_TYPE, 1.0),
        ..default()
    });
    cmds.insert_resource(NextState(DungeonState::Mapgen));
}

fn mapgen(mut cmds: Commands) {
    let mut rng = RandomNumberGenerator::new();
    let schema = Schema::new(&mut rng);
    cmds.insert_resource(schema.map);
    cmds.insert_resource(NextState(DungeonState::Ticking));
    info!("Mapgen finished.");
}
