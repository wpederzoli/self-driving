use crate::{
    direction::{Direction, DirectionType},
    position::Position,
    speed::{Acceleration, Speed},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Movement {
    position: Position,
    direction: Direction,
    last_direction: DirectionType,
    speed: Speed,
    acceleration: Acceleration,
}

impl Default for Movement {
    fn default() -> Self {
        Movement {
            position: Position::default(),
            direction: Direction::default(),
            last_direction: DirectionType::Stop,
            speed: Speed::new(0., 4., 0.05, 0.05),
            acceleration: Acceleration::new(0.2),
        }
    }
}

impl Movement {
    pub fn accelerate(&mut self) {
        self.speed.add(self.acceleration.get());

        match (self.direction.get(), self.last_direction) {
            (DirectionType::Right | DirectionType::Left, _) => self.decelerate(),
            (
                DirectionType::Stop,
                DirectionType::Forward
                | DirectionType::ForwardLeft
                | DirectionType::ForwardRight
                | DirectionType::Reverse
                | DirectionType::ReverseLeft
                | DirectionType::ReverseRight,
            ) => self.decelerate(),
            _ => {
                self.position.move_towards(
                    &self.direction.get(),
                    self.speed.get(),
                    self.speed.get_rotation(),
                );
            }
        }
    }

    pub fn get_x(&self) -> f32 {
        self.position.get_x()
    }

    pub fn get_y(&self) -> f32 {
        self.position.get_y()
    }

    pub fn set_direction(&mut self, direction: DirectionType) {
        if direction != self.direction.get() {
            self.last_direction = self.direction.get();
            self.direction.set(direction);
        }
    }

    pub fn get_angle(&self) -> f32 {
        self.position.get_angle()
    }

    pub fn get_speed(&self) -> f32 {
        self.speed.get()
    }

    pub fn get_direction(&self) -> DirectionType {
        self.direction.get()
    }

    pub fn get_last_direction(&self) -> DirectionType {
        self.last_direction
    }

    fn decelerate(&mut self) {
        if self.speed.get() > 0. {
            self.speed.add(-self.acceleration.get());
        }

        if self.speed.get() < 0. {
            self.speed.add(self.acceleration.get());
        }

        if self.speed.get().abs() > 0. {
            self.position.move_towards(
                &self.last_direction,
                self.speed.get(),
                self.speed.get_rotation(),
            );
        }
    }
}
