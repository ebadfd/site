use color_eyre::eyre::{Result, WrapErr};
use comrak::{format_html_with_plugins, parse_document, Arena, ComrakOptions, ComrakPlugins};
use lazy_static::lazy_static;
use lol_html::{element, html_content::ContentType};
use lol_html::{rewrite_str, RewriteStrSettings};
use syntax_highlighter::SyntectAdapter;

mod syntax_highlighter;

lazy_static! {
    static ref SYNTECT_ADAPTER: SyntectAdapter =
        SyntectAdapter::new("base16-mocha.dark", "code-highlight", true);
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("missing element attribute {0}")]
    MissingElementAttribute(String),
}

pub fn render(input: &str) -> Result<String> {
    let mut options = ComrakOptions::default();

    options.extension.autolink = true;
    options.extension.table = true;
    options.extension.description_lists = true;
    options.extension.superscript = true;
    options.extension.strikethrough = true;
    options.extension.footnotes = true;

    options.render.unsafe_ = true;

    let arena = Arena::new();
    let root = parse_document(&arena, input, &options);

    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&*SYNTECT_ADAPTER);

    let mut html = vec![];
    format_html_with_plugins(root, &options, &mut html, &plugins).unwrap();

    let html = String::from_utf8(html).wrap_err("invalid UTF-8")?;

    let html = rewrite_str(
        &html,
        RewriteStrSettings {
            element_content_handlers: vec![
                element!("yt-video", |el| {
                    let video_id = el
                        .get_attribute("id")
                        .ok_or(Error::MissingElementAttribute("id".to_string()))?;
                    el.replace(&site_templates::yt_video(video_id).0, ContentType::Html);
                    Ok(())
                }),
                element!("video", |el| {
                    let video_url = el
                        .get_attribute("url")
                        .ok_or(Error::MissingElementAttribute("url".to_string()))?;
                    el.replace(&site_templates::video(video_url).0, ContentType::Html);
                    Ok(())
                }), 
                element!("details", |el| {
                    let summary = el
                        .get_attribute("summary")
                        .ok_or(Error::MissingElementAttribute("summary".to_string()))?;

                    let details = el
                        .get_attribute("details")
                        .ok_or(Error::MissingElementAttribute("details".to_string()))?;

                    el.replace(&site_templates::details(summary, details).0, ContentType::Html);
                    Ok(())
                }),
                element!("carousel", |el| {
                    let data = el
                        .get_attribute("data-cell")
                        .ok_or(Error::MissingElementAttribute("data-cell".to_string()))?;

                    let image_list: Vec<String> = data
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();

                    el.replace(&site_templates::carousel(image_list).0, ContentType::Html);
                    Ok(())
                }),
            ],
            ..RewriteStrSettings::default()
        },
    )?;

    Ok(html)
}
