use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    x: f32,
    y: f32,
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 10., y: 10. }
    }
}

impl Position {
    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn move_towards(&mut self, direction: &DirectionType) {
        match direction {
            DirectionType::Forward => self.y += 1.,
            DirectionType::Reverse => self.y -= 1.,
            DirectionType::Left => self.x -= 1.,
            DirectionType::Right => self.x += 1.,
            DirectionType::ForwardLeft => {
                self.x -= 0.5;
                self.y += 0.5;
            }
            DirectionType::ForwardRight => {
                self.x += 0.5;
                self.y += 0.5;
            }
            DirectionType::ReverseLeft => {
                self.x -= 0.5;
                self.y -= 0.5;
            }
            DirectionType::ReverseRight => {
                self.x += 0.5;
                self.y -= 0.5;
            }
            DirectionType::Stop => {
                self.x = self.x;
                self.y = self.y;
            }
        }
    }
}

#[derive(Component)]
pub struct Direction(DirectionType);

impl Default for Direction {
    fn default() -> Self {
        Direction(DirectionType::Stop)
    }
}

impl Direction {
    pub fn get(&self) -> DirectionType {
        self.0
    }

    pub fn set(&mut self, new_direction: DirectionType) {
        self.0 = new_direction;
    }
}

#[derive(Clone, Copy)]
pub enum DirectionType {
    Right,
    Left,
    Forward,
    Reverse,
    ForwardRight,
    ForwardLeft,
    ReverseRight,
    ReverseLeft,
    Stop,
}
