use bevy::prelude::*;

use crate::net;

#[derive(Component)]
pub struct MenuEntity;

#[derive(Component)]
pub struct PlayerCountDisplay;

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default()).insert(MenuEntity);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::all(Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .insert(MenuEntity)
        .with_children(|container| {
            container
                .spawn(TextBundle::from_section(
                    "",
                    TextStyle {
                        color: Color::WHITE,
                        font_size: 64.0,
                        font: asset_server.load("ka1.ttf"),
                    },
                ))
                .insert(PlayerCountDisplay);
        });
}

pub fn update_player_count_display(
    mut query: Query<&mut Text, With<PlayerCountDisplay>>,
    player_count: Res<net::PlayerCount>,
) {
    let mut display_text = query.single_mut();
    display_text.sections[0].value = player_count.to_string();
}

pub fn teardown_menu(query: Query<Entity, With<MenuEntity>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
