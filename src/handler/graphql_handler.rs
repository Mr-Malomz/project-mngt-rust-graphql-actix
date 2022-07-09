use std::io::Error;

use juniper::graphql_object;
use mongodb::results::CollectionType;

use crate::{config::mongo, models::schema_graphql::{OwnerQL}};

struct Database {
    db: mongo::DBMongo,
}


impl juniper::Context for Database {}

struct Query {}

// #[graphql_object(context = Database)]
// impl Query {
//      async fn get_owners(&self, context: &Database) -> Result<Vec<OwnerQL>, Error> {
//         context.db.get_owners().await
//     }
// }
