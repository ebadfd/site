use axum::{extract::Extension, http::header, response::Response, routing::get, Router};
use color_eyre::eyre::Result;
use dotenv::dotenv;
use log::info;
use std::{
    env,
    net::{IpAddr, SocketAddr},
    str::FromStr,
    sync::Arc,
};
use tower_http::{cors::CorsLayer, set_header::SetResponseHeaderLayer};

pub mod app;
pub mod handlers;
pub mod post;
pub mod tmpl;

async fn healthcheck() -> &'static str {
    "OK"
}

fn cache_header(_: &Response) -> Option<header::HeaderValue> {
    Some(header::HeaderValue::from_static(
        "public, max-age=3600, stale-if-error=60",
    ))
}

pub async fn run_server() -> Result<()> {
    dotenv().ok();
    color_eyre::install()?;
    info!("starting the application");

    let state = Arc::new(
        app::init(
            env::var("CONFIG_FNAME")
                .unwrap_or("./Config.toml".into())
                .as_str()
                .into(),
        )
        .await?,
    );

    let middleware = tower::ServiceBuilder::new()
        .layer(Extension(state.clone()))
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            cache_header,
        ))
        .layer(CorsLayer::permissive());

    let app = Router::new()
        .route("/.within/health", get(healthcheck))
        .route("/", get(handlers::index))
        .layer(middleware);

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
