use crate::{GameState, loading::{FontAssets, TextureAssets}};
use bevy::{diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}, prelude::*};

pub struct PlayStatePlugin;

impl Plugin for PlayStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_event::<DateEvent>();
        app.init_resource::<ButtonMaterials>();
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(setup_camera.system())
                .with_system(setup_play_button.system())
                .with_system(setup_date.system())
                .with_system(setup_date_text.system())
                .with_system(setup_fps_text.system())
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(date_tick.system())
                .with_system(play_button_update.system())
                .with_system(date_text_update.system())
                .with_system(fps_text_update.system())
        );
    }
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}
impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

enum GameSpeed {
    Slow,
    Normal,
    Fast,
}
impl GameSpeed {
    fn ticks(&self) -> f64 {
        match *self {
            GameSpeed::Slow => 500.0,
            GameSpeed::Normal => 200.0,
            GameSpeed::Fast => 50.0,
        }
    }
}

pub struct PlayState {
    is_playing: bool,
    game_speed: GameSpeed,
    date: u32,
}

struct DateEvent {
    pub date: u32,
}

fn setup_date(
    mut commands: Commands,
) {
    commands.spawn().insert(PlayState {
        is_playing: false,
        game_speed: GameSpeed::Fast,
        date: 0,
    });
}

fn date_tick(
    mut query: Query<&mut PlayState>,
    mut last_time: Local<f64>,
    time: Res<Time>,
    mut date_event: EventWriter<DateEvent>,
) {
    let mut play_state = query.single_mut().unwrap();
    if play_state.is_playing {
        let time_since_update = time.seconds_since_startup() - *last_time;
        
        if time_since_update * 1000.0 > play_state.game_speed.ticks() {
            play_state.date += 1;
            date_event.send(DateEvent { date: play_state.date });
            // println!("Date: {}", play_state.date);
            *last_time = time.seconds_since_startup();
        }
    }
}

// UI

struct FpsText;
struct DateText;
struct PlayButton;

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_fps_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ui_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    };
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: ui_text_style.clone(),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: ui_text_style.clone(),
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);
}

fn setup_date_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ui_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    };
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position: Rect {
                    top: Val::Px(24.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Date: ".to_string(),
                        style: ui_text_style.clone(),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: ui_text_style.clone(),
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(DateText);
}

fn setup_play_button(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    button_materials: Res<ButtonMaterials>,
    // tilesets: Res<TextureAssets>,
) {
    commands
    .spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(40.0), Val::Px(24.0)),
            align_self: AlignSelf::FlexStart,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                ..Default::default()
            },
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        material: button_materials.normal.clone(),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Play",
                TextStyle {
                    font: fonts.fira_sans.clone(),
                    font_size: 16.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default(),
            ),
            ..Default::default()
        });
    })
    .insert(PlayButton);
}

fn play_button_update(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, (With<Button>, With<PlayButton>)),
    >,
    mut text_query: Query<&mut Text>,
    mut play_query: Query<&mut PlayState>,
) {
    let mut play_state = play_query.single_mut().unwrap();
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                play_state.is_playing = !play_state.is_playing;
                if play_state.is_playing == true {
                    text.sections[0].value = "Pause".to_string();
                } else {
                    text.sections[0].value = "Play".to_string();
                }
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn fps_text_update(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}

fn date_text_update(
    mut query: Query<&mut Text, With<DateText>>,
    mut events: EventReader<DateEvent>,
) {
    for date_event in events.iter() {
        for mut text in query.iter_mut() {
            text.sections[1].value = format!("{}", &date_event.date);
        }
    }
}
