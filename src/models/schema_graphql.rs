use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct Owner {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(juniper::GraphQLEnum)]
pub enum Status {
    Pending,
    Doing,
    Completed,
}

#[derive(GraphQLObject)]
pub struct Project {
    pub id: String,
    pub owner: String,
    pub name: String,
    pub description: String,
    pub status: Status,
}
