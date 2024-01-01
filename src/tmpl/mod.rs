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

pub fn index(author: &Author, posts: &Vec<Post>, domain: &str, is_partial: bool) -> Markup {
    let today = Utc::now().date_naive();
    let markup = html! {
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
    };

    return if is_partial {
        markup
    } else {
        base(None, None, markup)
    };
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
                meta name="msapplication-config" content="/static/favicon/browserconfig.xml";
                meta name="theme-color" content="#181818";

                meta name="apple-mobile-web-app-title" content="z9fr blog";
                meta name="application-name" content="z9fr blog";

                link rel="manifest" href="/static/manifest.json";
                link rel="alternate" title="z9fr blog" type="application/rss+xml" href={"https://z9fr.xyz/blog.rss"};
                link rel="alternate" title="z9fr blog" type="application/json" href={"https://z9fr.xyz/blog.json"};

                link rel="apple-touch-icon" sizes="180x180" href="/static/favicon/apple-touch-icon.png";
                link rel="mask-icon" href="/static/favicon/safari-pinned-tab.svg" color="#181818";
                link rel="shortcut icon" href="/static/favicon/favicon.ico";

                link rel="icon" type="image/png" sizes="32x32" href="/static/favicon/favicon-32x32.png";
                link rel="icon" type="image/png" sizes="16x16" href="/static/favicon/favicon-16x16.png";

                // link rel="icon" href="/static/favicon/favicon.svg" type="image/svg+xml";
                link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/hack/0.8.1/hack.css";
                link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/hack/0.8.1/dark-grey.css";

                link rel="stylesheet" href={"/static/css/styles.css?bustCache=" (*CACHEBUSTER)};
                link rel="stylesheet" href={"/static/css/progress-bar.css?bustCache=" (*CACHEBUSTER)};

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

            div.progress style="height: 2px;"{
                div.indeterminate style="background-color: #ff2e88;"{}
            }

            input name="bustCache" value={(*CACHEBUSTER)} type="hidden" {}

            body.snow.hack.dark-grey hx-ext="preload" hx-indicator=".progress" {
                .container {
                    br;

                    header {
                        nav {
                            div hx-boost="true" hx-swap="innerHTML" hx-target=".snowframe" hx-include="[name='bustCache']" {
                                a.logo href="/" hx-push-url="/" { "> z9fr@blog:~$" }
                            }
                        }
                    }

                    br;
                    br;

                    .snowframe {
                        (content)
                    }
                    hr;
                    footer {
                        div hx-boost="true" hx-include="[name='bustCache']" hx-swap="innerHTML" hx-target=".snowframe" {
                            nav {
                                a href="/" hx-push-url="/" { "Home" }
                                " - "
                                a href="/blog" hx-push-url="/blog" { "Blog" }
                                " - "
                                a  href="/contact" hx-push-url="/contact" { "Contact" }
                                " - "
                                a href="/stack" hx-push-url="/stack" { "Uses" }
                            }
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

pub fn email_address() -> Markup {
    return html!(
        a href={"mailto:me@z9fr.xyz"} {"me@z9fr.xyz"}
    );
}

pub fn contact(links: &Vec<Link>, is_partial: bool) -> Markup {
    let markup = html! {
        h1 {"Contact Information"}

        br;
        br;

        .grid {
            .cell."-6of12" {
                h3 {"Email"}

                button."btn btn-default btn-ghost" hx-indicator="#spinner" hx-post="/email" hx-swap="outerHTML" {
                    "View email address"  span.loading id="spinner" style="display:none;"{}
                };

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
                    code {"z9fr"}
                    " Please note that Discord will automatically reject friend requests if you are not in a mutual server with me. I don't have control over this behavior."
                }
            }
        }
    };

    return if is_partial {
        markup
    } else {
        base(Some("Contact Information"), None, markup)
    };
}

pub fn stack(is_partial: bool) -> Markup {
    let markup = html! {
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
    };

    return if is_partial {
        markup
    } else {
        base(Some("Uses"), None, markup)
    };
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
