use bevy::{math::*, prelude::*};

use crate::{
    physics::Collider,
    wall::{BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, WALL_THICKNESS},
};

const PADDLE_INITIAL_POS: Vec3 = vec3(0., BOTTOM_WALL + 60., 0.);
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PADDLE_SPEED: f32 = 500.0;

pub struct PaddlePlugin;
impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_paddle)
            .add_systems(FixedUpdate, move_paddle);
    }
}

#[derive(Component)]
struct Paddle;

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

    let new_x = paddle_transform.translation.x
        + direction
            * PADDLE_SPEED
            * time_step
                .delta_seconds()
                .min(RIGHT_WALL - (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5)
                .max(LEFT_WALL + (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);

    paddle_transform.translation.x = new_x;
}
