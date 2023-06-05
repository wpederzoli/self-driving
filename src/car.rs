use bevy::{
    prelude::{default, Color, Commands, Component, Rect},
    sprite::{Sprite, SpriteBundle},
};

#[derive(Component)]
pub struct Car {
    x: f32,
    y: f32,
    width: u32,
    height: u32,
}

impl Car {
    pub fn new(x: f32, y: f32, width: u32, height: u32) -> Self {
        Car {
            x,
            y,
            width,
            height,
        }
    }

    pub fn draw(&self, mut commands: Commands) {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                rect: Some(Rect::new(
                    self.x - (self.width / 2) as f32,
                    self.y - (self.height / 2) as f32,
                    self.x + (self.width / 2) as f32,
                    self.y + (self.height / 2) as f32,
                )),
                ..default()
            },
            ..default()
        });
    }
}
