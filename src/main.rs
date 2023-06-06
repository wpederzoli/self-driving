use bevy::{prelude::*, window::WindowResolution};
use car::Car;
use controls::Controls;
use movement::{Direction, Position};

mod car;
mod controls;
mod movement;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(400., 800.),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_systems((car::move_car, controls::controls_system))
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Car,
        Controls,
        Position::default(),
        Direction::default(),
        car::draw_car(),
    ));
}
