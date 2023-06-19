use bevy::prelude::*;

use crate::{
    car::controller::Controller,
    car::{controller::Direction, Car},
    GameState, SCREEN_HEIGHT, SCREEN_WIDTH,
};

use self::road::{Lane, ROAD_LAYER};

pub mod road;

pub struct HighwayPlugin;

impl Plugin for HighwayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(move_road.run_if(in_state(GameState::Play)));
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

fn move_road(
    mut road: Query<&mut Transform, With<Lane>>,
    car: Query<(&Car, &Transform, &Controller), Without<Lane>>,
) {
    let (car, car_transform, controller) = car.single();

    for mut transform in road.iter_mut() {
        match car.get_direction() {
            Direction::Forward | Direction::ForwardRight | Direction::ForwardLeft => {
                transform.translation.y =
                    transform.translation.y - car_transform.up().y * car.speed;
            }
            Direction::Backwards | Direction::BackwardsLeft | Direction::BackwardsRight => {
                transform.translation.y =
                    transform.translation.y + car_transform.up().y * car.speed;
            }
            Direction::Stop => match car.get_last_direction() {
                Direction::Forward | Direction::ForwardLeft | Direction::ForwardRight => {
                    transform.translation.y =
                        transform.translation.y - car_transform.up().y * car.speed;
                }
                Direction::Backwards | Direction::BackwardsLeft | Direction::BackwardsRight => {
                    transform.translation.y =
                        transform.translation.y + car_transform.up().y * car.speed;
                }
                _ => (),
            },
            _ => (),
        }

        if transform.translation.y >= SCREEN_HEIGHT / 2. {
            transform.translation.y = -SCREEN_HEIGHT;
        }

        if transform.translation.y <= -SCREEN_HEIGHT / 2. {
            transform.translation.y = 0.;
        }
    }
}
