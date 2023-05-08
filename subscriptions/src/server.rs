use gamedig::protocols::valve::Response;
use juniper::graphql_object;
use context::Context;
use crate::player::Player;

pub struct Server {
    name: String,
    players: Vec<Player>
}

// Field resolvers implementation
#[graphql_object(context = Context)]
impl Server {
    fn name(&self) -> &str {
        &self.name
    }

    fn players(&self) -> &Vec<Player> {
        &self.players
    }
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
