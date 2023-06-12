use bevy::prelude::*;

use crate::road::{ROAD_HEIGHT, ROAD_WIDTH};

const LINE_HEIGHT: f32 = 35.;
const LINE_WIDTH: f32 = 8.;
const LINE_SPACE: f32 = 15.;

pub fn spawn_lanes(parent: &mut ChildBuilder, lane_count: u32) {
    for lane in 1..lane_count {
        for line in 0..(ROAD_HEIGHT / (LINE_HEIGHT + LINE_SPACE)) as u32 + 1 {
            let pos = Vec3::new(
                lane as f32 * (ROAD_WIDTH / lane_count as f32) - ROAD_WIDTH / 2.,
                (line as f32 * (LINE_SPACE + LINE_HEIGHT)) - ROAD_HEIGHT / 2.,
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
                    1.,
                )),
                ..default()
            });
        }
    }
}
