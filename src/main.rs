use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use car::CarPlugin;
use highway::HighwayPlugin;

mod car;
mod highway;

const SCREEN_WIDTH: f32 = 400.;
const SCREEN_HEIGHT: f32 = 800.;

#[derive(Debug, Clone, Eq, Default, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Play,
    GameOver,
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(CarPlugin)
        .add_plugin(HighwayPlugin)
        // .add_plugin(TrafficPlugin)
        // .add_state::<GameState>()
        // .add_startup_system(setup)
        // .add_systems((
        //     controls_system.run_if(in_state(GameState::Play)),
        //     move_car.run_if(in_state(GameState::Play)),
        //     move_road.run_if(in_state(GameState::Play)),
        //     sensors_collision,
        // ))
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // car::spawn_player(&mut commands);
    // road::spawn_road(&mut commands, 3, 0.);
    // road::spawn_road(&mut commands, 3, ROAD_HEIGHT);
    // road::spawn_road(&mut commands, 3, -ROAD_HEIGHT);
}
