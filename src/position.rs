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
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        Position { x, y, angle }
    }
    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
    }

    fn forward(&mut self, speed: f32) {
        let mut t = Transform::from_xyz(self.x, self.y, 1.);
        t.rotation = Quat::from_rotation_z(self.angle);

        self.x += speed * t.up().x;
        self.y += speed * t.up().y;

        if self.y > 800. {
            self.y = 0.
        }
    }

    fn backwards(&mut self, speed: f32) {
        let mut t = Transform::from_xyz(self.x, self.y, 1.);
        t.rotation = Quat::from_rotation_z(self.angle);

        self.x += speed * t.down().x;
        self.y += speed * t.down().y;
    }

    fn rotate(&mut self, speed: f32) {
        self.angle += speed;
    }

    pub fn move_towards(&mut self, direction: &DirectionType, speed: f32, rotation_speed: f32) {
        match direction {
            DirectionType::Forward => self.forward(speed),
            DirectionType::Reverse => self.backwards(speed),
            DirectionType::Left => self.rotate(rotation_speed),
            DirectionType::Right => self.rotate(-rotation_speed),
            DirectionType::ForwardLeft => {
                self.rotate(rotation_speed);
                self.forward(speed);
            }
            DirectionType::ForwardRight => {
                self.rotate(-rotation_speed);
                self.forward(speed);
            }
            DirectionType::ReverseLeft => {
                self.rotate(-rotation_speed);
                self.backwards(speed);
            }
            DirectionType::ReverseRight => {
                self.rotate(rotation_speed);
                self.backwards(speed);
            }
            DirectionType::Stop => {
                self.x = self.x;
                self.y = self.y;
            }
        }
    }
}
