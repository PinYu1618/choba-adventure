use bracket_random::prelude::RandomNumberGenerator;

use crate::prelude::*;

pub struct DungeonCrawlPlugin;

impl Plugin for DungeonCrawlPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::DungeonCrawl, setup_level)
            .add_exit_system(AppState::DungeonCrawl, despawn_with::<GameCleanup>)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::DungeonCrawl)
                    .with_system(systems::player_input)
                    .into(),
            );
    }
}

fn setup_level(mut cmds: Commands, textures: Res<Textures>) {
    let mut rng = RandomNumberGenerator::new();
    let schema = Schema::new(&mut rng);
    cmds.insert_resource(schema.map);

    let player_entity = cmds.spawn_empty().id();
    let player_start = TilePos {
        x: schema.player_start.x as u32,
        y: schema.player_start.y as u32,
    };

    // Layer 1
    let mut tile_storage = TileStorage::empty(MAP_SIZE);
    let tmap_entity = cmds.spawn((Name::new("Map Console"), GameCleanup)).id();
    cmds.insert_resource(MapConsole(tmap_entity));

    fill_tilemap(
        TileTextureIndex(0),
        MAP_SIZE,
        TilemapId(tmap_entity),
        &mut cmds,
        &mut tile_storage,
    );

    cmds.entity(tmap_entity).insert(TilemapBundle {
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
    let mut emap_storage = TileStorage::empty(MAP_SIZE);
    let emap_entity = cmds.spawn((Name::new("Entity Console"), GameCleanup)).id();
    cmds.insert_resource(EntityConsole(emap_entity));

    emap_storage.set(&player_start, player_entity);
    Player::spawn(&mut cmds, player_entity, emap_entity, player_start);

    cmds.entity(emap_entity).insert(TilemapBundle {
        grid_size: GRID_SIZE,
        map_type: MAP_TYPE,
        size: MAP_SIZE,
        storage: emap_storage,
        texture: TilemapTexture::Single(textures.cp437.clone()),
        tile_size: TILE_SIZE,
        transform: get_tilemap_center_transform(&MAP_SIZE, &GRID_SIZE, &MAP_TYPE, 1.0),
        ..default()
    });
    cmds.insert_resource(NextState(DungeonState::Ticking));
    info!("Mapgen finished.");
}
