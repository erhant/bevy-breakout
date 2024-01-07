use bevy::{
    math::{vec2, vec3},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{
    ball::Ball,
    game::GameState,
    physics::{Collider, Velocity},
    sounds::{CollisionSound, LoseGameSound},
    theme::MAIN_THEME,
};

pub const LEFT_WALL: f32 = -450.;
pub const RIGHT_WALL: f32 = 450.;
pub const BOTTOM_WALL: f32 = -300.;
pub const TOP_WALL: f32 = 300.;

pub const WALL_THICKNESS: f32 = 10.0;
const WALL_BLOCK_WIDTH: f32 = RIGHT_WALL - LEFT_WALL;
const WALL_BLOCK_HEIGHT: f32 = TOP_WALL - BOTTOM_WALL;
const WALL_COLOR: Color = MAIN_THEME.error;

pub struct WallPlugin;
impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_wall)
            .add_systems(
                FixedUpdate,
                wall_ball_collision.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(PartialEq)]
pub enum WallSide {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Component)]
struct Side {
    side: WallSide,
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    side: Side,
}

fn setup_wall(mut commands: Commands) {
    let vertical_wall_size = vec2(WALL_THICKNESS, WALL_BLOCK_HEIGHT + WALL_THICKNESS);
    let horizontal_wall_size = vec2(WALL_BLOCK_WIDTH + WALL_THICKNESS, WALL_THICKNESS);

    let walls = [
        (
            vec3(LEFT_WALL, 0.0, 0.0),
            vertical_wall_size,
            WallSide::Left,
        ),
        (
            vec3(RIGHT_WALL, 0.0, 0.0),
            vertical_wall_size,
            WallSide::Right,
        ),
        (
            vec3(0.0, BOTTOM_WALL, 0.0),
            horizontal_wall_size,
            WallSide::Bottom,
        ),
        (
            vec3(0.0, TOP_WALL, 0.0),
            horizontal_wall_size,
            WallSide::Top,
        ),
    ];

    for (wall_pos, wall_size, wall_side) in walls {
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
            side: Side { side: wall_side },
        });
    }
}

fn wall_ball_collision(
    mut commands: Commands,
    collision_sound: Res<CollisionSound>,
    lose_sound: Res<LoseGameSound>,
    mut ball_query: Query<(Entity, &mut Velocity, &Transform, &Ball)>,
    mut wall_query: Query<(&Transform, &Collider, &Side), With<Side>>,
) {
    // iterate over the elements in a query using a for loop!
    for (ball_entity, mut ball_velocity, ball_transform, ball) in &mut ball_query {
        for (transform, other, side) in &mut wall_query {
            // find collision via Bevy built-in utility
            let collision = collide(
                ball_transform.translation,
                ball.size,
                transform.translation,
                other.size,
            );

            if let Some(collision) = collision {
                // reflect ball on collision
                let mut reflect_x = false;
                let mut reflect_y = false;
                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                    Collision::Inside => { /* do nothing */ }
                }
                if reflect_x {
                    ball_velocity.x *= -1.;
                }
                if reflect_y {
                    ball_velocity.y *= -1.;
                }

                if side.side == WallSide::Bottom {
                    commands.spawn(AudioBundle {
                        source: lose_sound.clone(),
                        settings: PlaybackSettings::DESPAWN,
                    });
                    commands.entity(ball_entity).despawn();
                } else {
                    commands.spawn(AudioBundle {
                        source: collision_sound.clone(),
                        settings: PlaybackSettings::DESPAWN,
                    });
                }
            }
        }
    }
}
