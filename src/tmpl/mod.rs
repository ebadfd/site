use crate::{
    app::{Author, Link},
    post::Post,
};
use chrono::prelude::*;
use lazy_static::lazy_static;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};

pub mod blog;

lazy_static! {
    static ref CACHEBUSTER: String = uuid::Uuid::new_v4().to_string().replace('-', "");
    static ref TURNSTILE_SITE_KEY: String =
        std::env::var("TURNSTILE_SITE_KEY").expect("$TURNSTILE_SITE_KEY is not avaible");
}

/// Name of the site, as it appears in the tab title and in social cards.
pub const SITE_TITLE: &str = "ebadfd explorations";

/// Canonical origin. `base()` has no access to the loaded config, and the feed
/// and canonical links here were already hardcoding this host in four places.
pub const SITE_URL: &str = "https://ebadfd.tech";

/// Search-result snippet for the homepage. Kept under ~160 characters, which is
/// roughly where Google truncates.
pub const DESCRIPTION: &str =
    "Malware teardowns, phishing research and reverse engineering notes from \
     ebadfd — plus programming, cars, and whatever else holds my attention.";

/// Controls which third-party resources `base()` loads on a given page.
#[derive(Default)]
pub struct PageFeatures {
    pub flickity: bool,
    pub turnstile: bool,
    pub fontawesome: bool,
}

/// How a page's `<title>` is built.
///
/// Google truncates around 60 characters, and the site-name suffix costs 22 of
/// them. Section pages can afford it; post titles are long enough on their own
/// that adding it pushed several over the limit.
#[derive(Clone, Copy)]
pub enum PageTitle<'a> {
    /// The site name alone — the homepage.
    Site,
    /// `<title>` is `{0} - {SITE_TITLE}`.
    Suffixed(&'a str),
    /// `<title>` is `{0}` verbatim.
    Bare(&'a str),
}

impl PageTitle<'_> {
    fn render(&self) -> String {
        match self {
            PageTitle::Site => SITE_TITLE.to_string(),
            PageTitle::Suffixed(t) => format!("{t} - {SITE_TITLE}"),
            PageTitle::Bare(t) => (*t).to_string(),
        }
    }
}

/// Out-of-band `<title>` swap so htmx partials update the browser tab title.
/// Must build the title the same way `base()` does, or a boosted navigation
/// would set a different title than a full load of the same URL.
pub fn oob_title(title: PageTitle) -> Markup {
    html! {
        title hx-swap-oob="true" { (title.render()) }
    }
}

/// How a nav item relates to the page currently being viewed.
///
/// The distinction matters to screen readers: `page` means "this link points at
/// the document you are on", `true` means "you are somewhere inside this
/// section" — which is the honest answer for `Blog` while reading `/blog/foo`.
/// Both get the same visual treatment.
pub fn nav_state(href: &str, current_path: &str) -> Option<&'static str> {
    if current_path == href {
        Some("page")
    } else if href != "/" && current_path.starts_with(&format!("{href}/")) {
        Some("true")
    } else {
        None
    }
}

/// Render a boosted nav link, marked with `aria-current` when it is the
/// section or page being viewed.
fn nav_link(label: &str, href: &str, current_path: &str) -> Markup {
    html! {
        a href=(href) hx-push-url=(href) aria-current=[nav_state(href, current_path)] { (label) }
    }
}

pub fn error(why: impl Render) -> Markup {
    base(
        PageTitle::Suffixed("Error"),
        None,
        None,
        "/",
        PageFeatures::default(),
        html! {
            h1 {"Error"}

            pre {
                (why)
            }

            p {
                "You could try to "
                a href="/" {"go home"}
                " or "
                a href="https://github.com/ebadfd/site/issues/new" {"report this issue"}
                " so it can be fixed."
            }
        },
    )
}

