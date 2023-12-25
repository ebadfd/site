use chrono::prelude::*;
use color_eyre::eyre::Result;
use std::{fs, path::PathBuf, sync::Arc};

pub mod config;
pub use config::*;

use crate::post::Post;

pub struct State {
    pub cfg: Arc<Config>,
    pub blog: Vec<Post>,
    pub everything: Vec<Post>,
    pub sitemap: Vec<u8>,
}

pub async fn init(cfg: PathBuf) -> Result<State> {
    let toml_str = fs::read_to_string(cfg).unwrap();
    let res_cfg: Config = toml::from_str(&toml_str).unwrap();
    let cfg: Arc<Config> = Arc::new(res_cfg);
    let blog = crate::post::load("blog").await?;

    let mut everything: Vec<Post> = vec![];

    {
        let blog = blog.clone();
        everything.extend(blog.iter().cloned());
    };

    everything.sort();
    everything.reverse();

    let today = Utc::now().date_naive();
    let everything: Vec<Post> = everything
        .into_iter()
        .filter(|p| today.num_days_from_ce() >= p.date.num_days_from_ce())
        .take(5)
        .collect();

    let mut sm: Vec<u8> = vec![];
    let smw = sitemap::writer::SiteMapWriter::new(&mut sm);
    let mut urlwriter = smw.start_urlset()?;
    for url in &[
        "https://xeiaso.net/resume",
        "https://xeiaso.net/contact",
        "https://xeiaso.net/",
        "https://xeiaso.net/blog",
        "https://xeiaso.net/signalboost",
    ] {
        urlwriter.url(*url)?;
    }

    for post in &blog {
        urlwriter.url(format!("https://xeiaso.net/{}", post.link))?;
    }

    urlwriter.end()?;

    Ok(State {
        cfg,
        blog,
        everything,
        sitemap: sm,
    })
}
