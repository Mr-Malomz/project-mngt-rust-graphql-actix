use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    _id: Option<ObjectId>,
    name: String,
    email: String,
    phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    _id: Option<ObjectId>,
    owner_id: ObjectId,
    name: String,
    description: String,
    status: Status,
}
