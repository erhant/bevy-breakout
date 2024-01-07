use bevy::prelude::*;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct CollisionSound(Handle<AudioSource>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct LoseGameSound(Handle<AudioSource>);

pub struct SoundsPlugin;
impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_sounds);
    }
}

fn setup_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_collision_sound = asset_server.load("sounds/ball-hit.wav");
    commands.insert_resource(CollisionSound(ball_collision_sound));

    let lose_game_sound = asset_server.load("sounds/lose-game.wav");
    commands.insert_resource(LoseGameSound(lose_game_sound));
}
