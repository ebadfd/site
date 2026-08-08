use color_eyre::eyre::Result;
use std::{fs, path::PathBuf, sync::Arc};

pub mod config;
pub use config::*;

use crate::post::Post;

pub struct State {
    pub cfg: Arc<Config>,
    pub blog: Vec<Post>,
    pub products: Vec<Post>,
    pub everything: Vec<Post>,
    pub sitemap: Vec<u8>,
}

pub async fn init(cfg: PathBuf) -> Result<State> {
    let toml_str = fs::read_to_string(cfg).unwrap();
    let res_cfg: Config = toml::from_str(&toml_str).unwrap();
    let cfg: Arc<Config> = Arc::new(res_cfg);

    // Load blog posts
    let blog = crate::post::load("blog", &cfg.domain).await?;

    // Load products (similar to blog)
    let products = crate::post::load("products", &cfg.domain).await?;

    let mut everything: Vec<Post> = vec![];

    {
        let blog = blog.clone();
        let products = products.clone();
        everything.extend(blog.iter().cloned());
        everything.extend(products.iter().cloned());
    };

    everything.sort();
    everything.reverse();

    // Unpublished (future dated) posts are already gone, `post::load` drops them.
    // This is only about how many recent items the feeds carry.
    let everything: Vec<Post> = everything.into_iter().take(5).collect();

    // Build sitemap
    let mut sm: Vec<u8> = vec![];
    let smw = sitemap::writer::SiteMapWriter::new(&mut sm);
    let mut urlwriter = smw.start_urlset()?;
    for url in [
        format!("https://{}/resume", cfg.domain),
        format!("https://{}/contact", cfg.domain),
        format!("https://{}/blog", cfg.domain),
        format!("https://{}/products", cfg.domain),
        format!("https://{}/", cfg.domain),
    ] {
        urlwriter.url(url)?;
    }

    for post in &blog {
        urlwriter.url(format!("https://{}/{}", cfg.domain, post.link))?;
    }

    for product in &products {
        urlwriter.url(format!("https://{}/{}", cfg.domain, product.link))?;
    }

    urlwriter.end()?;

    Ok(State {
        cfg,
        blog,
        products,
        everything,
        sitemap: sm,
    })
}
