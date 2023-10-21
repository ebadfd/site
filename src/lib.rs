use axum::{routing::get, Router};
use color_eyre::eyre::Result;
use dotenv::dotenv;
use log::info;
use std::{
    env,
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

async fn healthcheck() -> &'static str {
    "OK"
}

pub async fn run_server() -> Result<()> {
    dotenv().ok();
    color_eyre::install()?;
    info!("starting the application");

    let app = Router::new().route("/health", get(healthcheck));

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