pub fn full_screen_player(video_url: String) -> Markup {
    base(
        PageTitle::Suffixed("Dash"),
        None,
        None,
        "/rr",
        PageFeatures {
            fontawesome: true,
            ..Default::default()
        },
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

pub fn index(
    author: &Author,
    posts: &[Post],
    products: &[Post],
    domain: &str,
    is_partial: bool,
) -> Markup {
    let today = Utc::now().date_naive();
    let home = format!("https://{domain}/");
    let image = format!("https://{domain}/static/img/avatar.jpeg");

    // `description` is what Google prints under the result for the root URL, and
    // it used to be the author's job title — two words, no context. og:site_name
    // was the job title too, where it should be the name of the site.
    let og_tags = html! {
        link rel="canonical" href=(home);

        meta name="twitter:card" content="summary";
        meta name="twitter:site" content=(author.twitter);
        meta name="twitter:title" content=(SITE_TITLE);
        meta name="twitter:description" content=(DESCRIPTION);
        meta name="twitter:image" content=(image);

        meta property="og:type" content="website";
        meta property="og:title" content=(SITE_TITLE);
        meta property="og:description" content=(DESCRIPTION);
        meta property="og:site_name" content=(SITE_TITLE);
        meta property="og:url" content=(home);
        meta property="og:image" content=(image);
        meta property="og:locale" content="en_US";

        meta name="description" content=(DESCRIPTION);
        meta name="author" content=(author.name);

        // Ties the domain to a person so search engines can associate the site
        // with the profiles in `sameAs`.
        script type="application/ld+json" {
            (PreEscaped(serde_json::json!({
                "@context": "https://schema.org",
                "@type": "WebSite",
                "name": SITE_TITLE,
                "url": home,
                "description": DESCRIPTION,
                "author": {
                    "@type": "Person",
                    "name": author.name,
                    "url": home,
                    "sameAs": author.same_as,
                },
            }).to_string()))
        }
    };

    let markup = html! {
        .content {
            h1 .visually-hidden { (author.name) }

            p {"I'm a Security Researcher and Hobbyist Programmer"}

            p {"My main interests revolve around Computers, History, Philosophy, Anime, and Cars."}

            h2 { "Recent Articles" }

            ul preload{
                @for post in posts.iter().take(5).filter(|p| today.num_days_from_ce() >= p.date.num_days_from_ce()) {
                    li {
                        (post.detri())
                            " - "
                            a href={ @if post.front_matter.redirect_to.as_ref().is_some() {(post.front_matter.redirect_to.as_ref().unwrap())} @else {"/" (post.link)}} { (post.front_matter.title) }
                        }
                    }
            }

            h2 { "Products" }

            ul preload{
                @for product in products.iter().take(5).filter(|p| today.num_days_from_ce() >= p.date.num_days_from_ce()) {
                    li {
                        (product.detri())
                            " - "
                            a href={ @if product.front_matter.redirect_to.as_ref().is_some() {(product.front_matter.redirect_to.as_ref().unwrap())} @else {"/" (product.link)}} { (product.front_matter.title) }
                        }
                    }
            }


            h2 { "Quick Links" }
            ul {
                li {a href={"https://github.com/" (author.github)} rel="me" {"GitHub"}}
                li {a href={"https://twitter.com/" (author.twitter)} rel="me" {"X formerly twitter"}}
                li {a href={"https://helm.ebadfd.tech/"} rel="repository" {"ebadfd Helm Repository"}}
            }
        }
    };

    if is_partial {
        html! { (oob_title(PageTitle::Site)) (markup) }
    } else {
        base(
            PageTitle::Site,
            None,
            Some(og_tags),
            "/",
            PageFeatures::default(),
            markup,
        )
    }
}

pub fn base(
    title: PageTitle,
    styles: Option<&str>,
    og_tags: Option<Markup>,
    current_path: &str,
    features: PageFeatures,
    content: Markup,
) -> Markup {
    let now = Utc::now();

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                title { (title.render()) }
                meta name="viewport" content="width=device-width, initial-scale=1.0";

                // Pages that build their own metadata (the homepage and posts)
                // set their canonical there; this covers the section pages,
                // which had none at all.
                @if og_tags.is_none() {
                    link rel="canonical" href={(SITE_URL) (current_path)};
                }

                // Preconnect to third-party origins so DNS + TLS happen early.
                link rel="preconnect" href="https://unpkg.com" crossorigin {};
                link rel="preconnect" href="https://cdnjs.cloudflare.com" crossorigin {};
                link rel="preconnect" href="https://challenges.cloudflare.com" crossorigin {};
                link rel="preconnect" href="https://js.sentry-cdn.com" crossorigin {};
                link rel="preconnect" href="https://kit.fontawesome.com" crossorigin {};

                @if let Some(og_tags) = og_tags {
                    (og_tags)
                }

                meta name="apple-mobile-web-app-title" content="ebadfd explorations";
                meta name="application-name" content="ebadfd";

                link rel="manifest" href="/static/manifest.json";
                link rel="alternate" title=(SITE_TITLE) type="application/rss+xml" href={(SITE_URL) "/blog.rss"};
                link rel="alternate" title=(SITE_TITLE) type="application/json" href={(SITE_URL) "/blog.json"};

                link rel="icon" type="image/png" href="/static/favicon/favicon-96x96.png" sizes="96x96";
                link rel="icon" type="image/svg+xml" href="/static/favicon/favicon.svg";
                link rel="shortcut icon" href="/static/favicon/favicon.ico";
                link rel="apple-touch-icon" sizes="180x180" href="/static/favicon/apple-touch-icon.png";

                link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/hack/0.8.1/hack.css";
                link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/hack/0.8.1/dark-grey.css";

                link rel="stylesheet" href={"/static/css/styles.css?bustCache=" (*CACHEBUSTER)};
                link rel="stylesheet" href={"/static/css/progress-bar.css?bustCache=" (*CACHEBUSTER)};

                // Flickity — only on pages with carousels.
                @if features.flickity {
                    link rel="stylesheet" href="https://unpkg.com/flickity@2/dist/flickity.min.css";
                    script src="https://unpkg.com/flickity@2/dist/flickity.pkgd.min.js" async defer {};
                }

                script src="https://unpkg.com/htmx.org@1.9.10" integrity={"sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC"} crossorigin={"anonymous"} {}
                // Pinned + SRI-verified preload extension.
                script src="https://unpkg.com/htmx.org@1.9.10/dist/ext/preload.js" integrity="sha384-NoiTdF/KjxH4wa6OHMdM4F6e/LIPNMcFBgOIINTTaczWo0vX1mWTOzckySqlGQS/" crossorigin="anonymous" {}

                // Turnstile — only on pages with forms.
                @if features.turnstile {
                    script src="https://challenges.cloudflare.com/turnstile/v0/api.js?render=explicit&onload=onTurnstileLoad" async defer {};
                }

                script src="https://js.sentry-cdn.com/5f4957f42fb5c2d26f0ad04867411b64.min.js" async defer{};
                // Modules are deferred by definition, so this runs after the htmx tag above
                // has executed — which matters, it calls htmx.on() at import time.
                script type="module" src={"/static/js/main.js?bustCache=" (*CACHEBUSTER)} {}

                // FontAwesome — only on pages that use icons.
                @if features.fontawesome {
                    script src="https://kit.fontawesome.com/eabf947950.js" crossorigin="anonymous" {};
                }

                @match now.month() {
                    // TODO Test this out
                    12|1|2 => {
                       link rel="stylesheet" href={"/static/css/snow.css?bustCache=" (*CACHEBUSTER)};
                    }
                   _ => {},
                }

                @if let Some(styles) = styles {
                    style {
                        (PreEscaped(styles))
                    }
                }
            }

            body.snow.hack.dark-grey hx-ext="preload" hx-indicator=".progress" {
                // Skip link — first tab stop on every page.
                a."skip-link" href="#content" { "Skip to content" }

                // Live region for page change announcements (htmx afterSwap).
                div #"page-announcer" aria-live="polite" aria-atomic="true" class="sr-only" {}

                div.progress style="height: 2px;"{
                    div.indeterminate style="background-color: #6aacac;"{}
                }

                input name="bustCache" value={(*CACHEBUSTER)} type="hidden" {}

                .container {
                    br;

                    header {
                        div."header-nav" hx-boost="true" hx-include="[name='bustCache']" hx-swap="innerHTML" hx-target=".snowframe" {
                            a."logo-link" hx-push-url="/" href="/" {
                                img src={"/static/logo-main.svg?bustCache=" (*CACHEBUSTER)} alt="ebadfd — home" style="max-width: 40%; height: auto;" {};
                            }
                            nav."primary-nav" aria-label="Primary" {
                                (nav_link("Blog", "/blog", current_path))
                                (nav_link("Products", "/products", current_path))
                                (nav_link("Contact", "/contact", current_path))
                                (nav_link("Uses", "/stack", current_path))
                            }
                        }
                    }

                    // tabindex="-1" so following the skip link actually moves
                    // focus here, not just the scroll position.
                    main #"content" tabindex="-1" {
                        .snowframe {
                            (content)
                        }
                    }


                    hr;
                    footer {
                        div hx-boost="true" hx-include="[name='bustCache']" hx-swap="innerHTML" hx-target=".snowframe" {
                            // Labelled so it is distinguishable from the primary
                            // nav when jumping between landmarks.
                            nav aria-label="Footer" {
                                (nav_link("Home", "/", current_path))
                                " - "
                                (nav_link("Blog", "/blog", current_path))
                                " - "
                                (nav_link("Products", "/products", current_path))
                                " - "
                                (nav_link("Contact", "/contact", current_path))
                                " - "
                                (nav_link("Uses", "/stack", current_path))
                                " - "
                                (nav_link("Privacy Policy", "/privacy-policy", current_path))
                            }
                        }
                    }
                }

                // The emulator is fetched once and then kept alive for the life of the page;
                // script.js intercepts later clicks and just un-hides the panel.
                button."termx-open btn btn-default btn-ghost" hx-get="/termx" hx-trigger="click once" hx-swap="innerHTML" hx-target="#termx" aria-label="Open Arch Linux terminal" title="Open Arch Linux terminal (Ctrl + `)" {
                    "tty"
                }

                div #"termx"."card" hidden role="dialog" aria-label="Arch Linux terminal" {}
            }
        }
    }
}

