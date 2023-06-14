use bevy::prelude::*;

use crate::{car::Car, direction::DirectionType, movement::Movement};

#[derive(Component)]
pub struct Controls;

impl Controls {
    fn on_keydown(&self, input: &Res<Input<KeyCode>>, car: &mut Movement) {
        match (
            input.pressed(KeyCode::Up),
            input.pressed(KeyCode::Down),
            input.pressed(KeyCode::Left),
            input.pressed(KeyCode::Right),
        ) {
            (true, false, false, false) => car.set_direction(DirectionType::Forward),
            (true, true, false, false) => car.set_direction(DirectionType::Stop),
            (true, true, true, false) => car.set_direction(DirectionType::Right),
            (true, true, true, true) => car.set_direction(DirectionType::Stop),
            (true, false, true, true) => car.set_direction(DirectionType::Forward),
            (true, false, false, true) => car.set_direction(DirectionType::ForwardRight),
            (true, false, true, false) => car.set_direction(DirectionType::ForwardLeft),
            (false, true, false, false) => car.set_direction(DirectionType::Reverse),
            (false, true, true, false) => car.set_direction(DirectionType::ReverseLeft),
            (false, true, true, true) => car.set_direction(DirectionType::Reverse),
            (false, true, false, true) => car.set_direction(DirectionType::ReverseRight),
            (false, false, true, false) => car.set_direction(DirectionType::Left),
            (false, false, true, true) => car.set_direction(DirectionType::Stop),
            (false, false, false, true) => car.set_direction(DirectionType::Right),
            (true, true, false, true) => car.set_direction(DirectionType::Right),
            (false, false, false, false) => car.set_direction(DirectionType::Stop),
        }
    }
}

pub fn controls_system(
    mut car: Query<(&Car, &mut Movement, &Controls)>,
    input: Res<Input<KeyCode>>,
) {
    let (_, mut movement, controls) = car.single_mut();
    controls.on_keydown(&input, &mut movement);
}
