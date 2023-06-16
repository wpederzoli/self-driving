use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::{
        ActiveCollisionTypes, ActiveEvents, Collider, CollisionEvent, ExternalForce, GravityScale,
        QueryFilter, RapierContext, Restitution, RigidBody, Sensor, Velocity,
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
