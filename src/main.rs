use bevy::{prelude::*, window::WindowResolution};
use collision::{Collider, CollisionType};

mod car;
mod collision;
mod controls;
mod direction;
mod movement;
mod position;
mod road;
mod speed;

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
                resolution: WindowResolution::new(400., 800.),
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .add_startup_system(setup)
        .add_systems((
            car::move_car.run_if(in_state(GameState::Play)),
            controls::controls_system.run_if(in_state(GameState::Play)),
        ))
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(car::init_car());
    commands.spawn(road::draw_road());
    commands.spawn(Collider::new(
        Transform::from_xyz(0., 400., 4.).with_scale(Vec3::new(800., 150., 0.)),
        CollisionType::TopBound,
    ));
    commands.spawn(Collider::new(
        Transform::from_xyz(0., -400., 4.).with_scale(Vec3::new(800., 150., 0.)),
        CollisionType::BottomBound,
    ));
    road::draw_lanes(&mut commands, 3, 10., 30., 50.);
}
