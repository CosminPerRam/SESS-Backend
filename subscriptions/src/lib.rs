use std::pin::Pin;
use juniper::{graphql_object, graphql_subscription, FieldError};
use futures::Stream;
use async_stream::stream;
use context::Context;
use gamedig::valve_master_server::{query_singular, Region, SearchFilters, Filter};
use gamedig::protocols::valve::{Engine, query, GatheringSettings};

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
    async fn servers() -> ServersStream {
        let stream = stream! {
            loop {
                // see warp_subscriptions example from juniper

                let search_filters = SearchFilters::new()
                    .insert(Filter::RunsAppID(440))
                    .insert(Filter::CanBeEmpty(false))
                    .insert(Filter::CanBeFull(false))
                    .insert(Filter::CanHavePassword(false))
                    .insert(Filter::IsSecured(true))
                    .insert(Filter::HasTags(&["minecraft"]));

                let responses = query_singular(Region::Europe, Some(search_filters)).unwrap();

                for server in responses {
                    let ip = server.0.to_string();
                    let port = server.1;
                    println!("{ip}:{port}");
                    let gather_settings = GatheringSettings {
                        players: false,
                        rules: false
                    };

                    let response = query(&ip, port, Engine::Source(None), Some(gather_settings), None).unwrap();

                    yield Ok(Server {
                        name: response.info.name
                    })
                }

                return
            }
        };

        Box::pin(stream)
    }
}
