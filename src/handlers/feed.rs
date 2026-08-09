use super::{Result, LAST_MODIFIED};
use crate::{app::State, templates};
use axum::{body, extract::Extension, response::Response};
use lazy_static::lazy_static;
use prometheus::{opts, register_int_counter_vec, IntCounterVec};
use std::sync::Arc;
use tracing::instrument;

lazy_static! {
    pub static ref HIT_COUNTER: IntCounterVec = register_int_counter_vec!(
        opts!("feed_hits", "Number of hits to various feeds"),
        &["kind"]
    )
    .unwrap();
    pub static ref ETAG: String = format!(
        r#"W/"{}""#,
        uuid::Uuid::new_v4().to_string().replace('-', "")
    );
    pub static ref CACHEBUSTER: String = uuid::Uuid::new_v4().to_string().replace('-', "");
}

#[instrument(skip(state))]
pub async fn rss(Extension(state): Extension<Arc<State>>) -> Result<Response> {
    HIT_COUNTER.with_label_values(&["rss"]).inc();
    let state = state.clone();
    let mut buf = Vec::new();
    templates::blog_rss_xml(&mut buf, state.everything.clone())?;
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/rss+xml")
        .header("ETag", ETAG.clone())
        .header("Last-Modified", &*LAST_MODIFIED)
        .body(body::boxed(body::Full::from(buf)))?)
}

/// JSON Feed 1.1. Advertised in every page `<head>` as `application/json` and
/// linked from the blog index, so it has to exist — it was a dead link before.
#[instrument(skip(state))]
pub async fn json(Extension(state): Extension<Arc<State>>) -> Result<Response> {
    HIT_COUNTER.with_label_values(&["json"]).inc();
    let state = state.clone();
    let domain = &state.cfg.domain;
    let author = &state.cfg.default_author;

    let items: Vec<serde_json::Value> = state
        .everything
        .iter()
        .map(|post| {
            let url = format!("https://{}/{}", domain, post.link);
            let mut item = serde_json::json!({
                "id": url,
                "url": url,
                "title": post.front_matter.title,
                "summary": post.front_matter.about,
                "content_html": post.body_html,
                "date_published": post.date.to_rfc3339(),
            });
            if let Some(image) = &post.front_matter.image {
                item["image"] = serde_json::json!(image);
            }
            if let Some(tags) = &post.front_matter.tags {
                item["tags"] = serde_json::json!(tags);
            }
            item
        })
        .collect();

    let feed = serde_json::json!({
        "version": "https://jsonfeed.org/version/1.1",
        "title": state.cfg.site_title,
        "home_page_url": format!("https://{}/", domain),
        "feed_url": format!("https://{}/blog.json", domain),
        "description": author.job_title,
        "authors": [{
            "name": author.name,
            "url": format!("https://github.com/{}", author.github),
        }],
        "language": "en",
        "items": items,
    });

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/feed+json")
        .header("ETag", ETAG.clone())
        .header("Last-Modified", &*LAST_MODIFIED)
        .body(body::boxed(body::Full::from(feed.to_string())))?)
}

/// The sitemap is built once at startup in `app::init`; this just serves it.
/// It was previously built and then never routed, so `/sitemap.xml` 404'd.
#[instrument(skip(state))]
pub async fn sitemap(Extension(state): Extension<Arc<State>>) -> Result<Response> {
    HIT_COUNTER.with_label_values(&["sitemap"]).inc();
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/xml")
        .header("ETag", ETAG.clone())
        .header("Last-Modified", &*LAST_MODIFIED)
        .body(body::boxed(body::Full::from(state.sitemap.clone())))?)
}

#[instrument(skip(state))]
pub async fn atom(Extension(state): Extension<Arc<State>>) -> Result<Response> {
    HIT_COUNTER.with_label_values(&["atom"]).inc();
    let state = state.clone();
    let mut buf = Vec::new();
    templates::blog_atom_xml(&mut buf, state.everything.clone())?;
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/atom+xml")
        .header("ETag", ETAG.clone())
        .header("Last-Modified", &*LAST_MODIFIED)
        .body(body::boxed(body::Full::from(buf)))?)
}
