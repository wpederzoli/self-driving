use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, Anchor},
};

use crate::{
    car::Car,
    collision::{Collider, CollisionType},
    controls::Controls,
};

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
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                transform,
                ..default()
            },
            collider: Collider::new(CollisionType::None),
        }
    }
}

pub fn sensors_collision(
    mut sensors: Query<(&Collider, &Transform, &mut Sprite), With<Sensor>>,
    colliders: Query<(&Collider, &Transform), Without<Car>>,
) {
    for (_, transform, mut sprite) in sensors.iter_mut() {
        for (cc, collider) in colliders.iter() {
            if cc.collider_type != CollisionType::None {
                if let Some(c) = collide(
                    transform.translation,
                    Vec2::new(transform.scale.x, transform.scale.y),
                    collider.translation,
                    Vec2::new(collider.scale.x, collider.scale.y),
                ) {
                    println!("Sensor colliding {:?} {:?}", c, cc.collider_type);
                }
            }
        }
        // for other_collider in colliders.iter() {
        //     collider.set_transform(transform.compute_transform());
        //     collider.check_collision(&other_collider);
        //     match collider.get_collision() {
        //         CollisionType::RightBorder => {
        //             println!("Sensor collided with right border!");
        //         }
        //         CollisionType::LeftBorder => {
        //             println!("Sensor collided with left border!");
        //         }
        //         CollisionType::Car => {
        //             sprite.color = Color::YELLOW_GREEN;
        //             println!("Sensor clollided with a Car");
        //         }
        //         _ => (),
        //     }
        // }
    }
}
