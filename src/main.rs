use bevy::{prelude::*, window::WindowResolution};
use car::Car;
use movement::{Direction, Position};

mod car;
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
        .add_system(car::move_car)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Car,
        Position::default(),
        Direction::default(),
        car::draw_car(),
    ));
}
