use std::sync::{Arc, Mutex};

use actix_web::{guard, web, HttpRequest, HttpResponse, Result};
use async_graphql::dataloader::*;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, Schema};
use async_graphql_actix_web::{Request, Response, WSSubscription};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

mod connection;
use crate::connection::PgPool;


//cfg: 
pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/")
       .route(web::post().to(index))
       .route(web::get().guard(guard::Header("upgrade", "webscoket")).to(index_ws))
       .route(web::get().to(index_playground))
    );
}

pub fn index(schema: web::Data<AppSchema>, http_request: HttpRequest, request: Request) -> Response {
	let mut query = request.into_inner();
    let role_option = common_utils::get_role(http_request);

	if let Some(role) = role_option {
		query = query.data(role);
    }

	schema.execute(query).await.into()
}
