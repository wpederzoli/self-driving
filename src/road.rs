use bevy::prelude::*;

#[derive(Component)]
pub struct Road {
    width: u32,
    height: u32,
    lanes: u32,
}

impl Road {
    pub fn new(width: u32, height: u32, lanes: u32) -> Self {
        Road {
            width,
            height,
            lanes,
        }
    }

    pub fn get_lanes(&self) -> u32 {
        self.lanes
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }
}

pub fn draw_road(width: u32, height: u32) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::GRAY,
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(width as f32, height as f32, 0.),
            ..default()
        },
        ..default()
    }
}

pub fn draw_lanes(
    commands: &mut Commands,
    lanes_count: u32,
    road_width: u32,
    road_height: u32,
    line_width: f32,
    line_height: f32,
) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::ANTIQUE_WHITE,
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(line_width as f32, road_height as f32, 0.),
            translation: Vec3::new(-(road_width as f32 / 2.), 0., 1.),
            ..default()
        },
        ..default()
    });

    for lane in 1..lanes_count {
        for line in 0..20 {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::ANTIQUE_WHITE,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(line_width, line_height, 0.),
                    translation: Vec3::new(
                        -(road_width as f32 / 2.)
                            + lane as f32 * road_width as f32 / lanes_count as f32,
                        -(road_height as f32 / 2.) + line as f32 * 50.,
                        1.,
                    ),
                    ..default()
                },
                ..default()
            });
        }
    }

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::ANTIQUE_WHITE,
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(line_width as f32, road_height as f32, 0.),
            translation: Vec3::new(road_width as f32 / 2., 0., 1.),
            ..default()
        },
        ..default()
    });
}

// pub fn draw_lanes(road: Query<&Road>, mut commands: Commands) {
//     let road = road.get_single().unwrap();
//
//     commands.spawn(SpriteBundle {
//         sprite: Sprite {
//             color: Color::ANTIQUE_WHITE,
//             ..default()
//         },
//         transform: Transform {
//             scale: Vec3::new(10., road.get_height() as f32, 0.),
//             translation: Vec3::new(-(road.get_width() as f32 / 2.), 0., 0.),
//             ..default()
//         },
//         ..default()
//     });
//
//     for lane in 0..road.get_lanes() {
//         commands.spawn(SpriteBundle {
//             sprite: Sprite {
//                 color: Color::GREEN,
//                 ..default()
//             },
//             transform: Transform {
//                 scale: Vec3::new(10., road.get_height() as f32, 1.),
//                 translation: Vec3::new(10. * lane as f32, 0., 0.),
//                 ..default()
//             },
//             ..default()
//         });
//     }

//     commands.spawn(SpriteBundle {
//         sprite: Sprite {
//             color: Color::ANTIQUE_WHITE,
//             ..default()
//         },
//         transform: Transform {
//             scale: Vec3::new(10., road.get_height() as f32, 0.),
//             translation: Vec3::new(road.get_width() as f32 / 2., 0., 0.),
//             ..default()
//         },
//         ..default()
//     });
// }
