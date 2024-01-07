use bevy::prelude::*;

use crate::theme::MAIN_THEME;

const BACKGROUND_COLOR: Color = MAIN_THEME.Neutral;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct CollisionSound(Handle<AudioSource>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct LoseGameSound(Handle<AudioSource>);

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_systems(Update, bevy::window::close_on_esc)
            .add_systems(Startup, setup_game);
    }
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn the camera
    commands.spawn(Camera2dBundle::default());

    // add collision sound as a resource
    let ball_collision_sound = asset_server.load("sounds/ball-hit.wav");
    commands.insert_resource(CollisionSound(ball_collision_sound));

    // add lose-game as a resource
    let lose_game_sound = asset_server.load("sounds/lose-game.wav");
    commands.insert_resource(LoseGameSound(lose_game_sound));
}
