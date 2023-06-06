use bevy::prelude::*;

use crate::{
    car::Car,
    movement::{Direction, DirectionType},
};

#[derive(Component)]
pub struct Controls;

impl Controls {
    fn on_keydown(&self, input: &Res<Input<KeyCode>>, direction: &mut Direction) {
        match (
            input.pressed(KeyCode::Up),
            input.pressed(KeyCode::Down),
            input.pressed(KeyCode::Left),
            input.pressed(KeyCode::Right),
        ) {
            (true, false, false, false) => direction.set(DirectionType::Forward),
            (true, true, false, false) => direction.set(DirectionType::Stop),
            (true, true, true, false) => direction.set(DirectionType::Right),
            (true, true, true, true) => direction.set(DirectionType::Stop),
            (true, false, true, true) => direction.set(DirectionType::Forward),
            (true, false, false, true) => direction.set(DirectionType::ForwardRight),
            (true, false, true, false) => direction.set(DirectionType::ForwardLeft),
            (false, true, false, false) => direction.set(DirectionType::Reverse),
            (false, true, true, false) => direction.set(DirectionType::ReverseLeft),
            (false, true, true, true) => direction.set(DirectionType::Reverse),
            (false, true, false, true) => direction.set(DirectionType::ReverseRight),
            (false, false, true, false) => direction.set(DirectionType::Left),
            (false, false, true, true) => direction.set(DirectionType::Stop),
            (false, false, false, true) => direction.set(DirectionType::Right),
            (true, true, false, true) => direction.set(DirectionType::Right),
            (false, false, false, false) => direction.set(DirectionType::Stop),
        }
    }

    pub fn handle_input(&self, input: &Res<Input<KeyCode>>, mut direction: &mut Direction) {
        self.on_keydown(&input, &mut direction);
    }
}

pub fn controls_system(
    mut car: Query<(&Car, &mut Direction, &Controls)>,
    input: Res<Input<KeyCode>>,
) {
    let (_, mut dir, controls) = car.single_mut();
    controls.handle_input(&input, &mut dir);
}

