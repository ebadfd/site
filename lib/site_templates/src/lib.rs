use maud::{html, Markup};

pub fn yt_video(id: String) -> Markup {
    let url = format!(
        "https://www.youtube-nocookie.com/embed/{}?amp;controls=1&iv_load_policy=3",
        id
    );
    html! {
        div.yt-video-container {
            iframe src={(url)} frameborder="0"
                style=".ytp-watermark"
                title="YouTube video player"
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                allowfullscreen{

                }
        }
    }
}

pub fn carousel(cells: Vec<String>) -> Markup {
    html! {
        div class="main-carousel" data-flickity="{ \"cellAlign\": \"left\", \"contain\": true, \"lazyLoad\": true, \"freeScroll\": true, \"pageDots\": false }" {
            @for cell in cells {
                div class="carousel-cell" {
                    img src=(cell) {}
                }
            }
        }
    }
}

pub fn video(url: String) -> Markup {
    html! {
        video.video-container preload="metadata" controls {
            source src=(url) type="video/mp4" {}
            "Your browser does not support the video tag."
        }
    }
}
