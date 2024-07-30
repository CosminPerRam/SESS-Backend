
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::pin::Pin;
use juniper::{graphql_subscription, FieldError};
use futures::Stream;
use async_stream::stream;
use gamedig::protocols::types::GatherToggle;
use context::DatabaseContext;
use gamedig::valve_master_server::{Region, ValveMasterServer};
use gamedig::protocols::valve::{Engine, query, GatheringSettings};
use gqls::filters::{ServersFilters, to_gamedig_filters};
use gqls::server::{Server, ServerInput};

pub struct Subscription;

type ServersStream = Pin<Box<dyn Stream<Item = Result<Server, FieldError>> + Send>>;

const GATHER_SETTINGS: GatheringSettings = GatheringSettings {
    players: GatherToggle::Enforce,
    rules: GatherToggle::Skip,
    check_app_id: false,
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
    async fn servers(&self, _context: &DatabaseContext,
                         filters: Option<ServersFilters>,
                         nor_filters: Option<ServersFilters>,
                         nand_filters: Option<ServersFilters>,
                         limit: Option<i32>) -> ServersStream {
        let limit = get_limit_amount(limit);
        let mut collected = 0;

        let search_filters = to_gamedig_filters(filters, nor_filters, nand_filters);

        // TODO: DNS THIS
        let servers_listings = match ValveMasterServer::new(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 69, 99, 29)), 27011)) {
            Ok(mut ms) => ms.query(Region::Europe, Some(search_filters)).inspect_err(|e| println!("Master server query error {e}")).unwrap_or_default(),
            Err(e) => {
                println!("Master server creation error: {e}");
                Vec::new()
            }
        };

        Box::pin(stream! {
            for listing in servers_listings {
                if collected == limit {
                    break;
                }

                let (ip, port) = listing;
                let server_response = query(&SocketAddr::new(ip, port), Engine::Source(None), Some(GATHER_SETTINGS), None);

                match server_response {
                    Err(e) => println!("Server query error: {e}"),
                    Ok(response) => {
                        collected += 1;
                        yield Ok(Server::from_valve_response(response))
                    }
                }
            }
        })
    }

    async fn confirm(&self, _context: &DatabaseContext, servers: Vec<ServerInput>) -> ServersStream {
        let stream = stream! {
            for server in servers {
                let ip = server.ip.parse().unwrap(); // TODO: Remove this shit
                let port = server.port as u16;

                let server_response = query(&SocketAddr::new(ip, port), Engine::Source(None), Some(GATHER_SETTINGS), None);

                match server_response {
                    Err(e) => println!("Server query error: {e}"),
                    Ok(response) => yield Ok(Server::from_valve_response(response))
                }
            }
        };

        Box::pin(stream)
    }
}
