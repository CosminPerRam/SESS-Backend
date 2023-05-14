use gamedig::protocols::valve::Response;
use juniper::GraphQLObject;
use crate::player::Player;

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
    //server_type: Server,
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
            protocol: response.info.protocol as i32,
            name: response.info.name,
            map: response.info.map,
            folder: response.info.folder,
            game: response.info.game,
            appid: response.info.appid as i32,
            players_online: response.info.players_online as i32,
            players_maximum: response.info.players_maximum as i32,
            players_bots: response.info.players_bots as i32,
            has_password: response.info.has_password,
            vac_secured: response.info.vac_secured,
            version: response.info.version,
            players: response.players.unwrap_or_default()
                .into_iter()
                .map(Player::from_valve_response)
                .collect(),
            is_mod: response.info.is_mod,
        }
    }
}
