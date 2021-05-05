use crate::{GameState, Inspected, loading::TextureAssets};
use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use rand::{Rng, thread_rng};

pub struct MapviewPlugin;

impl Plugin for MapviewPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(LogDiagnosticsPlugin::default());
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(setup_map.system())
        );
    }
}

const TILE_WIDTH: f32 = 32.0;
const TILE_HEIGHT: f32 = 32.0;

fn setup_map(
  mut commands: Commands,
  texture_assets: Res<TextureAssets>,
  textures: Res<Assets<Texture>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Setup game map");

    let texture = textures.get(&texture_assets.texture_tileset).unwrap();
    println!("Tileset loaded: {} {}", texture.size.width, texture.size.height);
    let asset = ColorMaterial::texture(texture_assets.texture_tileset.clone());
    let material_handle = materials.add(asset);

    let mut map = Map::new(
        Vec2::new(10.0, 10.0).into(), // size in chunks
        Vec2::new(10.0, 10.0).into(), 
        Vec2::new(TILE_WIDTH, TILE_HEIGHT), 
        Vec2::new(texture.size.width as f32, texture.size.height as f32), 
        0
    );
    map.mesher = Box::new(HexChunkMesher::new(HexType::ColumnEven));
    let map_entity = commands.spawn().id();
    map.build(&mut commands, &mut meshes, material_handle.clone(), map_entity, false);
    for x in 0..100 {
        for y in 0..100 {
            map.add_tile(&mut commands, MapVec2::new(x, y), Tile {
                texture_index: 1,
                ..Default::default()
            }).unwrap();
        }
    }
    commands.entity(map_entity).insert_bundle(MapBundle {
        map,
        ..Default::default()
    });
}
