use axum::{
    body,
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use maud::Markup;
use std::sync::Arc;
use tracing::instrument;

use crate::{app::State, tmpl};

#[instrument(skip(state))]
pub async fn index(Extension(state): Extension<Arc<State>>) -> Result<Markup> {
    //HIT_COUNTER.with_label_values(&["index"]).inc();
    let state = state.clone();
    let cfg = state.cfg.clone();

    Ok(tmpl::index(&cfg.default_author, &cfg.notable_projects))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("series not found: {0}")]
    SeriesNotFound(String),

    #[error("post not found: {0}")]
    PostNotFound(String),

    #[error("patreon key not working, poke me to get this fixed")]
    NoPatrons,

    #[error("io error: {0}")]
    IO(#[from] std::io::Error),

    #[error("axum http error: {0}")]
    AxumHTTP(#[from] axum::http::Error),

    #[error("string conversion error: {0}")]
    ToStr(#[from] http::header::ToStrError),
}

pub type Result<T = Html<Vec<u8>>> = std::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let result = tmpl::error(format!("{}", self));
        let result = result.0;

        let body = body::boxed(body::Full::from(result));

        Response::builder()
            .status(match self {
                Error::SeriesNotFound(_) | Error::PostNotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            })
            .body(body)
            .unwrap()
    }
}
