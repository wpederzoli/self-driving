use bevy::{
    prelude::{
        default, BuildChildren, Bundle, Color, Commands, Component, NextState, Quat, Query, ResMut,
        Transform, Vec3, Without,
    },
    sprite::{Sprite, SpriteBundle},
};

use crate::{
    collision::{Collider, CollisionType},
    controls::Controls,
    movement::Movement,
    sensor::SensorBundle,
    GameState,
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
        let transform = Transform::from_xyz(0., CAR_Y, CAR_LAYER).with_scale(CAR_SIZE);
        PlayerCar {
            car: Car,
            controls: Controls,
            movement: Movement::default(),
            collider: Collider::new(transform, CollisionType::Car),
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
            parent.spawn(SensorBundle::new());
        });
}

pub fn move_car(
    mut car: Query<(&Car, &mut Movement, &mut Transform, &mut Collider)>,
    colliders: Query<&Collider, Without<Car>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (_, mut movement, mut transform, mut car_collider) = car.single_mut();

    movement.accelerate();
    transform.translation.x = movement.get_x();
    transform.rotation = Quat::from_rotation_z(movement.get_angle());
    car_collider.set_transform(*transform);

    for other_collider in colliders.iter() {
        car_collider.check_collision(&other_collider);
        match car_collider.get_collision() {
            CollisionType::LeftBorder | CollisionType::RightBorder => {
                next_state.set(GameState::GameOver);
            }
            _ => (),
        }
    }
}
