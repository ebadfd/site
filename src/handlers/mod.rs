use crate::{app::State, tmpl};
use axum::{
    body,
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use chrono::{Datelike, Timelike, Utc, Weekday};
use lazy_static::lazy_static;
use maud::Markup;
use prometheus::{opts, register_int_counter_vec, IntCounterVec};
use std::sync::Arc;
use tracing::instrument;

pub mod blog;
pub mod feed;

fn weekday_to_name(w: Weekday) -> &'static str {
    use Weekday::*;
    match w {
        Sun => "Sun",
        Mon => "Mon",
        Tue => "Tue",
        Wed => "Wed",
        Thu => "Thu",
        Fri => "Fri",
        Sat => "Sat",
    }
}

fn month_to_name(m: u32) -> &'static str {
    match m {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "Unk",
    }
}

lazy_static! {
    pub static ref HIT_COUNTER: IntCounterVec =
        register_int_counter_vec!(opts!("hits", "Number of hits to various pages"), &["page"])
            .unwrap();
    pub static ref LAST_MODIFIED: String = {
        let now = Utc::now();
        format!(
            "{dayname}, {day} {month} {year} {hour}:{minute}:{second} GMT",
            dayname = weekday_to_name(now.weekday()),
            day = now.day(),
            month = month_to_name(now.month()),
            year = now.year(),
            hour = now.hour(),
            minute = now.minute(),
            second = now.second()
        )
    };
}

#[instrument(skip(state))]
pub async fn index(Extension(state): Extension<Arc<State>>) -> Result<Markup> {
    HIT_COUNTER.with_label_values(&["index"]).inc();
    let state = state.clone();
    let cfg = state.cfg.clone();

    Ok(tmpl::index(&cfg.default_author, &state.blog, &cfg.domain))
}

#[instrument(skip(state))]
pub async fn contact(Extension(state): Extension<Arc<State>>) -> Markup {
    HIT_COUNTER.with_label_values(&["contact"]).inc();
    let state = state.clone();
    let cfg = state.cfg.clone();

    crate::tmpl::contact(&cfg.contact_links)
}

#[instrument]
pub async fn stack() -> Markup {
    HIT_COUNTER.with_label_values(&["stack"]).inc();
    crate::tmpl::stack()
}

#[axum_macros::debug_handler]
pub async fn resume() -> Markup {
    //tmpl::resume()
    todo!()
}

#[instrument]
pub async fn not_found(uri: axum::http::Uri) -> (StatusCode, Markup) {
    HIT_COUNTER.with_label_values(&["not_found"]).inc();
    (StatusCode::NOT_FOUND, tmpl::not_found(uri.path()))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("post not found: {0}")]
    PostNotFound(String),

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
                Error::PostNotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            })
            .body(body)
            .unwrap()
    }
}
