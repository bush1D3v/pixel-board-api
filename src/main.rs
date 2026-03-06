use actix_web::{App, HttpServer, web};
use config::{api_doc::api_doc, cors::cors};
use infra::{migrations::run_migrations, postgres::Postgres, redis::Redis, storage::Storage};
use modules::{
    block::block_controllers::block_routes, board::board_controllers::board_routes,
    upload::upload_controllers::upload_routes, user::user_controllers::user_routes,
};
use std::{env, net::Ipv4Addr};

mod config;
mod infra;
mod middlewares;
mod modules;
mod shared;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let postgres_pool = Postgres::pool();

    // Run SQL migrations before serving requests.
    run_migrations(&postgres_pool).await?;

    let redis_pool = Redis::pool().await;

    // MinIO / S3 client
    let s3_client = Storage::client();
    let bucket = env::var("MINIO_BUCKET").unwrap_or_else(|_| "pixel-board".to_string());
    Storage::ensure_bucket(&s3_client, &bucket).await;

    let port: u16 = env::var("HTTP_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("HTTP_PORT must be a valid number");

    log::info!("Starting PixelBoard API on port {port}");

    HttpServer::new(move || {
        App::new()
            .wrap(cors())
            .app_data(web::Data::new(postgres_pool.clone()))
            .app_data(web::Data::new(redis_pool.clone()))
            .app_data(web::Data::new(s3_client.clone()))
            .service(user_routes())
            .service(block_routes())
            .service(board_routes())
            .service(upload_routes())
            .configure(api_doc())
    })
    .bind((Ipv4Addr::UNSPECIFIED, port))?
    .run()
    .await?;

    Ok(())
}
