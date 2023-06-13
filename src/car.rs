use bevy::{
    prelude::{
        default, BuildChildren, Bundle, Color, Commands, Component, NextState, Quat, Query, ResMut,
        Transform, Vec3, With, Without,
    },
    sprite::{Sprite, SpriteBundle},
};

use crate::{
    collision::{Collider, CollisionType},
    controls::Controls,
    movement::Movement,
    sensor::{SensorBundle, SENSOR_LAYER},
    GameState,
};

pub const CAR_LAYER: f32 = 3.;
pub const CAR_SIZE: Vec3 = Vec3::new(30., 50., 0.);
const CAR_Y: f32 = -150.;

#[derive(Component)]
pub struct Car;

#[derive(Bundle)]
pub struct PlayerCar {
    car: Car,
    controls: Controls,
    movement: Movement,
    collider: Collider,
    #[bundle]
    sprite: SpriteBundle,
}

impl Default for PlayerCar {
    fn default() -> Self {
        let transform = Transform::from_xyz(0., CAR_Y, CAR_LAYER).with_scale(CAR_SIZE);
        PlayerCar {
            car: Car,
            controls: Controls,
            movement: Movement::default(),
            collider: Collider::new(transform, CollisionType::Player),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    ..default()
                },
                transform,
                ..default()
            },
        }
    }
}

pub fn spawn_player(commands: &mut Commands) {
    commands
        .spawn(PlayerCar::default())
        .with_children(|parent| {
            parent.spawn(SensorBundle::new(
                Transform::from_xyz(0., 1., SENSOR_LAYER).with_scale(Vec3::new(0.25, 1.1, 1.)),
            ));
            parent.spawn(SensorBundle::new(
                Transform::from_xyz(0.5, 0.95, SENSOR_LAYER)
                    .with_scale(Vec3::new(0.25, 1.1, 1.))
                    .with_rotation(Quat::from_rotation_z(-10.)),
            ));
            parent.spawn(SensorBundle::new(
                Transform::from_xyz(-0.5, 0.95, SENSOR_LAYER)
                    .with_scale(Vec3::new(0.25, 1.15, 1.))
                    .with_rotation(Quat::from_rotation_z(10.)),
            ));
            parent.spawn(SensorBundle::new(
                Transform::from_xyz(1., 0., SENSOR_LAYER).with_scale(Vec3::new(1.1, 0.15, 1.)),
            ));
            parent.spawn(SensorBundle::new(
                Transform::from_xyz(-1., 0., SENSOR_LAYER).with_scale(Vec3::new(1.1, 0.15, 1.)),
            ));
            parent.spawn(SensorBundle::new(
                Transform::from_xyz(0., -1., SENSOR_LAYER).with_scale(Vec3::new(0.25, -1.1, 1.)),
            ));
        });
}

pub fn move_car(
    mut car: Query<(&mut Sprite, &mut Movement, &mut Transform, &mut Collider), With<Controls>>,
    colliders: Query<&Collider, Without<Controls>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (mut sprite, mut movement, mut transform, mut car_collider) = car.single_mut();

    movement.accelerate();
    transform.translation.x = movement.get_x();
    transform.rotation = Quat::from_rotation_z(movement.get_angle());
    car_collider.set_transform(*transform);

    for other_collider in colliders.iter() {
        car_collider.check_collision(&other_collider);
        match car_collider.get_collision() {
            CollisionType::LeftBorder | CollisionType::RightBorder | CollisionType::Car => {
                sprite.color = Color::RED.with_a(0.5);
                next_state.set(GameState::GameOver);
            }
            _ => (),
        }
    }
}
