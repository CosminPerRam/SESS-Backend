use gamedig::protocols::valve::ServerPlayer;
use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct Player {
    name: String,
    score: i32,
    duration: f64
}

impl Player {
    pub fn from_valve_response(player: ServerPlayer) -> Self {
        Self {
            name: player.name,
            score: player.score as i32,
            duration: player.duration as f64
        }
    }
}
