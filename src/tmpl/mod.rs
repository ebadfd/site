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

pub fn index(author: &Author, posts: &Vec<Post>, domain: &str) -> Markup {
    let today = Utc::now().date_naive();
    base(
        None,
        None,
        html! {
            link rel="canonical" href={"https://"(domain)"/"};

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
                p {"I'm Dasith, Security Researcher and Hobbyist Programmer"}

                p {"Software enginer at Surge.global. My main interests revolve around Computers, History, Philosophy and Anime."}

                h4 { "Recent Articles" }

                ul preload{
                    @for post in posts.iter().take(5).filter(|p| today.num_days_from_ce() >= p.date.num_days_from_ce()) {
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
                link rel="alternate" title="z9fr blog" type="application/rss+xml" href={"https://z9fr.xyz/blog.rss"};
                link rel="alternate" title="z9fr blog" type="application/json" href={"https://z9fr.xyz/blog.json"};

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
                        nav {
                            a.logo href="/" { "> z9fr@blog:~$" }
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
                            a href="/stack" preload{ "Uses" }
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

pub fn contact(links: &Vec<Link>) -> Markup {
    base(
        Some("Contact Information"),
        None,
        html! {
            h1 {"Contact Information"}

            br;
            br;

            .grid {
                .cell."-6of12" {
                    h3 {"Email"}
                    a href={"mailto:z9fr@protonmail.com"} {"z9fr@protonmail.com"}
                    br;
                    br;

                    h3 {"Other useful links:"}
                    ul {
                        @for link in links {
                            li {
                                a target="_blank" href=(link.url) {
                                    (link.title)
                                }
                            }
                        }
                    }
                }
                .cell."-6of12" {
                    h3 {"Discord"}
                    p {
                        code {"Cadey~#1337"}
                        " Please note that Discord will automatically reject friend requests if you are not in a mutual server with me. I don't have control over this behavior."
                    }
                }
            }
        },
    )
}

pub fn stack() -> Markup {
    base(
        Some("Uses"),
        None,
        html! {
            h1 {"Uses"}

            ul {
                li {
                    "Built on " a href={"https://github.com/tokio-rs/axum"} {"axum"}
                }

                li {
                    a href={"https://tokio.rs/"} {"tokio.rs"} " as the asynchronous runtime."
                }

                li {
                    a href="https://hackcss.egoist.dev" {"hackcss"}; " as the css framework"
                }

                li {
                    "Markdown rendering with " a href="https://docs.rs/comrak" {"cmark"};"."
                }

                li {
                    a href="https://docs.rs/syntect" {"Syntect"}; " for Syntax Highlighting."
                }

                li {
                    "Inspired by " a href="https://github.com/Xe/site" {"Xe/site"}; "."
                }
            }
        },
    )
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
