use bevy::core_pipeline::fxaa::Fxaa;
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_panorbit_camera::PanOrbitCamera;
use rand::prelude::*;

// use crate::enemy::components::*;
use super::components::*;
use crate::gui::components::ButtonColors;
use crate::AppState;

/// Marker to find the text entity so we can update it
#[derive(Component)]
pub struct DebugText;

pub fn setup_debug_overlay(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>, // mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
) {
    info!("Setting up debug overlay");

    commands
        .spawn((
            DebugText,
            NodeBundle {
                style: Style {
                    width: Val::Percent(40.0),
                    height: Val::Percent(40.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn((
                DebugText,
                TextBundle::from_section(
                    "Camera target: N/A".to_string(),
                    TextStyle {
                        font_size: 16.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        // if you want to use your game's font asset,
                        // uncomment this and provide the handle:
                        // font: my_font_handle
                        ..default()
                    },
                ),
            ));

            children.spawn((
                DebugText,
                TextBundle::from_section(
                    "Camera position: N/A".to_string(),
                    TextStyle {
                        font_size: 16.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        // if you want to use your game's font asset,
                        // uncomment this and provide the handle:
                        // font: my_font_handle
                        ..default()
                    },
                ),
            ));
        });
}

pub fn update_debug_overlay(
    mut text_query: Query<&mut Text, With<DebugText>>,
    mut camera_query: Query<(&mut PanOrbitCamera, &mut Transform), Without<DebugText>>,
) {
    if let Ok((mut camera, mut transform)) = camera_query.get_single_mut() {
        for (index, mut text) in &mut text_query.iter_mut().enumerate() {
            if index == 0 {
                text.sections[0].value = format!("Camera target: {}", camera.focus.to_string());
            }
            if index == 1 {
                text.sections[0].value =
                    format!("Camera position: {}", transform.translation.to_string());
            }
        }
    }

    // Ok(mut c, mut t) => {
    //     println!("hello {}", camera.focus);

    //     children.spawn(TextBundle::from_section(
    //         format!("Camera: {0} ", camera.focus),
    //         TextStyle {
    //             font_size: 40.0,
    //             color: Color::rgb(0.9, 0.9, 0.9),
    //             ..default()
    //         },
    //     ));
    // }
    // Err(QuerySingleError::NoEntities(_)) => {
    //     println!("no camera found");
    // }
    // Err(QuerySingleError::MultipleEntities(_)) => {
    //     println!("Error: There is more than one camera!");
    // }
}

pub fn cleanup_debug_overlay(mut commands: Commands, menu: Query<Entity, With<DebugText>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
