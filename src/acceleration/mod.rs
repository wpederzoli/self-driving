use bevy::prelude::*;

#[derive(Component)]
pub struct Gravity {
    value: f32,
}

impl Default for Gravity {
    fn default() -> Self {
        Gravity { value: 0. }
    }
}

impl Gravity {
    pub fn set(&mut self, gravity: f32) {
        self.value = gravity;
    }

    pub fn get(&self) -> f32 {
        self.value
    }
}
