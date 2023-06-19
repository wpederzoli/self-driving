use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    ActiveCollisionTypes, ActiveEvents, Collider, CollisionEvent, QueryFilter, RapierContext,
};

use crate::GameState;

use self::controller::{controller_system, Controller, Direction};

pub struct CarPlugin;
pub mod controller;
pub mod traffic;

const CAR_LAYER: f32 = 2.;
const CAR_SIZE: Vec2 = Vec2::new(40., 60.);

#[derive(Component)]
pub struct Car {
    pub speed: f32,
    max_speed: f32,
    direction: Direction,
    last_direction: Direction,
}

impl Car {
    pub fn accelerate(&mut self) {
        self.speed += 0.2;
        if self.speed > self.max_speed {
            self.speed = self.max_speed;
        }
    }

    pub fn decelerate(&mut self) {
        self.speed -= 0.2;
        if self.speed < 0. {
            self.speed = 0.;
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn get_last_direction(&self) -> Direction {
        self.last_direction
    }
}

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_systems((
            move_car,
            controller_system.run_if(in_state(GameState::Play)),
        ));
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(spawn_car(3., Color::RED, Vec2::new(0., -256.)))
        .insert(Controller)
        .insert(Collider::cuboid(20., 30.))
        .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::all())
        .insert(ActiveEvents::COLLISION_EVENTS);
}

fn move_car(
    mut car: Query<(&mut Car, &mut Transform), With<Controller>>,
    mut col: EventReader<CollisionEvent>,
    rp: Res<RapierContext>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (mut car, mut transform) = car.single_mut();

    let ray_pos = Vec2::new(transform.translation.x, transform.translation.y + 60.);
    let ray_dir = Vec2::new(0., 5.);
    let toi = 5.;

    if let Some((entity, toi)) = rp.cast_ray(ray_pos, ray_dir, toi, true, QueryFilter::default()) {
        // The first collider hit has the entity `entity` and it hit after
        // the ray travelled a distance equal to `ray_dir * toi`.
        let hit_point = ray_pos + ray_dir * toi;
        println!("Entity {:?} hit at point {}", entity, hit_point);
    }
    for _ in col.iter() {
        println!("collided");
        next_state.set(GameState::GameOver);
    }
}

pub fn spawn_car(max_speed: f32, color: Color, position: Vec2) -> (Car, SpriteBundle) {
    (
        Car {
            speed: 0.,
            max_speed,
            direction: Direction::Forward,
            last_direction: Direction::Forward,
        },
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(CAR_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, CAR_LAYER),
            ..default()
        },
    )
}
