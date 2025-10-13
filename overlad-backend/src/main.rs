use std::{fmt::Debug, fs::Permissions, os::unix::fs::PermissionsExt, path::PathBuf};

use axum::{
    extract::DefaultBodyLimit,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    routing::{get, post},
    serve::Listener,
};
use clap::{Args, Parser};
use dotenvy::dotenv;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use sqlx::SqlitePool;
use tokio::net::{TcpListener, UnixListener};
use tower_http::cors::CorsLayer;

use crate::api::{
    all_images::all_images, image::get_image, overlay::get_overlay, register::register, token::token, upload::upload, user::get_user, user_images::user_images
};

mod api;
mod db;
mod util;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    listen: Listen,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Listen {
    #[arg(short, long, group = "listen")]
    port: Option<u16>,

    #[arg(short, long, group = "listen")]
    uds: Option<PathBuf>,
}

#[derive(Clone)]
pub struct AppState {
    key: Hmac<Sha256>,
    pool: SqlitePool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    dotenv().unwrap();

    if let Some(port) = cli.listen.port {
        serve_with_listener(TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap()).await;
    } else if let Some(path) = cli.listen.uds {
        let _ = tokio::fs::remove_file(&path).await;
        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .unwrap();

        let listener = UnixListener::bind(path.clone()).unwrap();

        tokio::fs::set_permissions(path, Permissions::from_mode(0o775))
            .await
            .unwrap();

        serve_with_listener(listener).await;
    }
}

async fn serve_with_listener<L>(listener: L)
where
    L: Listener,
    L::Addr: Debug,
{
    let pool = SqlitePool::connect(
        &std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set"),
    )
    .await
    .unwrap();

    let state = AppState {
        key: Hmac::new_from_slice(
            std::env::var("KEY")
                .expect("KEY environment variable not set")
                .as_bytes(),
        )
        .unwrap(),
        pool,
    };

    let app = axum::Router::new()
        .route("/register", post(register))
        .route("/token", post(token))
        .route("/upload", post(upload))
        .route("/overlay/{id}", get(get_overlay))
        .route("/all_images", get(all_images))
        .route("/user/{user_id}", get(get_user))
        .route("/user/{user_id}/images", get(user_images))
        .route("/image/{id}", get(get_image))
        .layer(DefaultBodyLimit::max(8000000))
        .layer(CorsLayer::permissive().allow_headers([AUTHORIZATION, CONTENT_TYPE]))
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}
