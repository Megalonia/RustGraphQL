use std::sync::{Arc, Mutex};

use actix_web::{guard, web, HttpRequest, HttpResponse, Result};
use async_graphql::dataloader::DataLoader;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, Schema};
use async_graphql_actix_web::{Request, Response, WSSubscription};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

use crate::graphql::{AppSchema, DetailsLoader, Mutation, Query, Subscription};
use crate::persistence::connection::PgPool;

pub fn config_service(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/")
            .route(web::post().to(index))
            .route(web::get().guard(guard::Header("upgrade", "websocket")).to(index_ws))
            .route(web::get().to(index_playground))
        );
}

//process GraphQL queries and mutations
async fn index(schema: web::Data<AppSchema>, http_req: HttpRequest, req: Request) -> Response {
    let mut query = req.into_inner();

    let maybe_role = common_utils::get_role(http_req);
    if let Some(role) = maybe_role {
        query = query.data(role);
    }

    schema.execute(query).await.into()
}


//process GraphQL subscriptions
async fn index_ws(schema: web::Data<AppSchema>, req: HttpRequest, payload: web::Payload) -> Result<HttpResponse> {
    WSSubscription::start(Schema::clone(&*schema), &req, payload)
}

//provides playground GraphQL IDE
async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/")))
}
 
//Creates GraphQL schema with global contextual data that are accessible at runtime
pub fn create_schema_with_context(pool: PgPool) -> Schema<Query, Mutation, Subscription> {
    let arc_pool = Arc::new(pool);
    let cloned_pool = Arc::clone(&arc_pool);
    let details_batch_loader = Loader::new(DetailsBatchLoader {
        pool: cloned_pool
    }).with_max_batch_size(10);

    let kafka_consumer_counter = Mutex::new(0);

    Schema::build(Query, Mutation, Subscription)
        .data(arc_pool)
        .data(details_batch_loader)
        .data(kafka::create_producer())
        .data(kafka_consumer_counter)
        .finish()
}


