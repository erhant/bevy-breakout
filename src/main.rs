// This lint usually gives bad advice in the context of Bevy -- hiding complex queries behind
// type aliases tends to obfuscate code while offering no improvement in code cleanliness.
#![allow(clippy::type_complexity)]

use bevy::prelude::*;

mod ball;
mod bricks;
mod game;
mod menu;
mod paddle;
mod physics;
mod scoreboard;
mod sounds;
mod theme;
mod wall;

use ball::BallPlugin;
use bricks::BricksPlugin;
use game::GamePlugin;
use menu::MenuPlugin;
use paddle::PaddlePlugin;
use physics::PhysicsPlugin;
use scoreboard::ScoreboardPlugin;
use sounds::SoundsPlugin;
use wall::WallPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(SoundsPlugin)
        .add_plugins(ScoreboardPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(WallPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(BricksPlugin)
        .run();
}
