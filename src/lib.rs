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
    compression::{CompressionLayer, DefaultPredicate},
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};

pub mod app;
pub mod handlers;
pub mod post;
pub mod termx_registry;
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

fn _csp_header(_: &Response) -> Option<header::HeaderValue> {
    Some(header::HeaderValue::from_static(
        "base-uri 'self'; default-src 'self' https://cdnjs.cloudflare.com https://unpkg.com/ https://js.sentry-cdn.com; report-uri https://z9fr.report-uri.com/r/d/csp/wizard; object-src 'none'"
    ))
}

pub async fn run_server() -> Result<()> {
    dotenv().ok();
    env_logger::try_init().ok();
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

    let comression_layer: CompressionLayer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true)
        .compress_when(DefaultPredicate::new());

    let files = ServeDir::new("static");
    let well_known_files = ServeDir::new("well-known");

    let middleware = tower::ServiceBuilder::new()
        .layer(Extension(state.clone()))
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            cache_header,
        ))
        //.layer(SetResponseHeaderLayer::overriding(
        //    header::CONTENT_SECURITY_POLICY_REPORT_ONLY,
        //    csp_header,
        //))
        .layer(CorsLayer::permissive());

    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/health", get(healthcheck))
        .route("/metrics", get(metrics))
        .route("/stack", get(handlers::stack))
        .route("/contact", get(handlers::contact))
        .route("/privacy-policy", get(handlers::privacy_policy))
        .route("/email", post(handlers::email_address))
        // blog
        .route("/blog", get(handlers::blog::index))
        .route("/blog/:name", get(handlers::blog::post_view))
        // termx
        .route("/termx", post(handlers::term::termx_results))
        .route("/termx", get(handlers::term::termx))
        // feeds
        .route("/blog.rss", get(handlers::feed::rss))
        .route("/blog.atom", get(handlers::feed::atom))
        // static files
        .route(
            "/robots.txt",
            get_service(ServeFile::new("./static/robots.txt")),
        )
        .route("/rr", get(handlers::rr_handler))
        .nest_service("/static", files)
        .nest_service("/.well-known", well_known_files)
        .fallback(handlers::not_found)
        .layer(comression_layer)
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
