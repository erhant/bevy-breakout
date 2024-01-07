use bevy::{
    math::vec2,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{
    ball::Ball,
    game::CollisionSound,
    physics::{Collider, Velocity},
    scoreboard::Scoreboard,
    theme::MAIN_THEME,
    wall::{BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, TOP_WALL},
};

const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
const BRICK_COLOR: Color = MAIN_THEME.Primary;
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
const GAP_BETWEEN_BRICKS: f32 = 5.0;
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

#[derive(Component)]
pub struct Brick;

pub struct BricksPlugin;
impl Plugin for BricksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bricks)
            .add_systems(FixedUpdate, brick_ball_collision);
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

fn brick_ball_collision(
    mut commands: Commands,
    mut score: ResMut<Scoreboard>,
    collision_sound: Res<CollisionSound>,
    // query all balls
    mut ball_query: Query<(&mut Velocity, &Transform, &Ball)>,
    // query colliders, option brick will be true if this is some brick
    mut brick_query: Query<(Entity, &Transform, &Collider), With<Brick>>,
) {
    // iterate over the elements in a query using a for loop!
    for (mut ball_velocity, ball_transform, ball) in &mut ball_query {
        for (other_entity, transform, other) in &mut brick_query {
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

                // if brick is hit, remove it and increment score
                score.score += 1;
                commands.entity(other_entity).despawn();

                // play sound on collision
                commands.spawn(AudioBundle {
                    source: collision_sound.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
        }
    }
}
