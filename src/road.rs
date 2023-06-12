use bevy::prelude::*;

use crate::{
    collision::{Collider, CollisionType},
    lanes,
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
            collider: Collider::new(transform, collision_type),
        }
    }
}

#[derive(Component)]
pub struct Road;

pub const ROAD_WIDTH: f32 = 300.;
pub const ROAD_HEIGHT: f32 = 800.;
const ROAD_LAYER: f32 = 0.;

pub fn spawn_road(commands: &mut Commands, lane_count: u32) {
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
                transform: Transform::from_xyz(0., 0., ROAD_LAYER).with_scale(Vec3::new(
                    ROAD_WIDTH,
                    ROAD_HEIGHT,
                    1.,
                )),
                ..default()
            });
            parent.spawn(Border::new(-ROAD_WIDTH / 2., 0., CollisionType::LeftBorder));
            parent.spawn(Border::new(ROAD_WIDTH / 2., 0., CollisionType::RightBorder));

            lanes::spawn_lanes(parent, lane_count);
        });
}
