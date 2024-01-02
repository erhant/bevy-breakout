use bevy::{math::vec2, prelude::*};

use crate::{
    physics::Collider,
    wall::{BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, TOP_WALL},
};

// bricks
const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
const GAP_BETWEEN_BRICKS: f32 = 5.0;
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

#[derive(Component)]
pub struct Brick;

pub struct BricksPlugin;

impl Plugin for BricksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bricks);
    }
}

fn setup_bricks(mut commands: Commands) {
    let offset_x = LEFT_WALL + GAP_BETWEEN_BRICKS_AND_SIDES + BRICK_SIZE.x * 0.5;
    let offset_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_BRICKS + BRICK_SIZE.y * 0.5;

    let bricks_total_width = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
    let bricks_total_height =
        (TOP_WALL - BOTTOM_WALL) - GAP_BETWEEN_BRICKS_AND_CEILING - GAP_BETWEEN_PADDLE_AND_BRICKS;

    let rows = (bricks_total_height / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as i32;
    let columns = (bricks_total_width / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as i32;

    for row in 0..rows {
        for column in 0..columns {
            let brick_pos = vec2(
                offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
            );

            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: brick_pos.extend(0.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: BRICK_COLOR,
                        custom_size: Some(BRICK_SIZE),
                        ..default()
                    },
                    ..default()
                },
                Brick,
                Collider { size: BRICK_SIZE },
            ));
        }
    }
}
