pub mod frontmatter;
use chrono::prelude::*;
use color_eyre::eyre::{eyre, Result, WrapErr};
use glob::glob;
use log::info;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, path::PathBuf};
use tokio::fs;

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub front_matter: frontmatter::Data,
    pub link: String,
    pub body_html: String,
    pub date: DateTime<FixedOffset>,
    pub read_time_estimate_minutes: u64,
    pub summery: PostSummery,
}

impl Ord for Post {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Post {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.date.cmp(&other.date))
    }
}

impl Post {
    pub fn detri(&self) -> String {
        self.date.format("%Y-%m-%d").to_string()
    }

    pub fn detri_withmonth(&self) -> String {
        self.date.format("%B %e, %Y").to_string()
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct PostSummery {
    pub title: String,
    pub summary: String,
    pub link: String,
}

async fn read_post(dir: &str, fname: PathBuf) -> Result<Post> {
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

    let date: DateTime<FixedOffset> = DateTime::<Utc>::from_utc(
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
        link: format!("https://xeiaso.net/{}", link),
    };

    Ok(Post {
        front_matter,
        link,
        body_html,
        summery,
        date,
        read_time_estimate_minutes,
    })
}

pub async fn load(dir: &str) -> Result<Vec<Post>> {
    let result_futs = glob(&format!("{}/*.markdown", dir))?
        .filter_map(Result::ok)
        .map(|fname| read_post(dir, fname));

    let mut result: Vec<Post> = futures::future::join_all(result_futs)
        .await
        .into_iter()
        .map(Result::unwrap)
        .collect();

    if result.is_empty() {
        return Err(eyre!("no posts avaible"));
    }

    result.sort();
    result.reverse();

    Ok(result)
}
