use crate::{GameState, Inspected};
use bevy::{prelude::*, render::camera::{Camera}};

pub struct ViewportPlugin;

pub struct ViewportCamera;

impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(viewport_setup.system()));
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(viewport_camera.system()),);
    }
}

fn viewport_setup(mut commands: Commands,) {
    println!("Viewport setup");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Inspected)
        .insert(ViewportCamera);
}

pub fn viewport_camera(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, (With<Camera>, With<ViewportCamera>)>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let scale = transform.scale.x;

        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        let mut translation_speed = 1000.0;
        if keyboard_input.pressed(KeyCode::Z) {
            let scale = scale + (scale * 0.1);
            transform.scale = Vec3::new(scale, scale, 1.0);
        }

        if keyboard_input.pressed(KeyCode::X) && scale > 0.1 {
            let scale = scale - (scale * 0.1);
            transform.scale = Vec3::new(scale, scale, 1.0);
        }
        translation_speed *= transform.scale.x;

        transform.translation += time.delta_seconds() * direction * translation_speed;
    }
}
