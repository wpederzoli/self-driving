use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::SCREEN_HEIGHT;

pub const ROAD_WIDTH: f32 = 340.;
pub const ROAD_HEIGHT: f32 = 800.;
pub const ROAD_LAYER: f32 = 0.;
pub const LINES_LAYER: f32 = 1.;
pub const LINE_HEIGHT: f32 = 30.;
pub const LINE_WIDTH: f32 = 10.;
pub const LINE_SPACE: f32 = 10.;

#[derive(Component)]
pub struct Lane;

pub fn draw_borders(commands: &mut Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::ANTIQUE_WHITE,
                custom_size: Some(Vec2::new(10., SCREEN_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(-170., 0., LINES_LAYER),
            global_transform: GlobalTransform::from(Transform::from_xyz(
                -ROAD_WIDTH / 2.,
                0.,
                LINES_LAYER,
            )),
            ..default()
        })
        .insert(Collider::cuboid(5., SCREEN_HEIGHT / 2.));
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::ANTIQUE_WHITE,
                custom_size: Some(Vec2::new(10., SCREEN_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(170., 0., LINES_LAYER),
            global_transform: GlobalTransform::from(Transform::from_xyz(
                ROAD_WIDTH / 2.,
                0.,
                LINES_LAYER,
            )),
            ..default()
        })
        .insert(Collider::cuboid(5., SCREEN_HEIGHT / 2.));
}

pub fn draw_lines(commands: &mut Commands, lanes: u8) {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    ..default()
                },
                ..default()
            },
            Lane,
        ))
        .with_children(|parent| {
            for lane in 1..lanes {
                for line in 0..(ROAD_HEIGHT / (LINE_HEIGHT + LINE_SPACE)) as u32 + 1 {
                    let pos = Vec3::new(
                        lane as f32 * (ROAD_WIDTH / lanes as f32) - ROAD_WIDTH / 2.,
                        (line as f32 * (LINE_SPACE + LINE_HEIGHT)) - (ROAD_HEIGHT / 2.) + 400.,
                        1.,
                    );
                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::ANTIQUE_WHITE,
                            ..default()
                        },
                        transform: Transform::from_xyz(pos.x, pos.y, pos.z).with_scale(Vec3::new(
                            LINE_WIDTH,
                            LINE_HEIGHT,
                            4.,
                        )),
                        ..default()
                    });
                }
            }
        });
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    ..default()
                },
                ..default()
            },
            Lane,
        ))
        .with_children(|parent| {
            for lane in 1..lanes {
                for line in 0..(ROAD_HEIGHT / (LINE_HEIGHT + LINE_SPACE)) as u32 + 1 {
                    let pos = Vec3::new(
                        lane as f32 * (ROAD_WIDTH / lanes as f32) - ROAD_WIDTH / 2.,
                        (line as f32 * (LINE_SPACE + LINE_HEIGHT)) - (ROAD_HEIGHT / 2.) - 400.,
                        1.,
                    );
                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::ANTIQUE_WHITE,
                            ..default()
                        },
                        transform: Transform::from_xyz(pos.x, pos.y, pos.z).with_scale(Vec3::new(
                            LINE_WIDTH,
                            LINE_HEIGHT,
                            4.,
                        )),
                        ..default()
                    });
                }
            }
        });
}
