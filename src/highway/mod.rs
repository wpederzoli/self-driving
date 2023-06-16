use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody, Sensor};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

mod road;

pub struct HighwayPlugin;

impl Plugin for HighwayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::ANTIQUE_WHITE,
                custom_size: Some(Vec2::new(100., 40.)),
                ..default()
            },
            transform: Transform::from_xyz(0., 100., 0.),
            ..default()
        })
        .insert(Collider::cuboid(50., 20.))
        .insert(RigidBody::Fixed);
}
