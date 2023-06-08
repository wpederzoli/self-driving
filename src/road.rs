use bevy::prelude::*;

use crate::{direction::DirectionType, position::Position};

#[derive(Component)]
pub struct Lane {
    position: Position,
    initial_y: f32,
}

impl Lane {
    pub fn new(position: Position, initial_y: f32) -> Self {
        Lane {
            position,
            initial_y,
        }
    }

    pub fn get_y(&self) -> f32 {
        self.position.get_y()
    }

    pub fn move_lane(&mut self, direction: &DirectionType, speed: f32, angle: f32) {
        self.position.set_angle(angle);
        self.position.move_towards(&direction.inverse(), speed, 0.);
        if self.position.get_y() > self.initial_y + 95.
            || self.position.get_y() < self.initial_y - 50.
        {
            self.position = Position::new(
                self.position.get_x(),
                self.initial_y,
                self.position.get_angle(),
            );
        }
    }
}

pub fn draw_road(width: u32, height: u32) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::GRAY,
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(width as f32, height as f32, 0.),
            ..default()
        },
        ..default()
    }
}

//TODO: move magic numbers to constants
pub fn draw_lanes(
    commands: &mut Commands,
    lanes_count: u32,
    road_width: u32,
    road_height: u32,
    line_width: f32,
    line_height: f32,
) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::ANTIQUE_WHITE,
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(line_width as f32, road_height as f32, 0.),
            translation: Vec3::new(-(road_width as f32 / 2.), 0., 1.),
            ..default()
        },
        ..default()
    });

    for lane in 1..lanes_count {
        for line in 0..20 {
            let pos = Vec3::new(
                -(road_width as f32 / 2.) + lane as f32 * road_width as f32 / lanes_count as f32,
                -(road_height as f32 / 2.) - 100. + (line as f32 * 50.),
                1.,
            );
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::ANTIQUE_WHITE,
                        ..default()
                    },
                    transform: Transform {
                        scale: Vec3::new(line_width, line_height, 0.),
                        translation: pos,
                        ..default()
                    },
                    ..default()
                },
                Lane::new(Position::new(pos.x, pos.y, 0.), pos.y),
            ));
        }
    }

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::ANTIQUE_WHITE,
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(line_width as f32, road_height as f32, 0.),
            translation: Vec3::new(road_width as f32 / 2., 0., 1.),
            ..default()
        },
        ..default()
    });
}
