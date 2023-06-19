use bevy::prelude::*;

use super::Car;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forward,
    ForwardRight,
    ForwardLeft,
    Backwards,
    BackwardsRight,
    BackwardsLeft,
    Left,
    Right,
    Stop,
}

#[derive(Component)]
pub struct Controller;

pub fn controller_system(
    input: Res<Input<KeyCode>>,
    mut car: Query<(&mut Transform, &mut Car, &mut Controller)>,
) {
    let (mut transform, mut car, mut controller) = car.single_mut();

    match (
        input.pressed(KeyCode::Up),
        input.pressed(KeyCode::Down),
        input.pressed(KeyCode::Left),
        input.pressed(KeyCode::Right),
    ) {
        (true, false, false, false) => car.set_direction(Direction::Forward),
        (true, false, true, false) => car.set_direction(Direction::ForwardLeft),
        (true, false, false, true) => car.set_direction(Direction::ForwardRight),
        (false, true, false, false) => car.set_direction(Direction::Backwards),
        (false, true, true, false) => car.set_direction(Direction::BackwardsLeft),
        (false, true, false, true) => car.set_direction(Direction::BackwardsRight),
        (false, false, true, false) => car.set_direction(Direction::Left),
        (false, false, false, true) => car.set_direction(Direction::Right),
        _ => {
            if car.direction != Direction::Stop {
                car.last_direction = car.direction;
                car.set_direction(Direction::Stop);
            }
        }
    }

    match car.get_direction() {
        Direction::Forward => {
            car.accelerate();
            transform.translation.x = transform.translation.x + transform.up().x * car.speed;
        }
        Direction::ForwardLeft => {
            car.accelerate();
            transform.rotate_z(0.05);
            transform.translation.x = transform.translation.x + transform.up().x * car.speed;
        }
        Direction::ForwardRight => {
            car.accelerate();
            transform.rotate_z(-0.05);
            transform.translation.x = transform.translation.x + transform.up().x * car.speed;
        }
        Direction::Backwards => {
            car.accelerate();
            transform.translation.x = transform.translation.x + transform.down().x * car.speed / 2.;
        }
        Direction::BackwardsLeft => {
            car.accelerate();
            transform.rotate_z(0.05);
            transform.translation.x = transform.translation.x + transform.down().x * car.speed / 2.;
        }
        Direction::BackwardsRight => {
            car.accelerate();
            transform.rotate_z(-0.05);
            transform.translation.x = transform.translation.x + transform.down().x * car.speed / 2.;
        }
        Direction::Left => {
            if car.speed > 0. {
                car.decelerate();
                transform.rotate_z(0.05);
            }
        }
        Direction::Right => {
            if car.speed > 0. {
                car.decelerate();
                transform.rotate_z(-0.05);
            }
        }
        Direction::Stop => {
            car.decelerate();
            match car.last_direction {
                Direction::Forward => {
                    transform.translation.x = transform.translation.x + transform.up().x * car.speed
                }
                Direction::Backwards => {
                    transform.translation.x =
                        transform.translation.x + transform.down().x * car.speed
                }
                _ => (),
            }
        }
    }
}
