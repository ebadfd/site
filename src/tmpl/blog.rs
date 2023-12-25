use chrono::prelude::*;
use lazy_static::lazy_static;
use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::post::Post;

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

pub fn base(title: Option<&str>, styles: Option<&str>, content: Markup) -> Markup {
    let now = Utc::now();
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                title {
                    @if let Some(title) = title {
                        (title)
                        " - Xe Iaso"
                    } @else {
                        "Xe Iaso"
                    }
                }
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="manifest" href="/static/manifest.json";
                link rel="alternate" title="Xe's Blog" type="application/rss+xml" href="https://xeiaso.net/blog.rss";
                link rel="alternate" title="Xe's Blog" type="application/json" href="https://xeiaso.net/blog.json";
                link rel="apple-touch-icon" sizes="57x57" href="/static/favicon/apple-icon-57x57.png";
                link rel="apple-touch-icon" sizes="60x60" href="/static/favicon/apple-icon-60x60.png";
                link rel="apple-touch-icon" sizes="72x72" href="/static/favicon/apple-icon-72x72.png";
                link rel="apple-touch-icon" sizes="76x76" href="/static/favicon/apple-icon-76x76.png";
                link rel="apple-touch-icon" sizes="114x114" href="/static/favicon/apple-icon-114x114.png";
                link rel="apple-touch-icon" sizes="120x120" href="/static/favicon/apple-icon-120x120.png";
                link rel="apple-touch-icon" sizes="144x144" href="/static/favicon/apple-icon-144x144.png";
                link rel="apple-touch-icon" sizes="152x152" href="/static/favicon/apple-icon-152x152.png";
                link rel="apple-touch-icon" sizes="180x180" href="/static/favicon/apple-icon-180x180.png";
                link rel="icon" type="image/png" sizes="192x192" href="/static/favicon/android-icon-192x192.png";
                link rel="icon" type="image/png" sizes="32x32" href="/static/favicon/favicon-32x32.png";
                link rel="icon" type="image/png" sizes="32x32" href="/static/favicon/favicon-32x32.png";
                link rel="icon" type="image/png" sizes="96x96" href="/static/favicon/favicon-96x96.png";
                link rel="icon" type="image/png" sizes="16x16" href="/static/favicon/favicon-16x16.png";


                link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/hack/0.8.1/hack.css";
                link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/hack/0.8.1/dark-grey.css";
                @match now.month() {
                    //12|1|2 => {
                    //    link rel="stylesheet" href={"/static/css/snow.css?bustCache=" (*CACHEBUSTER)};
                   // }
                    _ => {},
                }

                meta name="msapplication-TileColor" content="#ffffff";
                meta name="msapplication-TileImage" content="/static/favicon/ms-icon-144x144.png";
                meta name="theme-color" content="#ffffff";
                link href="https://mi.within.website/api/webmention/accept" rel="webmention";
            }
            body.snow.hack.dark-grey {
                .container {
                    header {
                        span.logo {}
                        nav {
                            a href="/" { "Xe" }
                            " - "
                            a href="/blog" { "Blog" }
                            " - "
                            a href="/contact" { "Contact" }
                            " - "
                            a href="/resume" { "Resume" }
                            " - "
                            a href="/talks" { "Talks" }
                            " - "
                            a href="/signalboost" { "Signal Boost" }
                            " - "
                            a href="/vods" { "VODs" }
                            " | "
                            a target="_blank" rel="noopener noreferrer" href="https://graphviz.christine.website" { "Graphviz" }
                            " - "
                            a target="_blank" rel="noopener noreferrer" href="https://when-then-zen.christine.website/" { "When Then Zen" }
                        }
                    }

                    br;
                    br;

                    .snowframe {
                        (content)
                    }
                    hr;
                    footer {
                        blockquote {
                            "Copyright 2012-"
                            (now.year())
                            " Xe Iaso (Christine Dodrill). Any and all opinions listed here are my own and not representative of my employers; future, past and present."
                        }
                        p {
                            "Like what you see? Donate on "
                            a href="https://www.patreon.com/cadey" { "Patreon" }
                            " like "
                            a href="/patrons" { "these awesome people" }
                            "!"
                        }
                        p {
                            "Looking for someone for your team? Take a look "
                            a href="/signalboost" { "here" }
                            "."
                        }
                        p {
                            "See my salary transparency data "
                            a href="/salary-transparency" {"here"}
                            "."
                        }
                        p {
                            "Served by Hello"
                            "/bin/xesite, see "
                            a href="https://github.com/Xe/site" { "source code here" }
                            "."
                        }
                    }
                    script src="/static/js/installsw.js" defer {}
                }
            }
        }
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
                    "Read time in minutes: "
                    (post.read_time_estimate_minutes)
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

            @if let Some(series) = &post.front_matter.series {
                p {
                    "Series: "
                    a href={"/blog/series/" (series)} {(series)}
                }
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


            p {
                "The art for Mara was drawn by "
                a href="https://selic.re/" {"Selicre"}
                "."
            }

            p {
                "The art for Cadey was drawn by "
                a href="https://artzorastudios.weebly.com/" {"ArtZora Studios"}
                "."
            }

            p {
                "Some of the art for Aoi was drawn by "
                a href="https://twitter.com/Sandra_Thomas01" {"@Sandra_Thomas01"}
                "."
            }
        },
    )
}
