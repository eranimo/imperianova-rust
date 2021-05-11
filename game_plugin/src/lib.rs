mod loading;
mod viewport;
mod mapview;
mod menu;
mod playstate;

use crate::viewport::ViewportPlugin;
use crate::mapview::MapviewPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::playstate::PlayStatePlugin;

use bevy::app::AppBuilder;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_ecs_tilemap::TileMapPlugin;
use bevy_inspector_egui::{InspectorPlugin, widgets::InspectorQuery};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Playing,
    Menu,
}

struct Inspected;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(TileMapPlugin)
            .add_plugin(InspectorPlugin::<InspectorQuery<Entity, With<Inspected>>>::new())
            .add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)

            .add_plugin(PlayStatePlugin)
            .add_plugin(ViewportPlugin)
            .add_plugin(MapviewPlugin)
            // .add_plugin(FrameTimeDiagnosticsPlugin::default())
            // .add_plugin(LogDiagnosticsPlugin::default())
            ;
    }
}
