use bevy::prelude::*;

use crate::theme::MAIN_THEME;

const BACKGROUND_COLOR: Color = MAIN_THEME.neutral;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // player is in the main-menu
    #[default]
    Menu,
    // player is playing the game
    Playing,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_state::<GameState>()
            .add_systems(Update, (back_to_menu).run_if(in_state(GameState::Playing)))
            .add_systems(Startup, setup_game);
    }
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// why does this not work?
fn back_to_menu(
    windows: Query<(Entity, &Window)>,
    input: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
) {
    for (entity, window) in windows.iter() {
        if !window.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            state.set(GameState::Menu);
        }
    }
}
