use bevy::prelude::*;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct CollisionSound(Handle<AudioSource>);

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
            .add_systems(Update, bevy::window::close_on_esc)
            .add_systems(Startup, setup_game);
    }
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn the camera
    commands.spawn(Camera2dBundle::default());

    // add collision sound as a resource
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));
}
