use crate::{
    direction::{Direction, DirectionType},
    position::Position,
    speed::{Acceleration, Speed},
};
use bevy::prelude::*;

// Movement component that has position, direction, speed and acceleration components
#[derive(Component)]
pub struct Movement {
    position: Position,
    direction: Direction,
    speed: Speed,
    acceleration: Acceleration,
}

impl Default for Movement {
    fn default() -> Self {
        Movement {
            position: Position::default(),
            direction: Direction::default(),
            speed: Speed::default(),
            acceleration: Acceleration::default(),
        }
    }
}

impl Movement {
    pub fn locomote(&mut self) {
        self.position.move_towards(&self.direction.get());
    }

    pub fn get_x(&self) -> f32 {
        self.position.get_x()
    }

    pub fn get_y(&self) -> f32 {
        self.position.get_y()
    }

    pub fn set_direction(&mut self, direction: DirectionType) {
        self.direction.set(direction);
    }
}
