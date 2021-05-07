use std::collections::HashMap;

use crate::{GameState, Inspected, loading::TextureAssets};
use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use chickenwire::{coordinate::{CoordSys, MultiCoord, Offset}, hexgrid::{Parity, Tilt}, prelude::HexGrid};
use noise::{*, utils::{*}};
use rand::{Rng, thread_rng};

pub struct MapviewPlugin;

impl Plugin for MapviewPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(LogDiagnosticsPlugin::default());
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(
                    setup_tilemap.system()
                        .label("setup_tilemap")
                )
        );
    }
}

const TILE_WIDTH: f32 = 32.0;
const TILE_HEIGHT: f32 = 32.0;

#[derive(Debug, Copy, Clone)]
enum TerrainType {
    OCEAN = 0,
    LAND = 2,
}

#[derive(Debug, Copy, Clone)]
struct HexTile {
    terrain_type: TerrainType,
}

fn generate_heightmap(width: usize, height: usize) -> NoiseMap {
    let noise = Fbm::new()
        .set_seed(1234)
        .set_persistence(0.5)
        .set_frequency(1.0)
        .set_lacunarity(2.0);
    let builder = SphereMapBuilder::new(&noise);
    let noise_map = builder
        .set_bounds(-90., 90., -180., 180.)
        .set_size(width, height)
        .build();
    noise_map.write_to_file("noise_test.png");
    return noise_map;
}

const CHUNK_WIDTH: f32 = 6.0;
const CHUNK_HEIGHT: f32 = 3.0;
const CHUNK_SIZE_WIDTH: f32 = 64.0;
const CHUNK_SIZE_HEIGHT: f32 = 64.0;
const MAP_WIDTH: i32 = (CHUNK_WIDTH * CHUNK_SIZE_WIDTH) as i32;
const MAP_HEIGHT: i32 = (CHUNK_HEIGHT * CHUNK_SIZE_HEIGHT) as i32;

fn setup_tilemap(
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

    let heightmap = generate_heightmap(MAP_WIDTH as usize, MAP_HEIGHT as usize);
    // println!("(0,0) = {}", heightmap.get_value(0, 0));

    let mut hex_grid = HexGrid::<HexTile>::new(Tilt::Flat, Parity::Even, CoordSys::Offset);
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let height = heightmap.get_value(x as usize, y as usize);
            // println!("height at {},{} = {:.}", x as usize, y as usize, height);
            let mut terrain_type = TerrainType::OCEAN;
            if height >= 0.05 {
                terrain_type = TerrainType::LAND;
            }
            hex_grid.add(MultiCoord::from(Offset { row: x, col: y }), HexTile {
                terrain_type,
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
    println!("Map width: {}, Map height: {} ({} tiles)", MAP_WIDTH, MAP_HEIGHT, MAP_WIDTH * MAP_HEIGHT);
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let hex_tile = hex_grid.get(MultiCoord::from(Offset { row: x, col: y })).unwrap();
            let tile_pos = MapVec2::new(x, y);
            map.add_tile(&mut commands, tile_pos, Tile {
                texture_index: hex_tile.terrain_type as u32,
                ..Default::default()
            }).unwrap();
        }
    }

    commands.entity(map_entity).insert_bundle(MapBundle {
        map,
        ..Default::default()
    });
}
