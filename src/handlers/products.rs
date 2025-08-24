use super::{is_htmx_request, Result};
use crate::{app::State, post::Post, tmpl};
use axum::{
    extract::{Extension, Path},
    http::{HeaderMap, StatusCode},
};
use lazy_static::lazy_static;
use maud::Markup;
use prometheus::{opts, register_int_counter_vec, IntCounterVec};
use std::sync::Arc;
use tracing::instrument;

lazy_static! {
    static ref HIT_COUNTER: IntCounterVec = register_int_counter_vec!(
        opts!("products_hits", "Number of hits to products posts"),
        &["name"]
    )
    .unwrap();
}

#[instrument(skip(state, headers))]
pub async fn index(Extension(state): Extension<Arc<State>>, headers: HeaderMap) -> Result<Markup> {
    let state = state.clone();
    let result = tmpl::blog::post_index(&state.products, "FreeWare", false, is_htmx_request(headers));
    Ok(result)
}

#[instrument(skip(state, headers))]
pub async fn product_view(
    Path(name): Path<String>,
    Extension(state): Extension<Arc<State>>,
    headers: HeaderMap,
) -> Result<(StatusCode, Markup)> {
    let mut want: Option<&Post> = None;
    let want_link = format!("products/{}", name);

    for product in &state.products {
        if product.link == want_link {
            want = Some(product);
        }
    }

    match want {
        None => Ok((StatusCode::NOT_FOUND, tmpl::not_found(want_link))),
        Some(product) => {
            HIT_COUNTER
                .with_label_values(&[name.clone().as_str()])
                .inc();
            let body = maud::PreEscaped(&product.body_html);
            Ok((
                StatusCode::OK,
                tmpl::blog::post(
                    product,
                    body,
                    &state.cfg.default_author,
                    &state.cfg.domain,
                    is_htmx_request(headers),
                ),
            ))
        }
    }
}
