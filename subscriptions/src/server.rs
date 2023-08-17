use gamedig::protocols::valve::{Response, Server as ValveServerKind};
use juniper::{GraphQLEnum, GraphQLObject};
use crate::player::Player;

#[derive(GraphQLEnum)]
pub enum ServerKind {
    Dedicated,
    NonDedicated,
    TV
}

impl ServerKind {
    fn from_valve_response(kind: ValveServerKind) -> Self {
        match kind {
            ValveServerKind::Dedicated => ServerKind::Dedicated,
            ValveServerKind::NonDedicated => ServerKind::NonDedicated,
            ValveServerKind::TV => ServerKind::TV
        }
    }
}

#[derive(GraphQLObject)]
pub struct Server {
    protocol: i32,
    name: String,
    map: String,
    folder: String,
    game: String,
    appid: i32,
    players_online: i32,
    players_maximum: i32,
    players_bots: i32,
    server_type: ServerKind,
    //environment_type: Environment,
    has_password: bool,
    vac_secured: bool,
    version: String,
    //extra_data: Option<ExtraData>,
    is_mod: bool,
    //mod_data: Option<ModData>,
    players: Vec<Player>
}

impl Server {
    pub fn from_valve_response(response: Response) -> Self {
        Self {
            protocol: response.info.protocol_version as i32,
            name: response.info.name,
            map: response.info.map,
            folder: response.info.folder,
            game: response.info.game_mode,
            appid: response.info.appid as i32,
            players_online: response.info.players_online as i32,
            players_maximum: response.info.players_maximum as i32,
            players_bots: response.info.players_bots as i32,
            server_type: ServerKind::from_valve_response(response.info.server_type),
            has_password: response.info.has_password,
            vac_secured: response.info.vac_secured,
            version: response.info.game_version,
            players: response.players.unwrap_or_default()
                .into_iter()
                .map(Player::from_valve_response)
                .collect(),
            is_mod: response.info.is_mod,
        }
    }
}
