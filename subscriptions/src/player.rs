use gamedig::protocols::valve::ServerPlayer;
use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct Player {
    name: String
}

impl Player {
    pub fn from_valve_response(player: &ServerPlayer) -> Self {
        Self {
            name: player.clone().name
        }
    }
}