pub fn support(is_partial: bool) -> Markup {
    let markup = html! {
        h1 { "Support" }

        .grid {
            .cell."-12of12" {
                p {
                    "For support, please contact "
                    a href="mailto:help@ebadfd.tech" { "help@ebadfd.tech" }
                    "."
                }
            }
        }
    };

    if is_partial {
        html! { (oob_title(PageTitle::Suffixed("Support"))) (markup) }
    } else {
        base(
            PageTitle::Suffixed("Support"),
            None,
            None,
            "/support",
            PageFeatures::default(),
            markup,
        )
    }
}

pub fn email_address(validate: bool) -> Markup {
    if validate {
        return html!(
            a href={"mailto:root@ebadfd.tech"} {"root@ebadfd.tech"}
        );
    }

    html!(
        p {"CAPTCHA verification failed. please try again"}
    )
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

    if is_partial {
        html! { (oob_title(PageTitle::Suffixed("Contact Information"))) (markup) }
    } else {
        base(
            PageTitle::Suffixed("Contact Information"),
            None,
            None,
            "/contact",
            PageFeatures {
                turnstile: true,
                ..Default::default()
            },
            markup,
        )
    }
}

pub fn privacy_policy(is_partial: bool) -> Markup {
    let markup = html! {
        h1 { "Privacy Policy" }

        .grid {
            .cell."-12of12" {
                p {
                    "Your privacy is very important to us. We want to be transparent about what information we collect and how we use it."
                }

                h3 { "Information We Collect" }
                p {
                    "We do not collect any personal information from visitors."
                }
                p {
                    "However, we may collect anonymous data for metrics such as page visits and general geographic information like country location. This data is used solely to improve the website experience and is not linked to any individual."
                }

                h3 { "How We Use This Information" }
                p {
                    "The anonymous metrics we collect help us understand website usage patterns so we can improve performance and content."
                }

                h3 { "Cookies and Tracking" }
                p {
                    "Our website may use cookies or similar technologies only for anonymous analytics purposes. No personal data is stored or tracked."
                }

                h3 { "Third-Party Services" }
                p {
                    "We do not share any personal information with third parties. Any third-party services we use for analytics operate on anonymized data only."
                }

                h3 { "Your Rights" }
                p {
                    "Since we do not collect personal information, there are no personal data rights applicable. If you have questions, please contact us."
                }

                h3 { "Changes to This Policy" }
                p {
                    "We may update this privacy policy occasionally. Changes will be posted on this page with an updated effective date."
                }

                h3 { "Contact Us" }
                p {
                    "If you have any questions about this policy, please reach out to us via the contact page."
                }

                hr;

                p {
                    "This privacy policy is licensed under the "
                    a href="https://creativecommons.org/licenses/by/4.0/" target="_blank" { "Creative Commons Attribution 4.0 International License (CC BY 4.0)" }
                    ". You are free to share and adapt it, provided you give appropriate credit."
                }
            }
        }
    };

    if is_partial {
        html! { (oob_title(PageTitle::Suffixed("Privacy Policy"))) (markup) }
    } else {
        base(
            PageTitle::Suffixed("Privacy Policy"),
            None,
            None,
            "/privacy-policy",
            PageFeatures::default(),
            markup,
        )
    }
}

