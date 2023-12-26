use chrono::prelude::*;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};

use crate::{
    app::{Author, Link},
    post::Post,
};
use lazy_static::lazy_static;

pub mod blog;

lazy_static! {
    static ref CACHEBUSTER: String = uuid::Uuid::new_v4().to_string().replace('-', "");
}

pub fn error(why: impl Render) -> Markup {
    base(
        Some("Error"),
        None,
        html! {
            h1 {"Error"}

            pre {
                (why)
            }

            p {
                "You could try to "
                a href="/" {"go home"}
                " or "
                a href="https://github.com/z9fr/site/issues/new" {"report this issue"}
                " so it can be fixed."
            }
        },
    )
}

pub fn index(author: &Author, projects: &Vec<Link>, posts: &Vec<Post>) -> Markup {
    let today = Utc::now().date_naive();
    base(
        None,
        None,
        html! {
            link rel="authorization_endpoint" href="https://idp.christine.website/auth";
            link rel="canonical" href="https://xeiaso.net/";

            meta name="twitter:card" content="summary";
            meta name="twitter:site" content=(author.twitter);
            meta name="twitter:title" content=(author.name);
            meta name="twitter:description" content=(author.job_title);
            meta property="og:type" content="website";
            meta property="og:title" content=(author.name);
            meta property="og:site_name" content=(author.job_title);
            meta name="description" content=(author.job_title);
            meta name="author" content=(author.name);

            .content {
                h1 {(author.name)}

                p { "I'm Devin Schulz, front-end developer with a decade's worth of pixels and code under my belt, now crafting digital experiences at Cape Privacy. Remote work veteran since 2014, minimalist in progress, dad of two, and fuelled by a never-ending stream of coffee—because what's code without a little caffeine humour "}

                h4 { "Skills" }
                ul {
                    li { "Go, Lua, Haskell, C, Rust and other languages" }
                    li { "Docker (deployment, development & more)" }
                    li { "Mashups of data" }
                    li { "kastermakfa" }
                }

                h4 { "Recent Articles" }

                ul preload{
                    @for post in posts.iter().filter(|p| today.num_days_from_ce() >= p.date.num_days_from_ce()) {
                        li {
                            (post.detri())
                                " - "
                                a href={ @if post.front_matter.redirect_to.as_ref().is_some() {(post.front_matter.redirect_to.as_ref().unwrap())} @else {"/" (post.link)}} { (post.front_matter.title) }
                            }
                        }
                }

                h4 { "Quick Links" }
                ul {
                    li {a href={"https://github.com/" (author.github)} rel="me" {"GitHub"}}
                    li {a href={"https://twitter.com/" (author.twitter)} rel="me" {"Twitter"}}
                }
            }
        },
    )
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
                        " - z9fr blog"
                    } @else {
                        "z9fr blog"
                    }
                }
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="msapplication-TileColor" content="#ffffff";
                meta name="msapplication-TileImage" content="/static/favicon/ms-icon-144x144.png";
                meta name="theme-color" content="#ffffff";

                link rel="manifest" href="/static/manifest.json";
                link rel="alternate" title="Xe's Blog" type="application/rss+xml" href="https://xeiaso.net/blog.rss";
                link rel="alternate" title="Xe's Blog" type="application/json" href="https://xeiaso.net/blog.json";

                link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/hack/0.8.1/hack.css";
                link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/hack/0.8.1/dark-grey.css";
                link rel="stylesheet" href={"/static/css/styles.css?bustCache=" (*CACHEBUSTER)};

                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="manifest" href="/static/manifest.json";

                script src="https://unpkg.com/htmx.org@1.9.10" integrity={"sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC"} crossorigin={"anonymous"} {}
                script src="https://unpkg.com/htmx.org/dist/ext/preload.js" {};

                @match now.month() {
                   //12|1|2 => {
                   //    link rel="stylesheet" href={"/static/css/snow.css?bustCache=" (*CACHEBUSTER)};
                  // }
                   _ => {},
                }

                @if let Some(styles) = styles {
                    style {
                        (PreEscaped(styles))
                    }
                }
            }
            body.snow.hack.dark-grey hx-ext="preload" {
                .container {
                    br;

                    header {
                        span.logo {}
                        nav {
                            a.logo href="/" { "z9fr." }
                        }
                    }

                    br;
                    br;

                    .snowframe {
                        (content)
                    }
                    hr;
                    footer {
                        nav {
                            a href="/" preload{ "Home" }
                            " - "
                            a href="/blog" preload{ "Blog" }
                            " - "
                            a href="/contact" preload{ "Contact" }
                            " - "
                            a href="/resume" preload{ "Resume" }
                        }

                        blockquote {
                            small {
                                "copy right " (now.year())
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn not_found(path: impl Render) -> Markup {
    base(
        Some("Not found"),
        None,
        html! {
            h1 {"Not found"}
            p {
                "The path at "
                code {(path)}
                " could not be found. If you expected this path to exist, please "
                a href="https://github.com/z9fr/site/issues/new" {"report this issue"}
                " so it can be fixed."
            }
        },
    )
}
