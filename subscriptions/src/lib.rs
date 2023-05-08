mod filters;

use std::pin::Pin;
use juniper::{graphql_object, graphql_subscription, FieldError};
use futures::Stream;
use async_stream::stream;
use context::Context;
use gamedig::valve_master_server::{query_singular, Region};
use gamedig::protocols::valve::{Engine, query, GatheringSettings};
use crate::filters::{ServersFilters, to_gamedig_filters};

struct Server {
    name: String
}

// Field resolvers implementation
#[graphql_object(context = Context)]
impl Server {
    fn name(&self) -> &str {
        &self.name
    }
}

type ServersStream = Pin<Box<dyn Stream<Item = Result<Server, FieldError>> + Send>>;

pub struct Subscription;

#[graphql_subscription(context = Context)]
impl Subscription {
    async fn servers(filters: Option<ServersFilters>, nor_filters: Option<ServersFilters>, nand_filters: Option<ServersFilters>) -> ServersStream {
        let stream = stream! {
            let gather_settings = GatheringSettings {
                players: false,
                rules: false
            };

            let search_filters = to_gamedig_filters(filters, nor_filters, nand_filters);

            let servers_listings = query_singular(Region::Europe, Some(search_filters)).unwrap();

            for listing in servers_listings {
                let ip = listing.0.to_string();
                let port = listing.1;
                println!("{ip}:{port}");

                let server_response = query(&ip, port, Engine::Source(None), Some(gather_settings), None);

                if let Ok(server) = server_response {
                    yield Ok(Server {
                        name: server.info.name
                    })
                }
            }
        };

        Box::pin(stream)
    }
}
