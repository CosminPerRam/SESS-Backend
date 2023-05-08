use gamedig::protocols::valve::ServerPlayer;
use juniper::graphql_object;
use context::Context;

pub struct Player {
    name: String
}

// Field resolvers implementation
#[graphql_object(context = Context)]
impl Player {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Player {
    pub fn from_valve_response(player: &ServerPlayer) -> Self {
        Self {
            name: player.clone().name
        }
    }
}
