use bevy::{asset::LoadState, input::mouse::MouseWheel, prelude::*, render::camera::OrthographicProjection, sprite::TextureAtlasBuilder, window::WindowMode};
use bevy_tilemap::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Imperianova".to_string(),
            width: 1024.,
            height: 720.,
            vsync: false,
            resizable: true,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .init_resource::<SpriteHandles>()
        .init_resource::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapDefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup.system())
        .add_system(load.system())
        .add_system(build_world.system())
        // .add_system(zoom_system.system())
        .add_system(panning_system.system())
        .run()
}

#[derive(Default, Clone)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
    atlas_loaded: bool,
}

#[derive(Default, Clone)]
struct GameState {
    map_loaded: bool,
    spawned: bool,
}

struct MainCamera;

fn setup(mut tile_sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    tile_sprite_handles.handles = asset_server.load_folder("textures").unwrap();
}

fn panning_system(
    sprite_handles: Res<SpriteHandles>,
    keyboard_input: Res<Input<KeyCode>>,
    mut cam: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
) {
    if sprite_handles.atlas_loaded {
        let (mut pos, mut _cam) = cam.single_mut().unwrap();

        if keyboard_input.pressed(KeyCode::W) {
            pos.translation.y += 10.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            pos.translation.y -= 10.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            pos.translation.x -= 10.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            pos.translation.x += 10.0;
        }
    }
}

// TODO: fix this
fn zoom_system(
    mut whl: EventReader<MouseWheel>,
    mut cam: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
    windows: Res<Windows>,
) {
    const ZOOM_SPEED: f32 = 0.1;
    const MIN_ZOOM: f32 = 0.1;
    const MAX_ZOOM: f32 = 3.0;
    let delta_zoom: f32 = whl.iter().map(|e| e.y).sum();
    if delta_zoom == 0. {
        return;
    }

    let (mut pos, mut cam) = cam.single_mut().unwrap();

    let window = windows.get_primary().unwrap();
    let window_size = Vec2::new(window.width(), window.height());
    let mouse_normalized_screen_pos =
        (window.cursor_position().unwrap() / window_size) * 2. - Vec2::ONE;
    let mouse_world_pos = pos.translation.truncate()
        + mouse_normalized_screen_pos * Vec2::new(cam.right, cam.top) * cam.scale;

    cam.scale -= ZOOM_SPEED * delta_zoom * cam.scale;
    cam.scale = cam.scale.clamp(MIN_ZOOM, MAX_ZOOM);

    pos.translation = (mouse_world_pos
        - mouse_normalized_screen_pos * Vec2::new(cam.right, cam.top) * cam.scale)
        .extend(pos.translation.z);
}

fn load(
    mut commands: Commands,
    mut sprite_handles: ResMut<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    asset_server: Res<AssetServer>,
) {
    if sprite_handles.atlas_loaded {
        return;
    }

    // Lets load all our textures from our folder!
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        for handle in sprite_handles.handles.iter() {
            let texture = textures.get(handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), &texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let atlas_handle = texture_atlases.add(texture_atlas);

        let tilemap = Tilemap::builder()
            .auto_chunk()
            .auto_spawn(2, 2)
            .topology(GridTopology::HexEvenCols)
            .dimensions(4, 4)
            .chunk_dimensions(8, 4, 1)
            .texture_dimensions(37, 32)
            .texture_atlas(atlas_handle)
            .finish()
            .unwrap();

        let tilemap_components = TilemapBundle {
            tilemap,
            visible: Visible {
                is_visible: true,
                is_transparent: true,
            },
            transform: Default::default(),
            global_transform: Default::default(),
        };

        commands
            .spawn()
            .insert_bundle(OrthographicCameraBundle::new_2d())
            .insert(MainCamera);
        commands
            .spawn()
            .insert_bundle(tilemap_components)
            .insert(Timer::from_seconds(0.075, true));

        sprite_handles.atlas_loaded = true;
    }
}

fn build_world(
    mut game_state: ResMut<GameState>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Tilemap>,
) {
    if game_state.map_loaded {
        return;
    }

    for mut map in query.iter_mut() {
        let chunk_width = (map.width().unwrap() * map.chunk_width()) as i32;
        let chunk_height = (map.height().unwrap() * map.chunk_height()) as i32;

        let grass_floor: Handle<Texture> =
            asset_server.get_handle("textures/hex-floor-grass_alt.png");
        let texture_atlas = texture_atlases.get(map.texture_atlas()).unwrap();
        let grass_index = texture_atlas.get_texture_index(&grass_floor).unwrap();

        let mut tiles = Vec::new();
        for y in 0..chunk_height {
            for x in 0..chunk_width {
                let y = y - chunk_height / 2;
                let x = x - chunk_width / 2;
                let tile = Tile {
                    point: (x, y),
                    sprite_index: grass_index,
                    ..Default::default()
                };
                tiles.push(tile);
            }
        }
        map.insert_tiles(tiles).unwrap();
        println!("Chunk width: {} height: {}", chunk_width, chunk_height);

        map.spawn_chunk((-1, 0)).unwrap();
        map.spawn_chunk((0, 0)).unwrap();
        map.spawn_chunk((1, 0)).unwrap();
        map.spawn_chunk((-1, 1)).unwrap();
        map.spawn_chunk((0, 1)).unwrap();
        map.spawn_chunk((1, 1)).unwrap();
        map.spawn_chunk((-1, -1)).unwrap();
        map.spawn_chunk((0, -1)).unwrap();
        map.spawn_chunk((1, -1)).unwrap();

        game_state.map_loaded = true;
    }
}
