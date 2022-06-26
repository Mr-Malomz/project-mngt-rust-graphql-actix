use juniper::{graphql_object, GraphQLObject};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Owner {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[juniper::graphql_object(description = "Owner")]
impl Owner {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn email(&self) -> &str {
        self.email.as_str()
    }
    pub fn phone(&self) -> &str {
        self.phone.as_str()
    }
}

#[derive(juniper::GraphQLEnum)]
#[derive(Clone, Copy)]
pub enum Status {
    Pending,
    Doing,
    Completed
}


pub struct Project {
    pub id: String,
    pub owner: Vec<Owner>,
    pub name: String,
    pub description: String,
    pub status: Status,
}

#[juniper::graphql_object(description = "Project")]
impl Project {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }
    pub fn owner(&self) -> Vec<Owner> {
        self.owner.to_vec()
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
    pub fn status(&self) -> Status {
        self.status
    }
}
