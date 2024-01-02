use bevy::prelude::*;

mod ball;
mod bricks;
mod game;
mod paddle;
mod physics;
mod scoreboard;
mod wall;

use ball::BallPlugin;
use bricks::BricksPlugin;
use game::GamePlugin;
use paddle::PaddlePlugin;
use physics::PhysicsPlugin;
use scoreboard::ScoreboardPlugin;
use wall::WallPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .add_plugins(ScoreboardPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(WallPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(BricksPlugin)
        .run();
}
