use dotenv::dotenv;
use std::{env, io::Error};

use mongodb::{Client, Collection, Database};

use crate::models::project_model::Owner;

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

    fn colHelper<T>(dataSource: &Self, collectionName: &str) -> Collection<T> {
        dataSource.db.collection(collectionName)
    }

    pub async fn create_owner(&self, new_owner: Owner) -> Result<Owner, Error> {
        let new_doc = Owner {
            _id: None,
            name: new_owner.name.clone(),
            email: new_owner.email.clone(),
            phone: new_owner.phone.clone(),
        };

        let col = MongoDB::colHelper::<Owner>(&self, "owner");

        let user = col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating owner");

        let new_owner = Owner {
            _id: user.inserted_id.as_object_id(),
            name: new_owner.name.clone(),
            email: new_owner.email.clone(),
            phone: new_owner.phone.clone(),
        };

        Ok(new_owner)
    }
}
