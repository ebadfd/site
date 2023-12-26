use chrono::prelude::*;
use lazy_static::lazy_static;
use maud::{html, Markup, PreEscaped};

use crate::post::Post;

use super::base;

lazy_static! {
    static ref CACHEBUSTER: String = uuid::Uuid::new_v4().to_string().replace('-', "");
}

fn post_metadata(post: &Post) -> Markup {
    html! {
        meta name="twitter:card" content="summary";
        meta name="twitter:site" content="@theprincessxena";
        meta name="twitter:title" content={(post.front_matter.title)};
        meta property="og:type" content="website";
        meta property="og:title" content={(post.front_matter.title)};
        meta property="og:site_name" content="Xe's Blog";
        meta name="description" content={(post.front_matter.title) " - Xe's Blog"};
        meta name="author" content="Xe Iaso";

        @if let Some(redirect_to) = &post.front_matter.redirect_to {
            link rel="canonical" href=(redirect_to);
            meta http-equiv="refresh" content=(format!("0;URL='{redirect_to}'"));
        } @else {
            link rel="canonical" href={"https://xeiaso.net/" (post.link)};
        }

        //script type="application/ld+json" {(json)}
    }
}

pub fn post_index(posts: &[Post], title: &str, show_extra: bool) -> Markup {
    let today = Utc::now().date_naive();
    base(
        Some(title),
        None,
        html! {
            h1 { (title) }
            @if show_extra {
                p {
                    "If you have a compatible reader, be sure to check out my "
                    a href="/blog.rss" { "RSS feed" }
                    " for automatic updates. Also check out the "
                    a href="/blog.json" { "JSONFeed" }
                    "."
                }
                p {
                    "For a breakdown by post series, see "
                    a href="/blog/series" { "here" }
                    "."
                }
            }
            p {
                ul {
                    @for post in posts.iter().filter(|p| today.num_days_from_ce() >= p.date.num_days_from_ce()) {
                        li {
                            (post.detri())
                            " - "
                                a href={ @if post.front_matter.redirect_to.as_ref().is_some() {(post.front_matter.redirect_to.as_ref().unwrap())} @else {"/" (post.link)}} { (post.front_matter.title) }
                        }
                    }
                }
            }
        },
    )
}

pub fn post(post: &Post, body: PreEscaped<&String>) -> Markup {
    base(
        Some(&post.front_matter.title),
        None,
        html! {
            (post_metadata(post))
            article {
                h1 {(post.front_matter.title)}

                // (nag::prerelease(post))

                small {
                    "Published on " (post.detri()) ", " (post.read_time_estimate_minutes) " minutes to read"
                }

                div {
                    (body)
                }
            }

            hr;

            //(share_button(post))
            //(twitch_vod(post))

            p {
                "This article was posted on "
                (post.detri())
                ". Facts and circumstances may have changed since publication. Please "
                a href="/contact" {"contact me"}
                " before jumping to conclusions if something seems wrong or unclear."
            }

            @if let Some(tags) = &post.front_matter.tags {
               p {
                   "Tags: "
                    @for tag in tags {
                        code {(tag)}
                        " "
                    }
               }
            }
        },
    )
}
