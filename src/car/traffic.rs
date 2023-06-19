use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use super::{
    controller::{Controller, Direction},
    spawn_car, Car, CAR_SIZE,
};

pub struct Traffic;

impl Plugin for Traffic {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(move_traffic);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(spawn_car(1., Color::BLUE, Vec2::new(0., 126.)))
        .insert(Collider::cuboid(CAR_SIZE.x / 2., CAR_SIZE.y / 2.));
}

fn move_traffic(
    mut traffic: Query<(&mut Transform, &mut Car), Without<Controller>>,
    player: Query<&Car, With<Controller>>,
) {
    let player = player.single();
    for (mut transform, mut car) in traffic.iter_mut() {
        car.accelerate();
        match player.get_direction() {
            Direction::Forward | Direction::ForwardRight | Direction::ForwardLeft => {
                transform.translation.y += car.speed - player.speed / 2.;
            }
            Direction::Backwards | Direction::BackwardsRight | Direction::BackwardsLeft => {
                transform.translation.y -= car.speed - player.speed;
            }
            _ => transform.translation.y += car.speed,
        }
    }
}
