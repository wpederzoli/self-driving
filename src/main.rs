use bevy::{prelude::*, window::WindowResolution};
use car::Car;
use controls::Controls;
use movement::Movement;

mod car;
mod controls;
mod direction;
mod movement;
mod position;
mod road;
mod speed;

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
    commands.spawn((Car, Controls, Movement::default(), car::draw_car()));
    commands.spawn(road::draw_road(300, 800));
    road::draw_lanes(&mut commands, 3, 300, 800, 10., 30.);
}
