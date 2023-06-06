use bevy::prelude::*;

#[derive(Component)]
pub struct Speed(f32);

impl Default for Speed {
    fn default() -> Self {
        Speed(0.)
    }
}

impl Speed {
    pub fn new(speed: f32) -> Self {
        Speed(speed)
    }
    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn set(&mut self, speed: f32) {
        self.0 = speed;
    }

    pub fn add(&mut self, speed: f32) {
        self.0 += speed;
    }
}

#[derive(Component)]
pub struct Acceleration(f32);

impl Default for Acceleration {
    fn default() -> Self {
        Acceleration(0.)
    }
}

impl Acceleration {
    pub fn new(acc: f32) -> Self {
        Acceleration(acc)
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn set(&mut self, acc: f32) {
        self.0 = acc;
    }
}
