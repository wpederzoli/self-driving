use bevy::{prelude::*, window::WindowResolution};
use car::Car;

mod car;

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
        .add_system(draw)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Car::new(10., 10., 20, 30));
}

fn draw(query: Query<&Car>, commands: Commands) {
    let car = query.single();
    car.draw(commands);
}
