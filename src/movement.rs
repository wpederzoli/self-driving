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
            DirectionType::Forward => self.y += 10.,
            _ => println!("direction"),
        }
    }
}

#[derive(Component)]
pub struct Direction(DirectionType);

impl Default for Direction {
    fn default() -> Self {
        Direction(DirectionType::Forward)
    }
}

impl Direction {
    pub fn get(&self) -> DirectionType {
        self.0
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
