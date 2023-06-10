use bevy::{
    prelude::{
        default, Color, Component, NextState, Quat, Query, ResMut, Transform, Vec3, Without,
    },
    sprite::{Sprite, SpriteBundle},
};

use crate::{
    collision::{Collider, CollisionType},
    controls::Controls,
    direction::DirectionType,
    movement::Movement,
    road::Lane,
    GameState,
};

const CAR_LAYER: f32 = 2.;
const CAR_SIZE: Vec3 = Vec3::new(30., 50., 0.);
const CAR_Y: f32 = -150.;

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
        self.movement
            .set_position(self.movement.get_x(), CAR_Y, self.movement.get_angle());
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

pub fn init_car() -> (Car, Controls, Collider, SpriteBundle) {
    (
        Car::default(),
        Controls,
        Collider::new(
            Transform::from_xyz(0., 0., 2.).with_scale(CAR_SIZE),
            CollisionType::Car,
        ),
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., CAR_Y, 2.),
                scale: CAR_SIZE,
                ..default()
            },
            ..default()
        },
    )
}

pub fn move_car(
    mut car: Query<(&mut Car, &mut Transform, &mut Collider)>,
    mut lanes: Query<(&mut Transform, &mut Lane), Without<Car>>,
    colliders: Query<&Collider, Without<Car>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (mut car, mut transform, mut collider) = car.single_mut();
    car.locomote();
    *transform = car.get_transform();
    collider.set_transform(car.get_transform());
    for other_colliders in colliders.iter() {
        collider.check_collision(other_colliders);
        match collider.get_collision() {
            CollisionType::LeftBorder => next_state.set(GameState::GameOver),
            CollisionType::RightBorder => next_state.set(GameState::GameOver),
            _ => (),
        }
    }

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
