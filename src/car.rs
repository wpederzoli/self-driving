use bevy::{
    prelude::{default, Color, Component, Query, Transform, Vec3},
    sprite::{Sprite, SpriteBundle},
};

use crate::movement::{Direction, Position};

#[derive(Component)]
pub struct Car;

pub fn draw_car() -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(10., 10., 1.),
            scale: Vec3::new(30., 50., 1.),
            ..default()
        },
        ..default()
    }
}

pub fn move_car(mut car: Query<(&Car, &mut Position, &Direction, &mut Transform)>) {
    let (_, mut pos, dir, mut transform) = car.single_mut();
    pos.move_towards(&dir.get());
    transform.translation.x = pos.get_x();
    transform.translation.y = pos.get_y();
}
