use bevy::prelude::*;

use crate::collision::{Collider, CollisionType};

pub const SENSOR_LAYER: f32 = 2.;

#[derive(Component)]
pub struct Sensor;

#[derive(Bundle)]
pub struct SensorBundle {
    sensor: Sensor,
    #[bundle]
    sprite: SpriteBundle,
    collider: Collider,
}

impl SensorBundle {
    pub fn new(transform: Transform) -> Self {
        SensorBundle {
            sensor: Sensor,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    ..default()
                },
                transform,
                ..default()
            },
            collider: Collider::new(transform, CollisionType::Sensor),
        }
    }
}

pub fn sensors_collision(
    mut sensors: Query<(&Sensor, &mut Collider, &GlobalTransform, &mut Sprite)>,
    colliders: Query<&Collider, Without<Sensor>>,
) {
    for (_, mut collider, transform, mut sprite) in sensors.iter_mut() {
        for other_collider in colliders.iter() {
            collider.set_transform(transform.compute_transform());
            collider.check_collision(&other_collider);
            match collider.get_collision() {
                CollisionType::RightBorder => {
                    println!("Sensor collided with right border!");
                }
                CollisionType::LeftBorder => {
                    println!("Sensor collided with left border!");
                }
                CollisionType::Car => {
                    sprite.color = Color::YELLOW_GREEN;
                    println!("Sensor clollided with a Car");
                }
                _ => (),
            }
        }
    }
}
