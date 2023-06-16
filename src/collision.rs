use bevy::{prelude::*, sprite::collide_aabb::collide};

#[derive(Component, Debug, Clone)]
pub struct Collider {
    pub collider_type: CollisionType,
    collision: CollisionType,
}

impl Collider {
    pub fn new(collider_type: CollisionType) -> Self {
        Collider {
            collider_type,
            collision: CollisionType::None,
        }
    }

    pub fn get_collision(&self) -> CollisionType {
        self.collision
    }

    // pub fn set_transform(&mut self, transform: Transform) {
    //     self.transform = transform;
    // }

    // pub fn check_collision(&mut self, other: &Collider) {
    //     if other.collider_type != self.collider_type {
    //         if let Some(col) = collide(
    //             self.transform.translation,
    //             Vec2::new(self.transform.scale.x, self.transform.scale.y),
    //             other.transform.translation,
    //             Vec2::new(other.transform.scale.x, other.transform.scale.y),
    //         ) {
    //             println!("COL: {:?}", col);
    //             if other.collider_type != CollisionType::None {
    //                 self.collision = other.collider_type;
    //                 println!("Changing from NONE to {:?}", self.collision);
    //             }
    //         } else {
    //             println!("Changing from {:?} to None", self.collision);
    //             self.collision = CollisionType::None;
    //         }
    //     }
    // }
}

pub fn draw_colliders(mut col: Query<(&Collider, &mut Sprite)>) {
    for (_, mut sprite) in col.iter_mut() {
        sprite.color = Color::GREEN;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionType {
    None,
    Car,
    LeftBorder,
    RightBorder,
}
