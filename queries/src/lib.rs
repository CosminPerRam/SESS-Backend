use std::pin::Pin;
use std::time::Duration;
use juniper::{GraphQLEnum, graphql_object, graphql_subscription, graphql_value, FieldError};
use futures::Stream;
use async_stream::stream;
use context::Context;
use tokio;

#[derive(Clone, Copy, GraphQLEnum)]
enum UserKind {
    Admin,
    User,
    Guest,
}

struct User {
    id: i32,
    kind: UserKind,
    name: String,
}

// Field resolvers implementation
#[graphql_object(context = Context)]
impl User {
    fn id(&self) -> i32 {
        self.id
    }

    fn kind(&self) -> UserKind {
        self.kind
    }

    fn name(&self) -> &str {
        &self.name
    }

    async fn friends(&self) -> Vec<User> {
        if self.id == 1 {
            vec![
                User {
                    id: 11,
                    kind: UserKind::User,
                    name: "user11".into(),
                },
                User {
                    id: 12,
                    kind: UserKind::Admin,
                    name: "user12".into(),
                },
                User {
                    id: 13,
                    kind: UserKind::Guest,
                    name: "user13".into(),
                },
            ]
        } else if self.id == 2 {
            vec![User {
                id: 21,
                kind: UserKind::User,
                name: "user21".into(),
            }]
        } else if self.id == 3 {
            vec![
                User {
                    id: 31,
                    kind: UserKind::User,
                    name: "user31".into(),
                },
                User {
                    id: 32,
                    kind: UserKind::Guest,
                    name: "user32".into(),
                },
            ]
        } else {
            vec![]
        }
    }
}

type UsersStream = Pin<Box<dyn Stream<Item = Result<User, FieldError>> + Send>>;

pub struct Subscription;

#[graphql_subscription(context = Context)]
impl Subscription {
    async fn users() -> UsersStream {
        let mut counter = 0;
        let mut interval = tokio::time::interval(Duration::from_millis(300));
        let stream = stream! {
            loop {
                counter += 1;
                interval.tick().await;
                println!("{counter}");
                if counter == 4 {
                    yield Err(FieldError::new(
                        "some field error from handler",
                        graphql_value!("some additional string"),
                    ))
                } else if counter < 4 {
                    yield Ok(User {
                        id: counter,
                        kind: UserKind::Admin,
                        name: "stream user".into(),
                    })
                } else {
                    return;
                }
            }
        };

        Box::pin(stream)
    }
}
