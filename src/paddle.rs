use bevy::{
    math::*,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{
    ball::Ball,
    game::GameState,
    physics::{Collider, Velocity},
    sounds::CollisionSound,
    theme::MAIN_THEME,
    wall::{BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, WALL_THICKNESS},
};

const PADDLE_INITIAL_POS: Vec3 = vec3(0., BOTTOM_WALL + 60., 0.);
const PADDLE_SIZE: Vec2 = Vec2::new(140.0, 20.0);
const PADDLE_COLOR: Color = MAIN_THEME.accent;
const PADDLE_SPEED: f32 = 600.0;
// const PADDLE_HIT_BALL_SPEEDUP: f32 = 1.005;

pub struct PaddlePlugin;
impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_paddle)
            .add_systems(
                FixedUpdate,
                (move_paddle, ball_paddle_collision).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_paddle);
    }
}

#[derive(Component)]
pub struct Paddle;

fn setup_paddle(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: PADDLE_INITIAL_POS,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                custom_size: Some(PADDLE_SIZE),
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider { size: PADDLE_SIZE },
    ));
}

fn cleanup_paddle(mut commands: Commands, paddle: Query<Entity, With<Paddle>>) {
    let entity = paddle.single();
    commands.entity(entity).despawn_recursive();
}

fn move_paddle(
    input: Res<Input<KeyCode>>,
    time_step: Res<Time<Fixed>>,
    // query Paddle components with their mutable Transform components
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transform = query.single_mut();

    let mut direction = 0.0;
    if input.pressed(KeyCode::A) {
        direction -= 1.0;
    }
    if input.pressed(KeyCode::D) {
        direction += 1.0;
    }

    // translate paddle w.r.t direction & speed
    let new_x =
        paddle_transform.translation.x + direction * PADDLE_SPEED * time_step.delta_seconds();

    // clamp it so that it does not go out of bounds
    let new_x_clamped = new_x
        .min(RIGHT_WALL - (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5)
        .max(LEFT_WALL + (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);

    paddle_transform.translation.x = new_x_clamped;
}

fn ball_paddle_collision(
    mut commands: Commands,
    collision_sound: Res<CollisionSound>,
    mut ball_query: Query<(&mut Velocity, &Transform, &Ball)>,
    mut paddle_query: Query<(&Transform, &Collider), With<Paddle>>,
) {
    // iterate over the elements in a query using a for loop!
    for (mut ball_velocity, ball_transform, ball_collider) in &mut ball_query {
        for (paddle_transform, paddle_collider) in &mut paddle_query {
            // find collision via Bevy built-in utility
            let collision = collide(
                ball_transform.translation,
                ball_collider.size,
                paddle_transform.translation,
                paddle_collider.size,
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

                let ball_x = ball_transform.translation.x;
                // let ball_y = ball_transform.translation.y;
                // info!(ball_x, ball_y);

                let paddle_x = paddle_transform.translation.x;
                // let paddle_y = paddle_transform.translation.y;
                // info!(paddle_x, paddle_y);

                // add x speed w.r.t hit position
                let diff_x = ball_x - paddle_x;
                ball_velocity.x += diff_x * 2.;

                // if paddle is hit, increase speed
                // ball_velocity.x *= PADDLE_HIT_BALL_SPEEDUP;
                // ball_velocity.y *= PADDLE_HIT_BALL_SPEEDUP;

                // play sound on collision
                commands.spawn(AudioBundle {
                    source: collision_sound.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
        }
    }
}
