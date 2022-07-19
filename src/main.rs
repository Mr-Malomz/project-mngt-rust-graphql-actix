use actix_web::{web::Data, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use handler::graphql_handler::ProjectSchema;

mod config;
mod handler;
mod schemas;

//graphql entry
async fn index(schema: Data<ProjectSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

fn main() {
    println!("Hello, world!");
}
