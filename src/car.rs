use bevy::{
    prelude::{
        default, BuildChildren, Bundle, Color, Commands, Component, Quat, Query, Transform, Vec3,
    },
    sprite::{Sprite, SpriteBundle},
};

use crate::{
    collision::{Collider, CollisionType},
    controls::Controls,
    movement::Movement,
    sensor::SensorBundle,
};

const CAR_LAYER: f32 = 2.;
const CAR_SIZE: Vec3 = Vec3::new(30., 50., 0.);
const CAR_Y: f32 = -150.;

#[derive(Component)]
pub struct Car;

#[derive(Bundle)]
pub struct PlayerCar {
    car: Car,
    controls: Controls,
    movement: Movement,
    collider: Collider,
    sprite: SpriteBundle,
}

impl Default for PlayerCar {
    fn default() -> Self {
        PlayerCar {
            car: Car,
            controls: Controls,
            movement: Movement::default(),
            collider: Collider::new(
                Transform::from_xyz(0., 0., 2.).with_scale(CAR_SIZE),
                CollisionType::Car,
            ),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0., CAR_Y, CAR_LAYER),
                    scale: CAR_SIZE,
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn spawn_player(commands: &mut Commands) {
    commands
        .spawn(PlayerCar::default())
        .with_children(|parent| {
            parent.spawn(SensorBundle::new());
        });
}

pub fn move_car(mut car: Query<(&Car, &mut Movement, &mut Transform)>) {
    let (_, mut movement, mut transform) = car.single_mut();

    movement.accelerate();
    transform.translation.x = movement.get_x();
    transform.rotation = Quat::from_rotation_z(movement.get_angle());
}
