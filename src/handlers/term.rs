use super::{HIT_COUNTER, V86_ASSET_BASE};
use crate::app::State;
use axum::{extract::Extension, Json};
use maud::Markup;
use serde::Serialize;
use std::sync::Arc;
use tracing::instrument;

/// Everything `static/v86/term.html` needs to boot, fetched by the frame on load.
///
/// It arrives over the wire rather than as query parameters on the iframe `src` so that a
/// visitor hitting `/static/v86/term.html` directly cannot point the emulator at images of
/// their choosing.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BootConfig {
    /// Base URL for the guest images, see `$V86_ASSET_BASE`.
    pub asset_base: String,
    /// Blog posts, written into the guest as `/root/blog/<name>`.
    pub files: Vec<BootFile>,
}

#[derive(Debug, Serialize)]
pub struct BootFile {
    pub name: String,
    pub content: String,
}

#[instrument]
pub async fn termx() -> Markup {
    HIT_COUNTER.with_label_values(&["v86"]).inc();
    crate::tmpl::termx_v86()
}

#[instrument(skip(state))]
pub async fn termx_boot(Extension(state): Extension<Arc<State>>) -> Json<BootConfig> {
    let files = state
        .blog
        .iter()
        .map(|post| BootFile {
            // `link` is "blog/<slug>"; the guest wants "<slug>.md".
            name: format!(
                "{}.md",
                post.link.rsplit('/').next().unwrap_or(post.link.as_str())
            ),
            content: post.body_md.clone(),
        })
        .collect();

    Json(BootConfig {
        asset_base: V86_ASSET_BASE.clone(),
        files,
    })
}
