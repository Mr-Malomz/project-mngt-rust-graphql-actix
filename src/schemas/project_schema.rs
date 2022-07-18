use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

//owner schema
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Owner {
    pub _id: Option<String>,
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(InputObject)]
pub struct CreateOwner {
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(InputObject)]
pub struct FetchOwner {
   pub _id: String,
}

//project schema
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Project {
    pub _id: Option<String>,
    pub owner_id: String,
    pub name: String,
    pub description: String,
    pub status: Status,
}

#[derive(InputObject)]
pub struct CreateProject {
    pub owner: String,
    pub name: String,
    pub description: String,
    pub status: Status,
}

#[derive(InputObject)]
pub struct FetchProject {
   pub _id: String,
}
