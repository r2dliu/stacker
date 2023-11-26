use bevy::core_pipeline::fxaa::Fxaa;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

// use crate::enemy::components::*;
use super::components::*;
use crate::gui::components::ButtonColors;
use crate::AppState;

pub fn setup_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Setting up in game menu");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                ..default()
            },
            InGameMenu,
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(300.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: button_colors.normal.into(),
                        ..default()
                    },
                    button_colors,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Test menu",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

pub fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<InGameMenu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
