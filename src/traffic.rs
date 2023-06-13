use bevy::prelude::*;

use crate::{
    car::{Car, CAR_SIZE},
    collision::{Collider, CollisionType},
    movement::Movement,
};

pub struct TrafficPlugin;

impl Plugin for TrafficPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    let transform = Transform::from_xyz(10., 10., 3.).with_scale(CAR_SIZE);

    commands.spawn(TrafficCar {
        car: Car,
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                ..default()
            },
            transform,
            ..default()
        },
        collider: Collider::new(transform, CollisionType::Car),
        movement: Movement::default(),
    });
}

#[derive(Bundle)]
pub struct TrafficCar {
    car: Car,
    #[bundle]
    sprite: SpriteBundle,
    collider: Collider,
    movement: Movement,
}
