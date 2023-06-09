use bevy::{
    prelude::{default, Color, Component, Quat, Query, Transform, Vec3, Without},
    sprite::{Sprite, SpriteBundle},
};

use crate::{direction::DirectionType, movement::Movement, road::Lane};

const CAR_LAYER: f32 = 2.;
const CAR_SIZE: Vec3 = Vec3::new(30., 50., 0.);

#[derive(Component)]
pub struct Car {
    movement: Movement,
}

impl Default for Car {
    fn default() -> Self {
        Car {
            movement: Movement::default(),
        }
    }
}

impl Car {
    pub fn locomote(&mut self) {
        self.movement.accelerate();
    }

    pub fn get_transform(&self) -> Transform {
        let mut t = Transform::from_xyz(self.movement.get_x(), self.movement.get_y(), CAR_LAYER);
        t.scale = CAR_SIZE;
        t.rotation = Quat::from_rotation_z(self.movement.get_angle());
        t
    }

    pub fn get_direction(&self) -> DirectionType {
        self.movement.get_direction()
    }

    pub fn get_last_direction(&self) -> DirectionType {
        self.movement.get_last_direction()
    }

    pub fn get_speed(&self) -> f32 {
        self.movement.get_speed()
    }

    pub fn get_angle(&self) -> f32 {
        self.movement.get_angle()
    }

    pub fn set_direction(&mut self, direction: DirectionType) {
        self.movement.set_direction(direction)
    }
}

pub fn draw_car() -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(10., 10., 2.),
            scale: CAR_SIZE,
            ..default()
        },
        ..default()
    }
}

pub fn move_car(
    mut car: Query<(&mut Car, &mut Transform)>,
    mut lanes: Query<(&mut Transform, &mut Lane), Without<Car>>,
) {
    let (mut car, mut transform) = car.single_mut();
    car.locomote();
    *transform = car.get_transform();

    for (mut t, mut lane) in lanes.iter_mut() {
        match car.get_direction() {
            DirectionType::Stop => {
                lane.move_lane(&car.get_last_direction(), car.get_speed(), car.get_angle())
            }
            _ => lane.move_lane(&car.get_direction(), car.get_speed(), car.get_angle()),
        }
        t.translation.y = lane.get_y();
    }
}
