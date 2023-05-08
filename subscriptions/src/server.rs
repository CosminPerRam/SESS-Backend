use gamedig::protocols::valve::Response;
use juniper::GraphQLObject;
use crate::player::Player;

#[derive(GraphQLObject)]
pub struct Server {
    name: String,
    players: Vec<Player>
}

impl Server {
    pub fn from_valve_response(response: Response) -> Self {
        Self {
            name: response.info.name,
            players: response.players.unwrap_or(Vec::new())
                .iter()
                .map(|p| Player::from_valve_response(p))
                .collect()
        }
    }
}
