use bevy::prelude::*;

use crate::{
    car::Car,
    collision::{Collider, CollisionType},
    direction::DirectionType,
    position::Position,
};

pub const ROAD_WIDTH: f32 = 300.;
pub const ROAD_HEIGHT: f32 = 800.;

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

    pub fn locomote(&mut self, car: &Car) {
        match (car.get_direction(), car.get_last_direction()) {
            (DirectionType::Left | DirectionType::Right, DirectionType::Stop) => (),
            (DirectionType::Left | DirectionType::Right, _) => {
                self.move_lane(&car.get_last_direction(), car.get_speed(), car.get_angle())
            }
            (DirectionType::Stop, _) => {
                self.move_lane(&car.get_last_direction(), car.get_speed(), car.get_angle())
            }
            _ => self.move_lane(&car.get_direction(), car.get_speed(), car.get_angle()),
        }
    }

    fn move_lane(&mut self, direction: &DirectionType, speed: f32, angle: f32) {
        self.position.set_angle(angle);
        self.position.move_towards(&direction.inverse(), speed, 0.);
        if self.position.get_y() > self.initial_y + 50.
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

pub fn draw_road() -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::GRAY,
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(ROAD_WIDTH, ROAD_HEIGHT, 0.),
            ..default()
        },
        ..default()
    }
}

//TODO: move magic numbers to constants
pub fn draw_lanes(
    commands: &mut Commands,
    lanes_count: u32,
    line_width: f32,
    line_height: f32,
    line_space: f32,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::ANTIQUE_WHITE,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(line_width as f32, ROAD_HEIGHT, 0.),
                translation: Vec3::new(-(ROAD_WIDTH / 2.), 0., 1.),
                ..default()
            },
            ..default()
        },
        Collider::new(
            Transform::from_xyz(-(ROAD_WIDTH / 2.), 0., 1.).with_scale(Vec3::new(
                line_width as f32,
                ROAD_HEIGHT,
                0.,
            )),
            CollisionType::LeftBorder,
        ),
    ));

    for lane in 1..lanes_count {
        for line in 0..800 / (line_height as u32 + line_space as u32) * 2 {
            let pos = Vec3::new(
                -(ROAD_WIDTH / 2.) + lane as f32 * ROAD_WIDTH / lanes_count as f32,
                -(ROAD_HEIGHT / 2.) + (line as f32 * line_space),
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

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::ANTIQUE_WHITE,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(line_width as f32, ROAD_HEIGHT, 0.),
                translation: Vec3::new(ROAD_WIDTH / 2., 0., 1.),
                ..default()
            },
            ..default()
        },
        Collider::new(
            Transform::from_xyz(ROAD_WIDTH / 2., 0., 1.).with_scale(Vec3::new(
                line_width as f32,
                ROAD_HEIGHT,
                0.,
            )),
            CollisionType::RightBorder,
        ),
    ));
}
