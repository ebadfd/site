use chrono::prelude::*;
use lazy_static::lazy_static;
use maud::{html, Markup, PreEscaped};

use crate::{
    app::Author,
    post::{schemaorg::Article, Post},
};

use super::base;

lazy_static! {
    static ref CACHEBUSTER: String = uuid::Uuid::new_v4().to_string().replace('-', "");
}

fn post_metadata(post: &Post, author: &Author, domain: &str) -> Markup {
    let art: Article = post.into();
    let json = PreEscaped(serde_json::to_string(&art).unwrap());

    html! {
        meta name="twitter:card" content="summary_large_image";
        meta name="twitter:site" content={(author.twitter)};
        meta name="twitter:title" content={(post.front_matter.title)};
        meta name="twitter:description" content={(post.front_matter.about)};
        meta name="twitter:image" content={(post.image_url(domain))};
        meta property="twitter:domain" content={(domain)};
        meta property="twitter:url" content={(format!("https://{}/{}", domain, post.link))};

        meta property="og:type" content="website";
        meta property="og:title" content={(post.front_matter.title)};
        meta property="og:description" content={(post.front_matter.about)};
        meta property="og:image" content={(post.image_url(domain))};
        meta property="og:site_name" content="z9fr blog";

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

pub fn post_index(posts: &[Post], title: &str, show_extra: bool, is_partial: bool) -> Markup {
    let today = Utc::now().date_naive();

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
                    @for post in posts.iter().filter(|p| today.num_days_from_ce() >= p.date.num_days_from_ce()) {
                        li {
                            (post.detri())
                            " - "
                                a href={(post_url(post))} hx-push-url={(post_url(post))} { (post.front_matter.title) }
                        }
                    }
                }
            }
        }
    };
    return if is_partial {
        markup
    } else {
        base(Some(title), None, markup)
    };
}

pub fn post(
    post: &Post,
    body: PreEscaped<&String>,
    author: &Author,
    domain: &str,
    is_partial: bool,
) -> Markup {
    let markup = html! {
        (post_metadata(post, author, domain))
         article {
            h1 class="baffle" {(post.front_matter.title)}

            img src={(post.image_url(domain))} alt={(post.front_matter.title)} {}

             // (nag::prerelease(post))

             small {
                 (post.read_time_estimate_minutes) " minute read,  Published: " (post.detri_withmonth())
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
    return if is_partial {
        markup
    } else {
        base(Some(&post.front_matter.title), None, markup)
    };
}
