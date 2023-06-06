use bevy::prelude::*;

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
