use bevy::prelude::*;

use crate::{
    car::Car,
    collision::{Collider, CollisionType},
    controls::Controls,
    direction::DirectionType,
    lanes,
    movement::Movement,
};

const LINES_LAYER: f32 = 1.;

#[derive(Bundle)]
pub struct Border {
    #[bundle]
    sprite: SpriteBundle,
    collider: Collider,
}

impl Border {
    pub fn new(x: f32, y: f32, collision_type: CollisionType) -> Self {
        let transform =
            Transform::from_xyz(x, y, LINES_LAYER).with_scale(Vec3::new(10., ROAD_HEIGHT, 1.));
        Border {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::ANTIQUE_WHITE,
                    ..default()
                },
                transform,
                ..default()
            },
            collider: Collider::new(collision_type),
        }
    }
}

#[derive(Component)]
pub struct Road;

pub const ROAD_WIDTH: f32 = 300.;
pub const ROAD_HEIGHT: f32 = 800.;
const ROAD_LAYER: f32 = 0.;

pub fn spawn_road(commands: &mut Commands, lane_count: u32, y_pos: f32) {
    commands
        .spawn((
            Road,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    ..default()
                },
                transform: Transform::from_xyz(0., y_pos, ROAD_LAYER).with_scale(Vec3::new(
                    ROAD_WIDTH,
                    ROAD_HEIGHT,
                    1.,
                )),
                ..default()
            });
            parent.spawn(Border::new(
                -ROAD_WIDTH / 2.,
                y_pos,
                CollisionType::LeftBorder,
            ));
            parent.spawn(Border::new(
                ROAD_WIDTH / 2.,
                y_pos,
                CollisionType::RightBorder,
            ));

            lanes::spawn_lanes(parent, lane_count, y_pos);
        });
}

pub fn move_road(
    mut road: Query<(&Road, &mut Transform)>,
    car: Query<(&Car, &Movement), With<Controls>>,
) {
    let (_, movement) = car.single();

    for (_, mut transform) in road.iter_mut() {
        if transform.translation.y <= -ROAD_HEIGHT || transform.translation.y >= ROAD_HEIGHT {
            transform.translation.y = 0.;
        }

        if movement.get_speed() > 0. {
            let mut t = Transform::from(transform.clone());
            t.rotation = Quat::from_rotation_z(movement.get_angle());
            match movement.get_direction() {
                DirectionType::Forward
                | DirectionType::ForwardRight
                | DirectionType::ForwardLeft => {
                    transform.translation.y += t.down().y * movement.get_speed();
                }
                DirectionType::Reverse
                | DirectionType::ReverseLeft
                | DirectionType::ReverseRight => {
                    transform.translation.y += t.up().y * movement.get_speed();
                }
                DirectionType::Stop | DirectionType::Left | DirectionType::Right => {
                    match movement.get_last_direction() {
                        DirectionType::Forward
                        | DirectionType::ForwardRight
                        | DirectionType::ForwardLeft => {
                            transform.translation.y += t.down().y * movement.get_speed();
                        }
                        DirectionType::Reverse
                        | DirectionType::ReverseLeft
                        | DirectionType::ReverseRight => {
                            transform.translation.y += t.up().y * movement.get_speed();
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}
