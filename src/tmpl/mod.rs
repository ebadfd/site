use chrono::prelude::*;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use termx::{default_help_short, registries::CommandRegistry};

use crate::{
    app::{Author, Link},
    post::Post,
    termx_registry::web::WebCommandRegistry,
};
use lazy_static::lazy_static;

pub mod blog;

lazy_static! {
    static ref CACHEBUSTER: String = uuid::Uuid::new_v4().to_string().replace('-', "");
    static ref TURNSTILE_SITE_KEY: String = String::from(
        std::env::var("TURNSTILE_SITE_KEY").expect("$TURNSTILE_SITE_KEY is not avaible")
    );
}

pub fn error(why: impl Render) -> Markup {
    base(
        Some("Error"),
        None,
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

pub fn full_screen_player(video_url: String) -> Markup {
    base(
        Some("Dash"),
        None,
        None,
        html!(
            link rel="stylesheet" href={"/static/css/controlbar.css?bustCache=" (*CACHEBUSTER)};

            script src="https://cdnjs.cloudflare.com/ajax/libs/dashjs/4.7.4/dash.all.debug.min.js" {}
            script async src="/static/rr.js" defer {}

            data #"video-url" value=(video_url)  {};

            div."fs-player" {
                video preload="auto" autoplay="" {};

                div #videoController .video-controller.unselectable {

                    div."btn-play-pause" #playPauseBtn title="Play/Pause" {
                        i."fa-solid"."fa-play" #iconPlayPause {};
                    }

                    span."time-display" #videoTime {"00:00:00"};

                    div."btn-fullscreen"."control-icon-layout" #fullscreenBtn title="Fullscreen" {
                        i."fullscreen-actions"."fa-solid"."fa-expand" {};
                    }

                    div."control-icon-layout" #bitrateListBtn title="Bitrate List" {
                        i."fa-solid"."fa-signal" {};
                    }

                    input.volumebar #volumebar type="range" value="1" min="0" max="1" step=".01" {};

                    div."btn-mute"."control-icon-layout" #muteBtn  title="Mute" {
                        i."fa-solid"."fa-volume-high" #iconMute {};
                    }

                    div."control-icon-layout" #trackSwitchBtn title="A/V Tracks"{
                        i."fa-solid"."fa-shuffle" {};
                    }

                    div."btn-caption"."control-icon-layout" #captionBtn title="Closed Caption" {
                        i."fa-solid"."fa-closed-captioning" {};
                    }

                    span."duration-display" #videoDuration {"00:00:00"};

                    div.seekContainer {
                        div.seekbar."seekbar-complete" #seekbar {
                            div.seekbar."seekbar-buffer"  #seekbar-buffer;
                            div.seekbar."seekbar-play" #seekbar-play ;
                        }
                    }

                    div #"thumbnail-container"."thumbnail-container" {
                        div #"thumbnail-elem" ."thumbnail-elem" {};
                        div #"thumbnail-time-label" ."thumbnail-time-label" {};
                    }
                }
            }
        ),
    )
}

pub fn index(author: &Author, posts: &Vec<Post>, domain: &str, is_partial: bool) -> Markup {
    let today = Utc::now().date_naive();
    let og_tags = html! {
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
    };

    let markup = html! {
        .content {
            p {"I'm Dasith, Security Researcher and Hobbyist Programmer"}

            p {"Software enginer at Surge.global. My main interests revolve around Computers, History, Philosophy and Anime."}

            h2 class="baffle" { "Recent Articles" }

            ul preload{
                @for post in posts.iter().take(5).filter(|p| today.num_days_from_ce() >= p.date.num_days_from_ce()) {
                    li {
                        (post.detri())
                            " - "
                            a href={ @if post.front_matter.redirect_to.as_ref().is_some() {(post.front_matter.redirect_to.as_ref().unwrap())} @else {"/" (post.link)}} { (post.front_matter.title) }
                        }
                    }
            }

            h2 class="baffle" { "Quick Links" }
            ul {
                li {a href={"https://github.com/" (author.github)} rel="me" {"GitHub"}}
                li {a href={"https://twitter.com/" (author.twitter)} rel="me" {"X formerly twitter"}}
            }
        }
    };

    return if is_partial {
        markup
    } else {
        base(None, None, Some(og_tags), markup)
    };
}

pub fn base(
    title: Option<&str>,
    styles: Option<&str>,
    og_tags: Option<Markup>,
    content: Markup,
) -> Markup {
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

                @if let Some(og_tags) = og_tags {
                    (og_tags)
                }

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
                script src="https://challenges.cloudflare.com/turnstile/v0/api.js" async defer {};
                script src="https://js.sentry-cdn.com/5f4957f42fb5c2d26f0ad04867411b64.min.js" async defer{};
                script async src="/static/baffle.min.js" defer {}
                script async src="/static/script.js" defer {}

                script src="https://kit.fontawesome.com/eabf947950.js" crossorigin="anonymous" {};

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
                            div hx-boost="true" hx-swap="innerHTML" hx-target=".snowframe" {
                                a.logo href="/" hx-push-url="/" { "> z9fr@blog:~$" }
                                input."hidden-input".hack hx-post="/termx" hx-trigger="keyup[keyCode==13]" name="cmd" {}
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

                button."termx-open btn btn-default btn-ghost" hx-get="/termx" hx-trigger="click" hx-swap="innerHTML" hx-target="#termx" {
                    i."fa-solid"."fa-terminal" {};
                }

                div #"termx"."card" style="display: none" {}
            }
        }
    }
}

pub fn email_address(validate: bool) -> Markup {
    if validate {
        return html!(
            a href={"mailto:me@z9fr.xyz"} {"me@z9fr.xyz"}
        );
    }

    return html!(
        p {"CAPTCHA verification failed. please try again"}
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

                form action="/email" hx-post="/email" hx-swap="outerHTML" {
                    div class="cf-turnstile" data-sitekey={(*TURNSTILE_SITE_KEY)} {};
                    button."btn btn-default btn-ghost"  type="submit" {
                        "View email address"
                    };
                };

                br;

                small {
                    "If the captcha is not working, please refresh the page."
                }

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
        base(Some("Contact Information"), None, None, markup)
    };
}

pub fn stack(is_partial: bool) -> Markup {
    let markup = html! {
         script src="https://unpkg.com/website-carbon-badges@1.1.3/b.min.js" defer {}

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

        div #"wcb" ."carbonbadge wcb-d" {};
    };

    return if is_partial {
        markup
    } else {
        base(Some("Uses"), None, None, markup)
    };
}

pub fn not_found(path: impl Render) -> Markup {
    base(
        Some("Not found"),
        None,
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

pub fn termx_default() -> Markup {
    let default_help = default_help_short();
    html!(
            header."card-header" {
                span."resize-notch" #"resizeHandle" {}
                div."grid -stretch" {
                    p."cloud-shell-title cell" { "cloud shell" }

                    div."cell" {
                        div."grid -right" {
                            i."fa-solid"."fa-x" {};
                        }
                    }
                }
            }
            div."card-content" {
                div #"termx-results" {
                    span."code" { (default_help)}
                }

                div style="display: flex;" {
                    span."user"{ (format!("{}@{}", WebCommandRegistry::get_user_name(), WebCommandRegistry::get_hostname()))} {}
                    span."path"{"$"}

                    input id="terminal-input" autofocus hx-post="/termx" hx-trigger="keyup[keyCode==13]"
                    name="cmd" hx-swap="beforeend scroll:bottom" hx-target="#termx-results"
                    hx-on-htmx-after-request="this.value = ''" {}
                }
            }
    )
}
