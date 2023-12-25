use std::sync::Arc;

use axum::Extension;
use color_eyre::Result;
use maud::Markup;

use crate::{app::State, tmpl};

pub async fn index(Extension(state): Extension<Arc<State>>) -> Result<Markup> {
    //HIT_COUNTER.with_label_values(&["index"]).inc();
    let state = state.clone();
    let cfg = state.cfg.clone();

    Ok(tmpl::index(&cfg.default_author, &cfg.notable_projects))
}
