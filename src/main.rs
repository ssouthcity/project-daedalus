mod components;
mod systems;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use components::{ChestBundle, KnightBundle, MinotaurBundle, WallBundle, Patrol};
use systems::{apply_velocity, item_glow, update_patrol_target, update_patrol_velocity, apply_input, camera_follow_player};

pub const PPU: f32 = 16.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .add_systems((
            apply_input,
            apply_velocity,
            item_glow, 
            update_patrol_target,
            update_patrol_velocity,
            camera_follow_player,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<ChestBundle>("Chest")
        .register_ldtk_entity::<KnightBundle>("Knight")
        .register_ldtk_entity::<MinotaurBundle>("Minotaur")
        .register_ldtk_int_cell::<WallBundle>(1)
        .register_type::<Patrol>()
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(16.0 * 16.0),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        Name::new("LDTK World"),
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("project-daedalus.ldtk"),
            ..default()
        },
    ));
}
