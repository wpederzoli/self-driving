use bevy::{prelude::*, sprite::collide_aabb::collide};

#[derive(Component, Clone)]
pub struct Collider {
    transform: Transform,
    collider_type: CollisionType,
    collision: CollisionType,
}

impl Collider {
    pub fn new(transform: Transform, collider_type: CollisionType) -> Self {
        Collider {
            transform,
            collider_type,
            collision: CollisionType::None,
        }
    }

    pub fn get_collision(&self) -> CollisionType {
        self.collision
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    pub fn check_collision(&mut self, other: &Collider) {
        if collide(
            self.transform.translation,
            Vec2::new(self.transform.scale.x, self.transform.scale.y),
            other.transform.translation,
            Vec2::new(other.transform.scale.x, other.transform.scale.y),
        )
        .is_some()
        {
            self.collision = other.collider_type;
        } else {
            self.collision = CollisionType::None;
        }
    }
}

#[derive(Clone, Copy)]
pub enum CollisionType {
    None,
    Car,
    TopBound,
    BottomBound,
    LeftBorder,
    RightBorder,
}
