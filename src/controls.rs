use bevy::prelude::*;

use crate::{car::Car, direction::DirectionType, movement::Movement};

#[derive(Component)]
pub struct Controls;

impl Controls {
    fn on_keydown(&self, input: &Res<Input<KeyCode>>, mv: &mut Movement) {
        match (
            input.pressed(KeyCode::Up),
            input.pressed(KeyCode::Down),
            input.pressed(KeyCode::Left),
            input.pressed(KeyCode::Right),
        ) {
            (true, false, false, false) => mv.set_direction(DirectionType::Forward),
            (true, true, false, false) => mv.set_direction(DirectionType::Stop),
            (true, true, true, false) => mv.set_direction(DirectionType::Right),
            (true, true, true, true) => mv.set_direction(DirectionType::Stop),
            (true, false, true, true) => mv.set_direction(DirectionType::Forward),
            (true, false, false, true) => mv.set_direction(DirectionType::ForwardRight),
            (true, false, true, false) => mv.set_direction(DirectionType::ForwardLeft),
            (false, true, false, false) => mv.set_direction(DirectionType::Reverse),
            (false, true, true, false) => mv.set_direction(DirectionType::ReverseLeft),
            (false, true, true, true) => mv.set_direction(DirectionType::Reverse),
            (false, true, false, true) => mv.set_direction(DirectionType::ReverseRight),
            (false, false, true, false) => mv.set_direction(DirectionType::Left),
            (false, false, true, true) => mv.set_direction(DirectionType::Stop),
            (false, false, false, true) => mv.set_direction(DirectionType::Right),
            (true, true, false, true) => mv.set_direction(DirectionType::Right),
            (false, false, false, false) => mv.set_direction(DirectionType::Stop),
        }
    }

    pub fn handle_input(&self, input: &Res<Input<KeyCode>>, mut mv: &mut Movement) {
        self.on_keydown(&input, &mut mv);
    }
}

pub fn controls_system(
    mut car: Query<(&Car, &mut Movement, &Controls)>,
    input: Res<Input<KeyCode>>,
) {
    let (_, mut mv, controls) = car.single_mut();
    controls.handle_input(&input, &mut mv);
}
