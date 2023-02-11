use bevy::{prelude::*, window::PresentMode};
use bevy_ecs_tilemap::{TilemapPlugin, prelude::{TilemapSize, TilemapId, TilemapType, TilemapTileSize, TilemapGridSize, get_tilemap_center_transform, TilemapTexture}, tiles::{TileStorage, TilePos, TileBundle}, TilemapBundle};

const CLEAR: Color = Color::rgb(0.5, 0.5, 0.5);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    width: 1600.0,
                    height: 900.0,
                    title: "Doors Galore".to_string(),
                    present_mode: PresentMode::Fifo,
                    resizable: false,
                    ..default()
                },
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .add_plugin(TilemapPlugin)
        .add_startup_system(init_camera)
        .add_startup_system(create_center_sprite)
        .add_startup_system(create_tilemap)
        .run();
}

fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.1,
            ..default()
        },
        ..default()
    });
}

fn create_center_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("../assets/mc.png"),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    });
}

fn create_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_size = TilemapSize {
        x: 32,
        y: 32,
    };

    let texture_handle: Handle<Image> = asset_server.load("../assets/0x72_DungeonTilesetII_v1.4/frames/floor_1.png");

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x,y };
            let tile_entity = commands.spawn(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                ..default()
            })
            .id();

            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize {x: 16.0, y: 16.0};
    let grid_size: TilemapGridSize = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..default()
    });


}
