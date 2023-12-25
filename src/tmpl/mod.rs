use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::app::{Author, Link};

pub mod blog;

pub fn index(author: &Author, projects: &Vec<Link>) -> Markup {
    base(
        None,
        None,
        html! {
            link rel="authorization_endpoint" href="https://idp.christine.website/auth";
            link rel="canonical" href="https://xeiaso.net/";
            meta name="google-site-verification" content="rzs9eBEquMYr9Phrg0Xm0mIwFjDBcbdgJ3jF6Disy-k";

            meta name="twitter:card" content="summary";
            meta name="twitter:site" content="@theprincessxena";
            meta name="twitter:title" content=(author.name);
            meta name="twitter:description" content=(author.job_title);
            meta property="og:type" content="website";
            meta property="og:title" content=(author.name);
            meta property="og:site_name" content=(author.job_title);
            meta name="description" content=(author.job_title);
            meta name="author" content=(author.name);

            .grid {
                .cell."-3of12".content {
                    img src="/static/img/avatar.png" alt="My Avatar";
                    br;
                    a href="/contact" class="justify-content-center" { "Contact me" }
                }
                .cell."-9of12".content {
                    h1 {(author.name)}
                    h4 {(author.job_title)}
                    h5 { "Skills" }
                    ul {
                        li { "Go, Lua, Haskell, C, Rust and other languages" }
                        li { "Docker (deployment, development & more)" }
                        li { "Mashups of data" }
                        li { "kastermakfa" }
                    }

                    h5 { "Highlighted Projects" }
                    ul {
                        @for project in projects {
                            li {(project)}
                        }
                    }

                    h5 { "Quick Links" }
                    ul {
                        li {a href="https://github.com/Xe" rel="me" {"GitHub"}}
                        li {a href="https://twitter.com/theprincessxena" rel="me" {"Twitter"}}
                        li {a href="https://pony.social/@cadey" rel="me" {"Fediverse"}}
                        li {a href="https://www.patreon.com/cadey" rel="me" {"Patreon"}}
                    }

                    p {
                        "Looking for someone for your team? Check "
                        a href="/signalboost" { "here" }
                        "."
                    }
                }
            }
        },
    )
}

pub fn base(title: Option<&str>, styles: Option<&str>, content: Markup) -> Markup {
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
                @if let Some(styles) = styles {
                    style {
                        (PreEscaped(styles))
                    }
                }
            }
            body.snow.hack.gruvbox-dark {
                .container {
                    header {
                        span.logo {}
                        nav {
                            a href="/" { "z9fr" }
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
                            "copy right 2023"
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
                    }
                }
            }
        }
    }
}
