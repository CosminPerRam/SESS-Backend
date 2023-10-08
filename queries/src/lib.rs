
use std::net::SocketAddr;
use gamedig::protocols::valve::{Engine, query, GatheringSettings};
use juniper::graphql_object;
use context::DatabaseContext;
use gqls::server::{Server, ServerInput};

pub struct Query;

const GATHER_SETTINGS: GatheringSettings = GatheringSettings {
    players: true,
    rules: false,
};

#[graphql_object(context = DatabaseContext)]
impl Query {
    pub async fn check(&self, _context: &DatabaseContext, server: ServerInput) -> Server {
        let ip = server.ip.parse().unwrap(); // TODO: Remove this shit
        let port = server.port as u16;

        let server_response = query(&SocketAddr::new(ip, port), Engine::Source(None), Some(GATHER_SETTINGS), None);
        Server::from_valve_response(server_response.unwrap())
    }
}
