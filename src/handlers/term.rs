use super::{Result, TermxCmd, HIT_COUNTER};
use crate::{app::State, termx_registry::web::WebCommandRegistry};
use axum::{body, extract::Extension, response::Response, Form};
use maud::{html, Markup};
use std::sync::Arc;
use termx::{evaluator::WebEvaluatorResult, registries::CommandRegistry};
use tracing::instrument;

#[instrument(skip(state))]
pub async fn termx_results(
    Extension(state): Extension<Arc<State>>,
    Form(params): Form<TermxCmd>,
) -> Result<Response> {
    HIT_COUNTER.with_label_values(&["cmd"]).inc();
    let state = state.clone();
    let _cfg = state.cfg.clone();

    let registry = WebCommandRegistry::new(state);
    let mut display_content = String::new();

    let mut do_redirect = false;
    let mut redirect_path = String::new();

    if params.cmd == "help" {
        let help = registry.help();
        display_content.push_str(&help)
    } else {
        let results = termx::run(params.cmd.clone(), registry);

        results.iter().for_each(|f| match f {
            WebEvaluatorResult::Display(x) => {
                display_content.push_str(x);
            }
            WebEvaluatorResult::Error(x) => {
                display_content.push_str(x);
            }
            WebEvaluatorResult::Navigate(x) => {
                do_redirect = true;
                redirect_path.push_str(x);
                display_content.push_str("Redirecting...");
            }
        });
    };

    let mark_up = html!(
      div."exec" {
          span."user"{ (format!("{}@{}", WebCommandRegistry::get_user_name(), WebCommandRegistry::get_hostname()))} {}
          span."path"{"$"}
          span."command" { (params.cmd)}
          div."result" { span {(display_content)}}
      }
    );

    if do_redirect {
        let body = body::boxed(body::Full::from(mark_up.0));
        return Ok(Response::builder()
            .status(200)
            .header("HX-Redirect", redirect_path)
            .body(body)
            .expect("Error"));
    }

    let body = body::boxed(body::Full::from(mark_up.0));
    return Ok(Response::builder()
        .status(200)
        .header("X-Hacker", "True")
        .body(body)
        .expect("Error"));
}

#[instrument(skip())]
pub async fn termx() -> Markup {
    HIT_COUNTER.with_label_values(&["cmd"]).inc();
    crate::tmpl::termx_default()
}
