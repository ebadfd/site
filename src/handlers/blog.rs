use super::Result;
use crate::{app::State, post::Post, tmpl};
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
};
use maud::Markup;
use std::sync::Arc;
use tracing::instrument;

#[instrument(skip(state))]
pub async fn index(Extension(state): Extension<Arc<State>>) -> Result<Markup> {
    let state = state.clone();
    let result = tmpl::blog::post_index(&state.blog, "Blogposts", true);
    Ok(result)
}

#[instrument(skip(state))]
pub async fn post_view(
    Path(name): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> Result<(StatusCode, Markup)> {
    let mut want: Option<&Post> = None;
    let want_link = format!("blog/{}", name);

    for post in &state.blog {
        if post.link == want_link {
            want = Some(post);
        }
    }

    match want {
        None => Ok((StatusCode::NOT_FOUND, tmpl::not_found(want_link))),
        Some(post) => {
            //HIT_COUNTER
            //   .with_label_values(&[name.clone().as_str()])
            //  .inc();
            let body = maud::PreEscaped(&post.body_html);
            Ok((StatusCode::OK, tmpl::blog::post(post, body)))
        }
    }
}
