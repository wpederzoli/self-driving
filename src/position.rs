use bevy::prelude::*;

use crate::direction::DirectionType;

#[derive(Component)]
pub struct Position {
    x: f32,
    y: f32,
    angle: f32,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            x: 10.,
            y: 10.,
            angle: 0.,
        }
    }
}

impl Position {
    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }

    fn rotate(&mut self, speed: f32) {
        self.angle += speed;
    }

    pub fn move_towards(&mut self, direction: &DirectionType, speed: f32) {
        match direction {
            DirectionType::Forward => self.y += speed,
            DirectionType::Reverse => self.y -= speed,
            DirectionType::Left => self.rotate(0.01),
            DirectionType::Right => self.rotate(-0.01),
            DirectionType::ForwardLeft => {
                self.x -= speed as f32 / 2.;
                self.y += speed as f32 / 2.;
            }
            DirectionType::ForwardRight => {
                self.x += speed as f32 / 2.;
                self.y += speed as f32 / 2.;
            }
            DirectionType::ReverseLeft => {
                self.x -= speed as f32 / 2.;
                self.y -= speed as f32 / 2.;
            }
            DirectionType::ReverseRight => {
                self.x += speed as f32 / 2.;
                self.y -= speed as f32 / 2.;
            }
            DirectionType::Stop => {
                self.x = self.x;
                self.y = self.y;
            }
        }
    }
}
