use gamedig::protocols::valve::Response;
use juniper::graphql_object;
use context::Context;

pub struct Server {
    name: String
}

impl Server {
    pub fn from_valve_response(response: Response) -> Self {
        Self {
            name: response.info.name
        }
    }
}

// Field resolvers implementation
#[graphql_object(context = Context)]
impl Server {
    fn name(&self) -> &str {
        &self.name
    }
}
