use bevy::prelude::*;

#[derive(Component)]
pub struct Speed {
    current: f32,
    max: f32,
    friction: f32,
}

impl Default for Speed {
    fn default() -> Self {
        Speed {
            current: 0.,
            max: 1.,
            friction: 0.,
        }
    }
}

impl Speed {
    pub fn new(speed: f32, max: f32, friction: f32) -> Self {
        Speed {
            current: speed,
            max,
            friction,
        }
    }

    pub fn get(&self) -> f32 {
        self.current
    }

    pub fn add(&mut self, speed: f32) {
        self.current += speed;
        if self.current > self.max {
            self.current = self.max;
        }

        if self.current < -self.max / 2. {
            self.current = -self.max / 2.;
        }

        self.add_friction();
    }

    fn add_friction(&mut self) {
        if self.current > 0. {
            self.current -= self.friction;
        }

        if self.current < 0. {
            self.current += self.friction;
        }

        if self.current.abs() < self.friction {
            self.current = 0.
        }
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
}
