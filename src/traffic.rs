use bevy::prelude::*;

use crate::{
    car::{Car, CAR_LAYER, CAR_SIZE},
    collision::{Collider, CollisionType},
    controls::Controls,
    direction::DirectionType,
    movement::Movement,
    GameState,
};

pub struct TrafficPlugin;

impl Plugin for TrafficPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(move_car.run_if(in_state(GameState::Play)));
    }
}

fn setup(mut commands: Commands) {
    let transform = Transform::from_xyz(0., 0., CAR_LAYER).with_scale(CAR_SIZE);

    commands.spawn(TrafficCar {
        car: Car,
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                ..default()
            },
            transform,
            ..default()
        },
        collider: Collider::new(transform, CollisionType::Car),
        movement: Movement::default(),
    });
}

fn move_car(
    mut car: Query<(&Car, &mut Movement, &mut Transform, &mut Collider), Without<Controls>>,
    player: Query<&Movement, With<Controls>>,
) {
    let player = player.single();
    for (_, mut movement, mut transform, mut collider) in car.iter_mut() {
        movement.set_speed(2.5, 3.0);
        movement.set_direction(DirectionType::Forward);
        movement.accelerate();
        transform.translation = Vec3::new(
            movement.get_x(),
            movement.get_y() - player.get_y(),
            CAR_LAYER,
        );
        transform.rotation = Quat::from_rotation_z(movement.get_angle());
        collider.set_transform(*transform);
    }
}

#[derive(Bundle)]
pub struct TrafficCar {
    car: Car,
    #[bundle]
    sprite: SpriteBundle,
    collider: Collider,
    movement: Movement,
}
