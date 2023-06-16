use bevy::prelude::*;

#[derive(Component)]
pub struct Acceleration {
    value: f32,
}

impl Default for Acceleration {
    fn default() -> Self {
        Acceleration { value: 0. }
    }
}

impl Acceleration {
    pub fn set(&mut self, gravity: f32) {
        self.value = gravity;
    }

    pub fn get(&self) -> f32 {
        self.value
    }
}
