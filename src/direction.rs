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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

impl DirectionType {
    pub fn inverse(&self) -> DirectionType {
        match self {
            DirectionType::Forward => DirectionType::Reverse,
            DirectionType::Reverse => DirectionType::Forward,
            DirectionType::Left => DirectionType::Right,
            DirectionType::Right => DirectionType::Left,
            DirectionType::ForwardLeft => DirectionType::ReverseRight,
            DirectionType::ForwardRight => DirectionType::ReverseLeft,
            DirectionType::ReverseLeft => DirectionType::ForwardRight,
            DirectionType::ReverseRight => DirectionType::ForwardLeft,
            _ => *self,
        }
    }
}
