use bevy::prelude::*;

use crate::collision::{Collider, CollisionType};

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
    pub fn new() -> Self {
        let transform = Transform::from_xyz(0., 1., 5.).with_scale(Vec3::new(0.25, 1.1, 1.));
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
