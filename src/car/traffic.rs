use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::GameState;

use super::{
    controller::{Controller, Direction},
    spawn_car, Car, CAR_SIZE,
};

pub struct Traffic;

impl Plugin for Traffic {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(move_traffic.run_if(in_state(GameState::Play)));
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(spawn_car(1.2, Color::BLUE, Vec2::new(0., 126.)))
        .insert(Collider::cuboid(CAR_SIZE.x / 2., CAR_SIZE.y / 2.));
}

fn move_traffic(
    mut traffic: Query<(&mut Transform, &mut Car), Without<Controller>>,
    player: Query<(&Car, &Transform), With<Controller>>,
) {
    let (player, car_transform) = player.single();
    for (mut transform, mut car) in traffic.iter_mut() {
        car.accelerate();
        match player.get_direction() {
            Direction::Forward | Direction::ForwardRight | Direction::ForwardLeft => {
                transform.translation.y =
                    transform.translation.y - (car_transform.up().y * player.speed) + car.speed;
            }
            Direction::Backwards | Direction::BackwardsRight | Direction::BackwardsLeft => {
                transform.translation.y =
                    transform.translation.y + (car_transform.up().y * player.speed) + car.speed;
            }
            _ => transform.translation.y += car.speed,
        }
    }
}
