use bevy::prelude::*;

use crate::{game::GameState, physics::Velocity, theme::MAIN_THEME};

const BALL_COLOR: Color = MAIN_THEME.secondary;
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const BALL_SPEED: f32 = 700.0;
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0., -1.);

#[derive(Component)]
pub struct Ball {
    pub size: Vec2,
}

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_ball)
            .add_systems(OnExit(GameState::Playing), cleanup_balls);
    }
}

fn setup_ball(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_texture = asset_server.load("textures/circle.png");
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: BALL_STARTING_POSITION,
                ..Default::default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                custom_size: Some(BALL_SIZE),
                ..Default::default()
            },
            texture: ball_texture.clone(),
            ..Default::default()
        },
        Ball { size: BALL_SIZE },
        Velocity(BALL_SPEED * BALL_INITIAL_DIRECTION),
    ));
}

// there may be multiple balls at some point!
fn cleanup_balls(mut commands: Commands, balls: Query<Entity, With<Ball>>) {
    for entity in balls.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
