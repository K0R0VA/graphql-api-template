mod query;
mod mutation;

use actix_web::{get, HttpResponse, post, web::{self, ServiceConfig}};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql::{EmptySubscription, Schema, SchemaBuilder};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sqlx::PgPool;

use crate::api::mutation::Mutation;
use crate::api::query::Query;

#[get("/")]
async fn playground() -> HttpResponse {
    let html = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[post("/")]
async fn graphql(
    pool: web::Data<PgPool>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    create_schema().data(pool).finish().execute(req.into_inner()).await.into()
}

pub fn api_config(config: &mut ServiceConfig) {
    config
        .service(graphql)
        .service(playground);
}

fn create_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query {}, Mutation {}, EmptySubscription::default())
}
