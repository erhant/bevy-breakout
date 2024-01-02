use bevy::{
    math::{vec2, vec3},
    prelude::*,
};

use crate::physics::Collider;

pub const LEFT_WALL: f32 = -450.;
pub const RIGHT_WALL: f32 = 450.;
pub const BOTTOM_WALL: f32 = -300.;
pub const TOP_WALL: f32 = 300.;

pub const WALL_THICKNESS: f32 = 10.0;
const WALL_BLOCK_WIDTH: f32 = RIGHT_WALL - LEFT_WALL;
const WALL_BLOCK_HEIGHT: f32 = TOP_WALL - BOTTOM_WALL;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

pub struct WallPlugin;
impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_wall);
    }
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

fn setup_wall(mut commands: Commands) {
    let vertical_wall_size = vec2(WALL_THICKNESS, WALL_BLOCK_HEIGHT + WALL_THICKNESS);
    let horizontal_wall_size = vec2(WALL_BLOCK_WIDTH + WALL_THICKNESS, WALL_THICKNESS);

    let walls = [
        (vec3(LEFT_WALL, 0.0, 0.0), vertical_wall_size),
        (vec3(RIGHT_WALL, 0.0, 0.0), vertical_wall_size),
        (vec3(0.0, BOTTOM_WALL, 0.0), horizontal_wall_size),
        (vec3(0.0, TOP_WALL, 0.0), horizontal_wall_size),
    ];

    for (wall_pos, wall_size) in walls {
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: wall_pos,
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider { size: wall_size },
        });
    }
}
