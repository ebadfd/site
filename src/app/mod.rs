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
    // This is only about how many recent items the feeds carry. At 5 a reader
    // polling weekly could miss posts entirely.
    let everything: Vec<Post> = everything.into_iter().take(20).collect();

    // Build sitemap
    let mut sm: Vec<u8> = vec![];
    let smw = sitemap::writer::SiteMapWriter::new(&mut sm);
    let mut urlwriter = smw.start_urlset()?;
    // Only routes that actually exist: `/resume` used to be listed here but its
    // handler is `todo!()` and it is not registered in the router, so the
    // sitemap was advertising a 404 to crawlers.
    for url in [
        format!("https://{}/", cfg.domain),
        format!("https://{}/blog", cfg.domain),
        format!("https://{}/products", cfg.domain),
        format!("https://{}/contact", cfg.domain),
        format!("https://{}/stack", cfg.domain),
        format!("https://{}/privacy-policy", cfg.domain),
    ] {
        urlwriter.url(url)?;
    }

    // `lastmod` lets crawlers skip unchanged posts on a recrawl.
    for post in blog.iter().chain(products.iter()) {
        urlwriter.url(
            sitemap::structs::UrlEntry::builder()
                .loc(format!("https://{}/{}", cfg.domain, post.link))
                .lastmod(post.date),
        )?;
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
