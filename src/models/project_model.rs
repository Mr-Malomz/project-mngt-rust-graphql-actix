use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub _id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub _id: Option<ObjectId>,
    pub owner_id: ObjectId,
    pub name: String,
    pub description: String,
    pub status: Status,
}
