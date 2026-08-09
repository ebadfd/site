use chrono::prelude::*;
use color_eyre::eyre::{eyre, Result, WrapErr};
use glob::glob;
use log::info;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, path::PathBuf};
use tokio::fs;

pub mod frontmatter;
pub mod schemaorg;

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub front_matter: frontmatter::Data,
    pub link: String,
    pub body_html: String,
    /// The markdown source, frontmatter stripped. Served to the v86 terminal, which writes it
    /// into the guest filesystem as `/root/blog/<slug>.md`.
    pub body_md: String,
    pub date: DateTime<FixedOffset>,
    pub read_time_estimate_minutes: u64,
    pub summery: PostSummery,
}

impl Ord for Post {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for Post {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Post {
    pub fn detri(&self) -> String {
        self.date.format("%Y-%m-%d").to_string()
    }

    pub fn detri_withmonth(&self) -> String {
        self.date.format("%B %e, %Y").to_string()
    }

    /// Section this post lives under — `blog` or `products`. `link` is built as
    /// `<dir>/<slug>` in `read_post`.
    pub fn section(&self) -> &str {
        self.link.split('/').next().unwrap_or("blog")
    }

    /// What goes in `<title>`: the frontmatter override if there is one, else
    /// the display title.
    pub fn seo_title(&self) -> &str {
        self.front_matter
            .seo_title
            .as_deref()
            .unwrap_or(&self.front_matter.title)
    }

    pub fn image_url(&self, domain: &str) -> String {
        self
            .front_matter
            .image
            .clone()
            .unwrap_or_else(|| format!("https://{}/static/img/avatar.jpeg", domain))
    }
}

impl schemaorg::Article {
    /// Build the JSON-LD for a post. Takes the author and domain rather than
    /// implementing `From<&Post>`, because both were previously hardcoded to
    /// `ebadfd.tech` and drifted from the configured domain.
    pub fn new(post: &Post, author: &crate::app::Author, domain: &str) -> Self {
        let url = format!("https://{}/{}", domain, post.link);
        let person = schemaorg::Person {
            r#type: "Person".to_string(),
            name: author.name.clone(),
            same_as: author.same_as.clone(),
        };

        schemaorg::Article {
            context: "https://schema.org".to_string(),
            r#type: "Article".to_string(),
            main_entity_of_page: schemaorg::MainEntity {
                r#type: "WebPage".to_string(),
                id: url.clone(),
            },
            headline: post.front_matter.title.clone(),
            description: post.front_matter.about.clone(),
            image: post.image_url(domain),
            url,
            author: person.clone(),
            publisher: person,
            date_published: post.date.to_rfc3339(),
            // No edit history is tracked, so the publication date is the most
            // accurate value available.
            date_modified: post.date.to_rfc3339(),
            keywords: post
                .front_matter
                .tags
                .as_ref()
                .filter(|t| !t.is_empty())
                .map(|t| t.join(", ")),
        }
    }
}

impl schemaorg::SoftwareApplication {
    pub fn new(post: &Post, author: &crate::app::Author, domain: &str) -> Self {
        schemaorg::SoftwareApplication {
            context: "https://schema.org".to_string(),
            r#type: "SoftwareApplication".to_string(),
            name: post.front_matter.title.clone(),
            description: post.front_matter.about.clone(),
            url: format!("https://{}/{}", domain, post.link),
            image: post.image_url(domain),
            application_category: "BrowserApplication".to_string(),
            author: schemaorg::Person {
                r#type: "Person".to_string(),
                name: author.name.clone(),
                same_as: author.same_as.clone(),
            },
            date_published: post.date.to_rfc3339(),
        }
    }
}

impl schemaorg::Breadcrumbs {
    /// Home → section index → this post.
    pub fn for_post(post: &Post, domain: &str) -> Self {
        let section = post.section();
        let crumb = |position, name: &str, path: &str| schemaorg::Crumb {
            r#type: "ListItem".to_string(),
            position,
            name: name.to_string(),
            item: format!("https://{domain}{path}"),
        };

        schemaorg::Breadcrumbs {
            context: "https://schema.org".to_string(),
            r#type: "BreadcrumbList".to_string(),
            items: vec![
                crumb(1, "Home", "/"),
                crumb(
                    2,
                    if section == "products" {
                        "Products"
                    } else {
                        "Blog"
                    },
                    &format!("/{section}"),
                ),
                crumb(3, &post.front_matter.title, &format!("/{}", post.link)),
            ],
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct PostSummery {
    pub title: String,
    pub summary: String,
    pub link: String,
}

async fn read_post(dir: &str, fname: PathBuf, domain: &str) -> Result<Post> {
    let file_name_as_str = fname.clone().into_os_string().into_string().unwrap();
    info!("Loading file {}", file_name_as_str);

    let body = fs::read_to_string(fname.clone())
        .await
        .wrap_err_with(|| format!("failed to read the file {:?}", fname))?;

    let (front_matter, content_offset) = frontmatter::parse(body.clone().as_str())
        .wrap_err_with(|| format!("failed to parse frontmatter of {:?}", fname))?;

    let body = &body[content_offset..];

    let date = NaiveDate::parse_from_str(&front_matter.clone().date, "%Y-%m-%d")
        .map_err(|why| eyre!("error parsing date in {:?}: {}", fname, why))?;

    let link = format!("{}/{}", dir, fname.file_stem().unwrap().to_str().unwrap());

    let body_html = markdown_render::render(body)
        .wrap_err_with(|| format!("can't parse markdown for {:?}", fname))?;

    let date: DateTime<FixedOffset> = DateTime::<Utc>::from_naive_utc_and_offset(
        NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
        Utc,
    )
    .with_timezone(&Utc)
    .into();

    let time_taken = estimated_read_time::text(
        body,
        &estimated_read_time::Options::new()
            .technical_document(true)
            .technical_difficulty(1)
            .build()
            .unwrap_or_default(),
    );
    let read_time_estimate_minutes = time_taken.seconds() / 60;

    let summery = PostSummery {
        title: front_matter.title.clone(),
        summary: format!("{} minute read", read_time_estimate_minutes),
        link: format!("https://{}/{}", domain, link),
    };

    Ok(Post {
        front_matter,
        link,
        body_html,
        body_md: body.to_string(),
        summery,
        date,
        read_time_estimate_minutes,
    })
}

pub async fn load(dir: &str, domain: &str) -> Result<Vec<Post>> {
    let result_futs = glob(&format!("{}/*.markdown", dir))?
        .filter_map(Result::ok)
        .map(|fname| read_post(dir, fname, domain));

    let mut result: Vec<Post> = futures::future::join_all(result_futs)
        .await
        .into_iter()
        .map(Result::unwrap)
        .collect();

    if result.is_empty() {
        return Err(eyre!("no posts avaible"));
    }

    // A post's frontmatter date is its publication date, so anything dated in the
    // future is unpublished. Dropping it here is the single choke point: it keeps
    // future posts out of the index, the feeds, the sitemap, direct URLs and the
    // v86 terminal's filesystem in one go.
    //
    // Note this is evaluated once, at startup — a post dated tomorrow goes live on
    // the next restart, not at midnight.
    let now = Utc::now();
    result.retain(|post| {
        let published = post.date.with_timezone(&Utc) <= now;
        if !published {
            info!("Skipping unpublished post {} ({})", post.link, post.detri());
        }
        published
    });

    result.sort();
    result.reverse();

    Ok(result)
}
