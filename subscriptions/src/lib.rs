mod filters;
mod server;
mod player;

use std::net::SocketAddr;
use std::pin::Pin;
use juniper::{graphql_subscription, FieldError};
use futures::Stream;
use async_stream::stream;
use context::DatabaseContext;
use gamedig::valve_master_server::{query_singular, Region};
use gamedig::protocols::valve::{Engine, query, GatheringSettings};

use crate::filters::{ServersFilters, to_gamedig_filters};
use crate::server::Server;

pub struct Subscription;

type ServersStream = Pin<Box<dyn Stream<Item = Result<Server, FieldError>> + Send>>;

const GATHER_SETTINGS: GatheringSettings = GatheringSettings {
    players: true,
    rules: false
};

const DEFAULT_LIMIT_AMOUNT: i32 = 48;
const fn get_limit_amount(limit: Option<i32>) -> i32 {
    match limit {
        Some(v) => {
            if v < 0 {
                DEFAULT_LIMIT_AMOUNT
            }
            else {
                v
            }
        },
        None => DEFAULT_LIMIT_AMOUNT
    }
}

#[graphql_subscription(context = DatabaseContext)]
impl Subscription {
    async fn servers(&self, context: &DatabaseContext,
                         filters: Option<ServersFilters>,
                         nor_filters: Option<ServersFilters>,
                         nand_filters: Option<ServersFilters>,
                         limit: Option<i32>) -> ServersStream {
        let limit = get_limit_amount(limit);
        let mut collected = 0;

        context.add_server_query_visit().await;

        let search_filters = to_gamedig_filters(filters, nor_filters, nand_filters);
        let servers_listings = query_singular(Region::Europe, Some(search_filters)).unwrap();

        let stream = stream! {
            for listing in servers_listings {
                if collected == limit {
                    break;
                }

                let ip = listing.0;
                let port = listing.1;

                let server_response = query(&SocketAddr::new(ip, port), Engine::Source(None), Some(GATHER_SETTINGS), None);

                if let Ok(response) = server_response {
                    collected += 1;
                    yield Ok(Server::from_valve_response(response))
                }
            }

            //context.add_processed_servers(collected as u32).await;
            //fucking problem
        };

        Box::pin(stream)
    }
}
