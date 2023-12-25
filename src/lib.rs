pub mod tmpl;

use axum::{
    routing::{get, get_service},
    Extension, Router,
};
use color_eyre::eyre::Result;
use dotenv::dotenv;
use log::info;
use std::{
    env,
    net::{IpAddr, SocketAddr},
    str::FromStr,
    sync::Arc,
};
use tower_http::services::ServeFile;

pub mod app;
pub mod handlers;
pub mod post;

async fn healthcheck() -> &'static str {
    "OK"
}

pub async fn run_server() -> Result<()> {
    dotenv().ok();
    color_eyre::install()?;
    info!("starting the application");

    let state = Arc::new(
        app::init(
            env::var("CONFIG_FNAME")
                .unwrap_or("./config.dhall".into())
                .as_str()
                .into(),
        )
        .await?,
    );

    let middleware = tower::ServiceBuilder::new().layer(Extension(state.clone()));

    let app = Router::new()
        .route("/health", get(healthcheck))
        .route(
            "/robots.txt",
            get_service(ServeFile::new("./static/robots.txt")),
        )
        .route("/", get(handlers::index));

    let addr: SocketAddr = (
        IpAddr::from_str(&env::var("HOST").unwrap_or("::".into()))?,
        env::var("PORT").unwrap_or("3030".into()).parse::<u16>()?,
    )
        .into();

    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
