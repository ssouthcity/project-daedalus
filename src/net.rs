use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;
use bevy_ggrs::{RollbackIdProvider, Session};
use bevy_matchbox::prelude::*;
use ggrs::{PlayerHandle, SessionBuilder};

use crate::GameStates;

pub const MATCHBOX_SERVER_URL: &'static str = "ws://127.0.0.1:3536";

pub const LOBBY_PLAYER_COUNT: usize = 2;
pub const LOBBY_FPS: usize = 60;

pub const INPUT_UP: u8 = 1 << 0;
pub const INPUT_DOWN: u8 = 1 << 1;
pub const INPUT_LEFT: u8 = 1 << 2;
pub const INPUT_RIGHT: u8 = 1 << 3;

pub struct GGRSConfig;

impl ggrs::Config for GGRSConfig {
    type Input = u8;
    type State = u8;
    type Address = PeerId;
}

#[derive(Component, Default)]
pub struct Player {
    pub handle: usize,
}

impl Player {
    pub fn from_entity(entity_instance: &EntityInstance) -> Self {
        let entity_name = entity_instance.identifier.as_str();

        match entity_name {
            "Icarus" => Self { handle: 0 },
            "Minotaur" => Self { handle: 1 },
            _ => panic!("invalid entity identifier {entity_name}"),
        }
    }
}

#[derive(Resource, DerefMut, Deref, Reflect, Default)]
#[reflect(Resource)]
pub struct PlayerCount(pub u8);

pub fn collect_input(_handle: In<PlayerHandle>, keyboard_input: Res<Input<KeyCode>>) -> u8 {
    let mut input: u8 = 0;

    if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        input |= INPUT_UP;
    }

    if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        input |= INPUT_DOWN;
    }

    if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        input |= INPUT_LEFT;
    }

    if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        input |= INPUT_RIGHT;
    }

    input
}

pub fn setup_matchbox_socket(mut commands: Commands) {
    let lobby_code = "code";

    let room_url = format!(
        "{}/{}?next={}",
        MATCHBOX_SERVER_URL, lobby_code, LOBBY_PLAYER_COUNT
    );

    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

pub fn update_matchmaking(
    mut commands: Commands,
    mut player_count: ResMut<PlayerCount>,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut rip: ResMut<RollbackIdProvider>,
    player_query: Query<Entity, With<Player>>,
    mut game_state: ResMut<NextState<GameStates>>,
) {
    socket.update_peers();

    let connected_peers = socket.connected_peers().count() + 1;
    player_count.0 = connected_peers as u8;

    if connected_peers < LOBBY_PLAYER_COUNT {
        return;
    }

    let mut session_builder = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(LOBBY_PLAYER_COUNT)
        .with_max_prediction_window(12)
        .with_input_delay(2)
        .with_fps(LOBBY_FPS)
        .expect("invalid fps");

    for (i, player) in socket.players().into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    let channel = socket.take_channel(0).unwrap();

    let session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(Session::P2PSession(session));

    for entity in player_query.iter() {
        commands.entity(entity).insert(rip.next());
    }

    game_state.set(GameStates::Lobby);
}
