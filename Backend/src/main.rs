use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let conn_pool = create_connection_pool();
    run_migrations(&conn_pool);

    let schema = create_schema_with_context(conn_pool);

    HttpServer::new(move || App::new()
		.configure(configure_service)
		.data(schema.clone())
    ).bind("0.0.0.0:8001")?
     .run()
     .await()
}
