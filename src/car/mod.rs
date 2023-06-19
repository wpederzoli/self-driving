use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{
        ActiveCollisionTypes, ActiveEvents, Collider, CollisionEvent, ExternalForce, GravityScale,
        QueryFilter, RapierContext, Restitution, RigidBody, Sensor, Velocity,
    },
    rapier::prelude::{ColliderSet, RigidBodySet},
};

use self::controller::{controller_system, Controller, Direction};

pub struct CarPlugin;
mod controller;

const CAR_LAYER: f32 = 2.;

#[derive(Component)]
pub struct Car {
    pub speed: f32,
    max_speed: f32,
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
}

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_systems((move_car, controller_system));
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Car {
                speed: 0.,
                max_speed: 3.,
            },
            Controller::new(Direction::Forward),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(40., 60.)),
                    ..default()
                },
                transform: Transform::from_xyz(0., -256., CAR_LAYER),
                ..default()
            },
        ))
        .insert(Collider::cuboid(20., 30.))
        .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::all())
        .insert(ActiveEvents::COLLISION_EVENTS);
}

fn move_car(
    mut car: Query<(&mut Car, &mut Transform)>,
    mut col: EventReader<CollisionEvent>,
    rp: Res<RapierContext>,
) {
    let (mut car, mut transform) = car.single_mut();

    let ray_pos = Vec2::new(transform.translation.x, transform.translation.y + 50.);
    let ray_dir = Vec2::new(0., 5.);
    let toi = 5.;

    if let Some((entity, toi)) = rp.cast_ray(ray_pos, ray_dir, toi, true, QueryFilter::default()) {
        // The first collider hit has the entity `entity` and it hit after
        // the ray travelled a distance equal to `ray_dir * toi`.
        let hit_point = ray_pos + ray_dir * toi;
        println!("Entity {:?} hit at point {}", entity, hit_point);
    }
    for c in col.iter() {
        println!("col: {:?}", c);
    }
}
