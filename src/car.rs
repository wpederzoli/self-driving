use bevy::{
    prelude::{default, Color, Component, Quat, Query, Transform, Vec3, Without},
    sprite::{Sprite, SpriteBundle},
};

use crate::{direction::DirectionType, movement::Movement, road::Lane};

#[derive(Component)]
pub struct Car;

pub fn draw_car() -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(10., 10., 2.),
            scale: Vec3::new(30., 50., 1.),
            ..default()
        },
        ..default()
    }
}

pub fn move_car(
    mut car: Query<(&Car, &mut Movement, &mut Transform)>,
    mut lanes: Query<(&mut Transform, &Lane), Without<Car>>,
) {
    let (_, mut m, mut transform) = car.single_mut();
    m.accelerate();
    transform.translation.x = m.get_x();
    transform.rotation = Quat::from_rotation_z(m.get_angle());

    for (mut t, lane) in lanes.iter_mut() {
        match m.get_direction() {
            DirectionType::Forward | DirectionType::ForwardRight | DirectionType::ForwardLeft => {
                t.translation.y -= m.get_speed();
                if t.translation.y < lane.0 - 95. {
                    t.translation.y = lane.0;
                }
            }
            DirectionType::Reverse | DirectionType::ReverseRight | DirectionType::ReverseLeft => {
                t.translation.y += m.get_speed();
                if t.translation.y > lane.0 + 50. {
                    t.translation.y = lane.0;
                }
            }
            DirectionType::Stop => match m.get_last_direction() {
                DirectionType::Forward
                | DirectionType::ForwardRight
                | DirectionType::ForwardLeft => {
                    t.translation.y -= m.get_speed();
                    if t.translation.y < lane.0 - 95. {
                        t.translation.y = lane.0;
                    }
                }
                DirectionType::Reverse
                | DirectionType::ReverseLeft
                | DirectionType::ReverseRight => {
                    t.translation.y += m.get_speed();
                    if t.translation.y > lane.0 + 50. {
                        t.translation.y = lane.0;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}
