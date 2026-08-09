use axum::{
    body,
    extract::Extension,
    http::header::{self, HeaderValue, CONTENT_TYPE},
    http::Request,
    middleware::{self, Next},
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
    compression::{
        predicate::{NotForContentType, Predicate},
        CompressionLayer, DefaultPredicate,
    },
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

pub mod app;
pub mod handlers;
pub mod post;
pub mod tmpl;

const APPLICATION_NAME: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

async fn healthcheck() -> &'static str {
    "OK"
}

/// Cache-Control is path dependent: the v86 guest images are content addressed (the chunk file
/// names are hashes) so they can be cached forever, everything else gets the short shared TTL.
async fn cache_control<B>(request: Request<B>, next: Next<B>) -> Response {
    let immutable = request.uri().path().starts_with("/static/v86/images/");
    let mut response = next.run(request).await;

    let value = if !response.status().is_success() {
        "no-store"
    } else if immutable {
        "public, max-age=31536000, immutable"
    } else {
        "public, max-age=3600, stale-if-error=60"
    };

    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static(value));

    response
}

// Not wired up. If it ever is, note that the v86 frame needs `frame-src 'self'` here and
// `'wasm-unsafe-eval'` in its own script-src.
fn _csp_header(_: &Response) -> Option<header::HeaderValue> {
    Some(header::HeaderValue::from_static(
        "base-uri 'self'; default-src 'self' https://cdnjs.cloudflare.com https://unpkg.com/ https://js.sentry-cdn.com; frame-src 'self'; child-src 'self'; connect-src 'self'; report-uri https://z9fr.report-uri.com/r/d/csp/wizard; object-src 'none'"
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

    // The v86 guest images (`.bin` 9p chunks, the `.zst` state snapshot) are already compressed
    // or incompressible, so re-encoding them on the fly is pure CPU burn. Everything else,
    // including v86.wasm, still gets compressed.
    let comression_layer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true)
        .compress_when(
            DefaultPredicate::new()
                .and(NotForContentType::const_new("application/octet-stream"))
                .and(NotForContentType::const_new("application/zstd")),
        );

    let files = ServeDir::new("static");
    let well_known_files = ServeDir::new("well-known");

    let middleware = tower::ServiceBuilder::new()
        .layer(Extension(state.clone()))
        .layer(middleware::from_fn(cache_control))
        //.layer(SetResponseHeaderLayer::overriding(
        //    header::CONTENT_SECURITY_POLICY_REPORT_ONLY,
        //    csp_header,
        //))
        // The v86 frame is sandboxed without `allow-same-origin`, so it fetches its own assets
        // from an opaque origin. Permissive CORS is what makes those same-site fetches work.
        .layer(CorsLayer::permissive());

    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/health", get(healthcheck))
        .route("/metrics", get(metrics))
        .route("/stack", get(handlers::stack))
        .route("/contact", get(handlers::contact))
        .route("/support", get(handlers::support))
        .route("/privacy-policy", get(handlers::privacy_policy))
        .route("/email", post(handlers::email_address))
        // blog
        .route("/blog", get(handlers::blog::index))
        .route("/blog/:name", get(handlers::blog::post_view))
        // products
        .route("/products", get(handlers::products::index))
        .route("/products/:name", get(handlers::products::product_view))
        // sandboxed v86 terminal
        .route("/termx", get(handlers::term::termx))
        .route("/termx/boot.json", get(handlers::term::termx_boot))
        // feeds
        .route("/blog.rss", get(handlers::feed::rss))
        .route("/blog.atom", get(handlers::feed::atom))
        .route("/blog.json", get(handlers::feed::json))
        .route("/sitemap.xml", get(handlers::feed::sitemap))
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
