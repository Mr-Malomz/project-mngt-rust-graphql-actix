use juniper::{GraphQLInputObject, GraphQLObject};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(GraphQLObject)]
pub struct OwnerQL {
    pub _id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(GraphQLInputObject)]
pub struct CreateOwnerQL {
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(juniper::GraphQLEnum)]
pub enum StatusQL {
    Pending,
    Doing,
    Completed,
}

#[derive(GraphQLObject)]
pub struct ProjectQL {
    pub _id: String,
    pub owner: String,
    pub name: String,
    pub description: String,
    pub status: StatusQL,
}

#[derive(GraphQLInputObject)]
pub struct CreateProjectQL {
    pub owner: String,
    pub name: String,
    pub description: String,
    pub status: StatusQL,
}
