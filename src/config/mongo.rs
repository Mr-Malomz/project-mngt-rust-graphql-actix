use dotenv::dotenv;
use std::env;

use mongodb::{Client, Database};

pub struct MongoDB {
    db: Database,
}

impl MongoDB {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = client.database("projectMngt");
        MongoDB { db }
    }
}
