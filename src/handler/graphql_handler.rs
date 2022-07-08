use juniper::graphql_object;
use mongodb::results::CollectionType;

use crate::{config::mongo, models::schema_graphql::{Owner, CreateOwner}};

struct Context {
    db: mongo::MongoDB,
}


impl juniper::Context for Context {}

struct Query {}

// #[graphql_object(context = Context)]
// impl Query {
//      fn create_owner(context: &Context, input: CreateOwner) -> Option<&Owner> {
//         context.db.create_owner()
//     }
// }
