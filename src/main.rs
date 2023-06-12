use bevy::{prelude::*, window::WindowResolution};
use car::{move_car, PlayerCar};
use collision::CollisionType;
use controls::controls_system;
use road::{Border, Road, ROAD_HEIGHT, ROAD_WIDTH};
use sensor::{Sensor, SensorBundle};

mod car;
mod collision;
mod controls;
mod direction;
mod movement;
mod position;
mod road;
mod sensor;
mod speed;

const SCREEN_WIDTH: f32 = 400.;
const SCREEN_HEIGHT: f32 = 800.;

#[derive(Debug, Clone, Eq, Default, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Play,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .add_startup_system(setup)
        .add_systems((controls_system.run_if(in_state(GameState::Play)), move_car))
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(PlayerCar::default())
        .with_children(|parent| {
            parent.spawn(SensorBundle::new());
        });
    commands
        .spawn((
            Road,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::Rgba {
                        red: 0.,
                        green: 0.,
                        blue: 0.,
                        alpha: 0.,
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(
                    ROAD_WIDTH,
                    ROAD_HEIGHT,
                    1.,
                )),
                ..default()
            });
            parent.spawn(Border::new(-ROAD_WIDTH / 2., 0., CollisionType::LeftBorder));
            parent.spawn(Border::new(ROAD_WIDTH / 2., 0., CollisionType::RightBorder));

            let lines_count = 10;
            let lane_count = 3;
            let line_width = 8.;
            let line_height = 35.;
            let line_space = 15.;

            for lane in 0..lane_count {
                for line in 0..(ROAD_HEIGHT / (line_height + line_space)) as u32 + 1 {
                    let pos = Vec3::new(
                        lane as f32 * (ROAD_WIDTH / lane_count as f32) - ROAD_WIDTH / 2.,
                        (line as f32 * (line_space + line_height)) - ROAD_HEIGHT / 2.,
                        1.,
                    );
                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::ANTIQUE_WHITE,
                            ..default()
                        },
                        transform: Transform::from_xyz(pos.x, pos.y, pos.z).with_scale(Vec3::new(
                            line_width,
                            line_height,
                            1.,
                        )),
                        ..default()
                    });
                }
                // for line in 0..SCREEN_HEIGHT as u32 / (10 + 12) {
                //     let pos = Vec3::new(
                //         -(ROAD_WIDTH / 2.) + lane as f32 * ROAD_WIDTH / 3.,
                //         -(ROAD_HEIGHT / 2.) + (line as f32),
                //         1.,
                //     );
                //     parent.spawn((SpriteBundle {
                //         sprite: Sprite {
                //             color: Color::ANTIQUE_WHITE,
                //             ..default()
                //         },
                //         transform: Transform {
                //             scale: Vec3::new(5., 10., 0.),
                //             translation: pos,
                //             ..default()
                //         },
                //         ..default()
                //     },));
                // }
            }
        });
    // commands.spawn(RoadBundle::new()).with_children(|parent| {
    //     parent.spawn(Border::new(-1., 0., CollisionType::LeftBorder));
    // });
    // commands.spawn(car::init_car()).with_children(|parent| {
    //     // parent.spawn(SensorBundle::new(0., 0.));
    //     parent.spawn(Sensor);
    // });
    // commands.spawn(road::draw_road());
    // road::draw_lanes(&mut commands, 3, 10., 30., 50.);
}
