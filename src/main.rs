mod components;
mod menu;
mod net;
mod systems;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_ggrs::{GGRSPlugin, GGRSSchedule};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use components::{ChestBundle, IcarusBundle, KnightBundle, MinotaurBundle, Patrol, WallBundle};
use systems::{
    apply_input, apply_velocity, item_glow, undo_collisions, update_patrol_target,
    update_patrol_velocity,
};

pub const PPU: f32 = 16.0;

#[derive(States, Default, Hash, Debug, Eq, PartialEq, Clone, Copy)]
pub enum GameStates {
    #[default]
    Matchmaking,
    Lobby,
}

fn main() {
    let mut app = App::new();

    GGRSPlugin::<net::GGRSConfig>::new()
        .with_input_system(net::collect_input)
        .build(&mut app);

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(LdtkPlugin)
        .add_state::<GameStates>()
        .insert_resource(net::PlayerCount(0))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(LevelSelection::Index(0));

    app.add_systems(
        (net::setup_matchbox_socket, menu::setup_menu)
            .in_schedule(OnEnter(GameStates::Matchmaking)),
    )
    .add_systems(
        (net::update_matchmaking, menu::update_player_count_display)
            .in_set(OnUpdate(GameStates::Matchmaking)),
    )
    .add_systems((menu::teardown_menu,).in_schedule(OnExit(GameStates::Matchmaking)));

    app.add_system(setup_lobby.in_schedule(OnEnter(GameStates::Lobby)))
        .add_system(apply_input.in_schedule(GGRSSchedule))
        .add_systems(
            (
                apply_velocity,
                undo_collisions.after(apply_velocity),
                //camera_follow_player,
                item_glow,
                update_patrol_target,
                update_patrol_velocity,
            )
                .chain()
                .in_set(OnUpdate(GameStates::Lobby)),
        );

    app.register_ldtk_entity::<ChestBundle>("Chest")
        .register_ldtk_entity::<KnightBundle>("Knight")
        .register_ldtk_entity::<MinotaurBundle>("Minotaur")
        .register_ldtk_entity::<IcarusBundle>("Icarus")
        .register_ldtk_int_cell::<WallBundle>(1);

    app.register_type::<Patrol>()
        .register_type::<net::PlayerCount>();

    app.run();
}

fn setup_lobby(mut commands: Commands, asset_server: Res<AssetServer>) {
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
