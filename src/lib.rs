use axum::{
    body,
    extract::Extension,
    http::header::{self, CONTENT_TYPE},
    response::Response,
    routing::get_service,
    routing::{get, post},
    Router,
};
use color_eyre::eyre::Result;
use dotenv::dotenv;
use log::info;
use prometheus::{Encoder, TextEncoder};
use std::{
    env,
    net::{IpAddr, SocketAddr},
    str::FromStr,
    sync::Arc,
};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};

pub mod app;
pub mod handlers;
pub mod post;
pub mod tmpl;

const APPLICATION_NAME: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

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

    let files = ServeDir::new("static");

    let middleware = tower::ServiceBuilder::new()
        .layer(Extension(state.clone()))
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            cache_header,
        ))
        .layer(CorsLayer::permissive());

    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/health", get(healthcheck))
        .route("/metrics", get(metrics))
        .route("/stack", get(handlers::stack))
        .route("/contact", get(handlers::contact))
        .route("/email", post(handlers::email_address))
        // blog
        .route("/blog", get(handlers::blog::index))
        .route("/blog/:name", get(handlers::blog::post_view))
        // feeds
        .route("/blog.rss", get(handlers::feed::rss))
        .route("/blog.atom", get(handlers::feed::atom))
        // static files
        .route(
            "/robots.txt",
            get_service(ServeFile::new("./static/robots.txt")),
        )
        .nest_service("/static", files)
        .fallback(handlers::not_found)
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

async fn metrics() -> Response {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "text/plain; charset=us-ascii")
        .body(body::boxed(body::Full::from(buffer)))
        .unwrap()
}

// And finally, include the generated code for templates and static files.
include!(concat!(env!("OUT_DIR"), "/templates.rs"));
