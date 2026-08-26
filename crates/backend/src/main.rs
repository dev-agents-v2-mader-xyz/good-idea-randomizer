// Template scaffold — agents add routes; stubs are unused until then.
#![allow(dead_code)]

use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::FileServer;
use rocket::{Request, Response};
use tracing_subscriber::EnvFilter;

mod auth;
mod config;
mod db;
mod error;
mod routes;

use routes::health::health;

struct SecurityHeaders;

#[rocket::async_trait]
impl Fairing for SecurityHeaders {
    fn info(&self) -> Info {
        Info { name: "Security Headers", kind: Kind::Response }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_raw_header("X-Content-Type-Options", "nosniff");
        res.set_raw_header("X-Frame-Options", "DENY");
        res.set_raw_header("Referrer-Policy", "strict-origin-when-cross-origin");
    }
}

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
        .attach(SecurityHeaders)
        .manage(pool)
        .manage(cfg)
        .mount("/", rocket::routes![health])
        // Serve the compiled Yew WASM frontend from dist/
        .mount("/", FileServer::from("dist").rank(10))
        .launch()
        .await?;

    Ok(())
}
