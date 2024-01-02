use bevy::{prelude::*, sprite::collide_aabb::*};

use crate::{ball::Ball, bricks::Brick, CollisionSound, Scoreboard};

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (apply_velocity, check_ball_collisions.after(apply_velocity)),
        );
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<Time<Fixed>>) {
    let dt = time_step.delta_seconds();
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }
}

fn check_ball_collisions(
    mut commands: Commands,
    mut score: ResMut<Scoreboard>,
    collision_sound: Res<CollisionSound>,
    mut ball_query: Query<(&mut Velocity, &Transform, &Ball)>,
    // optional Brick means that we "may" have a brick here
    mut collider_query: Query<(Entity, &Transform, &Collider, Option<&Brick>)>,
) {
    for (mut ball_velocity, ball_transform, ball) in &mut ball_query {
        for (other_entity, transform, other, opt_brick) in &mut collider_query {
            let collision = collide(
                ball_transform.translation,
                ball.size,
                transform.translation,
                other.size,
            );

            let mut reflect_x = false;
            let mut reflect_y = false;
            if let Some(collision) = collision {
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

                if opt_brick.is_some() {
                    score.score += 1;

                    // if brick is hit, remove it
                    commands.entity(other_entity).despawn();
                }

                // play sound
                commands.spawn(AudioBundle {
                    source: collision_sound.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
        }
    }
}