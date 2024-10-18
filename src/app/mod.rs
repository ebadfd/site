use chrono::prelude::*;
use color_eyre::eyre::Result;
use std::{fs, path::PathBuf, sync::Arc};
use termx::registries::{CommandRegistry, FS};

pub mod config;
pub use config::*;

use crate::{post::Post, termx_registry::web::WebCommandRegistry};

pub struct State {
    pub cfg: Arc<Config>,
    pub blog: Vec<Post>,
    pub everything: Vec<Post>,
    pub sitemap: Vec<u8>,
    pub fs: Vec<FS>,
}

pub async fn init(cfg: PathBuf) -> Result<State> {
    let toml_str = fs::read_to_string(cfg).unwrap();
    let res_cfg: Config = toml::from_str(&toml_str).unwrap();
    let cfg: Arc<Config> = Arc::new(res_cfg);
    let blog = crate::post::load("blog", &cfg.domain).await?;
    let mut fs = vec![];

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
    for url in [
        format!("https://{}/resume", cfg.domain),
        format!("https://{}/contact", cfg.domain),
        format!("https://{}/blog", cfg.domain),
        format!("https://{}/", cfg.domain),
    ] {
        urlwriter.url(url)?;
    }
    let user = WebCommandRegistry::get_user_name();

    for post in &blog {
        fs.push(FS {
            name: post.link.clone(),
            created_at: post.detri(),
            owner: user.clone(),
            group: user.clone(),
            summary: post.body_html.clone(),
            title: post.summery.title.clone(),
        });
        urlwriter.url(format!("https://{}/{}", cfg.domain, post.link))?;
    }

    urlwriter.end()?;

    Ok(State {
        cfg,
        blog,
        everything,
        sitemap: sm,
        fs,
    })
}
