use lazy_static::lazy_static;
use maud::{html, Markup, PreEscaped};

use crate::{
    app::Author,
    post::{schemaorg::Article, Post},
};

use super::{base, oob_title, PageFeatures};

lazy_static! {
    static ref CACHEBUSTER: String = uuid::Uuid::new_v4().to_string().replace('-', "");
}

fn post_metadata(post: &Post, author: &Author, domain: &str) -> Markup {
    let art = Article::new(post, author, domain);
    let json = PreEscaped(serde_json::to_string(&art).unwrap());

    html! {
        meta name="twitter:card" content="summary_large_image";
        meta name="twitter:site" content={(author.twitter)};
        meta name="twitter:title" content={(post.front_matter.title)};
        meta name="twitter:description" content={(post.front_matter.about)};
        meta name="twitter:image" content={(post.image_url(domain))};
        meta property="twitter:domain" content={(domain)};
        meta property="twitter:url" content={(format!("https://{}/{}", domain, post.link))};

        // `article`, not `website` — this is what tells crawlers and social
        // cards that the page is a dated piece of writing rather than a landing
        // page, and it is what unlocks the article:* properties below.
        meta property="og:type" content="article";
        meta property="og:title" content={(post.front_matter.title)};
        meta property="og:description" content={(post.front_matter.about)};
        meta property="og:image" content={(post.image_url(domain))};
        meta property="og:image:alt" content={(post.front_matter.title)};
        meta property="og:site_name" content=(super::SITE_TITLE);
        meta property="og:url" content={(format!("https://{}/{}", domain, post.link))};
        meta property="og:locale" content="en_US";

        meta property="article:published_time" content={(post.date.to_rfc3339())};
        meta property="article:author" content={(author.name)};
        @if let Some(tags) = &post.front_matter.tags {
            @for tag in tags {
                meta property="article:tag" content=(tag);
            }
        }

        meta name="description" content={(post.front_matter.about)};
        meta name="author" content={(author.name)};

        @if let Some(redirect_to) = &post.front_matter.redirect_to {
            link rel="canonical" href=(redirect_to);
            meta http-equiv="refresh" content=(format!("0;URL='{redirect_to}'"));
        } @else {
            link rel="canonical" href={(format!("https://{}/{}", domain, post.link))};
        }

        script type="application/ld+json" {(json)}
    }
}

pub fn post_index(posts: &[Post], title: &str, show_extra: bool, current_path: &str, is_partial: bool) -> Markup {
    fn post_url(post: &Post) -> String {
        if let Some(redirect_to) = &post.front_matter.redirect_to {
            redirect_to.clone()
        } else {
            "/".to_string() + &post.link
        }
    }

    let markup = html! {
        .content {
            h1 { (title) }
            @if show_extra {
                p {
                    "If you have a compatible reader, be sure to check out the "
                    a href="/blog.rss" { "RSS feed" }
                    " for automatic updates.
                        Also check out the "
                    a href="/blog.json" { "JSONFeed" }
                    "."
                }
            }
            p {
                ul hx-boost="true" hx-swap="innerHTML" hx-target=".snowframe" hx-include="[name='bustCache']"{
                    @for post in posts {
                        li {
                            (post.detri())
                            " - "
                                a href={(post_url(post))} hx-push-url={(post_url(post))} { (post.front_matter.title) }
                            @if !post.front_matter.about.is_empty() {
                                br;
                                small { (post.front_matter.about) }
                                " · "
                                small { (post.read_time_estimate_minutes) " min read" }
                            }
                        }
                    }
                }
            }
        }
    };
    if is_partial {
        html! { (oob_title(Some(title))) (markup) }
    } else {
        base(Some(title), None, None, current_path, PageFeatures::default(), markup)
    }
}

pub fn post(
    post: &Post,
    body: PreEscaped<&String>,
    author: &Author,
    domain: &str,
    is_partial: bool,
) -> Markup {
    let markup = html! {
         article {
            h1 {(post.front_matter.title)}

            @if post.front_matter.image.is_some() {
                img src={(post.image_url(domain))} alt="" {}
            }

             // (nag::prerelease(post))

             div."post-meta" {
                 small {
                     (post.read_time_estimate_minutes) " minute read,  Published: " (post.detri_withmonth())
                 }
                 button."btn-export-pdf" onclick="window.print()" title="Print or Save as PDF" {
                     i."fa-solid"."fa-file-pdf" {}
                     " Save as PDF"
                 }
             }

             div {
                 (body)
             }
         }

         hr;

         //(share_button(post))
         //(twitch_vod(post))

         p { (post.detri_withmonth()) }

         @if let Some(tags) = &post.front_matter.tags {
            p {
                "Tags: "
                    @for tag in tags {
                        code {(tag)}
                        " "
                    }
            }
        }
    };
    if is_partial {
        html! { (oob_title(Some(&post.front_matter.title))) (markup) }
    } else {
        // The post's own path, not "/blog" — `nav_state` needs it to tell
        // "you are on the blog index" from "you are inside the blog section".
        // `post.link` is already section-prefixed, e.g. `blog/some-slug`.
        base(
            Some(&post.front_matter.title),
            None,
            Some(post_metadata(post, author, domain)),
            &format!("/{}", post.link),
            PageFeatures { flickity: true, fontawesome: true, ..Default::default() },
            markup,
        )
    }
}
