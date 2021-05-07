use std::collections::HashMap;

use crate::{GameState, Inspected, loading::TextureAssets};
use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use chickenwire::{coordinate::{CoordSys, MultiCoord, Offset}, hexgrid::{Parity, Tilt}, prelude::HexGrid};
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

#[derive(Debug, Copy, Clone)]
enum TerrainType {
    OCEAN = 0,
    LAND = 1,
}

#[derive(Debug, Copy, Clone)]
struct HexTile {
    terrain_type: TerrainType,
}

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

    const CHUNK_WIDTH: f32 = 60.0;
    const CHUNK_HEIGHT: f32 = 30.0;
    const CHUNK_SIZE_WIDTH: f32 = 25.0;
    const CHUNK_SIZE_HEIGHT: f32 = 25.0;
    const MAP_WIDTH: i32 = (CHUNK_WIDTH * CHUNK_SIZE_WIDTH) as i32;
    const MAP_HEIGHT: i32 = (CHUNK_HEIGHT * CHUNK_SIZE_HEIGHT) as i32;

    let mut world_map = HexGrid::<HexTile>::new(Tilt::Flat, Parity::Even, CoordSys::Offset);
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            world_map.add(MultiCoord::from(Offset { row: x, col: y }), HexTile {
                terrain_type: TerrainType::OCEAN,
            }).unwrap();
        }
    }

    let mut map = Map::new(
        Vec2::new(CHUNK_WIDTH, CHUNK_HEIGHT).into(), // size in chunks
        Vec2::new(CHUNK_SIZE_WIDTH, CHUNK_SIZE_HEIGHT).into(), 
        Vec2::new(TILE_WIDTH, TILE_HEIGHT), 
        Vec2::new(texture.size.width as f32, texture.size.height as f32), 
        0
    );
    map.mesher = Box::new(HexChunkMesher::new(HexType::ColumnEven));
    let map_entity = commands.spawn().id();
    map.build(&mut commands, &mut meshes, material_handle.clone(), map_entity, false);
    println!("Map width: {}, Map height: {}", MAP_WIDTH, MAP_HEIGHT);
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            // let hex_tile = world_map.get(MultiCoord::from(Offset { row: x, col: y })).unwrap();
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
