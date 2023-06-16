use bevy::prelude::*;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use self::road::ROAD_LAYER;

mod road;

pub struct HighwayPlugin;

impl Plugin for HighwayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::DARK_GRAY,
            custom_size: Some(Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT)),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., ROAD_LAYER),
        ..default()
    });

    road::draw_borders(&mut commands);
    road::draw_lines(&mut commands, 3);
}
