use bevy::input::keyboard;
use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::text::Justify::Center;

#[derive(Component)]
pub struct IntroText;

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_intro_text);
        app.add_systems(Update, despawn_text_on_movement);
    }
}

#[derive(Bundle)]
struct IntroTextBundle {
    text: Text,
    font: TextFont,
    layout: TextLayout,
    node: Node,
    marker: IntroText,
}

fn spawn_intro_text(mut commands: Commands) {
    commands.spawn( IntroTextBundle {
        text: Text::new("Procedual Planets"),
        font: TextFont {
            // The size of the text will be 20% of the viewport height.
            font_size: FontSize::Vh(20.0),
            ..default()
        },
        layout: TextLayout::justify(Center),
        node: Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(15.),
            top: Val::Percent(20.),
            width: Val::Percent(70.),
                        
            // Das hier fehlte: Wir machen den Haupttext zum Flex-Container für sein Kind!
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        marker: IntroText
    })
    .with_child((
        Text::new("A simulation made in Bevy.\n Use WASD to move."),
        TextFont {
            // The size of the text will be 20% of the viewport height.
            font_size: FontSize::Vh(5.0),
            ..default()
        },
        TextLayout::justify(Center),
        Node {
            position_type: PositionType::Relative,
            margin: UiRect::top(Val::Vh(50.0)),
            ..default()
        },
    ));
}

fn despawn_text_on_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query: Query<Entity, With<IntroText>>,
) {
    if let Ok(text_entity) = query.single() {
        if keyboard_input.just_pressed(KeyCode::KeyW)
            || keyboard_input.just_pressed(KeyCode::KeyA)
            || keyboard_input.just_pressed(KeyCode::KeyS)
            || keyboard_input.just_pressed(KeyCode::KeyD)
        {
            commands.entity(text_entity).despawn();
        }
    }
}