use bevy::{
    prelude::{
        default, BuildChildren, Bundle, Children, Color, Commands, Component, Entity,
        GlobalTransform, NextState, Quat, Query, ResMut, Transform, Vec2, Vec3, With, Without,
    },
    reflect::erased_serde::__private::serde::__private::de,
    sprite::{
        collide_aabb::{self, collide},
        Sprite, SpriteBundle,
    },
};

use crate::{
    collision::{Collider, CollisionType},
    controls::Controls,
    movement::Movement,
    sensor::{Sensor, SensorBundle, SENSOR_LAYER},
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
            collider: Collider::new(CollisionType::None),
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
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    ..default()
                },
                transform: Transform::from_xyz(0., 1.5, SENSOR_LAYER)
                    .with_scale(Vec3::new(1., 1., 0.))
                    .with_rotation(Quat::from_rotation_z(0.)),
                ..default()
            });
        });
    // .with_children(|parent| {
    //     parent.spawn(SensorBundle::new(
    //         Transform::from_xyz(0., 1., SENSOR_LAYER).with_scale(Vec3::new(0.1, 1.1, 1.)),
    //     ));
    //     parent.spawn(SensorBundle::new(
    //         Transform::from_xyz(0.5, 0.95, SENSOR_LAYER)
    //             .with_scale(Vec3::new(0.1, 1.1, 1.))
    //             .with_rotation(Quat::from_rotation_z(-10.)),
    //     ));
    // parent.spawn(SensorBundle::new(
    //     Transform::from_xyz(-0.5, 0.95, SENSOR_LAYER)
    //         .with_scale(Vec3::new(0.1, 1.15, 1.))
    //         .with_rotation(Quat::from_rotation_z(10.)),
    // ));
    // parent.spawn(SensorBundle::new(
    //     Transform::from_xyz(1., 0.7, SENSOR_LAYER)
    //         .with_scale(Vec3::new(1.25, 0.1, 1.))
    //         .with_rotation(Quat::from_rotation_z(35.)),
    // ));
    // parent.spawn(SensorBundle::new(
    //     Transform::from_xyz(-1., 0.7, SENSOR_LAYER)
    //         .with_scale(Vec3::new(1.25, 0.1, 1.))
    //         .with_rotation(Quat::from_rotation_z(-35.)),
    // ));
    // });
}

pub fn move_car(
    mut car: Query<(&Car, &mut Transform, &mut Movement, &Children), With<Controls>>,
    mut sensor: Query<&mut Transform, (Without<Controls>, Without<Collider>)>,
    mut colliders: Query<&Transform, (With<Collider>, Without<Controls>)>,
) {
    let (_, mut transform, mut movement, children) = car.single_mut();
    movement.accelerate();
    transform.translation.x = movement.get_x();
    transform.rotation = Quat::from_rotation_z(movement.get_angle());

    println!("parent t: {:?}", transform);

    for &child in children.iter() {
        let mut t = sensor.get_mut(child).ok().unwrap();
        let mut vecT = t.translation;
        if vecT.x == 0. {
            vecT.x = transform.translation.x;
        } else {
            vecT.x *= transform.translation.x;
        }

        if vecT.y == 0. {
            vecT.y = transform.translation.y;
        } else {
            vecT.y *= transform.translation.y;
        }

        println!("vec3 {:?}", vecT);
        for collider in colliders.iter_mut() {
            if let Some(_) = collide(
                vecT,
                Vec2::new(CAR_SIZE.x, CAR_SIZE.y),
                collider.translation,
                Vec2::new(collider.scale.x, collider.scale.y),
            ) {
                println!("COLLISION!!!!");
            }
        }
    }
}
