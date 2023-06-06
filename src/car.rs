use bevy::{
    prelude::{default, Color, Component, Query, Transform, Vec3},
    sprite::{Sprite, SpriteBundle},
};

use crate::movement::Movement;

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

pub fn move_car(mut car: Query<(&Car, &mut Movement, &mut Transform)>) {
    let (_, mut m, mut transform) = car.single_mut();
    m.locomote();
    transform.translation.x = m.get_x();
    transform.translation.y = m.get_y();
}
