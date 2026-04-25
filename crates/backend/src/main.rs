// Template scaffold — agents add routes; stubs are unused until then.
#![allow(dead_code)]

use rocket::fs::FileServer;
use tracing_subscriber::EnvFilter;

mod auth;
mod config;
mod db;
mod error;
mod routes;

use routes::health::health;

#[rocket::main]
#[allow(clippy::result_large_err)]
async fn main() -> Result<(), rocket::Error> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cfg = config::Config::from_env().expect("configuration error");
    let pool = db::connect(&cfg.database_url, &cfg.supabase_schema)
        .await
        .expect("database connection failed");

    let _rocket = rocket::build()
        .manage(pool)
        .manage(cfg)
        .mount("/", rocket::routes![health])
        // Serve the compiled Yew WASM frontend from dist/
        .mount("/", FileServer::from("dist").rank(10))
        .launch()
        .await?;

    Ok(())
}