pub fn stack(is_partial: bool) -> Markup {
    let markup = html! {
         script src="https://unpkg.com/website-carbon-badges@1.1.3/b.min.js" defer {}

         h1 {"Uses"}
         ul {
             li {
                 "Built on " a target="_blank" href={"https://github.com/tokio-rs/axum"} {"axum"}
             }

             li {
                 a target="_blank" href={"https://tokio.rs/"} {"tokio.rs"} " as the asynchronous runtime."
             }

             li {
                 a target="_blank" href="https://hackcss.egoist.dev" {"hackcss"}; " as the css framework"
             }

             li {
                 "Markdown rendering with " a target="_blank" href="https://docs.rs/comrak" {"cmark"};"."
             }

             li {
                 a target="_blank" href="https://docs.rs/syntect" {"Syntect"}; " for Syntax Highlighting."
             }

             li {
                 "Inspired by " a target="_blank" href="https://github.com/Xe/site" {"Xe/site"}; "."
             }
        }

        div #"wcb" ."carbonbadge wcb-d" {};
    };

    if is_partial {
        html! { (oob_title(PageTitle::Suffixed("Uses"))) (markup) }
    } else {
        base(
            PageTitle::Suffixed("Uses"),
            None,
            None,
            "/stack",
            PageFeatures::default(),
            markup,
        )
    }
}

pub fn not_found(path: impl Render) -> Markup {
    base(
        PageTitle::Suffixed("Not found"),
        None,
        None,
        "/",
        PageFeatures::default(),
        html! {
            h1 {"Not found"}
            p {
                "The path at "
                code {(path)}
                " could not be found. If you expected this path to exist, please "
                a target="_blank" href="https://github.com/ebadfd/site/issues/new" {"report this issue"}
                " so it can be fixed."
            }
        },
    )
}

pub fn termx_v86() -> Markup {
    let src = format!("/static/v86/term.html?bustCache={}", *CACHEBUSTER);

    html!(
        header."termx-bar" {
            span."resize-notch" #"resizeHandle" title="Drag to resize" {}
            span."termx-title" { "arch linux — v86" }
            span."termx-hint" { "Esc Esc or Ctrl+` to close" }
            button #"termx-close" type="button" aria-label="Close terminal" title="Close (Esc Esc)" { "[x]" }
        }
        div."card-content v86-content" {
            iframe #"v86-frame"
                title="Sandboxed local Arch Linux v86 emulator"
                src=(src)
                sandbox="allow-scripts allow-pointer-lock"
                referrerpolicy="no-referrer" {}
        }
    )
}
