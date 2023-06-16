use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{
        ActiveCollisionTypes, ActiveEvents, Collider, CollisionEvent, ExternalForce, GravityScale,
        RapierContext, Restitution, RigidBody, Sensor, Velocity,
    },
    rapier::prelude::{ColliderSet, RigidBodySet},
};

pub struct CarPlugin;

#[derive(Component)]
struct Car {
    pub speed: f32,
    max_speed: f32,
}

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(move_car);
    }
}

fn setup(mut commands: Commands) {
    // commands
    //     .spawn((
    //         SpriteBundle {
    //             sprite: Sprite {
    //                 color: Color::RED,
    //                 custom_size: Some(Vec2::new(50., 50.)),
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         Car {
    //             speed: 0.,
    //             max_speed: 3.,
    //         },
    //     ))
    //     .insert(TransformBundle::from_transform(Transform::from_xyz(
    //         0., -200., 3.,
    //     )))
    //     .insert(Collider::cuboid(25., 25.))
    //     .insert(ActiveEvents::COLLISION_EVENTS);
    commands
        .spawn((
            Car {
                speed: 0.,
                max_speed: 3.,
            },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(50., 50.)),
                    ..default()
                },
                transform: Transform::from_xyz(0., -256., 0.),
                ..default()
            },
        ))
        .insert(Collider::cuboid(25., 25.))
        .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::all())
        .insert(ActiveEvents::COLLISION_EVENTS);
}

fn move_car(
    mut car: Query<(&mut Car, &mut Transform)>,
    input: Res<Input<KeyCode>>,
    mut col: EventReader<CollisionEvent>,
) {
    let (mut car, mut transform) = car.single_mut();

    for c in col.iter() {
        println!("col: {:?}", c);
    }

    if input.pressed(KeyCode::Up) {
        car.speed += 0.2;
        if car.speed <= -car.max_speed {
            car.speed = -car.max_speed;
        }
    } else {
        car.speed -= 0.2;
        if car.speed < 0. {
            car.speed = 0.
        }
    }

    transform.translation.y += car.speed;
}
