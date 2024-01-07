use bevy::{math::*, prelude::*};

use crate::{game::GameState, theme::MAIN_THEME};

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = MAIN_THEME.info;
const SCORE_COLOR: Color = MAIN_THEME.success;

pub struct ScoreboardPlugin;
impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scoreboard { score: 0 })
            .add_systems(OnEnter(GameState::Playing), setup_scoreboard)
            .add_systems(OnExit(GameState::Playing), cleanup_scoreboard)
            .add_systems(
                Update,
                update_scoreboard.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Resource, Clone, Copy)]
pub struct Scoreboard {
    pub score: usize,
}

#[derive(Component)]
struct ScoreboardText;

fn setup_scoreboard(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
        ScoreboardText,
    ));
}

fn cleanup_scoreboard(mut commands: Commands, scoreboard: Query<Entity, With<ScoreboardText>>) {
    let entity = scoreboard.single();
    commands.entity(entity).despawn_recursive();
}

fn update_scoreboard(score: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.score.to_string();
}
