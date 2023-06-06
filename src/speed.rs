use bevy::prelude::*;

#[derive(Component)]
pub struct Speed(f32);

impl Default for Speed {
    fn default() -> Self {
        Speed(0.)
    }
}

#[derive(Component)]
pub struct Acceleration(f32);

impl Default for Acceleration {
    fn default() -> Self {
        Acceleration(0.)
    }
}
