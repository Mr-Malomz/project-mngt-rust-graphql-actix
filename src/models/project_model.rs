use std::fmt::Debug;

use juniper::{graphql_object, GraphQLObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[graphql(description = "Owner details")]
pub struct Owner {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: Option<String>,
    pub owner: Vec<Owner>,
    pub name: String,
    pub description: String,
    pub status: String,
}

#[graphql_object]
impl Project {
    pub fn owner(&self) -> Option<&Owner> {
        self.owner.iter().find(|p| p.id == self.id)
    }
}
