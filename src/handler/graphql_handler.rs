use async_graphql::{Context, FieldResult, Object};

use crate::{
    config::mongo,
    schemas::project_schema::{CreateOwner, FetchOwner, FetchProject, Owner, Project},
};

struct Database {
    db: mongo::DBMongo,
}

struct Query;

#[Object(extends)]
impl Query {
    // async fn create_owner(&self, ctx: &Context<'_>, input: CreateOwner) -> FieldResult<Owner> {
    //     let db = &ctx.data_unchecked::<Database>().db;
    //     let new_owner = Owner {
    //         _id: None,
    //         email: input.email,
    //         name: input.name,
    //         phone: input.phone,
    //     };
    //     let owner = db.create_owner(new_owner).await.unwrap();
    //     Ok(owner)
    // }
    //owners query
    async fn owner(&self, ctx: &Context<'_>, input: FetchOwner) -> FieldResult<Owner> {
        let db = &ctx.data_unchecked::<Database>().db;
        let owner = db.single_owner(&input._id).await.unwrap();
        Ok(owner)
    }

    async fn get_owners(&self, ctx: &Context<'_>) -> FieldResult<Vec<Owner>> {
        let db = &ctx.data_unchecked::<Database>().db;
        let owners = db.get_owners().await.unwrap();
        Ok(owners)
    }

    //projects query
    async fn project(&self, ctx: &Context<'_>, input: FetchProject) -> FieldResult<Project> {
        let db = &ctx.data_unchecked::<Database>().db;
        let project = db.single_project(&input._id).await.unwrap();
        Ok(project)
    }

    async fn get_projects(&self, ctx: &Context<'_>) -> FieldResult<Vec<Project>> {
        let db = &ctx.data_unchecked::<Database>().db;
        let projects = db.get_projects().await.unwrap();
        Ok(projects)
    }
}
