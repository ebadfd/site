use crate::{app::State, termx_registry::web::WebCommandRegistry, tmpl};
use axum::{
    body,
    extract::Extension,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    Form,
};
use cf_turnstile::{SiteVerifyRequest, TurnstileClient};
use chrono::{Datelike, Timelike, Utc, Weekday};
use lazy_static::lazy_static;
use log::debug;
use maud::{html, Markup};
use prometheus::{opts, register_int_counter_vec, IntCounterVec};
use std::sync::Arc;
use termx::{evaluator::WebEvaluatorResult, registries::CommandRegistry};
use tracing::instrument;

pub mod blog;
pub mod feed;

#[derive(Debug, serde::Deserialize)]
pub struct CFTurnstileParams {
    #[serde(rename = "cf-turnstile-response")]
    pub cf_turnstile_response: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct TermxCmd {
    pub cmd: String,
}

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

fn is_htmx_request(headers: HeaderMap) -> bool {
    if let Some(_hx_request) = headers.get("Hx-Request") {
        true
    } else {
        false
    }
}

lazy_static! {
    static ref TURNSTILE_SITE_KEY: String = String::from(
        std::env::var("TURNSTILE_SITE_KEY").expect("$TURNSTILE_SITE_KEY is not avaible")
    );
    static ref TURNSTILE_SITE_SECRET: String = String::from(
        std::env::var("TURNSTILE_SITE_SECRET").expect("$TURNSTILE_SITE_SECRET is not avaible")
    );
    pub static ref HIT_COUNTER: IntCounterVec =
        register_int_counter_vec!(opts!("hits", "Number of hits to various pages"), &["page"])
            .unwrap();
    pub static ref RICK_ROLL_COUNTER: IntCounterVec = register_int_counter_vec!(
        opts!("rickroll", "Number people got rick rolled"),
        &["page"]
    )
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

#[instrument(skip(state, headers))]
pub async fn index(Extension(state): Extension<Arc<State>>, headers: HeaderMap) -> Result<Markup> {
    HIT_COUNTER.with_label_values(&["index"]).inc();
    let state = state.clone();
    let cfg = state.cfg.clone();

    Ok(tmpl::index(
        &cfg.default_author,
        &state.blog,
        &cfg.domain,
        is_htmx_request(headers),
    ))
}

#[instrument(skip())]
pub async fn rr_handler() -> Result<Markup> {
    RICK_ROLL_COUNTER.with_label_values(&["rr"]).inc();
    Ok(tmpl::full_screen_player(
        "https://cdn.z9fr.xyz/videos/rr/manifest.mpd".to_string(),
    ))
}

#[instrument(skip(state, headers))]
pub async fn contact(Extension(state): Extension<Arc<State>>, headers: HeaderMap) -> Markup {
    HIT_COUNTER.with_label_values(&["contact"]).inc();
    let state = state.clone();
    let cfg = state.cfg.clone();
    crate::tmpl::contact(&cfg.contact_links, is_htmx_request(headers))
}

#[instrument(skip(state))]
pub async fn termx_results(
    Extension(state): Extension<Arc<State>>,
    Form(params): Form<TermxCmd>,
) -> Markup {
    HIT_COUNTER.with_label_values(&["cmd"]).inc();
    let state = state.clone();
    let _cfg = state.cfg.clone();

    let registry = WebCommandRegistry::new();
    let results = termx::run(params.cmd.clone(), registry);

    let mut display_content = String::new();

    results.iter().for_each(|f| match f {
        WebEvaluatorResult::Display(x) => {
            display_content.push_str(x);
        }
        _ => display_content.push_str("not implemented"),
    });

    return html!(
       div."exec" {
           span."user"{ (format!("{}@{}", WebCommandRegistry::get_user_name(), WebCommandRegistry::get_hostname()))} {}
           span."path"{"$"}
           span."command" { (params.cmd)}
           div."result" { span {(display_content)}}
       }
    );
}

#[instrument(skip(state, headers))]
pub async fn termx(Extension(state): Extension<Arc<State>>, headers: HeaderMap) -> Markup {
    HIT_COUNTER.with_label_values(&["cmd"]).inc();
    let state = state.clone();
    let cfg = state.cfg.clone();
    crate::tmpl::termx_default()
}

#[instrument]
pub async fn email_address(Form(params): Form<CFTurnstileParams>) -> Markup {
    HIT_COUNTER.with_label_values(&["email_view"]).inc();

    let client = TurnstileClient::new(TURNSTILE_SITE_SECRET.to_string().into());

    let validated = client
        .siteverify(SiteVerifyRequest {
            response: params.cf_turnstile_response.into(),
            ..Default::default()
        })
        .await;

    debug!("{:?}", validated);
    crate::tmpl::email_address(validated.is_ok())
}

#[instrument(skip(headers))]
pub async fn stack(headers: HeaderMap) -> Markup {
    HIT_COUNTER.with_label_values(&["stack"]).inc();
    crate::tmpl::stack(is_htmx_request(headers))
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
