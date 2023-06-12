use bevy::{prelude::*, window::WindowResolution};
use car::move_car;
use controls::controls_system;
use road::{move_road, ROAD_HEIGHT};

mod car;
mod collision;
mod controls;
mod direction;
mod lanes;
mod movement;
mod position;
mod road;
mod sensor;
mod speed;

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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .add_startup_system(setup)
        .add_systems((
            controls_system.run_if(in_state(GameState::Play)),
            move_car.run_if(in_state(GameState::Play)),
            move_road.run_if(in_state(GameState::Play)),
        ))
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    car::spawn_player(&mut commands);
    road::spawn_road(&mut commands, 3, 0.);
    road::spawn_road(&mut commands, 3, ROAD_HEIGHT);
    road::spawn_road(&mut commands, 3, -ROAD_HEIGHT);
}
